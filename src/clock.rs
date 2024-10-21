use std::time::{Duration, Instant};
use crate::core::scene::TimeProvider;

pub struct Clock {
    last_tick_time: Option<Instant>,
    elapsed_time: Duration,
    last_dt: Duration
}
impl TimeProvider for Clock {
    fn total_time(&self) -> &Duration {
        &self.elapsed_time
    }

    fn dt(&self) -> &Duration {
        &self.last_dt
    }
}
impl Clock {
    pub fn new() -> Clock {
        Clock {
            last_tick_time: None,
            last_dt: Duration::from_millis(0),
            elapsed_time: Duration::from_millis(0),
        }
    }

    pub fn tick(&mut self) {
        if let Some(last_tick_time) = self.last_tick_time {
            let now = Instant::now();
            self.last_dt = now - last_tick_time;
            self.elapsed_time += self.last_dt;
        }

        self.last_tick_time = Some(Instant::now());
    }
}