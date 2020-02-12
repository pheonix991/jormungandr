use super::Status;
use std::{cmp, fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct Thresholds<T> {
    inner_thresholds: Vec<(Status, T)>,
    max: T,
}

impl<T: cmp::PartialOrd + Clone> Thresholds<T> {
    pub fn thresholds(&self) -> &Vec<(Status, T)> {
        &self.inner_thresholds
    }

    pub fn max(&self) -> T {
        self.max.clone()
    }

    pub fn green_threshold(&self) -> T {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Green)
            .expect("cannot find green threshold")
            .1
            .clone()
    }

    pub fn yellow_threshold(&self) -> T {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Yellow)
            .expect("cannot find green threshold")
            .1
            .clone()
    }

    pub fn red_threshold(&self) -> T {
        self.thresholds()
            .iter()
            .find(|(x, _)| *x == Status::Red)
            .expect("cannot find red threshold")
            .1
            .clone()
    }
}

impl Thresholds<Duration> {
    pub fn new(green: Duration, yellow: Duration, red: Duration, max: Duration) -> Self {
        Self {
            inner_thresholds: vec![
                (Status::Green, green),
                (Status::Yellow, yellow),
                (Status::Red, red),
            ],
            max: max,
        }
    }

    pub fn status(&self, actual: Duration) -> Status {
        let green = self.green_threshold();
        let yellow = self.yellow_threshold();

        if actual <= green {
            return Status::Green;
        }
        if actual <= yellow {
            return Status::Yellow;
        }
        Status::Red
    }
}

impl fmt::Display for Thresholds<Duration> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Green: {} s. Yellow: {} s. Red: {} s. Abort after: {} s",
            self.green_threshold().as_secs(),
            self.yellow_threshold().as_secs(),
            self.red_threshold().as_secs(),
            self.max().as_secs()
        )
    }
}

impl Thresholds<u64> {
    pub fn new(green: u64, yellow: u64, red: u64, max: u64) -> Self {
        Self {
            inner_thresholds: vec![
                (Status::Green, green),
                (Status::Yellow, yellow),
                (Status::Red, red),
            ],
            max: max,
        }
    }

    pub fn status(&self, actual: u64) -> Status {
        let green = self.green_threshold();
        let yellow = self.yellow_threshold();

        if actual >= green {
            return Status::Green;
        }
        if actual >= yellow {
            return Status::Yellow;
        }
        Status::Red
    }
}

impl fmt::Display for Thresholds<u64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Max: {}. Green: {}. Yellow: {}. Red: {}.",
            self.max()
            self.green_threshold(),
            self.yellow_threshold(),
            self.red_threshold(),
        )
    }
}
