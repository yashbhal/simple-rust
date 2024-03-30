use std::env;
use std::io;
use std::error::Error;
use reqwest::blocking::Response;
use serde_json::Value;

pub fn get_location() -> Result<(String, String), Box<dyn Error>> 
{
    println!("Please enter a latitude:");
    let lat = read_input()?;

    println!("Please enter a longitude:");
    let lon = read_input()?;

    Ok((lat, lon))
}

pub fn read_input() -> Result<String, Box<dyn Error>> 
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    let trimmed_input = input.trim().parse()?;
    Ok(trimmed_input)
}

pub fn get_api_key() -> Result<String, Box<dyn Error>> 
{
    match env::var("WEATHER_API_KEY") {
        Ok(val) => Ok(val),
        Err(_) => {
            let error_msg = "WEATHER_API_KEY environment variable is not set".to_string();
            eprintln!("{}", error_msg);
            Err(error_msg.into())
        }
    }
}

pub fn construct_url(lat: String, lon: String, api_key: String) -> Result<String, Box<dyn Error>> 
{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric", lat, lon, api_key);
    Ok(url)
}

pub fn parse_weather_data(response: Response) -> Result<Value, Box<dyn Error>> 
{
    let body = response.text()?;
    let json_response: Value = serde_json::from_str(&body)?;
    Ok(json_response)
}

pub fn print_menu()
{
    println!("Enter corresponding number to get data: \n");
    println!("Enter 1 to get temperature.");
    println!("Enter 2 to get what temperature it feels like outside.");
    println!("Enter 3 to get the maximum and minimum predicated temperatures for the day.");
    println!("Enter 4 to get humidity.");
}

pub fn run_program(user_choice: &str, json_response: &Value) -> Result<(), Box<dyn Error>> 
{
    match user_choice {
        "1" => println!("Temperature: {} C", get_temperature(json_response)?),
        "2" => println!("Feels like: {} C", get_feels_like_temperature(json_response)?),
        "3" => println!("{}", get_max_min_temp(json_response)?),
        "4" => println!("Humidity: {}", get_humidity(json_response)?),
        _ => println!("Invalid input, enter 1, 2, 3 or 4."),
    }
    Ok(())
}

fn get_temperature(json_response: &Value) -> Result<f64, Box<dyn Error>> 
{
    Ok(json_response["main"]["temp"].as_f64().unwrap_or_default())
}

fn get_feels_like_temperature(json_response: &Value) -> Result<f64, Box<dyn Error>> 
{
    Ok(json_response["main"]["feels_like"].as_f64().unwrap_or_default())
}

fn get_humidity(json_response: &Value) -> Result<f64, Box<dyn Error>> 
{
    Ok(json_response["main"]["humidity"].as_f64().unwrap_or_default())
}

fn get_max_min_temp(json_response: &Value) -> Result<String, Box<dyn Error>> 
{
    let max_temp = json_response["main"]["temp_max"].as_f64().unwrap_or_default();
    let min_temp = json_response["main"]["temp_min"].as_f64().unwrap_or_default();
    Ok(format!(
        "Max Temp: {} C, Min Temp: {} C",
        max_temp, min_temp
    ))
}
