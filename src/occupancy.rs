use crate::board::{bitboard::BitBoard};

pub static BISHOP_OCCUPANCY_BIT_COUNTS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

pub static ROOK_OCCUPANCY_BIT_COUNTS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

pub fn set_occupancy(index: usize, num_bits_in_mask: usize, mut attack_mask: BitBoard) -> BitBoard {
    let mut occupancy = BitBoard(0u64);

    for count in 0..num_bits_in_mask {
        let square = attack_mask.least_sig_bit().unwrap();
        attack_mask.clear_bit(square);

        if (index & (1 << count)) != 0 {
            occupancy.0 |= 1u64 << square as usize;
        }
    }

    occupancy
}

#[cfg(test)]
mod tests {
    use crate::board::{Square, mask_rook_attacks};
    use super::*;

    #[test]
    fn test_set_occupancy() {
        let attack_mask = mask_rook_attacks(Square::A1);
        for i in 0..=4096 {
            let occupancy_bb = set_occupancy(i, 12, attack_mask);
            occupancy_bb.print_board();
        }
    }
}
