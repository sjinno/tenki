use std::os::raw::c_long;

use dotenv;
use form_urlencoded::Serializer;
use serde_derive::Deserialize;
// use serde_json::Value;
use structopt::StructOpt;

#[allow(dead_code)]
#[derive(Debug)]
enum WeatherForecast {
    Current,
    Forecast(u8),
}

#[derive(Debug)]
struct WeatherRequest<'a> {
    days: WeatherForecast,
    location: &'a str,
}

impl<'a> WeatherRequest<'a> {
    fn new(days: u8, location: &'a str) -> Self {
        Self {
            days: days.into(),
            location: location,
        }
    }
}

/// Nicely outputs the weather forecast of the requested date or dates.
#[derive(StructOpt, Debug)]
struct Cli {
    /// Number of days. Default 0. Max 3.
    days: u8,

    /// Desired location.
    location: String,
}

impl Into<WeatherForecast> for u8 {
    fn into(self) -> WeatherForecast {
        if self == 0 {
            WeatherForecast::Current
        } else {
            WeatherForecast::Forecast(self)
        }
    }
}

#[derive(Deserialize, Debug)]
struct Weather {
    location: Location,
    current: Current,
    forecast: Option<Forecast>,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}

#[derive(Deserialize, Debug)]
struct Current {
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: u8,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: u8,
    cloud: u8,
    feelslike_c: f32,
    feelslike_f: f32,
    uv: f32,
}

#[derive(Deserialize, Debug)]
struct DayWeatherSummary {
    maxtemp_c: f32,
    maxtemp_f: f32,
    mintemp_c: f32,
    mintemp_f: f32,
    daily_chance_of_rain: u8,
    daily_chance_of_snow: u8,
    condition: Condition,
    uv: f32,
}

#[derive(Deserialize, Debug)]
struct Astro {
    sunrise: String,
    sunset: String,
}

#[derive(Deserialize, Debug)]
struct ForecastDay {
    date: String,
    day: DayWeatherSummary,
    astro: Astro,
}
#[derive(Deserialize, Debug)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}

fn text_transform_capitalize<'a>(mut chars: std::str::Chars) -> String {
    let mut new_string = String::new();
    // Capitalize the initial letter.
    if let Some(c) = chars.next() {
        new_string = c.to_ascii_uppercase().to_string();
    }
    // Lowercase the rest of the letters.
    let rest = chars.map(|c| c.to_ascii_lowercase()).collect::<String>();
    new_string.push_str(&rest);
    new_string
}

