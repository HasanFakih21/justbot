use std::sync::Mutex;

use crate::board::{Piece, Square, blocked_bishop_attacks, blocked_rook_attacks, count_bits, mask_bishop_attacks, mask_rook_attacks};
use crate::occupancy::{BISHOP_OCCUPANCY_BIT_COUNTS, ROOK_OCCUPANCY_BIT_COUNTS, set_occupancy};

static SEED: Mutex<u32> = Mutex::new(1804289383);

//XOR Shift Pseudo-Random Number Generator
pub fn get_random_num() -> u32 {
    let mut number = SEED.lock().unwrap();

    *number ^= *number << 13;
    *number ^= *number >> 17;
    *number ^= *number << 5;

    *number
}

pub fn get_random_u64_num() -> u64 {
    let nums: [u64; 4] = std::array::from_fn(|_e| {
        (get_random_num() as u64) & 0xFFFF
    });

    nums[0] | (nums[1] << 16) | (nums[2] << 32) | (nums[3] << 48)
}

pub fn generate_magic_number() -> u64 {
    get_random_u64_num() & get_random_u64_num() & get_random_u64_num()
}

pub fn find_magic_number(square: Square, piece: Piece) -> u64 {
    let mut occupancies: [u64; 4096] = [0; 4096];
    let mut attacks: [u64; 4096] = [0; 4096];
    let mut used_attacks: [u64; 4096];
    
    let attack_mask: u64 = match piece {
        Piece::Bishop => mask_bishop_attacks(square),
        Piece::Rook => mask_rook_attacks(square),
        _=> return 0
    };

    let relevant_bits = match piece {
        Piece::Bishop => BISHOP_OCCUPANCY_BIT_COUNTS[square as usize],
        Piece::Rook => ROOK_OCCUPANCY_BIT_COUNTS[square as usize],
        _=> return 0
    };

    for index in 0..(1 << relevant_bits) {
        occupancies[index] = set_occupancy(index, relevant_bits, attack_mask);

        attacks[index] = match piece {
            Piece::Bishop => blocked_bishop_attacks(square, occupancies[index]),
            Piece::Rook => blocked_rook_attacks(square, occupancies[index]),
            _=> return 0
        }
    }
    
    for _ in 0..10000000 {
        let magic_number = generate_magic_number();

        if count_bits(&((attack_mask * magic_number) & 0xFF00000000000000)) < 6 {
            continue;
        }

        used_attacks = [0; 4096];

        let (mut index, mut fail) = (0, false);
        while !fail && index < (1 << relevant_bits) {
            (index, fail) = (0, false);

            let magic_index = ((occupancies[index] * magic_number) >> (64 - relevant_bits)) as usize;

            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[index];
            }

            else if used_attacks[magic_index] != attacks[index] {
                fail = true;
            }
            index += 1;
        }

        if !fail {
            return magic_number;
        }
    }

    println!("Magic number generation failed");
    0
}
    
#[cfg(test)]
mod tests {
    use crate::board::print_board;
    use super::*;

    #[test]
    fn magic_test() {
        print_board(&(get_random_num() as u64));
        print_board(&(get_random_num() as u64));

        print_board(&get_random_u64_num());
        print_board(&generate_magic_number());
    }
}