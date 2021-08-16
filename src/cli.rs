use crate::WeatherRequest;
use structopt::StructOpt;

/// Nicely outputs the weather forecast of the requested date or dates.
#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab")]
pub struct Cli {
    /// Desired location.
    location: String,

    /// Number of days. Default 0. Max 3.
    days: Option<u8>,
}

// Extracts args into `WeatherRequest` format.
pub fn extract_args() -> WeatherRequest {
    let args = Cli::from_args();
    let days = match args.days {
        Some(n) if n <= 3 => 3,
        None => 0,
        _ => 3,
    };
    let location = text_transform_capitalize(args.location.chars());
    WeatherRequest::new(days, location)
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
