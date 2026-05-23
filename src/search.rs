use std::{cmp::Reverse, time::Instant};

use crate::board::Board;
use crate::board::movegen::MoveGenKind;
use crate::search::data::{SearchData, SearchKind};
use crate::types::*;

pub mod data;

impl Board {
    pub fn detect_repetitions(&self) -> usize {
        let half_moves = self.board_state.half_move_clock as usize;
        let mut count = 0;

        let last_halfmove_ply = self.game_history.len() - half_moves;
        for position in self.game_history[last_halfmove_ply..].iter() {
            if self.board_state.hash == *position {
                count += 1
            }
        }

        count
    }
}

pub fn search_runner(board: &mut Board, kind: SearchKind) -> Option<(Move, i32)> {
    let mut depth = 1;
    let mut data = SearchData::new(kind);

    //Initialize with move from first depth
    println!("info depth {depth}");
    let mut best_move = search(&mut data, depth, board);
    depth += 1;

    //Iterative Deepening
    loop {
        println!("info depth {depth}");
        let deeper_move = search(&mut data, depth, board);
        depth += 1;

        if data.over_limit() {
            println!("Searched for {}ms", data.elapsed().as_millis());
            println!("Time limit was {}", data.get_time_limit());
            break;
        }

        best_move = deeper_move;
    }

    best_move
}

pub fn search(data: &mut SearchData, depth: usize, board: &mut Board) -> Option<(Move, i32)> {
    //Root Search
    let mut best_score = -10000;
    let mut best_move: Option<(Move, i32)> = None;
    let mut total_nodes = 1;
    let ply = 1;

    let clock = Instant::now();
    for m in order_moves(board).iter() {
        if board.make_move(*m).is_ok() {
            let mut nodes = 0;

            let score = -negamax(data, depth - 1, board, -10000, 10000, &mut nodes, ply + 1);
            total_nodes += nodes;
            println!("info nodes {total_nodes}");
            let nps = total_nodes as f64 / clock.elapsed().as_secs_f64();
            println!("info nps {:.0}", nps);
            board.unmake_move();
            println!("{m}: {score}");
            if score >= best_score {
                best_score = score;
                best_move = Some((*m, score));
            }
        }
    }

    board.tt.add_entry(
        best_move.unwrap().0,
        best_move.unwrap().1,
        Bound::Exact,
        board.board_state.hash,
        ply
    );
    best_move
}

pub fn negamax(
    data: &mut SearchData,
    depth: usize,
    board: &mut Board,
    mut alpha: i32,
    beta: i32,
    nodes: &mut i32,
    ply: u8,
) -> i32 {
    if depth == 0 {
        return quiesce(board, alpha, beta, nodes, ply); //Horizon Node
    }

    *nodes += 1;

    if board.board_state.half_move_clock > 4 {
        //We need to check history if positions were repeated only for the side to move.
        let count = board.detect_repetitions();
        if count >= 2 {
            return 0;
        }
    }

    let mut legal_moves = 0;
    let mut best_score = -10000;
    let mut best_move: Option<Move> = None;
    let mut bound = Bound::Upper; //Fail-high means score is atleast this good so lower-bound/Fail-low means the score is an upper bound

    for m in order_moves(board).iter() {
        if board.make_move(*m).is_ok() {
            legal_moves += 1;
            let mut score;

            if legal_moves == 1 {
                //First Move
                score = -negamax(data, depth - 1, board, -beta, -alpha, nodes, ply + 1);
            } else {
                score = -negamax(data, depth - 1, board, -alpha - 1, -alpha, nodes, ply + 1);
                if score > alpha && score < beta {
                    score = -negamax(data, depth - 1, board, -beta, -alpha, nodes, ply + 1); //We want to search again
                }
            }

            board.unmake_move();

            if score > alpha {
                bound = Bound::Exact;
                alpha = score;
            }

            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }

            if score >= beta || data.over_limit() {
                if let Some(m) = best_move {
                    board
                        .tt
                        .add_entry(m, best_score, Bound::Upper, board.board_state.hash, ply);
                }
                return best_score;
            }
        }
    }

    if legal_moves == 0 {
        if board.is_king_in_attack(board.board_state.side_to_move) {
            return -9000 - depth as i32;
        } else {
            return 0;
        }
    }

    if let Some(m) = best_move {
        board
            .tt
            .add_entry(m, best_score, bound, board.board_state.hash, ply);
    }

    best_score
}

pub fn quiesce(board: &mut Board, mut alpha: i32, beta: i32, nodes: &mut i32, ply: u8) -> i32 {
    let static_eval = board.evaluate();
    *nodes += 1;

    if board.board_state.half_move_clock > 4 {
        //We need to check history if positions were repeated only for the side to move.
        let count = board.detect_repetitions();
        if count >= 2 {
            return 0;
        }
    }

    let mut best_score = static_eval;
    let mut bound = Bound::Upper;
    let mut best_move: Option<Move> = None;

    if best_score >= beta {
        return best_score;
    }
    if best_score > alpha {
        alpha = best_score;
    }

    for m in order_moves(board).iter() {
        if !m.get_kind().is_quiet() && board.make_move(*m).is_ok() {
            let score = -quiesce(board, -beta, -alpha, nodes, ply + 1);
            board.unmake_move();

            if score >= beta {
                if let Some(m) = best_move {
                    board
                        .tt
                        .add_entry(m, best_score, Bound::Lower, board.board_state.hash, ply);
                }
                return score;
            }
            if score > best_score {
                best_score = score;
                best_move = Some(*m);
            }
            if score > alpha {
                alpha = score;
                bound = Bound::Exact;
            }
        }
    }

    if let Some(m) = best_move {
        board
            .tt
            .add_entry(m, best_score, bound, board.board_state.hash, ply);
    }

    best_score
}

