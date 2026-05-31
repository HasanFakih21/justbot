use std::{
    fmt::Display,
    mem::{self, MaybeUninit},
};

use crate::types::{Piece, Square};

#[derive(Debug, Clone, Copy)]
pub struct MoveEntry {
    pub mv: Move,
    pub score: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct MoveList {
    inner: [MaybeUninit<MoveEntry>; 256],
    len: usize,
}

#[derive(Debug, Clone)]
pub struct MoveListError;

impl MoveList {
    pub fn new() -> Self {
        Self {
            inner: [mem::MaybeUninit::uninit(); 256],
            len: 0,
        }
    }

    pub fn push(&mut self, m: Move) {
        self.inner[self.len].write(MoveEntry { mv: m, score: None });
        self.len += 1;
    }

    pub fn replace(&mut self, m: MoveEntry, index: usize) -> MoveEntry {
        if index < self.len {
            let old_move = self.get(index);
            self.inner[index].write(m);
            old_move
        } else {
            panic!("Not a valid index!");
        }
    }

    //Instead of shifting entire list, pop the last element and place it at the removed spot
    pub fn remove(&mut self, index: usize) -> Option<MoveEntry> {
        if index == self.len - 1 {
            return self.pop();
        }

        let last = self.pop().unwrap();
        Some(self.replace(last, index))
    }

    pub fn get(&self, index: usize) -> MoveEntry {
        debug_assert!(index < self.len);
        unsafe { self.inner[index].assume_init() }
    }

    pub fn push_front(&mut self, m: Move) {
        for i in 0..self.len {
            unsafe {
                self.inner[i + 1].write(self.inner[i].assume_init());
            }
        }
        self.len += 1;
        self.inner[0].write(MoveEntry { mv: m, score: None });
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn pop(&mut self) -> Option<MoveEntry> {
        if self.len == 0 {
            None
        } else {
            let e = unsafe { Some(self.inner[self.len - 1].assume_init()) };
            self.len -= 1;
            e
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MoveEntry> {
        unsafe { self.inner[..self.len].assume_init_ref().iter() }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MoveEntry> {
        unsafe { self.inner[..self.len].assume_init_mut().iter_mut() }
    }
}

impl FromIterator<Move> for MoveList {
    fn from_iter<T: IntoIterator<Item = Move>>(iter: T) -> Self {
        let mut ml = MoveList::new();
        for e in iter {
            ml.push(e);
        }

        ml
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for m in self.iter() {
            output = format!("{output}{} ", m.mv);
        }

        write!(f, "{output}")
    }
}

//12 bits for to and from square and 4 bits for move type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move(u16);

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        Move(from as u16 | ((to as u16) << 6) | ((kind as u16) << 12))
    }

    pub const fn get_from(&self) -> Square {
        Square::from((0x003F & self.0) as usize)
    }

    pub const fn get_to(&self) -> Square {
        Square::from(((0x0FC0 & self.0) >> 6) as usize)
    }

    pub const fn get_kind(&self) -> MoveKind {
        MoveKind::from(((0xF000 & self.0) >> 12) as u8)
    }

    pub fn get_promoted_piece(&self) -> Option<Piece> {
        let kind = self.get_kind();
        let mut promoted_piece = None;

        if kind.is_knight_promotion() {
            promoted_piece = Some(Piece::Knight)
        } else if kind.is_bishop_promotion() {
            promoted_piece = Some(Piece::Bishop)
        } else if kind.is_rook_promotion() {
            promoted_piece = Some(Piece::Rook)
        } else if kind.is_queen_promotion() {
            promoted_piece = Some(Piece::Queen)
        }
        promoted_piece
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut promotion_piece = "";
        match self.get_kind() {
            MoveKind::BPromotion | MoveKind::BPromCapture => promotion_piece = "b",
            MoveKind::NPromotion | MoveKind::NPromCapture => promotion_piece = "n",
            MoveKind::RPromotion | MoveKind::RPromCapture => promotion_piece = "r",
            MoveKind::QPromotion | MoveKind::QPromCapture => promotion_piece = "q",
            _ => (),
        }
        write!(f, "{}{}{}", self.get_from(), self.get_to(), promotion_piece)
    }
}

#[derive(Debug)]
pub enum MoveKind {
    QuietMove = 0b0000,
    DoublePawn = 0b0001,
    KingCastle = 0b0010,
    QueenCastle = 0b0011,
    Capture = 0b0100,
    EnPassant = 0b0101,
    NPromotion = 0b1000,
    BPromotion = 0b1001,
    RPromotion = 0b1010,
    QPromotion = 0b1011,
    NPromCapture = 0b1100,
    BPromCapture = 0b1101,
    RPromCapture = 0b1110,
    QPromCapture = 0b1111,
}

impl MoveKind {
    pub const fn from(value: u8) -> Self {
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
            0b1111 => QPromCapture,
            _ => panic!("Not a valid move kind!!"),
        }
    }

    pub const fn is_quiet(&self) -> bool {
        use MoveKind::*;
        matches!(self, QuietMove | DoublePawn | KingCastle | QueenCastle)
    }

    pub const fn is_promotion(&self) -> bool {
        use MoveKind::*;
        matches!(
            self,
            NPromotion
                | BPromotion
                | RPromotion
                | QPromotion
                | NPromCapture
                | BPromCapture
                | RPromCapture
                | QPromCapture
        )
    }

    pub const fn is_knight_promotion(&self) -> bool {
        use MoveKind::*;
        matches!(self, NPromCapture | NPromotion)
    }

    pub const fn is_bishop_promotion(&self) -> bool {
        use MoveKind::*;
        matches!(self, BPromCapture | BPromotion)
    }

    pub const fn is_rook_promotion(&self) -> bool {
        use MoveKind::*;
        matches!(self, RPromCapture | RPromotion)
    }

    pub const fn is_queen_promotion(&self) -> bool {
        use MoveKind::*;
        matches!(self, QPromCapture | QPromotion)
    }

    pub const fn is_capture(&self) -> bool {
        use MoveKind::*;
        matches!(
            self,
            Capture | NPromCapture | BPromCapture | RPromCapture | QPromCapture | EnPassant
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        board::{Board, movegen::MoveGenKind},
        types::{Move, MoveKind, STARTING_FEN, Square},
    };

    #[test]
    fn test_move_list() {
        let board = Board::from_fen(STARTING_FEN);
        let mut move_list = board.generate_moves(MoveGenKind::All);
        assert_eq!(move_list.len(), 20);
        let m = Move::new(Square::A1, Square::A3, MoveKind::QuietMove);
        move_list.push_front(m);
        println!("{move_list}");
        assert_eq!(move_list.get(0).mv, m);
        assert_eq!(move_list.len(), 21);
    }
}
