use std::io;

fn main() 
{
    let mut user_choice = String::new();
    let mut user_temp = String::new();

    println!("Welcome to the temperature converter.");
    println!("If you would like to convert Fahrenheit to Celsius, enter 'f2c'.");
    println!("If you would like to convert Celsius to Fahrenheit, enter 'c2f'.");

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Enter a valid string.");

    let user_choice: String = user_choice.trim().to_lowercase();

    if user_choice == "f2c"
    {
        println!("Enter a temperature in Fahrenheit: ");

        io::stdin()
            .read_line(&mut user_temp)
            .expect("Enter an integer Fahrenheit temperature.");
    
        let user_temp: u32 = user_temp.trim().parse().expect("Please enter an integer Fahrenheit temperature.");
        
        println!("The Fahrenheit temperature you entered is: {user_temp}");
    
        let result_temp: f32 = fahrenheit_to_celsius(&user_temp);
    
        println!("{user_temp} degrees Fahrenheit is equal to {result_temp} degrees Celsius."); 
    }
    else if user_choice == "c2f"
    {
        println!("Enter a temperature in Celsius: ");

        io::stdin()
            .read_line(&mut user_temp)
            .expect("Enter an integer Celsius temperature.");
    
        let user_temp: u32 = user_temp.trim().parse().expect("Please enter an integer Celsius temperature.");
        
        println!("The Celsius temperature you entered is: {user_temp}");
    
        let result_temp: f32 = celsius_to_fahrenheit(&user_temp);
    
        println!("{user_temp} degrees Celsius is equal to {result_temp} degrees Fahrenheit."); 
        
    }
    else
    {
        println!("Enter either 'f2c to 'c2f' without the quotations to convert temperatures.");
    }
}

fn fahrenheit_to_celsius(&fahrenheit_temp: &u32) -> f32
{
    let converted_celsius_temp = ((fahrenheit_temp as f32 - 32.0) * 5.0) / 9.0;
    return converted_celsius_temp;
}

fn celsius_to_fahrenheit(&celsius_temp: &u32) -> f32
{
    let converted_fahrenheit_temp = ((celsius_temp as f32 * 9.0) / 5.0) + 32.0;
    return converted_fahrenheit_temp;
}