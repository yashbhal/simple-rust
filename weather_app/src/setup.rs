#[derive(Deserialize)]
struct WeatherData 
{
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main
{
    temp: f64,
}

#[derive(Deserialize)]
struct Weather
{
    description: String,
}