use crate::{Side, Square};
use crate::board::constants::*;

pub fn mask_pawn_attacks(side: Side, square: Square) -> u64 {
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

pub fn mask_knight_attacks(square: Square) -> u64 {
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

pub fn mask_king_attacks(square: Square) -> u64 {
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

pub fn mask_bishop_attacks(square: Square) -> u64 {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();

    let (mut r, mut f) = (rank, file);
    while r < 6 && f < 6 {
        r += 1;
        f += 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
    }

    let (mut r, mut f) = (rank, file);
    while r > 1 && f < 6 {
        r -= 1;
        f += 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
    }

    let (mut r, mut f) = (rank, file);
    while r < 6 && f > 1 {
        r += 1;
        f -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
    }

    let (mut r, mut f) = (rank, file);
    while r > 1 && f > 1 {
        r -= 1;
        f -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
    }
    
    attacks
}

pub fn mask_rook_attacks(square: Square) -> u64 {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();
    
    let mut r = rank;
    while r > 1 {
        r -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
    }

    let mut r = rank;
    while r < 6 {
        r += 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
    }

    let mut f = file;
    while f < 6 {
        f += 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
    }

    let mut f = file;
    while f > 1 {
        f -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
    }      

    attacks  
}

pub fn blocked_bishop_attacks(square: Square, block_board: u64) -> u64 {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();

    let (mut r, mut f) = (rank, file);

    while r < 7 && f < 7 {
        r += 1;
        f += 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r > 0 && f < 7 {
        r -= 1;
        f += 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r < 7 && f > 0 {
        r += 1;
        f -= 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r > 0 && f > 0 {
        r -= 1;
        f -= 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board != 0 {
            break;
        }
    }
    
    attacks
}

pub fn blocked_rook_attacks(square: Square, block_board: u64) -> u64 {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();
    
    let mut r = rank;
    while r > 0 {
        r -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
        if (1u64 << Square::from_rank_and_file(r, file) as u64) & block_board != 0 {
            break;
        }
    }

    let mut r = rank;
    while r < 7 {
        r += 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
        if (1u64 << Square::from_rank_and_file(r, file) as u64) & block_board != 0 {
            break;
        }
    }

    let mut f = file;
    while f < 7 {
        f += 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
        if (1u64 << Square::from_rank_and_file(rank, f) as u64) & block_board != 0 {
            break;
        }
    }

    let mut f = file;
    while f > 0 {
        f -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
        if (1u64 << Square::from_rank_and_file(rank, f) as u64) & block_board != 0 {
            break;
        }
    }      

    attacks
}

#[cfg(test)]
mod tests {
    use crate::board::{print_board, set_bit};
    use super::*;

    #[test]
    fn test_pawn_attack_mask() {
        let b1 = mask_pawn_attacks(Side::White, Square::C2);
        print_board(&b1);
        assert_eq!(b1, 655360);

        let b2 = mask_pawn_attacks(Side::Black, Square::A1);
        print_board(&b2);
        assert_eq!(b2, 0);

        let b3 = mask_pawn_attacks(Side::Black, Square::A4);
        print_board(&b3);
        assert_eq!(b3, 131072);

        let b4 = mask_pawn_attacks(Side::White, Square::H3);
        print_board(&b4);
        assert_eq!(b4, 1073741824);
    }

    #[test]
    fn test_knight_attack_mask() {
        for i in 0..64 {
            let b = mask_knight_attacks(Square::from(i));
            print_board(&b);
        }
    }

    #[test]
    fn test_king_attack_mask() {
        for i in 0..64 {
            let b = mask_king_attacks(Square::from(i));
            print_board(&b);
        }
    }

    #[test]
    fn test_bishop_attack_mask() {
        for i in 0..64 {
            let b = mask_bishop_attacks(Square::from(i));
            print_board(&b);
        }
    }

    #[test]
    fn test_rook_attack_mask() {
        for i in 0..64 {
            let b = mask_rook_attacks(Square::from(i));
            print_board(&b);
        }
    }

    #[test]
    fn test_blocked_bishop_attacks() {
        let mut blocked_bitboard = 0u64;
        set_bit(&mut blocked_bitboard, Square::C5);
        set_bit(&mut blocked_bitboard, Square::G5);
        set_bit(&mut blocked_bitboard, Square::G1);
        set_bit(&mut blocked_bitboard, Square::D2);

        print_board(&blocked_bitboard);

        let b1 = blocked_bishop_attacks(Square::A3, blocked_bitboard);

        print_board(&b1);
    }

    #[test]
    fn test_blocked_rook_attacks() {
        let mut blocked_bitboard = 0u64;
        set_bit(&mut blocked_bitboard, Square::C3);
        set_bit(&mut blocked_bitboard, Square::H3);
        set_bit(&mut blocked_bitboard, Square::E6);
        set_bit(&mut blocked_bitboard, Square::E2);

        print_board(&blocked_bitboard);

        let b1 = blocked_rook_attacks(Square::E3, blocked_bitboard);

        print_board(&b1);
    }
}
