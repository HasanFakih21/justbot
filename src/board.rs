pub mod squares;
pub mod pieces;
pub mod sides;
pub mod constants;
pub mod moves;
pub mod parser;
pub mod bitboard;

use std::fmt::Display;

pub use squares::*;
pub use pieces::*;
pub use sides::*;

pub use crate::attacks::*;
use crate::{magics::{BISHOP_MAGIC_NUMBERS, ROOK_MAGIC_NUMBERS, get_magic_index}, occupancy::{BISHOP_OCCUPANCY_BIT_COUNTS, ROOK_OCCUPANCY_BIT_COUNTS, set_occupancy}};

pub struct Board {
    pub board_pieces: [[u64; 6]; 2],
    pub board_occupancies: [u64; 2],
    pub side_to_move: Side,
    pub enpassant: Option<Square>,
    pub castling_rights: CastlingRights,

    pub bishop_masks: [u64; 64],
    pub rook_masks: [u64; 64],

    pub pawn_attacks: [[u64; 64]; 2],
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
    pub bishop_attacks: Vec<u64>,
    pub rook_attacks: Vec<u64>,
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
            board_pieces: [[0; 6]; 2],
            board_occupancies: [0; 2],
            side_to_move: Side::White,
            enpassant: None,
            castling_rights: CastlingRights::new(),

            bishop_masks: std::array::from_fn(|i| {
                mask_bishop_attacks(Square::from(i))
            }),
            rook_masks: std::array::from_fn(|i| {
                mask_rook_attacks(Square::from(i))
            }), 

