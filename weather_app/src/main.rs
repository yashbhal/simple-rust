use reqwest;
use std::env;
use serde_json::Value;
use std::error::Error;
// mod setup;
// use setup::WeatherData;

fn main() -> Result<(), Box<dyn Error>>
{
    let api_key = match env::var("WEATHER_API_KEY")
    {
        Ok(val) => val,
        Err(_) => 
        {
            eprintln!("WEATHER_API_KEY environment variable is not set");
            return Err("WEATHER_API_KEY environment variable is not set".into());
        }
    };
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat=44.34&lon=10.99&appid={}", api_key);
    let response = reqwest::blocking::get(url)?;

    if response.status().is_success()
    {
        let body = response.text()?;
        let json_response: Value = serde_json::from_str(&body)?;

        println!("Response: {}", json_response);
        parse_weather_data(json_response);
    }
    else
    {
        println!("Error! {}", response.status());
    }

    Ok(())
}

fn parse_weather_data(json_response: Value)
{
    let temperature = json_response["main"]["temp"].as_f64().unwrap_or_default();
    println!("Temperature: {} K", temperature);
}