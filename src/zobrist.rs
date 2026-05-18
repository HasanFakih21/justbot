use std::{array, sync::Mutex};

use crate::magics::get_random_u64_num;


#[derive(Debug)]
pub struct Zobrist {
    pieces: [[u64; 64]; 12],
    side: u64,
    castling: [u64; 16],
    enpassant: [u64; 8],
}

pub static ZOBRIST: Mutex<Zobrist> = {
    Mutex::new(Zobrist {
        pieces: [[0; 64]; 12],
        side: 0,
        castling: [0; 16],
        enpassant: [0; 8],
    })
};

pub fn init_zobrist_nums() {
    let mut zobrist = ZOBRIST.lock().unwrap();
    zobrist.pieces.iter_mut().for_each(|e| *e = array::from_fn(|_| get_random_u64_num()));
    zobrist.side = get_random_u64_num();
    zobrist.castling = array::from_fn(|_| get_random_u64_num());
    zobrist.enpassant = array::from_fn(|_| get_random_u64_num());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_zobrist() {
        init_zobrist_nums();
        let zobrist = ZOBRIST.lock().unwrap();
        println!("{:?}", zobrist);

        //want to check if every number is unique
        let mut seen = HashSet::new(); 
        assert!(zobrist.pieces
                .iter().flatten()
                .chain(zobrist.castling.iter())
                .chain(zobrist.enpassant.iter())
                .chain([zobrist.side].iter())
                .all(|e| seen.insert(e))); 
    }
}