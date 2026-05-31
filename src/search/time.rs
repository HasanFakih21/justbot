use std::time::{Duration, Instant};

use crate::types::Side;

#[derive(Debug)]
pub struct TimeManager {
    pub clock: Instant,
    pub settings: TimeSettings,
    pub limit: Limit,
}

#[derive(Debug)]
pub struct TimeSettings {
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub movestogo: usize,
    pub depth: usize,
    pub nodes: usize,
    pub mate: usize,
    pub movetime: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum Limit {
    Time(u64),
    Depth(u64),
}

impl Default for TimeSettings {
    fn default() -> Self {
        Self {
            wtime: 10000,
            btime: 10000,
            winc: 0,
            binc: 0,
            movestogo: 0,
            depth: 0,
            nodes: 0,
            mate: 0,
            movetime: 0,
        }
    }
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            clock: Instant::now(),
            settings: TimeSettings::default(),
            limit: Limit::Time(1000),
        }
    }

    pub fn reset_clock(&mut self, side: Side) {
        self.clock = Instant::now();
        let remaining_time;
        let increment;

        match side {
            Side::White => {
                remaining_time = self.settings.wtime;
                increment = self.settings.winc;
            }
            Side::Black => {
                remaining_time = self.settings.btime;
                increment = self.settings.binc;
            }
        }

        self.limit = Limit::Time((remaining_time / 20) + (increment / 2)); //Simple time managment strategy: remaining time/20 + increment/2
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.elapsed()
    }

    pub fn get_limit(&self) -> u64 {
        match self.limit {
            Limit::Time(t) => t,
            Limit::Depth(d) => d,
        }
    }

    pub fn over_limit(&self) -> bool {
        self.elapsed().as_millis() as u64 > self.get_limit() + 20 //So it uses the same time as an older version that didn't stop exactly over the limit
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}
