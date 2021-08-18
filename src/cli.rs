use crate::WeatherRequest;
use structopt::StructOpt;

/// Nicely outputs the weather forecast of the requested date or dates.
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Desired location.
    location: Vec<String>,

    /// Number of days. Default 0. Max 3.
    #[structopt(short = "n", long = "days", default_value = "0")]
    days: u8,

    /// Opt in air quality
    #[structopt(short, long)]
    aqi: bool,
}

// Extracts args into `WeatherRequest` format.
pub fn extract_args() -> WeatherRequest {
    let args = Cli::from_args();
    let days = if args.days > 3 { 3 } else { args.days };
    let location = args.location.join(" ");
    // println!("{:#?}", args.aqi);
    WeatherRequest::new(days, location, args.aqi)
}
