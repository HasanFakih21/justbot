use crate::board::{CastlingRights, Piece, Side, Square, bitboard::BitBoard, constants::{KING_SIDE_ROOK_BLACK, KING_SIDE_ROOK_WHITE, QUEEN_SIDE_ROOK_BLACK, QUEEN_SIDE_ROOK_WHITE}, moves::{Move, MoveKind, MoveList}};
use super::Board;

pub struct BoardState {
    pub board_pieces: [[BitBoard; 6]; 2],
    pub pieces_on_squares: [Option<(Side, Piece)>; 64],
    pub board_occupancies: [BitBoard; 2],
    pub side_to_move: Side,
    pub enpassant: Option<Square>,
    pub castling_rights: CastlingRights,
    pub move_list: MoveList,
}

impl Board {
    pub fn make_move(&mut self, m: Move) {
        let from = m.get_from();
        let to = m.get_to();
        let kind = m.get_kind();
        let (side, piece) = self.get_piece_at_square(from).unwrap();
        let king_rook_square = match side {Side::White => KING_SIDE_ROOK_WHITE, Side::Black => KING_SIDE_ROOK_BLACK};
        let queen_rook_square = match side {Side::White => QUEEN_SIDE_ROOK_WHITE, Side::Black => QUEEN_SIDE_ROOK_BLACK};
        self.copy_state();

        self.enpassant = None;
        if let Piece::King = piece {
            self.castling_rights.clear_king_side(side);
            self.castling_rights.clear_queen_side(side);
        }
        self.side_to_move = self.side_to_move.other();
        if let Piece::Rook = piece {
            if from == king_rook_square  && self.castling_rights.can_king_side(side)  {self.castling_rights.clear_king_side(side);}
            if from == queen_rook_square && self.castling_rights.can_queen_side(side) {self.castling_rights.clear_queen_side(side);}
        }

        if kind.is_quiet() {
            match kind {
                MoveKind::KingCastle => {
                    self.remove_piece(side, piece, from);
                    self.remove_piece(side, Piece::Rook, king_rook_square);
                    self.castling_rights.clear_king_side(side);
                    self.castling_rights.clear_queen_side(side);
                    self.place_piece(side, piece, to);
                    self.place_piece(side, Piece::Rook, from.shift(1).unwrap());
                },
                MoveKind::QueenCastle => {
                    self.remove_piece(side, piece, from);
                    self.remove_piece(side, Piece::Rook, queen_rook_square);
                    self.castling_rights.clear_queen_side(side);
                    self.castling_rights.clear_king_side(side);
                    self.place_piece(side, piece, to);
                    self.place_piece(side, Piece::Rook, from.shift(-1).unwrap());
                },
                MoveKind::DoublePawn => {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                    self.enpassant = Some(Square::from(to as usize ^ 8))
                },
                _=> {
                    self.remove_piece(side, piece, from);
                    self.place_piece(side, piece, to);
                }
            }
        }

        else {
            match kind {
                MoveKind::EnPassant => {
                    let (other_side, captured_piece) = self.get_piece_at_square(Square::from(to as usize ^ 8)).unwrap();
                    self.remove_piece(other_side, captured_piece, to);
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
    }

    pub fn unmake_move(&mut self) {
        if let Some(prev_state) = self.state_stack.pop() {
            self.board_pieces = prev_state.board_pieces;
            self.pieces_on_squares = prev_state.pieces_on_squares;
            self.board_occupancies = prev_state.board_occupancies;
            self.side_to_move = prev_state.side_to_move;
            self.enpassant = prev_state.enpassant;
            self.castling_rights = prev_state.castling_rights;
            self.move_list = prev_state.move_list;
        }
    }

    pub fn copy_state(&mut self) {
        self.state_stack.push(
            BoardState {
                board_pieces: self.board_pieces,
                pieces_on_squares: self.pieces_on_squares,
                board_occupancies: self.board_occupancies,
                side_to_move: self.side_to_move,
                enpassant: self.enpassant,
                castling_rights: self.castling_rights,
                move_list: self.move_list.clone(),
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{Board, Square, moves::{Move, MoveKind}};

    #[test]
    fn test_make_move() {
        let mut board = Board::from_fen("r3k2r/p3n2p/n1pp2pb/1p2p1N1/8/2PBP3/PP1B1PPP/R3K2R w KQkq b6 0 16");
        println!("{board}");

        let m = Move::new(Square::A2, Square::A4, MoveKind::DoublePawn);
        board.make_move(m);

        println!("{board}");

        board.unmake_move();
        println!("{board}");
    }
}