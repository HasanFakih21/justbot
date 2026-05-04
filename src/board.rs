#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl TryFrom<usize> for Square {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 63 {
            Ok(unsafe { std::mem::transmute::<u8, Square>(value as u8) })
        } else {
            Err("Invalid value for Square".to_string())
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Piece {
    Pawns,
    Knights,
    Bishops,
    Rooks,
    Queens,
    King,
}

impl TryFrom<usize> for Piece {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 5 {
            Ok(unsafe { std::mem::transmute::<u8, Piece>(value as u8) })
        } else {
            Err("Invalid value for Piece".to_string())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Side {
    White,
    Black,
}

#[derive(Default)]
pub struct BitBoard {
    pub bit_board_pieces: [[u64; 6]; 2],
}

//Little-Endian Rank-File Mapping
impl BitBoard {
    pub fn new() -> Self {
        BitBoard {
            bit_board_pieces: [
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
        }
    }
    

    pub fn set_bit(&mut self, side: Side, piece: Piece, position: Square) {
        let b = 1u64 << position as u64;
        self.bit_board_pieces[side as usize][piece as usize] |= b;
    }

    pub fn clear_bit(&mut self, side: Side, piece: Piece, position: Square) {
        let b = 1u64 << position as u64;
        if self.get_bit(side, piece, position) {
            self.bit_board_pieces[side as usize][piece as usize] ^= b;
        }
    }

    pub fn get_bit(&self, side: Side, piece: Piece, position: Square) -> bool {
        let b = 1u64 << position as u64;
        (self.bit_board_pieces[side as usize][piece as usize] & b) != 0
    }

    pub fn get_piece_at_square(&self, side: Side, position: Square) -> Option<Piece> {
        for piece_index in 0..6 {
            if self.get_bit(side, Piece::try_from(piece_index).unwrap(), position) {
                return Some(Piece::try_from(piece_index).unwrap());
            }
        }
        None
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
}