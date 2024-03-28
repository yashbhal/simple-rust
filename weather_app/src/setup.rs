use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WeatherData 
{
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
pub struct Main
{
    temp: f64,
}

#[derive(Deserialize)]
pub struct Weather
{
    description: String,
}