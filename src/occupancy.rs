use crate::board::{least_sig_bit, clear_bit};

pub static BISHOP_OCCUPANCY_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];

pub static ROOK_OCCUPANCY_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];

pub fn set_occupancy(index: usize, num_bits_in_mask: usize, mut attack_mask: u64) -> u64 {
    let mut occupancy = 0u64;

    for count in 0..num_bits_in_mask {
        let square = least_sig_bit(&attack_mask);
        clear_bit(&mut attack_mask, square);

        if (index & (1 << count)) != 0 {
            occupancy |= 1u64 << square as usize;
        }
    }

    occupancy
}
