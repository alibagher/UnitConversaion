mod time; 
use time::Time;

fn main() {
    let time_in_seconds = Time::Seconds(1.0);
    let time_in_minutes = Time::Min(1.0);
    let time_in_hours = Time::Hour(1.0);
    let time_in_days = Time::Day(1.0);

    println!("Time in seconds: {} seconds", time_in_seconds.in_seconds());
    println!("Time in minutes: {} seconds", time_in_minutes.in_seconds());
    println!("Time in hours: {} seconds", time_in_hours.in_seconds());
    println!("Time in days: {} seconds", time_in_days.in_seconds());

    // Create instances of Time variants.
    let time1 = Time::Min(1.4);  // 1 minute
    let time2 = Time::Seconds(30.0);  // 30 seconds

    // Add the two Time instances.
    let total_time = time1 + time2;

    // Print the result in seconds.
    println!("Total time in seconds: {} ", total_time);
}