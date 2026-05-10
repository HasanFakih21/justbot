#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White
        }
    }
}

#[repr(u8)]
pub enum Castling {
    WhiteKing  = 0b0001,
    WhiteQueen = 0b0010,
    BlackKing  = 0b0100,
    BlackQueen = 0b1000,
}

pub struct CastlingRights(u8);

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights(0)
    }

    pub const fn can_king_side(&self, side: Side) -> bool {
        match side {
            Side::White => (Castling::WhiteKing as u8 & self.0) > 0,
            Side::Black => (Castling::BlackKing as u8 & self.0) > 0
        }
    }

    pub const fn can_queen_side(&self, side: Side) -> bool {
        match side {
            Side::White => (Castling::WhiteQueen as u8 & self.0) > 0,
            Side::Black => (Castling::BlackQueen as u8 & self.0) > 0
        }
    }

    pub fn set_king_side(&mut self, side: Side) {
        match side {
            Side::White => self.0 |= Castling::WhiteKing as u8,
            Side::Black => self.0 |= Castling::BlackKing as u8,
        }
    }

    pub fn set_queen_side(&mut self, side: Side) {
        match side {
            Side::White => self.0 |= Castling::WhiteQueen as u8,
            Side::Black => self.0 |= Castling::BlackQueen as u8,
        }
    }

    pub fn set(&mut self, mask: u8) {
        self.0 |= mask;
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights::new()
    }
}