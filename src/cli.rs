use crate::WeatherRequest;
use structopt::StructOpt;

/// Nicely outputs the weather forecast of the requested date or dates.
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Number of days. Default 0. Max 3.
    days: u8,

    /// Desired location.
    location: String,
}

// Extracts args to `WeatherRequest` format.
pub fn extract_args() -> WeatherRequest {
    let args = Cli::from_args();
    let location = text_transform_capitalize(args.location.chars());
    WeatherRequest::new(args.days, location)
}

// Capitalizes the initial letter and lowercase the rest.
fn text_transform_capitalize(chars: std::str::Chars) -> String {
    chars
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}
