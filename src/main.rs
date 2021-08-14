mod cli;
mod processor;
mod request;
mod weather;

use weather::{Weather, WeatherForecast, WeatherRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://api.weatherapi.com/v1";
    let weather_request = cli::extract_args();
    println!("{:#?}", weather_request);
    let url = processor::create_url(BASE_URL, &weather_request);
    let response = request::get(&url);
    println!("{:#?}", response);

    // 5. Style and format data

    // 6. Print formatted response

    Ok(())
}
