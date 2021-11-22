use std::time::Instant;

#[derive(Copy, Clone)]
pub struct Timer {
    start: Instant,
    max_time: u64, //seconds
}

impl Timer {
    pub fn new(max_time: u64) -> Self {
        Self {
            start: Instant::now(),
            max_time,
        }
    }

    pub fn elapsed(&self) -> bool {
        self.start.elapsed().as_secs() >= self.max_time
    }
}
