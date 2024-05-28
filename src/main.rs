mod time; 
use time::Time;
mod unit;
use unit::Length;
mod angle;
use angle::Angle;


fn main() {
    ////////////////// Time ////////////////////
    let time_in_seconds = Time::Seconds(1.0);
    let time_in_minutes = Time::Min(1.0);
    let time_in_hours = Time::Hour(1.0);
    let time_in_days = Time::Day(1.0);
    println!("Time in seconds: {} seconds", time_in_seconds.to_seconds());
    println!("Time in minutes: {} seconds", time_in_minutes.to_seconds());
    println!("Time in hours: {} seconds", time_in_hours.to_seconds());
    println!("Time in days: {} seconds", time_in_days.to_seconds());

    // Create instances of Time variants.
    let time1 = Time::Min(1.4);  // 1 minute
    let time2 = Time::Seconds(30.0);  // 30 seconds
    // Add the two Time instances.
    let total_time = time1 + time2;
    // Print the result in seconds.
    println!("Total time in seconds: {} ", total_time);
    //let new_time_total = time1 + time2;

    ////////////////// Length ////////////////////
    let length1 = Length::Meters(10.0);
    let length2 = Length::Centimeters(50.0);

    let area = length1 * length2;
    let dimensionless = length1 / length2;

    println!("The area of {} by {} is {}", length1, length2, area, );
    println!( "{} divided by {} is {}", length1, length2, dimensionless);

    /////////// Area and Volume ///////////////
    
    let volume = area * length1;

    println!("The volume of {} by {} by {} is {}", length1, length1, length2, volume);

    let area = volume / length2;

    let new_length = area / length1;

    let scalar_length = length1 * (0.5);

    println!("The new length is {}", new_length);
    println!("The scalar length is {}", scalar_length);


    /////////// Angle //////////////
    
    let angle1 = Angle::Revolutions(1.0);
    let angle2 = Angle::Degrees(90.0);

    println!(
        "{} is equal to {} degrees",
        angle1,
        angle1.to_degrees()
    );

    println!(
        "{} is equal to {} radians",
        angle2,
        angle2.to_radians()
    );

    println!(
        "{} is equal to {} gradians",
        angle1,
        angle1.to_gradians()
    );

}