use std::io;

fn main() {
    println!("Please input ");

    let mut user_input = 0;

    let converted_fahrenheit: f32 = celsius_to_fahrenheit(user_input);
    println!("Converted fahrenheit temperature is: {}", converted_fahrenheit);

    let converted_celsius = fahrenheit_to_celsius(32);
    println!("Converted Celsius temperature is: {}", converted_celsius);
}

fn celsius_to_fahrenheit(celsius: i32) -> f32 {
    (celsius * (9/5) + 32) as f32
}

fn fahrenheit_to_celsius(fahrenheit: i32) -> f32 {
    (fahrenheit * (9/5) - 32) as f32
}

