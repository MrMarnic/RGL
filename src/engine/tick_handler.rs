use crate::engine::game_engine::GameEngine;

pub struct TickHandler {
    pub tick_speed: f64,
    pub accumulator: f64,
    pub current_tick: i32,
    pub ticks_for_second: i32,
    pub paused: bool,
    pub tick_alpha: f64
}

impl TickHandler {
    pub fn update(&mut self,frame_time: f64) {
        if !self.paused {
            self.accumulator += frame_time;
        }
    }

    pub fn tick(&mut self) -> bool {
        if !self.paused {
            let b = self.accumulator >= self.tick_speed;

            if b {
                self.accumulator -= self.tick_speed;
                if self.current_tick == self.ticks_for_second {
                    self.current_tick = 0;
                }
                if !b {
                    self.tick_alpha = self.accumulator / self.tick_speed;
                }
            }

            return b
        }

        false
    }

    pub fn new(ticks_for_second:i32) -> TickHandler {
        let tick_speed = (1000.0 / ticks_for_second as f64) / 1000.0;
        return TickHandler { tick_speed, accumulator: 0.0 , current_tick: 0,ticks_for_second, paused: false, tick_alpha: 0.0 }
    }
    pub fn set_ticks_for_second(&mut self,ticks_for_second:i32) {
        if ticks_for_second > 0 {
            let tick_speed = (1000.0 / ticks_for_second as f64) / 1000.0;
            self.ticks_for_second = ticks_for_second;
            self.tick_speed = tick_speed;
        }
    }
}