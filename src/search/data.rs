use std::time::{Duration, Instant};

use crate::types::{Move, MoveList};

use crate::types::TranspositionTable;

#[derive(Debug)]
pub struct SearchData {
    nodes_searched: usize,
    time: Instant,
    depth: usize,
    time_limit: u128,
    pv: Vec<MoveList>,
    pub tt: TranspositionTable,
}

#[derive(Debug)]
pub enum SearchKind {
    Depth(usize),
    Exact(u128),
    Normal(u128, u128),
}

#[derive(Debug)]
pub struct SearchCancelled;

impl SearchData {
    pub fn new(kind: SearchKind) -> Self {
        SearchData {
            nodes_searched: 0,
            time: Instant::now(),
            depth: 0,
            time_limit: match kind {
                SearchKind::Depth(_) => 0,
                SearchKind::Normal(remaining_time, increment) => {
                    (remaining_time / 20) + (increment / 2) //Simple time managment strategy: remaining time/20 + increment/2
                }
                SearchKind::Exact(thinking_time) => thinking_time, //Simple time managment strategy: remaining time/20 + increment/2
            },
            pv: vec![MoveList::new(); 256],
            tt: TranspositionTable::new(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.time.elapsed()
    }

    pub fn over_limit(&self) -> bool {
        self.elapsed().as_millis() >= self.time_limit - 2 //Some buffer room
    }

    pub fn get_time_limit(&self) -> u128 {
        self.time_limit
    }

    pub fn get_searched_depth(&self) -> usize {
        self.depth
    }

    pub fn get_total_nodes_searched(&self) -> usize {
        self.nodes_searched
    }

    pub fn get_pv(&self) -> &MoveList {
        &self.pv[0]
    }

    pub fn add_nodes(&mut self, nodes: usize) {
        self.nodes_searched += nodes;
    }

    pub fn increase_depth(&mut self) {
        self.depth += 1;
    }

    pub fn nodes_per_second(&self) -> f32 {
        self.get_total_nodes_searched() as f32 / self.elapsed().as_secs_f32()
    }

    pub fn add_pv_move(&mut self, m: Move, ply: usize) {
        self.pv[ply].clear();
        self.pv[ply].push(m);
        for child_m in self.pv[ply + 1].clone().iter() {
            self.pv[ply].push(*child_m);
        }
    }

    pub fn clear_pv(&mut self, ply: usize) {
        self.pv[ply].clear();
    }

    pub fn start_time(&mut self) {
        self.time = Instant::now();
    }

    pub fn set_limit(&mut self, kind: SearchKind) {
        self.time_limit = match kind {
            SearchKind::Depth(_) => 0,
            SearchKind::Normal(remaining_time, increment) => {
                (remaining_time / 20) + (increment / 2) //Simple time managment strategy: remaining time/20 + increment/2
            }
            SearchKind::Exact(thinking_time) => thinking_time,
        }
    }

    pub fn clear_table(&mut self) {
        self.tt.clear();
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self::new(SearchKind::Exact(5000))
    }
}
