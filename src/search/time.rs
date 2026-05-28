use std::time::{Duration, Instant};

use crate::types::Side;

#[derive(Debug)]
pub struct TimeManager {
    pub clock: Instant,
    pub settings: TimeSettings,
}

#[derive(Debug)]
pub struct TimeSettings {
    pub wtime: u128,
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub movestogo: usize,
    pub depth: usize,
    pub nodes: usize,
    pub mate: usize,
    pub movetime: u128,
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
        TimeManager { clock: Instant::now(), settings: TimeSettings::default() } 
    }

    pub fn reset_clock(&mut self) {
        self.clock = Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        self.clock.elapsed()
    }

    pub fn over_limit(&self, side: Side) -> bool {
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

        let time_limit = (remaining_time / 20) + (increment / 2); //Simple time managment strategy: remaining time/20 + increment/2
        let over_limit = self.elapsed().as_millis() >= time_limit; 

        //For debugging//
        // if over_limit {
        //     println!("Searched for {}ms", self.elapsed().as_millis());
        //     println!("Time limit was {}", time_limit);
        // }

        over_limit
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}