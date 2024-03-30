use reqwest;
use std::error::Error;
mod setup;

fn main() -> Result<(), Box<dyn Error>> 
{
    let (lat, lon) = setup::get_location()?;
    let api_key = setup::get_api_key()?;
    let url = setup::construct_url(lat, lon, api_key)?;
    let response = reqwest::blocking::get(url)?;

    if response.status().is_success() 
    {
        setup::print_menu();
        let weather_data = setup::parse_weather_data(response)?;
        let user_choice = setup::read_input()?;
        let _ = setup::run_program(&user_choice, &weather_data);
    } 
    else 
    {
        println!("Error! {}", response.status());
    }

    Ok(())
}