use std::{fmt::Display, slice::Iter};

use crate::board::{Board, Castling, Piece, Side, Square, bitboard::BitBoard, constants::{B_FILE, NORTH, RANK_1, RANK_4, RANK_5, RANK_8, SOUTH, WK_SIDE, WQ_SIDE}};

#[derive(Default, Debug)]
pub struct MoveList(Vec<Move>);

impl MoveList {
    pub fn new() -> Self {
        MoveList(Vec::new())
    }

    pub fn add(&mut self, m: Move) {
        self.0.push(m);
    }

    pub fn iter(&self) -> Iter<'_, Move> {
        self.0.iter()
    }
}

//12 bits for to and from square and 4 bits for move type
#[derive(Debug)]
pub struct Move(u16);

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        Move(from as u16 | ((to as u16) << 6) | ((kind as u16) << 12))
    }

    pub fn get_from(&self) -> Square {
        Square::from((0x003F & self.0) as usize)
    }

    pub fn get_to(&self) -> Square {
        Square::from(((0x0FC0 & self.0) >> 6) as usize)
    }

    pub fn get_kind(&self) -> MoveKind {
        MoveKind::from(((0xF000 & self.0) >> 12) as u8)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.get_from(), self.get_to(), self.get_kind())
    }
}

#[derive(Debug)]
pub enum MoveKind {
    QuietMove    = 0b0000,
    DoublePawn   = 0b0001,
    KingCastle   = 0b0010,
    QueenCastle  = 0b0011,
    Capture      = 0b0100,
    EnPassant    = 0b0101,
    NPromotion   = 0b1000,
    BPromotion   = 0b1001,
    RPromotion   = 0b1010,
    QPromotion   = 0b1011,
    NPromCapture = 0b1100,
    BPromCapture = 0b1101,
    RPromCapture = 0b1110,
    QPromCapture = 0b1111,
}

impl MoveKind {
    fn from(value: u8) -> Self {
        use MoveKind::*;
        match value {
            0b0000 => QuietMove, 
            0b0001 => DoublePawn, 
            0b0010 => KingCastle, 
            0b0011 => QueenCastle, 
            0b0100 => Capture, 
            0b0101 => EnPassant, 
            0b1000 => NPromotion, 
            0b1001 => BPromotion, 
            0b1010 => RPromotion, 
            0b1011 => QPromotion, 
            0b1100 => NPromCapture, 
            0b1101 => BPromCapture, 
            0b1110 => RPromCapture, 
            0b1111 => QPromCapture , 
            _ => panic!("Not a valid move kind!!")
        }
    }
}

impl Board {
    pub fn is_attacked_at_by(&self, square: Square, side: Side) -> bool {
        let pawns = self.board_pieces[side as usize][Piece::Pawn as usize];
        if (pawns & self.get_pawn_attacks(square, side.other())) != BitBoard(0) {return true}

        let knights = self.board_pieces[side as usize][Piece::Knight as usize];
        if (knights & self.get_knight_attacks(square)) != BitBoard(0) {return true}

        let king = self.board_pieces[side as usize][Piece::King as usize];
        if (king & self.get_king_attacks(square)) != BitBoard(0) {return true}

        let bishop_queens = self.board_pieces[side as usize][Piece::Bishop as usize] | self.board_pieces[side as usize][Piece::Queen as usize];
        if (bishop_queens & self.get_bishop_attacks(square, self.get_all_occupancy())) != BitBoard(0) {return true}

        let rook_queens = self.board_pieces[side as usize][Piece::Rook as usize] | self.board_pieces[side as usize][Piece::Queen as usize];
        if (rook_queens & self.get_rook_attacks(square, self.get_all_occupancy())) != BitBoard(0) {return true}

        false
    }

    pub fn pawns_with_pushes(&self, side: Side) -> BitBoard {
        let mut empty = !self.get_all_occupancy();
        let pawns = self.board_pieces[side as usize][Piece::Pawn as usize];
        let offset = match side {
            Side::White => SOUTH,
            Side::Black => NORTH,
        };

        empty.shift(offset);
        empty & pawns
    }

    pub fn pawns_with_double_pushes(&self, side: Side) -> BitBoard {
        let mut empty = !self.get_all_occupancy();
        let pawns = self.board_pieces[side as usize][Piece::Pawn as usize];

        let offset = match side {
            Side::White => SOUTH,
            Side::Black => NORTH,
        };

        let mut second_rank = match side {
            Side::White => empty & BitBoard(RANK_4),
            Side::Black => empty & BitBoard(RANK_5),
        };

        second_rank.shift(offset);
        empty &= second_rank;
        empty.shift(offset);
        empty & pawns
    }

