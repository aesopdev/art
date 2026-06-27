use std:: f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: INFINITY, max: NEG_INFINITY }
    }
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub const fn size(&self) -> f64 {
        self.max - self.min
    }
    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub const fn clamp(&self, x: f64) -> f64 {
        if x < self.min { return self.min };
        if x > self.max { return self.max };
        x
    }
    
    pub const EMPTY: Interval = Interval::new(INFINITY, NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(NEG_INFINITY, INFINITY);
}