pub fn order_moves(board: &mut Board) -> MoveList {
    //We want to sort the moves based on most valuable victim / least valuable attacker
    //Sort captures based on (attacked piece value - attacking piece value)
    let mut full_list = MoveList::new();
    let mut captures: MoveList = board.generate_moves(MoveGenKind::Captures);
    let mut best_move: Option<Move> = None;

    //Want to add the best move from the transposition table if it exists to the beginning of the list
    if let Some(e) = board.tt.get_entry(board.board_state.hash)
        && board.board_state.hash == e.get_key()
    {
        let bm = e.get_best_move();
        full_list.push(bm);
        best_move = Some(bm);
    }

    captures.iter_mut().into_slice().sort_by_key(|m| {
        let attacker = board.get_piece_at_square(m.get_from()).unwrap().1;
        let victim = match m.get_kind() {
            MoveKind::EnPassant => {
                board
                    .get_piece_at_square(Square::from(m.get_to() as usize ^ 8))
                    .unwrap()
                    .1
            }
            _ => board.get_piece_at_square(m.get_to()).unwrap().1,
        };
        Reverse(victim.value() - attacker.value())
    });

    let quiet = board.generate_moves(MoveGenKind::Quiet);
    let pawn_promos = board.generate_moves(MoveGenKind::NonCapturePromotions);

    for m in captures
        .iter()
        .chain(pawn_promos.iter())
        .chain(quiet.iter())
    {
        if let Some(bm) = best_move
            && *m == bm
        {
            continue;
        }
        full_list.push(*m);
    }

    full_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn test_search() {
        let mut board = Board::from_fen(STARTING_FEN);
        let mut data = SearchData::default();
        let best_move = search(&mut data, 5, &mut board);
        if let Some(m) = best_move {
            println!("Best move: {}", m.0);
        }
    }

    #[test]
    fn test_order_moves() {
        let mut board = Board::from_fen(STARTING_FEN);
        let move_list = order_moves(&mut board);
        println!("{board}");
        println!("{move_list}");
        println!();

        let mut board =
            Board::from_fen("rnbqkb1r/pp3p2/4pnpp/1p1p2N1/1Q1P4/BP2P3/P1PN1PPP/R3K2R b KQkq - 0 1");
        let move_list = order_moves(&mut board);

        println!("{board}");
        println!("{move_list}");
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(
            *first_move,
            Move::new(Square::F8, Square::B4, MoveKind::Capture)
        );

        let mut board =
            Board::from_fen("rnbq1rk1/pN1p1ppp/4n2b/2p1p3/N1BP3R/2P2Q2/PP3PPP/2B1K2R w K - 0 1");
        let move_list = order_moves(&mut board);

        println!("{board}");
        for m in move_list.iter() {
            print!("{m}, ");
        }
        println!();

        let first_move = move_list.iter().next().unwrap();
        assert_eq!(
            *first_move,
            Move::new(Square::B7, Square::D8, MoveKind::Capture)
        );
    }

    #[test]
    fn test_repetion_detection() {
        use MoveKind::*;
        use Square::*;

        let mut board = Board::from_fen("8/6K1/3N4/8/5Q2/8/1kr5/8 w - - 0 1");
        let _ = board.make_move(Move::new(F4, E4, QuietMove));
        let _ = board.make_move(Move::new(C2, C1, QuietMove));
        let _ = board.make_move(Move::new(E4, F4, QuietMove));
        let _ = board.make_move(Move::new(C1, C2, QuietMove));
        let _ = board.make_move(Move::new(F4, E4, QuietMove));
        let _ = board.make_move(Move::new(C2, C1, QuietMove));
        let _ = board.make_move(Move::new(E4, F4, QuietMove));

        let mut data = SearchData::default();
        let (m, score) = search(&mut data, 3, &mut board).unwrap();
        println!(
            "{:?}\nCurrent Hash: {}",
            board.game_history, board.board_state.hash
        );
        println!("Repetions counted: {}", board.detect_repetitions());
        assert_eq!(score, 0);
        assert_eq!(m, Move::new(C1, C2, QuietMove));
    }

    #[test]
    fn test_mate_in_one() {
        let mut data = SearchData::default();
        let mut board =
            Board::from_fen("r1b4r/p1p1q3/1bppk3/4pp2/3PP1Q1/2P1R3/PP3PPP/RN4K1 w - - 0 18");
        let best_move = search(&mut data, 1, &mut board);
        println!("Best Move: {}", best_move.unwrap().0);
        assert_eq!(
            Move::new(Square::G4, Square::F5, MoveKind::Capture),
            best_move.unwrap().0
        );
    }

    #[test]
    fn test_mate_in_four() {
        let mut data = SearchData::default();
        let mut board = Board::from_fen("6k1/5pp1/5n1p/8/5P1q/2RQ3P/B5PK/8 b - - 0 36");
        let best_move = search(&mut data, 4, &mut board);
        println!("Best Move: {}", best_move.unwrap().0);
        assert_eq!(
            Move::new(Square::F6, Square::G4, MoveKind::QuietMove),
            best_move.unwrap().0
        );
    }
}
