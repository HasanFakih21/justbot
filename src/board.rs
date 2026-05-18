use std::fmt::Display;
use crate::{board::bitboard::BitBoard, magics::{BISHOP_MAGIC_NUMBERS, ROOK_MAGIC_NUMBERS, get_magic_index}, occupancy::{BISHOP_OCCUPANCY_BIT_COUNTS, ROOK_OCCUPANCY_BIT_COUNTS, set_occupancy}};
pub use crate::attacks::*;
pub use squares::*;
pub use pieces::*;
pub use sides::*;

pub mod squares;
pub mod pieces;
pub mod sides;
pub mod constants;
pub mod moves;
pub mod parser;
pub mod bitboard;
pub mod makemove;

#[derive(Debug, Clone)]
pub struct BoardState {
    pub board_pieces: [[BitBoard; 6]; 2],
    pub pieces_on_squares: [Option<(Side, Piece)>; 64],
    pub board_occupancies: [BitBoard; 2],
    pub side_to_move: Side,
    pub enpassant: Option<Square>,
    pub castling_rights: CastlingRights,
    pub material_value: [i32; 2],
    pub piece_square_value: [i32; 2],
}

impl BoardState {
    pub fn new() -> Self {
        BoardState { 
            board_pieces: [[BitBoard(0); 6]; 2], 
            pieces_on_squares: [None; 64],
            board_occupancies: [BitBoard(0); 2],
            side_to_move: Side::White,
            enpassant: None,
            castling_rights: CastlingRights::new(),
            material_value: [0; 2],
            piece_square_value: [0; 2],
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Board {
    pub bishop_masks: [BitBoard; 64],
    pub rook_masks: [BitBoard; 64],

    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub king_attacks: [BitBoard; 64],
    pub bishop_attacks: Vec<BitBoard>,
    pub rook_attacks: Vec<BitBoard>,

    pub board_state: BoardState,
    pub state_stack: Vec<BoardState>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

//Little-Endian Rank-File Mapping
impl Board {
    pub fn new() -> Self {
        let mut b = Board {
            bishop_masks: std::array::from_fn(|i| {
                mask_bishop_attacks(Square::from(i))
            }),
            rook_masks: std::array::from_fn(|i| {
                mask_rook_attacks(Square::from(i))
            }), 

            pawn_attacks: [[BitBoard(0); 64]; 2], knight_attacks: [BitBoard(0); 64], king_attacks: [BitBoard(0); 64], bishop_attacks: vec![BitBoard(0); 64 * 512], rook_attacks: vec![BitBoard(0); 64 * 4096],

            state_stack: Vec::new(),
            board_state: BoardState::new(),
        };

        b.init_leaping_attacks();
        b.init_bishop_attacks();
        b.init_rook_attacks();
        b
    }

    pub fn is_there(&self, side: Side, piece: Piece, square: Square) -> bool {
        let b = 1u64 << square as u64;
        (self.board_state.board_pieces[side as usize][piece as usize].0 & b) != 0
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<(Side, Piece)> {
        self.board_state.pieces_on_squares[square as usize]
    }

    pub fn place_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.board_state.board_pieces[side as usize][piece as usize].set_bit(square);
        self.board_state.board_occupancies[side as usize].set_bit(square); 
        self.board_state.pieces_on_squares[square as usize] = Some((side, piece));
        self.board_state.material_value[side as usize] += piece.value();
        self.board_state.piece_square_value[side as usize] += self.get_piece_square_score(piece, square, side);
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.board_state.board_pieces[side as usize][piece as usize].clear_bit(square);
        self.board_state.board_occupancies[side as usize].clear_bit(square);
        self.board_state.pieces_on_squares[square as usize] = None;
        self.board_state.material_value[side as usize] -= piece.value();
        self.board_state.piece_square_value[side as usize] -= self.get_piece_square_score(piece, square, side);
    }

    pub fn get_piece_attack(&self, side: Side, square: Square, piece: Piece) -> BitBoard {
        match piece {
            Piece::Pawn   => self.get_pawn_attacks(square, side),
            Piece::Knight => self.get_knight_attacks(square),
            Piece::Bishop => self.get_bishop_attacks(square, self.get_all_occupancy()),
            Piece::Rook   => self.get_bishop_attacks(square, self.get_all_occupancy()),
            Piece::Queen  => self.get_queen_attacks(square, self.get_all_occupancy()),
            Piece::King   => self.get_king_attacks(square),
        }
    }

    pub fn get_all_attacks(&self, side: Side) -> BitBoard {
        let mut attacks = BitBoard(0);
        for (i, e) in self.board_state.board_pieces[side as usize].iter().enumerate() {
            e.iter().for_each(|s| attacks |= self.get_piece_attack(side, s, Piece::from(i)));
        }

        attacks & !self.board_state.board_occupancies[side as usize]
    }

    pub fn init_leaping_attacks(&mut self) {
        for i in 0..64 {
            let square = Square::from(i);
            self.pawn_attacks[Side::White as usize][i] = mask_pawn_attacks(Side::White, square);
            self.pawn_attacks[Side::Black as usize][i] = mask_pawn_attacks(Side::Black, square);
            self.knight_attacks[i] = mask_knight_attacks(square);
            self.king_attacks[i] = mask_king_attacks(square);
        }
    }

    pub fn init_rook_attacks(&mut self) {
        for i in 0..64 {
            let square = Square::from(i);
            let relevant_bits = ROOK_OCCUPANCY_BIT_COUNTS[square as usize]; 
            let magic_number = ROOK_MAGIC_NUMBERS[square as usize];

            for index in 0..4096 {
                let occupancy_bb = set_occupancy(index, relevant_bits, self.rook_masks[square as usize]);
                let magic_index = get_magic_index(occupancy_bb, relevant_bits, magic_number);
                self.rook_attacks[(square as usize * 4096) + magic_index] = blocked_rook_attacks(square, occupancy_bb);
            }
        }
    }

    pub fn init_bishop_attacks(&mut self) {
        for i in 0..64 {
            let square = Square::from(i);
            let relevant_bits = BISHOP_OCCUPANCY_BIT_COUNTS[square as usize]; 
            let magic_number = BISHOP_MAGIC_NUMBERS[square as usize];

            for index in 0..512 {
                let occupancy_bb = set_occupancy(index, relevant_bits, self.bishop_masks[square as usize]);
                let magic_index = get_magic_index(occupancy_bb, relevant_bits, magic_number);
                self.bishop_attacks[(square as usize * 512) + magic_index] = blocked_bishop_attacks(square, occupancy_bb);
            }
        }
    }

    pub fn get_pawn_attacks(&self, square: Square, side: Side) -> BitBoard {
        self.pawn_attacks[side as usize][square as usize]
    }

    pub fn get_knight_attacks(&self, square: Square) -> BitBoard {
        self.knight_attacks[square as usize]
    }

    pub fn get_king_attacks(&self, square: Square) -> BitBoard {
        self.king_attacks[square as usize]
    }

    pub fn get_bishop_attacks(&self, square: Square, board_occupancy: BitBoard) -> BitBoard {
        let occupancy = board_occupancy & self.bishop_masks[square as usize];
        let magic_index = get_magic_index(occupancy, BISHOP_OCCUPANCY_BIT_COUNTS[square as usize], BISHOP_MAGIC_NUMBERS[square as usize]);
        let offset = (square as usize * 512) + magic_index;

        self.bishop_attacks[offset]
    }

    pub fn get_rook_attacks(&self, square: Square, board_occupancy: BitBoard) -> BitBoard {
        let occupancy = board_occupancy & self.rook_masks[square as usize];
        let magic_index = get_magic_index(occupancy, ROOK_OCCUPANCY_BIT_COUNTS[square as usize], ROOK_MAGIC_NUMBERS[square as usize]);
        let offset = (square as usize * 4096) + magic_index;

        self.rook_attacks[offset]
    }

    pub fn get_queen_attacks(&self, square: Square, board_occupancy: BitBoard) -> BitBoard {
        self.get_bishop_attacks(square, board_occupancy) | self.get_rook_attacks(square, board_occupancy)
    }

    pub fn get_all_occupancy(&self) -> BitBoard {
        self.board_state.board_occupancies[Side::White as usize] | self.board_state.board_occupancies[Side::Black as usize]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("\n\n");
        for rank in (0..8).rev() {
            output.push_str(&format!("{}   ", 1 + rank));
            for file in 0..8 {
                let square = Square::from_rank_and_file(rank, file);
                let piece: Option<(Side, Piece)> = self.get_piece_at_square(square);

                if let Some(p) = piece {
                    let mut s = format!(" {} ", p.1);
                    if let Side::Black = p.0 {
                        s = s.to_lowercase();
                    }
                    output.push_str(&s);
                } else {
                    output.push_str(" . ");
                }
            }
            output.push('\n');
        }
        output.push_str("\n     A  B  C  D  E  F  G  H\n");
        output.push_str(&format!("\n     Side to move: {} \n     Castling: {}\n     Enpassant: {:?}\n", self.board_state.side_to_move, self.board_state.castling_rights, self.board_state.enpassant));
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{constants::STARTING_FEN, moves::Move};
    use super::*;

    #[test]
    fn test_get_rook_attack() {
        let board = Board::new();
        let mut occ = BitBoard(0);

        board.get_rook_attacks(Square::A6, occ).print_board();

        occ.set_bit(Square::E3);
        occ.set_bit(Square::G5);
        occ.set_bit(Square::G3);

        board.get_rook_attacks(Square::G3, occ).print_board();
    }

    #[test]
    fn test_get_bishop_attack() {
        let board = Board::new();
        let mut occ = BitBoard(0);

        board.get_bishop_attacks(Square::A3, occ).print_board();
        
        occ.set_bit(Square::D6);
        board.get_bishop_attacks(Square::G3, occ).print_board();
    }

    #[test]
    fn test_get_queen_attack() {
        let board = Board::new();
        let mut occ = BitBoard(0);

        board.get_queen_attacks(Square::A6, occ).print_board();

        occ.set_bit(Square::E3);
        occ.set_bit(Square::G5);
        occ.set_bit(Square::G3);
        occ.set_bit(Square::D6);

        board.get_queen_attacks(Square::G3, occ).print_board();
        board.get_queen_attacks(Square::E4, occ).print_board();
    }

    #[test]
    fn test_board_occupancy() {
        let mut board = Board::from_fen(STARTING_FEN);
        board.remove_piece(Side::White, Piece::Pawn, Square::A2);
        board.get_all_occupancy().print_board();
        board.board_state.board_occupancies[Side::Black as usize].print_board();
        board.board_state.board_occupancies[Side::White as usize].print_board();
    }

    #[test]
    fn test_full_board_print() {
        let board = Board::new();
        println!("{board}");
    }

    #[test]
    fn test_get_all_attacks() {
        let mut board = Board::from_fen(STARTING_FEN);
        let m = Move::new(Square::E2, Square::E4, moves::MoveKind::DoublePawn);
        let _ = board.make_move(m);
        println!("{board}");
        board.get_all_attacks(Side::White).print_board();
    }
}