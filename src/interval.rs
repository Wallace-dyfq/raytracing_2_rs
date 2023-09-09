use crate::utils::*;
use std::ops;

#[derive(Debug, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}
pub const EMPTY_INTERVAL: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    // merge two intervals
    pub fn merge(interval1: &Self, interval2: &Self) -> Self {
        Self {
            min: interval1.min.min(interval2.min),
            max: interval1.max.max(interval2.max),
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl ops::Add<f64> for &Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}
