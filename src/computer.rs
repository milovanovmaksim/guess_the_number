use std::ops::Range;
use rand::Rng;

pub struct Computer {
    range: Range<u32>,
    secret: u32,
}

impl Computer {
    ///
    pub fn new(range: Range<u32>) -> Self {
        Self {
            range: range.clone(),
            secret: 0,
        }
    }
    ///
    pub fn restart(&mut self) {
        self.secret = rand::thread_rng().gen_range(self.range.clone());
    }
    pub fn secret(&self) -> u32 {
        self.secret
    }
}