use crate::board::{Board, moves::{Move, MoveList}};

pub fn best_move(depth: usize, board: &mut Board) -> Option<(Move, i32)> { 
    let mut max = -10000;
    let mut best_move: Option<(Move, i32)> = None;

    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {
            let score = -negamax(depth - 1, board, -10000, 10000);
            board.unmake_move();
            println!("{m}: {score}");
            if score >= max {
                max = score;
                best_move = Some((*m, score));
            }
        }
    }

    best_move
}

pub fn negamax(depth: usize, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return board.evaluate();
    }

    let mut legal_moves = 0;
    let mut max = -10000;
    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {
            legal_moves += 1;
            let score = -negamax(depth - 1, board, -beta, -alpha);
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

pub fn mvv_lva(board: &mut Board) -> MoveList {
    let move_list = board.generate_all_moves();
    //let attack_mask = board.get
    

    move_list
}

pub fn quiesce(board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
    let static_eval = board.evaluate();

    let mut best_value = static_eval;
    if best_value >= beta {
        return best_value;
    }
    if best_value > alpha {
        alpha = best_value;
    }

    for m in board.generate_all_moves().iter() {
        if !m.get_kind().is_quiet() && board.make_move(*m).is_ok() {
            let score = -quiesce(board, -beta, -alpha);
            board.unmake_move();

            if score >= beta {return score}
            if score > best_value {best_value = score}
            if score > alpha {alpha = score}
        }
    }

    best_value
}

#[cfg(test)]
mod tests {
    use crate::{board::{Board, constants::STARTING_FEN}, search::best_move};

    #[test]
    fn test_negamax() {
        let mut board = Board::from_fen(STARTING_FEN); 
        let best_move = best_move(5, &mut board);
        if let Some(m) = best_move {
            println!("Best move: {}", m.0);
        }
    }
}