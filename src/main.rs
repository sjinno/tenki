use dotenv;
use structopt::StructOpt;

#[allow(dead_code)]
enum WeatherRequest {
    Current,
    Future,
}

/// Nicely outputs the weather forecast of the requested date or dates.
#[derive(StructOpt, Debug)]
struct Cli {
    /// Number of days. Default 0. Max 3.
    days: u8,

    /// Desired location.
    location: String,
}

#[allow(dead_code, unused_variables)]
fn main() {
    // 1. Constant values
    const BASE_URL: &str = "http://api.weatherapi.com/v1";

    // 2. Read environment variables from .env
    dotenv::dotenv().ok();
    let key = "WEATHER_API_KEY";
    let value = dotenv::var(key).unwrap();
    // println!("{:?}", value);

    // 3. Read arguments
    println!("{:?}", Cli::from_args());

    // 4. Request data
    // Current forecast URL:
    // https://api.weatherapi.com/v1/current.json?key={KEY}&q={LOCATION}&aqi=no

    // Forecast URL:
    // https://api.weatherapi.com/v1/forecast.json?key={KEY}&q={LOCATION}&days={N}&aqi=no&alerts=no

    // 5. Style and format data

    // 6. Print formatted response
}
