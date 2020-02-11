use super::Status;
use std::{fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct Thresholds {
    inner_thresholds: Vec<(Status, Duration)>,
    timeout: Duration,
}

impl Thresholds {
    pub fn new(green: u64, yellow: u64, red: u64, timeout: u64) -> Self {
        Self {
            inner_thresholds: vec![
                (Status::Green, Duration::from_secs(green)),
                (Status::Yellow, Duration::from_secs(yellow)),
                (Status::Red, Duration::from_secs(red)),
            ],
            timeout: Duration::from_secs(timeout),
        }
    }

    pub fn thresholds(&self) -> &Vec<(Status, Duration)> {
        &self.inner_thresholds
    }

    pub fn timeout(&self) -> Duration {
        self.timeout.clone()
    }

    pub fn green_threshold(&self) -> Duration {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Green)
            .expect("cannot find green threshold")
            .1
    }

    pub fn yellow_threshold(&self) -> Duration {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Yellow)
            .expect("cannot find green threshold")
            .1
    }

    pub fn red_threshold(&self) -> Duration {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Red)
            .expect("cannot find red threshold")
            .1
    }
    pub fn status(&self, actual: &Duration) -> Status {
        let green = self.green_threshold();
        let yellow = self.yellow_threshold();

        if *actual <= green {
            return Status::Green;
        }
        if *actual <= yellow {
            return Status::Yellow;
        }
        Status::Red
    }
}

impl fmt::Display for Thresholds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Green: {} s. Yellow: {} s. Red: {} s. Abort after: {} s",
            self.green_threshold().as_secs(),
            self.yellow_threshold().as_secs(),
            self.red_threshold().as_secs(),
            self.timeout().as_secs()
        )
    }
}
