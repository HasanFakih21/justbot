use crate::{board::{Piece, Side, Square, constants::{KING_SIDE_ROOK_BLACK, KING_SIDE_ROOK_WHITE, QUEEN_SIDE_ROOK_BLACK, QUEEN_SIDE_ROOK_WHITE}, moves::{Move, MoveKind}}, zobrist::ZOBRIST};
use super::Board;

pub struct LegalMove;
pub struct IllegalMove;



impl Board {
    pub fn make_move(&mut self, m: Move) -> Result<LegalMove, IllegalMove> {
        let from = m.get_from();
        let to = m.get_to();
        let kind = m.get_kind();
        let (side, piece) = self.get_piece_at_square(from).ok_or(IllegalMove)?;

        let king_rook_square = match side {Side::White => KING_SIDE_ROOK_WHITE, Side::Black => KING_SIDE_ROOK_BLACK};
        let queen_rook_square = match side {Side::White => QUEEN_SIDE_ROOK_WHITE, Side::Black => QUEEN_SIDE_ROOK_BLACK};
        let opp_king_rook_square = match side {Side::White => KING_SIDE_ROOK_BLACK, Side::Black => KING_SIDE_ROOK_WHITE};
        let opp_queen_rook_square = match side {Side::White => QUEEN_SIDE_ROOK_BLACK, Side::Black => QUEEN_SIDE_ROOK_WHITE};

        self.copy_state();

        self.board_state.hash ^= ZOBRIST.get_castling_num(self.board_state.castling_rights);

        if let Some(square) = self.board_state.enpassant {
            self.board_state.hash ^= ZOBRIST.get_enpassant_num(square);
            self.board_state.enpassant = None;
        }

        if let Piece::King = piece {
            self.board_state.castling_rights.clear_king_side(side);
            self.board_state.castling_rights.clear_queen_side(side);
        }

        self.board_state.side_to_move = self.board_state.side_to_move.other();
        self.board_state.hash ^= ZOBRIST.get_side_num();

        if let Piece::Rook = piece {
            if from == king_rook_square  && self.board_state.castling_rights.can_king_side(side)  {self.board_state.castling_rights.clear_king_side(side);}
            if from == queen_rook_square && self.board_state.castling_rights.can_queen_side(side) {self.board_state.castling_rights.clear_queen_side(side);}
        }

        if kind.is_quiet() {
            match kind {
                MoveKind::KingCastle => {
                    self.remove_piece(side, piece, from);
                    self.remove_piece(side, Piece::Rook, king_rook_square);
                    self.board_state.castling_rights.clear_king_side(side);
                    self.board_state.castling_rights.clear_queen_side(side);
                    self.place_piece(side, piece, to);
                    self.place_piece(side, Piece::Rook, from.shift(1).unwrap());
                },
                MoveKind::QueenCastle => {
                    self.remove_piece(side, piece, from);
                    self.remove_piece(side, Piece::Rook, queen_rook_square);
                    self.board_state.castling_rights.clear_queen_side(side);
                    self.board_state.castling_rights.clear_king_side(side);
                    self.place_piece(side, piece, to);
                    self.place_piece(side, Piece::Rook, from.shift(-1).unwrap());
                },
                MoveKind::DoublePawn => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                    self.board_state.enpassant = Some(Square::from(to as usize ^ 8));
                    self.board_state.hash ^= ZOBRIST.get_enpassant_num(Square::from(to as usize ^ 8));
                },
                _=> {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                }
            }
        }

