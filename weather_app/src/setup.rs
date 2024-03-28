use std::env;
use std::io;
use std::error::Error;

pub fn get_location() -> Result<(String, String), Box<dyn Error>> {
    println!("Please enter a latitude:");
    let lat = read_input("latitude")?;

    println!("Please enter a longitude:");
    let lon = read_input("longitude")?;

    Ok((lat, lon))
}

pub fn read_input(_input_value: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let trimmed_input = input.trim().parse()?;
    Ok(trimmed_input)
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    match env::var("WEATHER_API_KEY") {
        Ok(val) => Ok(val),
        Err(_) => {
            let error_msg = "WEATHER_API_KEY environment variable is not set".to_string();
            eprintln!("{}", error_msg);
            Err(error_msg.into())
        }
    }
}

pub fn construct_url(lat: String, lon: String, api_key: String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, lon, api_key);
    Ok(url)
}