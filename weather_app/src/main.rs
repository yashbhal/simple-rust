use reqwest;
use serde_json::Value;
use std::error::Error;
use reqwest::blocking::Response;
mod setup;

fn main() -> Result<(), Box<dyn Error>> {
    let (lat, lon) = setup::get_location()?;
    let api_key = setup::get_api_key()?;
    let url = setup::construct_url(lat, lon, api_key)?;
    let response = reqwest::blocking::get(url)?;

    if response.status().is_success() {
        parse_weather_data(response)?;
    } else {
        println!("Error! {}", response.status());
    }

    Ok(())
}

fn parse_weather_data(response: Response) -> Result<(), Box<dyn Error>> {
    let body = response.text()?;
    let json_response: Value = serde_json::from_str(&body)?;

    let temperature = json_response["main"]["temp"].as_f64().unwrap_or_default();
    println!("Temperature: {} K", temperature);

    Ok(())
}
