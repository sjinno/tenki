use dotenv;
use form_urlencoded::Serializer;
use serde_json::Value;
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

// #[derive(Deserialize)]
// struct WeatherForecast {
//     location:
// }

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
    let location = "Portland";
    println!("{:?}", Cli::from_args());

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
    // let body_json = reqwest::blocking::get(url)?.text()?;
    // let value: Value = serde_json::from_str(&body_json)?;
    // println!("{:?}", value);
    let data = r#"{
        "location": {
            "name": "Nagasaki-Shi",
            "region": "Nagasaki",
            "country": "Japan",
            "lat": 32.74,
            "lon": 129.87,
            "tz_id": "Asia/Tokyo",
            "localtime_epoch": 1628905254,
            "localtime": "2021-08-14 10:40"
        },
        "current": {
            "last_updated": "2021-08-14 10:30",
            "temp_c": 28.0,
            "temp_f": 82.4,
            "is_day": 1,
            "condition": {
                "text": "Light rain"
            },
            "wind_mph": 32.2,
            "wind_kph": 51.8,
            "wind_degree": 210,
            "precip_mm": 67.7,
            "precip_in": 2.67,
            "humidity": 84,
            "cloud": 75,
            "feelslike_c": 35.2,
            "feelslike_f": 95.3,
            "uv": 6.0
        }
    }"#;
    let v: Value = serde_json::from_str(data)?;
    println!("{:?}", v);

    // 5. Style and format data

    // 6. Print formatted response

    Ok(())
}
