mod cli;
mod processor;
mod request;
mod types;
mod weather;

use weather::{Weather, WeatherBuilder, WeatherForecast, WeatherRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://api.weatherapi.com/v1";
    let weather_request: WeatherRequest = cli::extract_args();
    let url = processor::create_url(BASE_URL, &weather_request);
    let wb: WeatherBuilder = request::get(&url)?;
    let response: Weather = wb.build(weather_request.cels);
    println!("{}", response);
    // TEST
    // let response: Weather = serde_json::from_reader(std::fs::File::open("data/new_data.json")?)?;
    // println!("{}", response);
    // println!("{:#?}", response);
    // println!("{:#?}", response.current.air_quality.get("us-epa-index"));
    Ok(())
}