            pawn_attacks: [[0; 64]; 2], knight_attacks: [0; 64], king_attacks: [0; 64], bishop_attacks: vec![0; 64 * 512], rook_attacks: vec![0; 64 * 4096],
        };

        b.init_leaping_attacks();
        b.init_bishop_attacks();
        b.init_rook_attacks();
        b
    }

    pub fn get_bit(&self, side: Side, piece: Piece, square: Square) -> bool {
        let b = 1u64 << square as u64;
        (self.board_pieces[side as usize][piece as usize] & b) != 0
    }

    pub fn get_piece_at_square(&self, square: Square) -> Option<(Piece, Side)> {
        for piece_index in 0..6 {
            if self.get_bit(Side::White, Piece::from(piece_index), square) {
                return Some((Piece::from(piece_index), Side::White));
            }

            if self.get_bit(Side::Black, Piece::from(piece_index), square) {
                return Some((Piece::from(piece_index), Side::Black))
            }
        }
        None
    }

    pub fn place_piece(&mut self, side: Side, piece: Piece, square: Square) {
        set_bit(&mut self.board_pieces[side as usize][piece as usize], square);
        set_bit(&mut self.board_occupancies[side as usize], square);
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        clear_bit(&mut self.board_pieces[side as usize][piece as usize], square);
        clear_bit(&mut self.board_occupancies[side as usize], square);
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

    pub fn get_pawn_attacks(&self, square: Square, side: Side) -> u64 {
        self.pawn_attacks[side as usize][square as usize]
    }

    pub fn get_knight_attacks(&self, square: Square) -> u64 {
        self.knight_attacks[square as usize]
    }

    pub fn get_king_attacks(&self, square: Square) -> u64 {
        self.king_attacks[square as usize]
    }

    pub fn get_bishop_attacks(&self, square: Square, board_occupancy: u64) -> u64 {
        let occupancy = board_occupancy & self.bishop_masks[square as usize];
        let magic_index = get_magic_index(occupancy, BISHOP_OCCUPANCY_BIT_COUNTS[square as usize], BISHOP_MAGIC_NUMBERS[square as usize]);
        let offset = (square as usize * 512) + magic_index;

        self.bishop_attacks[offset]
    }

    pub fn get_rook_attacks(&self, square: Square, board_occupancy: u64) -> u64 {
        let occupancy = board_occupancy & self.rook_masks[square as usize];
        let magic_index = get_magic_index(occupancy, ROOK_OCCUPANCY_BIT_COUNTS[square as usize], ROOK_MAGIC_NUMBERS[square as usize]);
        let offset = (square as usize * 4096) + magic_index;

        self.rook_attacks[offset]
    }

    pub fn get_queen_attacks(&self, square: Square, board_occupancy: u64) -> u64 {
        self.get_bishop_attacks(square, board_occupancy) | self.get_rook_attacks(square, board_occupancy)
    }

    pub const fn get_all_occupancy(&self) -> u64 {
        self.board_occupancies[Side::White as usize] | self.board_occupancies[Side::Black as usize]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("\n\n");
        for rank in (0..8).rev() {
            output.push_str(&format!("{}   ", 1 + rank));
            for file in 0..8 {
                let square = Square::from_rank_and_file(rank, file);
                let piece: Option<(Piece, Side)> = self.get_piece_at_square(square);

                if let Some(p) = piece {
                    let mut s = format!(" {} ", p.0);
                    if let Side::Black = p.1 {
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
        output.push_str(&format!("\n     Side to move: {} \n     Castling: {}\n     Enpassant: {:?}\n", self.side_to_move, self.castling_rights, self.enpassant));
        write!(f, "{}", output)
    }
}

pub fn print_board(bit_board: &u64) {
    println!();

    for rank in (0..8).rev() {
        print!("{}   ", 1 + rank);
        for file in 0..8 {
            let board_index = (rank * 8) + file; 
            let bit_state = bit_board & (1u64 << board_index);

            print!("{}  ", if bit_state != 0 {1} else {0});
        }

        println!();
    }
    println!("\n    A  B  C  D  E  F  G  H");
    println!("\nBitboard: {bit_board}");
}

pub const fn set_bit(bit_board: &mut u64, square: Square) {
    *bit_board |= 1u64 << square as u64;
}

pub const fn clear_bit(bit_board: &mut u64, square: Square) {
    *bit_board &= !(1u64 << square as u64);
}

pub const fn count_bits(bit_board: &u64) -> usize {
    bit_board.count_ones() as usize
}

pub const fn least_sig_bit(bit_board: &u64) -> Square {
    Square::from(bit_board.trailing_zeros() as usize)
}

pub const fn shift(bit_board: &mut u64, offset: i8) {
    if offset > 0 {*bit_board <<= offset} else {*bit_board >>= -offset}
} 

#[cfg(test)]
mod tests {
    use crate::board::constants::STARTING_FEN;
    use super::*;

    #[test]
    fn test_count_bits() {
        let mut bb = 0u64;
        set_bit(&mut bb, Square::A1);

        assert_eq!(count_bits(&bb), 1);

        set_bit(&mut bb, Square::A3);
        set_bit(&mut bb, Square::A2);
        set_bit(&mut bb, Square::A1);

        assert_eq!(count_bits(&bb), 3);
    }

    #[test]
    fn test_least_sig_bit() {
        let mut bb = 0u64;
        set_bit(&mut bb, Square::A3);

        assert_eq!(least_sig_bit(&bb), Square::A3);

        set_bit(&mut bb, Square::B3);
        set_bit(&mut bb, Square::B2);
        set_bit(&mut bb, Square::H8);
        set_bit(&mut bb, Square::C2);

        assert_eq!(least_sig_bit(&bb), Square::B2);
        clear_bit(&mut bb, Square::B2);
        assert_eq!(least_sig_bit(&bb), Square::C2);
    }

    #[test]
    fn test_rook_attack_map() {
        let board = Board::new();
        print_board(&board.bishop_attacks[4]);
    }

    #[test]
    fn test_get_rook_attack() {
        let board = Board::new();
        let mut occ = 0u64;

        print_board(&board.get_rook_attacks(Square::A6, occ));

        set_bit(&mut occ, Square::E3);
        set_bit(&mut occ, Square::G5);
        set_bit(&mut occ, Square::G3);

        print_board(&board.get_rook_attacks(Square::G3, occ));
    }

    #[test]
    fn test_get_bishop_attack() {
        let board = Board::new();
        let mut occ = 0u64;

        print_board(&board.get_bishop_attacks(Square::A3, occ));
        
        set_bit(&mut occ, Square::D6);
        print_board(&board.get_bishop_attacks(Square::G3, occ));
    }

    #[test]
    fn test_get_queen_attack() {
        let board = Board::new();
        let mut occ = 0u64;

        print_board(&board.get_queen_attacks(Square::A6, occ));

        set_bit(&mut occ, Square::E3);
        set_bit(&mut occ, Square::G5);
        set_bit(&mut occ, Square::G3);
        set_bit(&mut occ, Square::D6);

        print_board(&board.get_queen_attacks(Square::G3, occ));
        print_board(&board.get_queen_attacks(Square::E4, occ));
    }

    #[test]
    fn test_board_occupancy() {
        let mut board = Board::from_fen(STARTING_FEN);
        board.remove_piece(Side::White, Piece::Pawn, Square::A2);
        print_board(&board.get_all_occupancy());
        print_board(&board.board_occupancies[Side::Black as usize]);
        print_board(&board.board_occupancies[Side::White as usize]);
    }

    #[test]
    fn test_full_board_print() {
        let board = Board::new();
        println!("{board}");
    }
}