mod status;
mod thresholds;

pub use status::Status;
pub use thresholds::Thresholds;

use std::{cmp, fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct Measurement<T> {
    info: String,
    actual: T,
    thresholds: Thresholds<T>,
}

impl<T: cmp::PartialOrd + Clone> Measurement<T> {
    pub fn new(info: String, actual: T, thresholds: Thresholds<T>) -> Self {
        Self {
            info,
            actual,
            thresholds,
        }
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }

    pub fn actual(&self) -> T {
        self.actual.clone()
    }

    pub fn thresholds(&self) -> Thresholds<T> {
        self.thresholds.clone()
    }
}

impl Measurement<Duration> {
    pub fn result(&self) -> Status {
        self.thresholds().status(self.actual)
    }
}

impl fmt::Display for Measurement<Duration> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Measurement: {}. Result: {}. Actual: {:.3} s. Thresholds: {}",
            self.info(),
            self.result().to_string(),
            self.actual.as_millis() as f32 / 1000.0,
            self.thresholds
        )
    }
}

impl Measurement<u64> {
    pub fn result(&self) -> Status {
        self.thresholds().status(self.actual)
    }
}

impl fmt::Display for Measurement<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Measurement: {}. Result: {}. Actual: {}. Thresholds: {}",
            self.info(),
            self.result().to_string(),
            self.actual,
            self.thresholds
        )
    }
}
