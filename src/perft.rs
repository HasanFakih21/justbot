use crate::board::Board;

pub fn perft(depth: usize, board: &mut Board, nodes_count: &mut usize) {
    if depth == 0 {
        *nodes_count += 1;
        return;
    }

    for m in board.generate_all_moves().iter() {
        if board.make_move(*m).is_ok() {
            perft(depth - 1, board, nodes_count);
        };
        board.unmake_move();
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, perft::perft};

    #[test]
    fn test_perft() {
        let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
        let mut nodes_count = 0;
        perft(4, &mut board, &mut nodes_count);

        println!("Number of nodes: {nodes_count}")
    }
}