    pub fn gen_pawn_moves(&self) {
        let side = self.side_to_move;
        let single_push_source = self.pawns_with_pushes(side);
        let double_push_source = self.pawns_with_double_pushes(side);
        //let movelist = MoveList::new();
        let promotion_rank = match side {Side::White => BitBoard(RANK_8), Side::Black => BitBoard(RANK_1)};
        let pawns = self.board_pieces[side as usize][Piece::Pawn as usize];
        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let offset = match side {Side::White => NORTH, Side::Black => SOUTH};

        for source in pawns.iter() {
            if double_push_source.get_bit(source) {
                let target = source.shift(offset * 2).unwrap();
                println!("From: {:?} To: {:?}", source, target);
            }

            if single_push_source.get_bit(source) {
                let target = source.shift(offset).unwrap();
                if promotion_rank.get_bit(target) {
                    println!("Knight promotion from {:?} to {:?}", source, target);
                    println!("Bishop promotion from {:?} to {:?}", source, target);
                    println!("Rook promotion from {:?} to {:?}", source, target);
                    println!("Queen promotion from {:?} to {:?}", source, target);
                } else {
                    println!("From: {:?} To: {:?}", source, target);
                }
            }

            let attacks = self.pawn_attacks[side as usize][source as usize] & opponent_pieces;
            if attacks.0 != 0 {
                for target in attacks.iter() {
                    if promotion_rank.get_bit(target) {
                        println!("Knight promotion capture from {:?} to {:?}", source, target);
                        println!("Bishop promotion capture from {:?} to {:?}", source, target);
                        println!("Rook promotion capture from {:?} to {:?}", source, target);
                        println!("Queen promotion capture from {:?} to {:?}", source, target);
                    } else {
                        println!("Capture from: {:?} To: {:?}", source, target);
                    }
                }
            }

            if let Some(target) = self.enpassant && self.pawn_attacks[side as usize][source as usize].get_bit(target) {
                println!("Enpassant capture from {:?} to {:?}", source, target);
            }
        }
    }

    pub fn gen_castling_moves(&self) {
        let side = self.side_to_move;
        let king = self.board_pieces[side as usize][Piece::King as usize];
        let occupancies = self.get_all_occupancy();
        let mut king_side_occ = BitBoard(WK_SIDE);
        let mut queen_side_occ = BitBoard(WQ_SIDE);
        if side == Side::Black {king_side_occ.shift(NORTH * 7); queen_side_occ.shift(NORTH * 7);}
        let need_to_be_safe = (queen_side_occ ^ BitBoard(B_FILE)) & queen_side_occ;

        if self.castling_rights.can_king_side(side) && 
            ((king_side_occ & occupancies).0 == 0)  && 
            !(king_side_occ | king).iter().any(|e| self.is_attacked_at_by(e, side.other()))
        {
            let target = match side {
                Side::White => Castling::WhiteKing.king_landing_square(),
                Side::Black => Castling::BlackKing.king_landing_square(),
            };
            println!("King side castle from: {:?} To: {:?}", king.least_sig_bit().unwrap(), target);
        }

        if self.castling_rights.can_queen_side(side) &&
            ((queen_side_occ & occupancies).0 == 0)  &&
            !(need_to_be_safe | king).iter().any(|e| self.is_attacked_at_by(e, side.other()))
        {
            let target = match side {
                Side::White => Castling::WhiteQueen.king_landing_square(),
                Side::Black => Castling::BlackQueen.king_landing_square(),
            };
            println!("Queen side castle from: {:?} To: {:?}", king.least_sig_bit().unwrap(), target);
        }
    }

    pub fn gen_knight_moves(&self) {
        let side = self.side_to_move;
        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_occupancies[side as usize];

        for source in self.board_pieces[side as usize][Piece::Knight as usize].iter() {
            let targets = self.get_knight_attacks(source) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target) {
                    println!("Knight capture from {:?} to {:?}", source, target);
                } else {
                    println!("Knight quiet move from {:?} to {:?}", source, target);
                }
            }
        } 
    }

    pub fn gen_bishop_moves(&self) {
        let side = self.side_to_move;

        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_occupancies[side as usize];

        for source in self.board_pieces[side as usize][Piece::Bishop as usize].iter() {
            let targets = self.get_bishop_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target) {
                    println!("Bishop capture from {:?} to {:?}", source, target);
                } else {
                    println!("Bishop quiet move from {:?} to {:?}", source, target);
                }
            }
        } 
    }

    pub fn gen_rook_moves(&self) {
        let side = self.side_to_move;

        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_occupancies[side as usize];

        for source in self.board_pieces[side as usize][Piece::Rook as usize].iter() {
            let targets = self.get_rook_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target) {
                    println!("Rook capture from {:?} to {:?}", source, target);
                } else {
                    println!("Rook quiet move from {:?} to {:?}", source, target);
                }
            }
        } 
    }

    pub fn gen_queen_moves(&self) {
        let side = self.side_to_move;

        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_occupancies[side as usize];

        for source in self.board_pieces[side as usize][Piece::Queen as usize].iter() {
            let targets = self.get_queen_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target) {
                    println!("Queen capture from {:?} to {:?}", source, target);
                } else {
                    println!("Queen quiet move from {:?} to {:?}", source, target);
                }
            }
        } 
    }

    pub fn gen_king_moves(&self) {
        let side = self.side_to_move;

        let opponent_pieces = self.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_occupancies[side as usize];

        for source in self.board_pieces[side as usize][Piece::King as usize].iter() {
            let targets = self.get_king_attacks(source) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target) {
                    println!("King capture from {:?} to {:?}", source, target);
                } else {
                    println!("King quiet move from {:?} to {:?}", source, target);
                }
            }
        } 
    }
}

