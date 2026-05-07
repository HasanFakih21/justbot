pub mod squares;
pub mod pieces;
pub mod sides;
pub mod constants;

pub use squares::*;
pub use pieces::*;
pub use sides::*;

pub use crate::attacks::*;

pub struct Board {
    pub board_pieces: [[u64; 6]; 2],
    pub pawn_attacks: [[u64; 64]; 2],
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
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
            board_pieces: [
                [0x0000_0000_0000_FF00,
                 0x0000_0000_0000_0042,
                 0x0000_0000_0000_0024,
                 0x0000_0000_0000_0081,
                 0x0000_0000_0000_0008,
                 0x0000_0000_0000_0010],

                [0x00FF_0000_0000_0000,
                 0x4200_0000_0000_0000,
                 0x2400_0000_0000_0000,
                 0x8100_0000_0000_0000,
                 0x0800_0000_0000_0000,
                 0x1000_0000_0000_0000]
            ],
            pawn_attacks: [[0; 64]; 2],
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
        };

        b.init_leaping_attacks();
        b
    }

    pub fn set_piece_bit(&mut self, side: Side, piece: Piece, position: Square) {
        set_bit(&mut self.board_pieces[side as usize][piece as usize], position);
    }

    pub fn clear_piece_bit(&mut self, side: Side, piece: Piece, position: Square) {
        clear_bit(&mut self.board_pieces[side as usize][piece as usize], position);
    }

    pub fn get_bit(&self, side: Side, piece: Piece, position: Square) -> bool {
        let b = 1u64 << position as u64;
        (self.board_pieces[side as usize][piece as usize] & b) != 0
    }

    pub fn get_piece_at_square(&self, side: Side, position: Square) -> Option<Piece> {
        for piece_index in 0..6 {
            if self.get_bit(side, Piece::try_from(piece_index).unwrap(), position) {
                return Some(Piece::try_from(piece_index).unwrap());
            }
        }
        None
    }

    pub fn init_leaping_attacks(&mut self) {
        for i in 0..64 {
            let square = Square::try_from(i).unwrap();
            self.pawn_attacks[Side::White as usize][i] = mask_pawn_attacks(Side::White, square);
            self.pawn_attacks[Side::Black as usize][i] = mask_pawn_attacks(Side::Black, square);
            self.knight_attacks[i] = mask_knight_attacks(square);
            self.king_attacks[i] = mask_king_attacks(square);
        }
    }
}

pub fn print_board(bit_board: &u64) {
    println!();

    for rank in (0..8).rev() {
        for file in 0..8 {
            let board_index = (rank * 8) + file; 
            let bit_state = bit_board & (1u64 << board_index);

            if file == 0 { print!("{}   ", 1 + rank);}
            print!("{}  ", if bit_state != 0 {1} else {0});
        }

        println!();
    }
    println!("\n    A  B  C  D  E  F  G  H");
    println!("\nBitboard: {bit_board}");
}

pub fn set_bit(bit_board: &mut u64, square: Square) {
    *bit_board |= 1u64 << square as u64;
}

pub fn clear_bit(bit_board: &mut u64, square: Square) {
    *bit_board &= !(1u64 << square as u64);
}