use std::ops::Add;
use std::fmt;

pub enum Time {
    Seconds(f64),
    Min(f64),
    Hour(f64),
    Day(f64),
}

impl Time {
    pub fn to_seconds(&self) -> f64 {
        match self {
            Time::Seconds(seconds) => *seconds,
            Time::Min(minutes) => minutes * 60.0,
            Time::Hour(hours) => hours * 3600.0,
            Time::Day(days) => days * 86400.0,
        }
    }

    pub fn to_min(&self) -> f64 {
      self.to_seconds() / 60.0
    }

    pub fn to_hour(&self) -> f64 {
      self.to_seconds() / 3600.0
    }

    pub fn to_day(&self) -> f64 {
      self.to_seconds() / 86400.0
    }
    // // Method to convert seconds to the same variant as self.
    // pub fn from_seconds(&self, total_seconds: f64) -> Self {
    //     match self {
    //         Time::Seconds(_) => Time::Seconds(total_seconds),
    //         Time::Min(_) => Time::Min(total_seconds / 60.0),
    //         Time::Hour(_) => Time::Hour(total_seconds / 3600.0),
    //         Time::Day(_) => Time::Day(total_seconds / 86400.0),
    //     }
    // }
}

// Implement the Add trait for Time.
impl Add for Time {
    type Output = Time;

    // TODO: Instead of returning in SECONDS, return in the version of the self. 

    fn add(self, other: Time) -> Time {
        Time::Seconds(self.to_seconds() + other.to_seconds())
    }
}

// Implement the Display trait for Time to define how it should be formatted when printed.
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Time::Seconds(seconds) => write!(f, "{} seconds", seconds),
            Time::Min(minutes) => write!(f, "{} minutes", minutes),
            Time::Hour(hours) => write!(f, "{} hours", hours),
            Time::Day(days) => write!(f, "{} days", days),
        }
    }
}