#[allow(dead_code, unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Constant values
    const BASE_URL: &str = "https://api.weatherapi.com/v1";
    const AQI: &str = "no"; // air quality

    // 2. Read environment variables from .env
    dotenv::dotenv().ok();
    let key = "WEATHER_API_KEY";
    let value: &str = &dotenv::var(key).unwrap();
    // println!("{:?}", value);

    // 3. Read arguments
    // let location = "Portland";
    // println!("{:?}", Cli::from_args());
    let args = Cli::from_args();
    // let req = WeatherRequest::new(args.days, &args.location);
    let location = text_transform_capitalize(args.location.chars());
    let req = WeatherRequest::new(args.days, &location);
    println!("{:?}", req);
    // println!("{:?}", args.days);

    // let req: WeatherRequest = WeatherRequest::new();

    // // 4. Request data
    // // Current forecast URL:
    // // https://api.weatherapi.com/v1/current.json?key={KEY}&q={LOCATION}&aqi=no

    // // Forecast URL:
    // // https://api.weatherapi.com/v1/forecast.json?key={KEY}&q={LOCATION}&days={N}&aqi=no&alerts=no
    // let parameters = Serializer::new(String::new())
    //     .append_pair("key", value)
    //     .append_pair("q", location)
    //     .append_pair("aqi", "no")
    //     .finish();

    // let url = format!("{}/current.json?{}", BASE_URL, parameters);
    // // println!("{}", url);
    // let w: Weather = reqwest::blocking::get(url)?.json()?;
    // println!("{:#?}", w);

    // // let value: Value = serde_json::from_str(&body_json)?;
    // // println!("{:?}", value);

    // // TEST DATA
    // let data = r#"{
    //     "location": {
    //         "name": "Nagasaki-Shi",
    //         "region": "Nagasaki",
    //         "country": "Japan",
    //         "lat": 32.74,
    //         "lon": 129.87,
    //         "tz_id": "Asia/Tokyo",
    //         "localtime_epoch": 1628905254,
    //         "localtime": "2021-08-14 10:40"
    //     },
    //     "current": {
    //         "last_updated": "2021-08-14 10:30",
    //         "temp_c": 28.0,
    //         "temp_f": 82.4,
    //         "is_day": 1,
    //         "condition": {
    //             "text": "Light rain"
    //         },
    //         "wind_mph": 32.2,
    //         "wind_kph": 51.8,
    //         "wind_degree": 210,
    //         "precip_mm": 67.7,
    //         "precip_in": 2.67,
    //         "humidity": 84,
    //         "cloud": 75,
    //         "feelslike_c": 35.2,
    //         "feelslike_f": 95.3,
    //         "uv": 6.0
    //     }
    // }"#;
    // // // With serde_json:
    // // let v: Value = serde_json::from_str(data)?;
    // // println!("{:?}", v);
    // let w: Weather = serde_json::from_str(data)?;
    // println!("{:#?}", w);

    // let data_long = r#"{
    //     "location": {
    //         "name": "Portland",
    //         "region": "Oregon",
    //         "country": "United States of America",
    //         "lat": 45.52,
    //         "lon": -122.68,
    //         "tz_id": "America/Los_Angeles",
    //         "localtime_epoch": 1628918055,
    //         "localtime": "2021-08-13 22:14"
    //     },
    //     "current": {
    //         "last_updated": "2021-08-13 21:00",
    //         "temp_c": 28.3,
    //         "temp_f": 82.9,
    //         "is_day": 0,
    //         "condition": {
    //             "text": "Partly cloudy"
    //         },
    //         "wind_mph": 3.8,
    //         "wind_kph": 6.1,
    //         "precip_mm": 0.0,
    //         "precip_in": 0.0,
    //         "humidity": 59,
    //         "cloud": 50,
    //         "feelslike_c": 27.0,
    //         "feelslike_f": 80.7,
    //         "uv": 9.0
    //     },
    //     "forecast": {
    //         "forecastday": [
    //             {
    //                 "date": "2021-08-13",
    //                 "day": {
    //                     "maxtemp_c": 40.9,
    //                     "maxtemp_f": 105.6,
    //                     "mintemp_c": 23.2,
    //                     "mintemp_f": 73.8,
    //                     "daily_chance_of_rain": 0,
    //                     "daily_chance_of_snow": 0,
    //                     "condition": {
    //                         "text": "Partly cloudy"
    //                     },
    //                     "uv": 8.0
    //                 },
    //                 "astro": {
    //                     "sunrise": "06:09 AM",
    //                     "sunset": "08:20 PM"
    //                 }
    //             },
    //             {
    //                 "date": "2021-08-14",
    //                 "day": {
    //                     "maxtemp_c": 38.7,
    //                     "maxtemp_f": 101.7,
    //                     "mintemp_c": 22.8,
    //                     "mintemp_f": 73.0,
    //                     "daily_chance_of_rain": 0,
    //                     "daily_chance_of_snow": 0,
    //                     "condition": {
    //                         "text": "Partly cloudy"
    //                     },
    //                     "uv": 8.0
    //                 },
    //                 "astro": {
    //                     "sunrise": "06:10 AM",
    //                     "sunset": "08:19 PM"
    //                 }
    //             },
    //             {
    //                 "date": "2021-08-15",
    //                 "day": {
    //                     "maxtemp_c": 39.9,
    //                     "maxtemp_f": 103.8,
    //                     "mintemp_c": 22.4,
    //                     "mintemp_f": 72.3,
    //                     "daily_chance_of_rain": 0,
    //                     "daily_chance_of_snow": 0,
    //                     "condition": {
    //                         "text": "Partly cloudy"
    //                     },
    //                     "uv": 8.0
    //                 },
    //                 "astro": {
    //                     "sunrise": "06:12 AM",
    //                     "sunset": "08:17 PM"
    //                 }
    //             }
    //         ]
    //     }
    // }"#;

    // let w: Weather = serde_json::from_str(data_long)?;
    // println!("{:#?}", w);

    // 5. Style and format data

    // 6. Print formatted response

    Ok(())
}
