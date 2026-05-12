use super::Square;

pub struct BitBoard(u64);

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

    pub const fn set_bit(&mut self, square: Square) {
        self.0 |= 1u64 << square as u64;
    }

    pub const fn clear_bit(&mut self, square: Square) {
        self.0  &= !(1u64 << square as u64);
    }

    pub const fn count_bits(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn least_sig_bit(&self) -> Square {
        Square::from(self.0.trailing_zeros() as usize)
    }

    pub const fn shift(&mut self, offset: i8) {
        if offset > 0 {self.0 <<= offset} else {self.0 >>= -offset}
    } 
}