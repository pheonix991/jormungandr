mod status;
mod thresholds;

pub use status::Status;
pub use thresholds::Thresholds;

use std::{fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct Measurement {
    info: String,
    actual: Duration,
    thresholds: Thresholds,
}

impl Measurement {
    pub fn new(info: String, actual: Duration, thresholds: Thresholds) -> Self {
        Self {
            info,
            actual,
            thresholds,
        }
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }

    pub fn actual(&self) -> Duration {
        self.actual
    }

    pub fn thresholds(&self) -> Thresholds {
        self.thresholds.clone()
    }

    pub fn result(&self) -> Status {
        self.thresholds.status(&self.actual)
    }
}

impl fmt::Display for Measurement {
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
