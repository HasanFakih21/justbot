#[derive(Debug)]
pub struct InvalidSquare;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8
}

impl TryFrom<usize> for Square {
    type Error = InvalidSquare;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= 63 {
            Ok(unsafe { std::mem::transmute::<u8, Square>(value as u8) })
        } else {
            Err(InvalidSquare)
        }
    }
}

impl TryFrom<i8> for Square {
    type Error = InvalidSquare;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if (0..=63).contains(&value) {
            Ok(unsafe { std::mem::transmute::<u8, Square>(value as u8) })
        } else {
            Err(InvalidSquare)
        }
    }
}

impl TryFrom<&str> for Square {
    type Error = InvalidSquare;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut i = value.chars();
        let mut file = i.next().ok_or(InvalidSquare)? as u8;
        let mut rank = i.next().ok_or(InvalidSquare)? as u8;

        if (b'a'..b'h').contains(&file) && (b'1'..b'8').contains(&rank) {
            file -= b'a';
            rank -= b'1';
        }
        else {
            return Err(InvalidSquare);
        }

        Square::try_from(((rank * 8) + file) as usize)
    }
}

impl Square {
    pub const fn from(value: usize) -> Self {
        debug_assert!(value < 64);
        unsafe { std::mem::transmute(value as u8) }
    }

    pub const fn to_rank_and_file(&self) -> (usize, usize) {
        (*self as usize / 8, *self as usize % 8)
    }

    pub const fn from_rank_and_file(rank: usize, file: usize) -> Square {
        Square::from((rank * 8) + file)
    }

    pub fn shift(&self, offset: i8) -> Option<Square> {
        Square::try_from((*self as i8) + offset).ok() 
    }
}