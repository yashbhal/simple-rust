use reqwest;
use std::env;
use std::io;
use serde_json::Value;
use std::error::Error;
use reqwest::blocking::Response;
mod setup;

fn main() -> Result<(), Box<dyn Error>> {
    let (lat, lon) = get_location()?;
    let api_key = get_api_key()?;
    let url = construct_url(lat, lon, api_key)?;
    let response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        parse_weather_data(response)?;
    } else {
        println!("Error! {}", response.status());
    }

    Ok(())
}

fn get_location() -> Result<(String, String), Box<dyn Error>> {
    println!("Please enter a latitude:");
    let lat = read_input("latitude")?;

    println!("Please enter a longitude:");
    let lon = read_input("longitude")?;

    Ok((lat, lon))
}

fn read_input(_input_value: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let trimmed_input = input.trim().parse()?;
    Ok(trimmed_input)
}

fn get_api_key() -> Result<String, Box<dyn Error>> {
    match env::var("WEATHER_API_KEY") {
        Ok(val) => Ok(val),
        Err(_) => {
            let error_msg = "WEATHER_API_KEY environment variable is not set".to_string();
            eprintln!("{}", error_msg);
            Err(error_msg.into())
        }
    }
}

fn construct_url(lat: String, lon: String, api_key: String) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, lon, api_key);
    Ok(url)
}

fn parse_weather_data(response: Response) -> Result<(), Box<dyn Error>> {
    let body = response.text()?;
    let json_response: Value = serde_json::from_str(&body)?;

    println!("Response: {}", json_response);

    Ok(())
}
