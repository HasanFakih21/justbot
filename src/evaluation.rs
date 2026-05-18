use crate::board::{Board, Piece, Side, Square};

pub const PAWN_TABLE: [i32; 64] = [
 0,  0,  0,  0,  0,  0,  0,  0,
50, 50, 50, 50, 50, 50, 50, 50,
10, 10, 20, 30, 30, 20, 10, 10,
 5,  5, 10, 25, 25, 10,  5,  5,
 0,  0,  0, 20, 20,  0,  0,  0,
 5, -5,-10,  0,  0,-10, -5,  5,
 5, 10, 10,-20,-20, 10, 10,  5,
 0,  0,  0,  0,  0,  0,  0,  0
];

pub const KNIGHT_TABLE: [i32; 64] = [
-50,-40,-30,-30,-30,-30,-40,-50,
-40,-20,  0,  0,  0,  0,-20,-40,
-30,  0, 10, 15, 15, 10,  0,-30,
-30,  5, 15, 20, 20, 15,  5,-30,
-30,  0, 15, 20, 20, 15,  0,-30,
-30,  5, 10, 15, 15, 10,  5,-30,
-40,-20,  0,  5,  5,  0,-20,-40,
-50,-40,-30,-30,-30,-30,-40,-50,
];

pub const BISHOP_TABLE: [i32; 64] = [
-20,-10,-10,-10,-10,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5, 10, 10,  5,  0,-10,
-10,  5,  5, 10, 10,  5,  5,-10,
-10,  0, 10, 10, 10, 10,  0,-10,
-10, 10, 10, 10, 10, 10, 10,-10,
-10,  5,  0,  0,  0,  0,  5,-10,
-20,-10,-10,-10,-10,-10,-10,-20,
];

pub const ROOK_TABLE: [i32; 64] = [
  0,  0,  0,  0,  0,  0,  0,  0,
  5, 10, 10, 10, 10, 10, 10,  5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
 -5,  0,  0,  0,  0,  0,  0, -5,
  0,  0,  0,  5,  5,  0,  0,  0
];

pub const QUEEN_TABLE: [i32; 64] = [
-20,-10,-10, -5, -5,-10,-10,-20,
-10,  0,  0,  0,  0,  0,  0,-10,
-10,  0,  5,  5,  5,  5,  0,-10,
 -5,  0,  5,  5,  5,  5,  0, -5,
  0,  0,  5,  5,  5,  5,  0, -5,
-10,  5,  5,  5,  5,  5,  0,-10,
-10,  0,  5,  0,  0,  0,  0,-10,
-20,-10,-10, -5, -5,-10,-10,-20
];

pub const MG_KING_TABLE: [i32; 64] = [
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-30,-40,-40,-50,-50,-40,-40,-30,
-20,-30,-30,-40,-40,-30,-30,-20,
-10,-20,-20,-20,-20,-20,-20,-10,
 20, 20,  0,  0,  0,  0, 20, 20,
 20, 30, 10,  0,  0, 10, 30, 20
];

pub const EG_KING_TABLE: [i32; 64] = [
-50,-40,-30,-20,-20,-30,-40,-50,
-30,-20,-10,  0,  0,-10,-20,-30,
-30,-10, 20, 30, 30, 20,-10,-30,
-30,-10, 30, 40, 40, 30,-10,-30,
-30,-10, 30, 40, 40, 30,-10,-30,
-30,-10, 20, 30, 30, 20,-10,-30,
-30,-30,  0,  0,  0,  0,-30,-30,
-50,-30,-30,-30,-30,-30,-30,-50
];

pub const MG_TABLE_ARRAY: [[i32; 64]; 6] = [PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE, ROOK_TABLE, QUEEN_TABLE, MG_KING_TABLE];
pub const EG_TABLE_ARRAY: [[i32; 64]; 6] = [PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE, ROOK_TABLE, QUEEN_TABLE, EG_KING_TABLE];

impl Board {
    pub const fn get_material_evaluation(&self) -> i32 {
        let side = self.board_state.side_to_move;
        self.board_state.material_value[side as usize]
        - self.board_state.material_value[side.other() as usize]
    }

    pub const fn evaluate(&self) -> i32 {
        self.get_material_evaluation() + self.get_piece_square_evaluation()
    }

    pub fn get_piece_square_score(&mut self, piece: Piece, square: Square, side: Side) -> i32 {
        let table = MG_TABLE_ARRAY[piece as usize];
        match side {
            Side::White => table[(square ^ Square::A8) as usize],
            Side::Black => table[square as usize]
        }
    }

    pub const fn get_piece_square_evaluation(&self) -> i32 {
        let side = self.board_state.side_to_move;
        self.board_state.piece_square_value[side as usize]
        - self.board_state.piece_square_value[side.other() as usize]
    }

    pub fn has_legal_move(&mut self) -> bool {
        let move_list = self.generate_all_moves();
        for m in move_list.iter() {
            if self.make_move(*m).is_ok() {
                self.unmake_move();
                return true;
            }
        }

        false
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

    #[test]
    fn test_piece_square_evaluation() {
        let mut board = Board::from_fen(STARTING_FEN);
        assert_eq!(0, board.get_piece_square_evaluation());

        let score = board.get_piece_square_score(Piece::Pawn, Square::E2, Side::White);
        assert_eq!(score, -20);

        board.remove_piece(Side::White, Piece::Pawn, Square::E2);
        assert_eq!(20, board.get_piece_square_evaluation());
    }
}
