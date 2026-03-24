use std::f64::{INFINITY, NEG_INFINITY};

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY
        }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval {
            min,
            max
        }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Interval {
            min: a.min.min(b.min),
            max: a.max.max(b.max)
        }
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min { return self.min }
        else if x > self.max { return self.max }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding
        }
    }

    pub fn size(&self) -> f64 {
        (self.max - self.min).max(0.0)
    }

    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: NEG_INFINITY
    };

    pub const UNIVERSE: Interval = Interval {
        min: NEG_INFINITY,
        max: INFINITY
    };

    pub const POSITIVE: Interval = Interval {
        min: 0.0,
        max: INFINITY
    };

    pub const NEGATIVE: Interval = Interval {
        min: NEG_INFINITY,
        max: 0.0
    };

    pub const PSEUDO_POSITIVE: Interval = Interval {
        min: 0.001,
        max: INFINITY
    };

    pub const PSEUDO_UNIT: Interval = Interval {
        min: 0.0,
        max: 0.999
    };
}

impl Add<f64> for Interval {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs
        }
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        rhs + self
    }
}