use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::Square;

pub struct BitBoardIter {
    bit_board: BitBoard    
}

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.bit_board.least_sig_bit();
        if let Some(square) = next {
            self.bit_board.clear_bit(square);
        }

        next
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn print_board(&self) {
        println!();

        for rank in (0..8).rev() {
            print!("{}   ", 1 + rank);
            for file in 0..8 {
                let board_index = (rank * 8) + file; 
                let bit_state = self.0 & (1u64 << board_index);

                print!("{}  ", if bit_state != 0 {1} else {0});
            }

            println!();
        }
        println!("\n    A  B  C  D  E  F  G  H");
        println!("\nBitboard: {}", self.0);
    }

    pub const fn iter(&self) -> BitBoardIter {
        BitBoardIter{ bit_board: *self }
    }

    pub const fn set_bit(&mut self, square: Square) {
        self.0 |= 1u64 << square as u64;
    }

    pub const fn clear_bit(&mut self, square: Square) {
        self.0  &= !(1u64 << square as u64);
    }

    pub const fn count_bits(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn least_sig_bit(&self) -> Option<Square> {
        if self.0 != 0 {Some(Square::from(self.0.trailing_zeros() as usize))} else {None}
    }

    pub const fn shift(&mut self, offset: i8) {
        if offset > 0 {self.0 <<= offset} else {self.0 >>= -offset}
    } 

    pub const fn get_bit(&self, square: Square) -> bool {
        let b = 1u64 << square as u64;
        (self.0 & b) != 0
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        let mut bb = BitBoard(0);
        bb.set_bit(Square::A1);

        assert_eq!(bb.count_bits(), 1);

        bb.set_bit(Square::A3);
        bb.set_bit(Square::A2);
        bb.set_bit(Square::A1);

        assert_eq!(bb.count_bits(), 3);
    }

    #[test]
    fn test_least_sig_bit() {
        let mut bb = BitBoard(0);
        bb.set_bit(Square::A3);

        assert_eq!(bb.least_sig_bit().unwrap(), Square::A3);

        bb.set_bit(Square::B3);
        bb.set_bit(Square::B2);
        bb.set_bit(Square::H8);
        bb.set_bit(Square::C2);

        assert_eq!(bb.least_sig_bit().unwrap(), Square::B2);
        bb.clear_bit(Square::B2);
        assert_eq!(bb.least_sig_bit().unwrap(), Square::C2);
    }

    #[test]
    fn test_bitboard_iter() {
        let mut bb = BitBoard(0);

        bb.set_bit(Square::A3);
        bb.set_bit(Square::B3);
        bb.set_bit(Square::B2);
        bb.set_bit(Square::H8);
        bb.set_bit(Square::C2);

        for square in bb.iter() {
            println!("{:?}", square);
        }
    }
}