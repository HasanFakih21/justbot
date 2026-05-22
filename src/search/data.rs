use std::time::{Duration, Instant};

pub struct SearchData {
    nodes_searched: usize,
    time: Instant,
    depth: usize,
    time_limit: u128,
}

pub enum SearchKind {
    Depth(usize),
    Exact(u128),
    Normal(u128, u128),
}

impl SearchData {
    pub fn new(kind: SearchKind) -> Self {
        SearchData {
            nodes_searched: 0,
            time: Instant::now(),
            depth: 0,
            time_limit: match kind {
                SearchKind::Depth(_) => 0,
                SearchKind::Normal(remaining_time, increment) => {
                    (remaining_time / 20) + (increment / 2)
                }
                SearchKind::Exact(thinking_time) => thinking_time,
            }, //Simple time managment strategy: remaining time/20 + increment/2
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

    pub fn add_nodes(&mut self, nodes: usize) {
        self.nodes_searched += nodes;
    }

    pub fn increase_depth(&mut self) {
        self.depth += 1;
    }
}

impl Default for SearchData {
    fn default() -> Self {
        Self::new(SearchKind::Exact(5000))
    }
}
