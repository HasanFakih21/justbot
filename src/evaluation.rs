use crate::board::Board;

impl Board {
    pub const fn get_material_evaluation(&self) -> i32 {
        let side = self.board_state.side_to_move;
        self.board_state.material_value[side as usize] - self.board_state.material_value[side.other() as usize] 
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Piece, Side, Square, constants::STARTING_FEN};


    #[test]
    fn test_get_material_evaluation() {
        let mut board = Board::from_fen(STARTING_FEN);
        assert_eq!(0, board.get_material_evaluation());

        board.remove_piece(Side::White, Piece::Pawn, Square::E2);
        assert_eq!(-100, board.get_material_evaluation());

        board.remove_piece(Side::Black, Piece::Rook, Square::H8);
        assert_eq!(425, board.get_material_evaluation());

        board.place_piece(Side::Black, Piece::Queen, Square::E5);
        assert_eq!(-575, board.get_material_evaluation());
    }
}