pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

pub const NUM_OF_PIECES: usize = 6;
pub const NUM_OF_SQUARES: usize = 64;


#[derive(Debug)]
pub struct InvalidSquare;
#[derive(Debug)]
pub struct InvalidPiece;

#[derive(Copy, Clone, Debug, PartialEq)]
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
    type Error = InvalidSquare;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 63 {
            Ok(unsafe { std::mem::transmute::<u8, Square>(value as u8) })
        } else {
            Err(InvalidSquare)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Piece { //Num 0 to 5
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<usize> for Piece {
    type Error = InvalidPiece;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 5 {
            Ok(unsafe { std::mem::transmute::<u8, Piece>(value as u8) })
        } else {
            Err(InvalidPiece)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White
        }
    }
}

pub struct BitBoard {
    pub bit_board_pieces: [[u64; 6]; 2],
    pub pawn_attacks: [[u64; 64]; 2],
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
}

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
}

//Little-Endian Rank-File Mapping
impl BitBoard {
    pub fn new() -> Self {
        let mut b = BitBoard {
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
            pawn_attacks: [[0; 64]; 2],
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
        };

        b.init_leaping_attacks();
        b
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

    pub fn init_leaping_attacks(&mut self) {
        for i in 0..64 {
            let square = Square::try_from(i).unwrap();
            self.pawn_attacks[Side::White as usize][i] = self.mask_pawn_attacks(Side::White, square);
            self.pawn_attacks[Side::Black as usize][i] = self.mask_pawn_attacks(Side::Black, square);
            self.knight_attacks[i] = self.mask_knight_attacks(square);
            self.king_attacks[i] = self.mask_king_attacks(square);
        }
    }

    pub fn mask_pawn_attacks(&self, side: Side, square: Square) -> u64 {
        let current = 1u64 << square as u64;
        let mut top_left = 0u64; 
        let mut top_right = 0u64;

        match side {
            Side::White => {
                if current & A_FILE == 0 {
                    top_left = current << 7;
                }

                if current & H_FILE == 0 {
                    top_right = current << 9;
                } 
            },
            Side::Black => {
                if current & H_FILE == 0 {
                    top_left = current >> 7;
                }

                if current & A_FILE == 0 {
                    top_right = current >> 9;
                }
            }
        }

        top_left | top_right
    }

    pub fn mask_knight_attacks(&self, square: Square) -> u64 {
        let current = 1u64 << square as u64;

        let tl1 = if (current & A_FILE) == 0 {current << 15} else {0};
        let tl2 = if (current & (A_FILE | B_FILE)) == 0 {current << 6} else {0};

        let bl1 = if (current & (A_FILE | B_FILE)) == 0 {current >> 10} else {0};
        let bl2 = if (current & A_FILE) == 0 {current >> 17} else {0};

        let tr1 = if (current & H_FILE) == 0 {current << 17} else {0};
        let tr2 = if (current & (H_FILE | G_FILE)) == 0 {current << 10} else {0};

        let br1 = if (current & (H_FILE | G_FILE)) == 0 {current >> 6} else {0};
        let br2 = if (current & H_FILE) == 0 {current >> 15} else {0};

        tl1 | tl2 | bl1 | bl2 | tr1 | tr2 | br1 | br2
    }

    pub fn mask_king_attacks(&self, square: Square) -> u64 {
        let current = 1u64 << square as u64;
        let n = current << 8;
        let nw = if current & A_FILE == 0 {current << 7} else {0};
        let w = if current & A_FILE == 0 {current >> 1} else {0};
        let sw = if current & A_FILE == 0 {current >> 9} else {0};
        let s = current >> 8;
        let se = if current & H_FILE == 0 {current >> 7} else {0};
        let e = if current & H_FILE == 0 {current << 1} else {0};
        let ne = if current & H_FILE == 0 {current << 9} else {0};

        n | nw | w | sw | s | se | e | ne
    }

    pub fn mask_bishop_attacks(&self, square: Square) -> u64 {
        let mut attacks = 0u64;
        let (rank, file) = to_rank_and_file(square);

        let (mut r, mut f) = (rank, file);
        while r < 6 && f < 6 {
            r += 1;
            f += 1;
           attacks |= 1u64 << to_square(r, f).unwrap() as u64;
        }

        let (mut r, mut f) = (rank, file);
        while r > 1 && f < 6 {
            r -= 1;
            f += 1;
           attacks |= 1u64 << to_square(r, f).unwrap() as u64;
        }

        let (mut r, mut f) = (rank, file);
        while r < 6 && f > 1 {
            r += 1;
            f -= 1;
            attacks |= 1u64 << to_square(r, f).unwrap() as u64;
        }

        let (mut r, mut f) = (rank, file);
        while r > 1 && f > 1 {
            r -= 1;
            f -= 1;
            attacks |= 1u64 << to_square(r, f).unwrap() as u64;
        }
       
        attacks
    }

    pub fn mask_rook_attacks(&self, square: Square) -> u64 {
        let mut attacks = 0u64;
        let (rank, file) = to_rank_and_file(square);
        
        let mut r = rank;
        while r > 1 {
            r -= 1;
            attacks |= 1u64 << to_square(r, file).unwrap() as u64;
        }

        let mut r = rank;
        while r < 6 {
            r += 1;
            attacks |= 1u64 << to_square(r, file).unwrap() as u64;
        }

        let mut f = file;
        while f < 6 {
            f += 1;
            attacks |= 1u64 << to_square(rank, f).unwrap() as u64;
        }

        let mut f = file;
        while f > 1 {
            f -= 1;
            attacks |= 1u64 << to_square(rank, f).unwrap() as u64;
        }      

       attacks  
    }
}

pub fn to_rank_and_file(square: Square) -> (usize, usize) {
    (square as usize / 8, square as usize % 8)
}

pub fn to_square(rank: usize, file: usize) -> Result<Square, InvalidSquare> {
    Square::try_from((rank * 8) + file)
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