#[cfg(test)]
mod tests {
    use crate::board::BitBoard;
    use super::*;
    use Square::*;
    use Side::*;

    #[test]
    fn test_is_attacked_at_by() {
        let board = Board::from_fen("8/8/8/3p4/8/8/5N2/8 w - - 0 1");
        assert!(board.is_attacked_at_by(C4, Black));
        assert!(board.is_attacked_at_by(D3, White));
        assert!(!board.is_attacked_at_by(F2, Black));

        let board2 = Board::from_fen("6Q1/8/2R5/8/5b2/1q6/8/6K1 w - - 0 1");
        assert!(board2.is_attacked_at_by(C3, White));
        assert!(board2.is_attacked_at_by(B3, White));
        assert!(board2.is_attacked_at_by(F1, White));
        assert!(board2.is_attacked_at_by(A2, Black));
        assert!(board2.is_attacked_at_by(H6, Black));
        assert!(!board2.is_attacked_at_by(F5, Black));
        assert!(!board2.is_attacked_at_by(F5, White));
    }
    
    #[test]
    fn test_move_create() {
        let from = A2;
        let to = A4;
        let kind = MoveKind::DoublePawn;

        let m = Move::new(from, to, kind);
        println!("{:?}, {:?}, {:?}", m.get_from(), m.get_to(), m.get_kind());
    }

    #[test]
    fn test_source_pawn_push() {
        let board = Board::from_fen("1K6/3pp3/4R3/7p/2n5/4b3/PPP1P1P1/8 w - - 0 1");
        let w_bb = board.pawns_with_pushes(White);
        let b_bb = board.pawns_with_pushes(Black);

        w_bb.print_board();
        b_bb.print_board();

        let mut w_ver = BitBoard(0);
        w_ver.set_bit(A2);
        w_ver.set_bit(B2);
        w_ver.set_bit(G2);
        w_ver.set_bit(C2);

        let mut b_ver = BitBoard(0);
        b_ver.set_bit(D7);
        b_ver.set_bit(H5);

        assert_eq!(w_bb, w_ver);
        assert_eq!(b_bb, b_ver);
    }

    #[test]
    fn test_source_double_push() {
        let board = Board::from_fen("1K6/3pp3/4R3/7p/2n5/4b3/PPP1P1P1/8 w - - 0 1");
        println!("{}", board);
        let w_bb = board.pawns_with_double_pushes(White);
        let b_bb = board.pawns_with_double_pushes(Black);

        let mut w_ver = BitBoard(0);
        w_ver.set_bit(A2);
        w_ver.set_bit(B2);
        w_ver.set_bit(G2);

        let mut b_ver = BitBoard(0);
        b_ver.set_bit(D7);

        assert_eq!(w_bb, w_ver);
        assert_eq!(b_bb, b_ver);
    }

    #[test]
    fn test_pawn_move_gen() {
        let board = Board::from_fen("1K6/3pp1P1/4R3/3k3p/Ppn5/4b3/1PP1P1p1/7B b - a3 0 1");

        board.gen_pawn_moves();
    }

    #[test]
    fn test_castling_move_gen() {
        let board = Board::from_fen("r3k2r/p1q1n2p/nQpp2pb/8/2B1N3/2P1P3/PP1B1PPP/R3K2R b KQkq - 2 17");

        board.gen_castling_moves();
    }

    #[test]
    fn test_knight_move_gen() {
        let board = Board::from_fen("r3k2r/p2q3p/nQp3pb/2Np1n2/8/1BP1P3/PP1B1PPP/R3K2R w KQkq - 4 20");

        board.gen_knight_moves();
    }

    #[test]
    fn test_bishop_move_gen() {
        let board = Board::from_fen("r3k2r/p2q3p/nQp3pb/2Np1n2/8/1BP1P3/PP1B1PPP/R3K2R w KQkq - 4 20");

        board.gen_bishop_moves();
    }

    #[test]
    fn test_general_move_gen() {
        let board = Board::from_fen("r3k2r/p2q3p/nQp3pb/2Np1n2/8/1BP1P3/PP1B1PPP/R3K2R w KQkq - 4 20");

        board.gen_queen_moves();
    }
}