use crate::board::bitboard::BitBoard;
use crate::{Side, Square};
use crate::board::constants::*;

pub fn mask_pawn_attacks(side: Side, square: Square) -> BitBoard {
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

    BitBoard(top_left | top_right)
}

pub fn mask_knight_attacks(square: Square) -> BitBoard {
    let current = 1u64 << square as u64;

    let tl1 = if (current & A_FILE) == 0 {current << 15} else {0};
    let tl2 = if (current & (A_FILE | B_FILE)) == 0 {current << 6} else {0};

    let bl1 = if (current & (A_FILE | B_FILE)) == 0 {current >> 10} else {0};
    let bl2 = if (current & A_FILE) == 0 {current >> 17} else {0};

    let tr1 = if (current & H_FILE) == 0 {current << 17} else {0};
    let tr2 = if (current & (H_FILE | G_FILE)) == 0 {current << 10} else {0};

    let br1 = if (current & (H_FILE | G_FILE)) == 0 {current >> 6} else {0};
    let br2 = if (current & H_FILE) == 0 {current >> 15} else {0};

    BitBoard(tl1 | tl2 | bl1 | bl2 | tr1 | tr2 | br1 | br2)
}

pub fn mask_king_attacks(square: Square) -> BitBoard {
    let current = 1u64 << square as u64;
    let n = current << 8;
    let nw = if current & A_FILE == 0 {current << 7} else {0};
    let w = if current & A_FILE == 0 {current >> 1} else {0};
    let sw = if current & A_FILE == 0 {current >> 9} else {0};
    let s = current >> 8;
    let se = if current & H_FILE == 0 {current >> 7} else {0};
    let e = if current & H_FILE == 0 {current << 1} else {0};
    let ne = if current & H_FILE == 0 {current << 9} else {0};

    BitBoard(n | nw | w | sw | s | se | e | ne)
}

pub fn mask_bishop_attacks(square: Square) -> BitBoard {
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
    
    BitBoard(attacks)
}

pub fn mask_rook_attacks(square: Square) -> BitBoard {
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

    BitBoard(attacks)  
}

pub fn blocked_bishop_attacks(square: Square, block_board: BitBoard) -> BitBoard {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();

    let (mut r, mut f) = (rank, file);

    while r < 7 && f < 7 {
        r += 1;
        f += 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r > 0 && f < 7 {
        r -= 1;
        f += 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r < 7 && f > 0 {
        r += 1;
        f -= 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let (mut r, mut f) = (rank, file);
    while r > 0 && f > 0 {
        r -= 1;
        f -= 1;

        attacks |= 1u64 << Square::from_rank_and_file(r, f) as u64;
        if (1u64 << Square::from_rank_and_file(r, f) as u64) & block_board.0 != 0 {
            break;
        }
    }
    
    BitBoard(attacks)
}

pub fn blocked_rook_attacks(square: Square, block_board: BitBoard) -> BitBoard {
    let mut attacks = 0u64;
    let (rank, file) = square.to_rank_and_file();
    
    let mut r = rank;
    while r > 0 {
        r -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
        if (1u64 << Square::from_rank_and_file(r, file) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let mut r = rank;
    while r < 7 {
        r += 1;
        attacks |= 1u64 << Square::from_rank_and_file(r, file) as u64;
        if (1u64 << Square::from_rank_and_file(r, file) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let mut f = file;
    while f < 7 {
        f += 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
        if (1u64 << Square::from_rank_and_file(rank, f) as u64) & block_board.0 != 0 {
            break;
        }
    }

    let mut f = file;
    while f > 0 {
        f -= 1;
        attacks |= 1u64 << Square::from_rank_and_file(rank, f) as u64;
        if (1u64 << Square::from_rank_and_file(rank, f) as u64) & block_board.0 != 0 {
            break;
        }
    }      

    BitBoard(attacks)
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::BitBoard;
    use super::*;

    #[test]
    fn test_pawn_attack_mask() {
        let b1 = mask_pawn_attacks(Side::White, Square::C2);
        b1.print_board();
        assert_eq!(b1, BitBoard(655360));

        let b2 = mask_pawn_attacks(Side::Black, Square::A1);
        b2.print_board();
        assert_eq!(b2, BitBoard(0));

        let b3 = mask_pawn_attacks(Side::Black, Square::A4);
        b3.print_board();
        assert_eq!(b3, BitBoard(131072));

        let b4 = mask_pawn_attacks(Side::White, Square::H3);
        b4.print_board();
        assert_eq!(b4, BitBoard(1073741824));
    }

    #[test]
    fn test_knight_attack_mask() {
        for i in 0..64 {
            let b = mask_knight_attacks(Square::from(i));
            b.print_board();
        }
    }

    #[test]
    fn test_king_attack_mask() {
        for i in 0..64 {
            let b = mask_king_attacks(Square::from(i));
            b.print_board();
        }
    }

    #[test]
    fn test_bishop_attack_mask() {
        for i in 0..64 {
            let b = mask_bishop_attacks(Square::from(i));
            b.print_board();
        }
    }

    #[test]
    fn test_rook_attack_mask() {
        for i in 0..64 {
            let b = mask_rook_attacks(Square::from(i));
            b.print_board();
        }
    }

    #[test]
    fn test_blocked_bishop_attacks() {
        let mut blocked_bitboard = BitBoard(0u64);
        blocked_bitboard.set_bit(Square::C5);
        blocked_bitboard.set_bit(Square::G5);
        blocked_bitboard.set_bit(Square::G1);
        blocked_bitboard.set_bit(Square::D2);

        blocked_bitboard.print_board();

        let b1 = blocked_bishop_attacks(Square::A3, blocked_bitboard);

        b1.print_board();
    }

    #[test]
    fn test_blocked_rook_attacks() {
        let mut blocked_bitboard = BitBoard(0u64);
        blocked_bitboard.set_bit(Square::C3);
        blocked_bitboard.set_bit(Square::H3);
        blocked_bitboard.set_bit(Square::E6);
        blocked_bitboard.set_bit(Square::E2);

        blocked_bitboard.print_board();

        let b1 = blocked_rook_attacks(Square::E3, blocked_bitboard);

        b1.print_board();
    }
}