        else {
            if let Some((other_side, captured_piece)) = self.get_piece_at_square(to)
                && captured_piece == Piece::Rook {
                    if to == opp_king_rook_square {self.board_state.castling_rights.clear_king_side(other_side);}
                    if to == opp_queen_rook_square {self.board_state.castling_rights.clear_queen_side(other_side);}
                }
            match kind {
                MoveKind::EnPassant => {
                    let pawn_square = Square::from(to as usize ^ 8);
                    let (other_side, captured_piece) = self.get_piece_at_square(pawn_square).unwrap();
                    self.remove_piece(other_side, captured_piece, pawn_square);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                },
                MoveKind::BPromotion => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Bishop, to);
                },
                MoveKind::NPromotion => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Knight, to);
                },
                MoveKind::RPromotion => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Rook, to);
                },
                MoveKind::QPromotion => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Queen, to);
                },
                MoveKind::BPromCapture => {
                    let (other_side, captured_piece) = self.get_piece_at_square(to).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Bishop, to);
                },
                MoveKind::NPromCapture => {
                    let (other_side, captured_piece) = self.get_piece_at_square(to).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Knight, to);
                },
                MoveKind::RPromCapture => {
                    let (other_side, captured_piece) = self.get_piece_at_square(to).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Rook, to);
                },
                MoveKind::QPromCapture => {
                    let (other_side, captured_piece) = self.get_piece_at_square(to).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, Piece::Queen, to);
                },
                _=> {
                    let (other_side, captured_piece) = self.get_piece_at_square(to).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                }
            }
        }

        //Irreversible Move
        if kind.is_capture() || piece == Piece::Pawn {self.board_state.half_move_clock = 0} else {self.board_state.half_move_clock += 1}

        if self.board_state.side_to_move == Side::White {self.board_state.full_move += 1}
        self.board_state.hash ^= ZOBRIST.get_castling_num(self.board_state.castling_rights);
        self.board_state.game_history.push(self.board_state.hash);

        if self.is_king_in_attack(side) {
            self.unmake_move();
            Err(IllegalMove)
        } else {
            Ok(LegalMove)
        }
    }

    pub fn unmake_move(&mut self) {
        if let Some(prev_state) = self.state_stack.pop() {
            self.board_state = prev_state;
        }
    }

    pub fn copy_state(&mut self) {
        self.state_stack.push(self.board_state.clone());
    }

    pub fn is_king_in_attack(&self, side: Side) -> bool {
        let king_square = self.board_state.board_pieces[(Piece::King as usize) + (side as usize * 6)].least_sig_bit().unwrap();
        self.is_attacked_at_by(king_square, side.other())
    }

    pub fn king_in_check(&self) -> bool {
        self.is_king_in_attack(Side::White) || self.is_king_in_attack(Side::Black)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Piece, Side, Square, moves::{Move, MoveKind}};

    #[test]
    fn test_make_move() {
        let mut board = Board::from_fen("1K6/3pp1P1/4R3/3k3p/Ppn5/4b3/1PP1P1p1/7B b - a3 0 1");
        println!("{board}");

        let m = Move::new(Square::B4, Square::A3, MoveKind::EnPassant);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::A3).unwrap(), (Side::Black, Piece::Pawn));
        assert!(board.get_piece_at_square(Square::A4).is_none());

        board.unmake_move();
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::B4).unwrap(), (Side::Black, Piece::Pawn));

        let m = Move::new(Square::C4, Square::B2, MoveKind::Capture);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::B2).unwrap(), (Side::Black, Piece::Knight));

        let mut board = Board::from_fen("r3k2r/pppqn2p/n1bp2pb/1N2p3/2B5/1QP1PN2/PP1B1PPP/R3K2R w KQkq - 10 12");
        println!("{board}");

        let m = Move::new(Square::E1, Square::G1, MoveKind::KingCastle);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::G1).unwrap(), (Side::White, Piece::King));
        assert_eq!(board.get_piece_at_square(Square::F1).unwrap(), (Side::White, Piece::Rook));
        assert!(board.get_piece_at_square(Square::E1).is_none());

        let m = Move::new(Square::E8, Square::C8, MoveKind::QueenCastle);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::C8).unwrap(), (Side::Black, Piece::King));
        assert_eq!(board.get_piece_at_square(Square::D8).unwrap(), (Side::Black, Piece::Rook));
        assert!(board.get_piece_at_square(Square::E8).is_none());

        let mut board = Board::from_fen("2kr3r/pppqn2p/n1b3pb/1N2p3/2B5/1QP4N/PP2pPPP/R1B2R1K b - - 1 17");
        println!("{board}");

        let m = Move::new(Square::E2, Square::E1, MoveKind::BPromotion);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::E1).unwrap(), (Side::Black, Piece::Bishop));
        assert!(board.get_piece_at_square(Square::E2).is_none());

        board.unmake_move();
        let m = Move::new(Square::E2, Square::F1, MoveKind::QPromCapture);
        let _ = board.make_move(m);
        println!("{board}");
        assert_eq!(board.get_piece_at_square(Square::F1).unwrap(), (Side::Black, Piece::Queen));
        assert!(board.get_piece_at_square(Square::E2).is_none());

        let mut board = Board::from_fen("2kr3r/pppqn2p/n1b3pb/1N2p3/2B5/1QP4N/PP3PPP/R1B2q1K w - - 0 18");
        println!("{board}");

        let m = Move::new(Square::C1, Square::H6, MoveKind::Capture);
        assert!(board.make_move(m).is_err());
        board.unmake_move();
        let m = Move::new(Square::H3, Square::G1, MoveKind::QuietMove);
        assert!(board.make_move(m).is_ok());

        let mut board = Board::from_fen("r3k2N/p1ppqpb1/bn2pn2/3P4/1p2P3/2N2Q2/PPPBBPpP/R3K2R b KQq - 0 2");
        println!("{board}");

        let m = Move::new(Square::G2, Square::H1, MoveKind::NPromCapture);
        assert!(board.make_move(m).is_ok());
        assert!(!board.board_state.castling_rights.can_king_side(Side::White));
        println!("{board}");
    }
}