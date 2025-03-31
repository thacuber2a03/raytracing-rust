#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Interval(pub f64, pub f64);

impl Default for Interval {
    fn default() -> Self {
        Interval::EMPTY
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.1 - self.0
    }

    pub fn contains(&self, x: f64) -> bool {
        self.0 <= x && x <= self.1
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.0 {
            self.0
        } else if x > self.1 {
            self.1
        } else {
            x
        }
    }

    const EMPTY: Interval = Interval(f64::INFINITY, f64::NEG_INFINITY);
    const UNIVERSE: Interval = Interval(f64::NEG_INFINITY, f64::INFINITY);
}
