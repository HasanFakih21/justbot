use crate::board::Board;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub enum MoveGenKind {
    All,
    Quiet,
    Noisy,
    Captures,
    NonCapturePromotions,
}

impl Board {
    pub fn is_attacked_at_by(&self, square: Square, side: Side) -> bool {
        let pawns = self.board_state.board_pieces[(Piece::Pawn as usize) + (side as usize * 6)];
        if (pawns & self.get_pawn_attacks(square, side.other())) != BitBoard(0) {
            return true;
        }

        let knights = self.board_state.board_pieces[(Piece::Knight as usize) + (side as usize * 6)];
        if (knights & self.get_knight_attacks(square)) != BitBoard(0) {
            return true;
        }

        let king = self.board_state.board_pieces[(Piece::King as usize) + (side as usize * 6)];
        if (king & self.get_king_attacks(square)) != BitBoard(0) {
            return true;
        }

        let bishop_queens = self.board_state.board_pieces
            [(Piece::Bishop as usize) + (side as usize * 6)]
            | self.board_state.board_pieces[(Piece::Queen as usize) + (side as usize * 6)];
        if (bishop_queens & self.get_bishop_attacks(square, self.get_all_occupancy()))
            != BitBoard(0)
        {
            return true;
        }

        let rook_queens = self.board_state.board_pieces
            [(Piece::Rook as usize) + (side as usize * 6)]
            | self.board_state.board_pieces[(Piece::Queen as usize) + (side as usize * 6)];
        if (rook_queens & self.get_rook_attacks(square, self.get_all_occupancy())) != BitBoard(0) {
            return true;
        }

        false
    }

    pub fn pawns_with_pushes(&self, side: Side) -> BitBoard {
        let mut empty = !self.get_all_occupancy();
        let pawns = self.board_state.board_pieces[(Piece::Pawn as usize) + (side as usize * 6)];
        let offset = match side {
            Side::White => SOUTH,
            Side::Black => NORTH,
        };

        empty.shift(offset);
        empty & pawns
    }

