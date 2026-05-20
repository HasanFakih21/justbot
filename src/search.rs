use std::{cmp::Reverse, time::Instant};

use crate::{board::{Board, Square, moves::{Move, MoveKind, MoveList}}, transposition::{Entry, NodeType}};

pub fn search(depth: usize, board: &mut Board) -> Option<(Move, i32)> { 
    let mut max = -10000;
    let mut best_move: Option<(Move, i32)> = None;
    let mut total_nodes = 1;

    let clock = Instant::now();
    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {
            let mut nodes = 0;
            let score = -negamax(depth - 1, board, -10000, 10000, &mut nodes);
            total_nodes += nodes;
            println!("info nodes {total_nodes}");  
            let nps = total_nodes as f64 / clock.elapsed().as_secs_f64();
            println!("info nps {:.0}", nps);
            board.unmake_move();
            println!("{m}: {score}");
            if score >= max {
                max = score;
                best_move = Some((*m, score));
            }
        }
    }

    board.tt.add_entry(Entry::new(board.board_state.hash, best_move.unwrap().0, best_move.unwrap().1, NodeType::PV), board.board_state.hash);
    best_move
}

pub fn negamax(depth: usize, board: &mut Board, mut alpha: i32, beta: i32, nodes: &mut i32) -> i32 {
    if depth == 0 {
        return quiesce(board, alpha, beta, nodes); 
    }

    *nodes += 1;

    let mut legal_moves = 0;
    let mut max = -10000;
    for m in mvv_lva(board).iter() {
        if board.make_move(*m).is_ok() {
            legal_moves += 1;
            let score = -negamax(depth - 1, board, -beta, -alpha, nodes);
            board.unmake_move();
            if score > max {
                max = score;
                if score > alpha {alpha = score;}
            }
            if score >= beta {return max}
        }
    }

    if legal_moves == 0 {
        if board.is_king_in_attack(board.board_state.side_to_move) {
            return -9000 - depth as i32; 
        } else {
            return 0;
        }
    }

    max
}

pub fn quiesce(board: &mut Board, mut alpha: i32, beta: i32, nodes: &mut i32) -> i32 {
    let static_eval = board.evaluate();
    *nodes += 1;

    let mut best_value = static_eval;
    if best_value >= beta {
        return best_value;
    }
    if best_value > alpha {
        alpha = best_value;
    }

    for m in mvv_lva(board).iter() {
        if !m.get_kind().is_quiet() && board.make_move(*m).is_ok() {
            let score = -quiesce(board, -beta, -alpha, nodes);
            board.unmake_move();

            if score >= beta {return score}
            if score > best_value {best_value = score}
            if score > alpha {alpha = score}
        }
    }

    best_value
}

pub fn mvv_lva(board: &mut Board) -> MoveList {
    let move_list = board.generate_all_moves();
    //We want to sort the moves based on most valuable victim / least valuable attacker
    //Sort captures based on (attacked piece value - attacking piece value)
    let (mut captures, mut others): (Vec<Move>, Vec<Move>) = move_list.into_iter().partition(|m| m.get_kind().is_capture());

    captures.sort_by_key(|m| {
        let attacker = board.get_piece_at_square(m.get_from()).unwrap().1;
        let victim = match m.get_kind() {
            MoveKind::EnPassant => board.get_piece_at_square(Square::from(m.get_to() as usize ^ 8)).unwrap().1,
            _ => board.get_piece_at_square(m.get_to()).unwrap().1,
        };
        Reverse(victim.value() - attacker.value()) 
    });

    captures.append(&mut others);
    MoveList(captures)
}


#[cfg(test)]
mod tests {
    use crate::{board::{Board, Square, constants::STARTING_FEN, moves::{Move, MoveKind}}, search::{search, mvv_lva}};

    #[test]
    fn test_negamax() {
        let mut board = Board::from_fen(STARTING_FEN); 
        let best_move = search(5, &mut board);
        if let Some(m) = best_move {
            println!("Best move: {}", m.0);
        }
    }

    #[test]
    fn test_mvv_lva() {
        let mut board = Board::from_fen("rnbqkb1r/pp3p2/4pnpp/1p1p2N1/1Q1P4/BP2P3/P1PN1PPP/R3K2R b KQkq - 0 1"); 
        let move_list = mvv_lva(&mut board);

        for m in move_list.iter() {
            print!("{m}, ");
        }
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(*first_move, Move::new(Square::F8, Square::B4, MoveKind::Capture));

        let mut board = Board::from_fen("rnbq1rk1/pN1p1ppp/4n2b/2p1p3/N1BP3R/2P2Q2/PP3PPP/2B1K2R w K - 0 1"); 
        let move_list = mvv_lva(&mut board);

        for m in move_list.iter() {
            print!("{m}, ");
        }
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(*first_move, Move::new(Square::B7, Square::D8, MoveKind::Capture));
    }
}