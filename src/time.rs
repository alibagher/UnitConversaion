pub enum Time {
    Seconds(i64),
    Min(i64),
    Hour(i64),
    Day(i64),
}

impl Time {
    pub fn in_seconds(&self) -> i64 {
        match self {
            Time::Seconds(seconds) => *seconds,
            Time::Min(minutes) => minutes * 60,
            Time::Hour(hours) => hours * 3600,
            Time::Day(days) => days * 86400,
        }
    }
}

