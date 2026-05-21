use crate::board::moves::Move;

const TT_SIZE: usize = 64;
const MEGABYTE: usize = 1024 * 1024;
const ENTRIES: usize = TT_SIZE * MEGABYTE / std::mem::size_of::<Entry>();

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    PV,
    All,
    Cut,
}

#[derive(Debug, Clone)]
pub struct Entry {
    key: u64,
    best_move: Move,
    //depth: u8,
    score: i32,
    node: NodeType,
    //age
}

impl Entry {
    pub fn new(key: u64, best_move: Move, score: i32, node: NodeType) -> Self {
        Entry { key, best_move, score, node}
    }

    pub fn get_key(&self) -> u64 {
        self.key
    }

    pub fn get_best_move(&self) -> Move {
        self.best_move
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }
    
    pub fn get_node_type(&self) -> NodeType {
        self.node
    }
}

const fn index(hash: u64) -> usize {
    (((hash as u128) * (ENTRIES as u128)) >> 64) as usize
}

#[derive(Debug, Clone)]
pub struct TranspositionTable(Vec<Option<Entry>>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(vec![None; ENTRIES])
    }

    pub fn add_entry(&mut self, best_move: Move, score: i32, node: NodeType, hash: u64) {
        let entry = Entry::new(hash, best_move, score, node);
        self.0[index(hash)] = Some(entry);
    }

    pub fn get_entry(&self, hash: u64) -> &Option<Entry> {
        &self.0[index(hash)]
    }

    pub fn get_best_move(&self, hash: u64) -> Option<Move> {
        self.0[index(hash)].as_ref().map(|e| e.get_best_move())
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{board::Board, search::search};

    #[test]
    fn test_transposition_table() {
        let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
        if let Some((best_move, score)) = search(3, &mut board) {
            let hash = board.board_state.hash;
            let entry = board.tt.get_entry(hash);

            let m = entry.as_ref().unwrap().get_best_move();
            let s = entry.as_ref().unwrap().get_score();

            assert_eq!(best_move, m);
            assert_eq!(score, s);

            let _ = board.make_move(best_move);
            let _ = search(2, &mut board);

            let entry = board.tt.get_entry(hash);

            let m = entry.as_ref().unwrap().get_best_move();
            let s = entry.as_ref().unwrap().get_score();

            assert_eq!(best_move, m);
            assert_eq!(score, s);
        }
    }
}