mod cli;
mod processor;
mod request;
mod types;
mod weather;

use weather::{Weather, WeatherForecast, WeatherRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://api.weatherapi.com/v1";
    let weather_request = cli::extract_args();
    let url = processor::create_url(BASE_URL, &weather_request);
    println!("{}", url);
    // let response: Weather = request::get(&url)?;
    // println!("{}", response);
    // TEST
    let response: Weather =
        serde_json::from_reader(std::fs::File::open("data/data_air_hour.json")?)?;
    println!("{:#?}", response);
    // println!("{:#?}", response.current.air_quality.get("us-epa-index"));
    Ok(())
}
