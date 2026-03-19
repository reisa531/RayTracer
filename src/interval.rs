use std::f64::{INFINITY, NEG_INFINITY};

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