    pub fn pawns_with_double_pushes(&self, side: Side) -> BitBoard {
        let mut empty = !self.get_all_occupancy();
        let pawns = self.board_state.board_pieces[(Piece::Pawn as usize) + (side as usize * 6)];

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

    pub fn gen_pawn_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;
        let single_push_source = self.pawns_with_pushes(side);
        let double_push_source = self.pawns_with_double_pushes(side);

        let promotion_rank = match side {
            Side::White => BitBoard(RANK_8),
            Side::Black => BitBoard(RANK_1),
        };
        let pawns = self.board_state.board_pieces[(Piece::Pawn as usize) + (side as usize * 6)];
        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let offset = match side {
            Side::White => NORTH,
            Side::Black => SOUTH,
        };

        for source in pawns.iter() {
            if double_push_source.get_bit(source)
                && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
            {
                let target = source.shift(offset * 2).unwrap();
                move_list.push(Move::new(source, target, MoveKind::DoublePawn));
            }

            if single_push_source.get_bit(source) {
                let target = source.shift(offset).unwrap();
                if promotion_rank.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Noisy | MoveGenKind::NonCapturePromotions
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::NPromotion));
                    move_list.push(Move::new(source, target, MoveKind::BPromotion));
                    move_list.push(Move::new(source, target, MoveKind::RPromotion));
                    move_list.push(Move::new(source, target, MoveKind::QPromotion));
                } else if !promotion_rank.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }

            if matches!(
                kind,
                MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
            ) {
                let attacks = self.pawn_attacks[side as usize][source as usize] & opponent_pieces;
                if attacks.0 != 0 {
                    for target in attacks.iter() {
                        if promotion_rank.get_bit(target) {
                            move_list.push(Move::new(source, target, MoveKind::NPromCapture));
                            move_list.push(Move::new(source, target, MoveKind::BPromCapture));
                            move_list.push(Move::new(source, target, MoveKind::RPromCapture));
                            move_list.push(Move::new(source, target, MoveKind::QPromCapture));
                        } else {
                            move_list.push(Move::new(source, target, MoveKind::Capture));
                        }
                    }
                }

                if let Some(target) = self.board_state.enpassant
                    && self.pawn_attacks[side as usize][source as usize].get_bit(target)
                {
                    move_list.push(Move::new(source, target, MoveKind::EnPassant));
                }
            }
        }
    }

    pub fn gen_castling_moves(&self, move_list: &mut MoveList) {
        let side = self.board_state.side_to_move;
        let king = self.board_state.board_pieces[(Piece::King as usize) + (side as usize * 6)];
        let occupancies = self.get_all_occupancy();
        let mut king_side_occ = BitBoard(WK_SIDE);
        let mut queen_side_occ = BitBoard(WQ_SIDE);
        if side == Side::Black {
            king_side_occ.shift(NORTH * 7);
            queen_side_occ.shift(NORTH * 7);
        }
        let need_to_be_safe = (queen_side_occ ^ BitBoard(B_FILE)) & queen_side_occ;

        if self.board_state.castling_rights.can_king_side(side)
            && ((king_side_occ & occupancies).0 == 0)
            && !(king_side_occ | king)
                .iter()
                .any(|e| self.is_attacked_at_by(e, side.other()))
        {
            let target = match side {
                Side::White => Castling::WhiteKing.king_landing_square(),
                Side::Black => Castling::BlackKing.king_landing_square(),
            };
            move_list.push(Move::new(
                king.least_sig_bit().unwrap(),
                target,
                MoveKind::KingCastle,
            ));
        }

        if self.board_state.castling_rights.can_queen_side(side)
            && ((queen_side_occ & occupancies).0 == 0)
            && !(need_to_be_safe | king)
                .iter()
                .any(|e| self.is_attacked_at_by(e, side.other()))
        {
            let target = match side {
                Side::White => Castling::WhiteQueen.king_landing_square(),
                Side::Black => Castling::BlackQueen.king_landing_square(),
            };
            move_list.push(Move::new(
                king.least_sig_bit().unwrap(),
                target,
                MoveKind::QueenCastle,
            ));
        }
    }

    pub fn gen_knight_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;
        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_state.board_occupancies[side as usize];

        for source in
            self.board_state.board_pieces[(Piece::Knight as usize) + (side as usize * 6)].iter()
        {
            let targets = self.get_knight_attacks(source) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::Capture));
                } else if !opponent_pieces.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }
        }
    }

    pub fn gen_bishop_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;
        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_state.board_occupancies[side as usize];

        for source in
            self.board_state.board_pieces[(Piece::Bishop as usize) + (side as usize * 6)].iter()
        {
            let targets =
                self.get_bishop_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::Capture));
                } else if !opponent_pieces.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }
        }
    }

    pub fn gen_rook_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;

        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_state.board_occupancies[side as usize];

        for source in
            self.board_state.board_pieces[(Piece::Rook as usize) + (side as usize * 6)].iter()
        {
            let targets =
                self.get_rook_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::Capture));
                } else if !opponent_pieces.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }
        }
    }

    pub fn gen_queen_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;

        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_state.board_occupancies[side as usize];

        for source in
            self.board_state.board_pieces[(Piece::Queen as usize) + (side as usize * 6)].iter()
        {
            let targets =
                self.get_queen_attacks(source, self.get_all_occupancy()) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::Capture));
                } else if !opponent_pieces.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }
        }
    }

    pub fn gen_king_moves(&self, move_list: &mut MoveList, kind: MoveGenKind) {
        let side = self.board_state.side_to_move;

        let opponent_pieces = self.board_state.board_occupancies[side.other() as usize];
        let friendly_pieces = self.board_state.board_occupancies[side as usize];

        for source in
            self.board_state.board_pieces[(Piece::King as usize) + (side as usize * 6)].iter()
        {
            let targets = self.get_king_attacks(source) & !friendly_pieces;
            for target in targets.iter() {
                if opponent_pieces.get_bit(target)
                    && matches!(
                        kind,
                        MoveGenKind::All | MoveGenKind::Captures | MoveGenKind::Noisy
                    )
                {
                    move_list.push(Move::new(source, target, MoveKind::Capture));
                } else if !opponent_pieces.get_bit(target)
                    && matches!(kind, MoveGenKind::All | MoveGenKind::Quiet)
                {
                    move_list.push(Move::new(source, target, MoveKind::QuietMove));
                }
            }
        }
    }

    pub fn generate_moves(&self, kind: MoveGenKind) -> MoveList {
        let mut move_list = MoveList::new();
        self.gen_pawn_moves(&mut move_list, kind);
        self.gen_knight_moves(&mut move_list, kind);
        self.gen_bishop_moves(&mut move_list, kind);
        self.gen_rook_moves(&mut move_list, kind);
        self.gen_queen_moves(&mut move_list, kind);
        self.gen_king_moves(&mut move_list, kind);
        if matches!(kind, MoveGenKind::All | MoveGenKind::Quiet) {
            self.gen_castling_moves(&mut move_list)
        }
        move_list
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::movegen::MoveGenKind;
    use crate::types::Square::*;
    use crate::types::{BitBoard, Move, MoveKind, STARTING_FEN, Side};
    use Side::*;

    #[test]
    fn test_is_attacked_at_by() {
        let board = Board::from_fen("8/8/8/3p4/8/8/5N2/8 w - - 0 1");
        assert!(board.is_attacked_at_by(C4, Black));
        assert!(board.is_attacked_at_by(E4, Black));
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
    fn test_move_gen_kind() {
        let board = Board::from_fen("1K6/3pp3/4R3/7p/2n5/4b3/PPP1P1P1/6k1 w - - 0 1");
        let captures = board.generate_moves(MoveGenKind::Captures);
        for m in captures.iter() {
            println!("{}", m.mv);
        }
        assert_eq!(captures.len(), 2);
        println!();

        let board = Board::from_fen("1K6/3pp3/4R3/7p/2n5/4b3/PPP1P1P1/6k1 b - - 0 1");
        let captures = board.generate_moves(MoveGenKind::Captures);
        for m in captures.iter() {
            println!("{}", m.mv);
        }
        assert_eq!(captures.len(), 3);
        println!();

        let board = Board::from_fen(STARTING_FEN);
        let all = board.generate_moves(MoveGenKind::All);
        for m in all.iter() {
            println!("{}", m.mv);
        }
        println!();

        let board =
            Board::from_fen("rnbqkb1r/pp3p2/4pnpp/1p1p2N1/1Q1P4/BP2P3/P1PN1PPP/R3K2R b KQkq - 0 1");
        let captures = board.generate_moves(MoveGenKind::Captures);
        let quiet = board.generate_moves(MoveGenKind::Quiet);
        println!("Captures: {captures}");
        println!();
        println!("Quiet Moves: {quiet}");
    }
}
