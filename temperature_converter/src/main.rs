use std::io;

fn main() {

    let mut user_temp = String::new();

    println!("Welcome to the temperature converter.");
    println!("Enter a temperature in Fahrenheit: ");

    io::stdin()
        .read_line(&mut user_temp)
        .expect("Enter an integer Fahrenheit temperature.");

    let user_temp: u32 = user_temp.trim().parse().expect("Please enter an integer Fahrenheit temperature.");
    
    println!("The Fahrenheit temperature you entered is: {user_temp}");

    let result_temp: f32 = fahrenheit_to_celcius(&user_temp);
    // let result_temp: f32 = ( (user_temp as f32 - 32.0) * 5.0) / 9.0;

    println!("{user_temp} degrees Fahrenheit is equal to {result_temp} degrees Celsius."); 
}

fn fahrenheit_to_celcius(&fahrenheit_temp: &u32) -> f32{
    let celcius_temp = ( (fahrenheit_temp as f32 - 32.0) * 5.0) / 9.0;
    return celcius_temp;
}
