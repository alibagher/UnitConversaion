mod time; 
use time::Time;

fn main() {
    let time_in_seconds = Time::Seconds(1);
    let time_in_minutes = Time::Min(1);
    let time_in_hours = Time::Hour(1);
    let time_in_days = Time::Day(1);

    println!("Time in seconds: {} seconds", time_in_seconds.in_seconds());
    println!("Time in minutes: {} seconds", time_in_minutes.in_seconds());
    println!("Time in hours: {} seconds", time_in_hours.in_seconds());
    println!("Time in days: {} seconds", time_in_days.in_seconds());
}