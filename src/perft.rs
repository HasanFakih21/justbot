use crate::board::Board;

pub fn perft(depth: usize, board: &mut Board, nodes_count: &mut usize) {
    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {  
            let divided_nodes = perft_divide(depth - 1, board);
            println!("{m}: {divided_nodes}");
            *nodes_count += divided_nodes;
            board.unmake_move();
        } 
    }
}

pub fn perft_divide(depth: usize, board: &mut Board) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {
            nodes += perft_divide(depth - 1, board);
            board.unmake_move();
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::{board::{Board, constants::STARTING_FEN}, perft::perft};

    #[test]
    fn test_perft() {
        let mut board = Board::from_fen(STARTING_FEN);
        println!("{board}");
        let mut nodes_count = 0;
        let clock = Instant::now();
        perft(6, &mut board, &mut nodes_count);
        println!("Number of nodes: {nodes_count}\nTime: {}s", clock.elapsed().as_secs()); 
    }
}