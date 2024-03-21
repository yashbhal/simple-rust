use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the guessing game!");
    println!("Guess the secret number between 0 and 101 that the program has chosen.");

    loop
    {
        println!("Enter your guess: ");
    
        let mut user_guess = String::new();
    
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line.");
    
        
        let user_guess: u32 = match user_guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {user_guess}");
        
        match user_guess.cmp(&secret_number)
        {
            Ordering::Less => println!("The number you guessed is lower than the secret number."),
            Ordering::Greater => println!("The number you guessed is higher than the secret number."),
            Ordering::Equal => 
            {
                println!("You have guessed the secret nummber, congratulations!.");
                break;
            }
        }
    }
}
