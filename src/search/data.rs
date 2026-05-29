use crate::search::time::{TimeManager, TimeSettings};
use crate::types::{Move, MoveList, Side};

use crate::types::TranspositionTable;

#[derive(Debug)]
pub struct SearchData {
    playing_as: Side,
    nodes_searched: usize,
    depth: usize,
    pv: Vec<MoveList>,

    pub tt: TranspositionTable,
    pub time: TimeManager,
}

#[derive(Debug)]
pub struct SearchCancelled;

impl SearchData {
    pub fn new() -> Self {
        SearchData {
            playing_as: Side::White,
            nodes_searched: 0,
            depth: 0,
            pv: vec![MoveList::new(); 256],
            tt: TranspositionTable::new(),
            time: TimeManager::new()
        }
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

    pub fn nodes_per_second(&self) -> usize {
        (self.get_total_nodes_searched() as f32 / self.time.elapsed().as_secs_f32()) as usize
    }

    pub fn start_time(&mut self) {
        self.time.reset_clock();
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

    pub fn clear_table(&mut self) {
        self.tt.clear();
    }

    pub fn get_time_settings(&mut self) -> &mut TimeSettings {
        &mut self.time.settings
    }

    pub fn over_limit(&self) -> bool {
        self.time.over_limit(self.playing_as)
    }

    pub fn set_playing_as(&mut self, side: Side) {
        self.playing_as = side;
    }

    pub fn clear_node_count(&mut self) {
        self.nodes_searched = 0;
    }

    pub fn reset_pv(&mut self) {
        self.pv = vec![MoveList::new(); 256];
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self::new()
    }
}
