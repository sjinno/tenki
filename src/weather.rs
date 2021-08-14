use serde_derive::Deserialize;

#[derive(Debug)]
pub enum WeatherForecast {
    Current,
    Forecast(String),
}

impl From<u8> for WeatherForecast {
    fn from(n: u8) -> WeatherForecast {
        if n == 0 {
            WeatherForecast::Current
        } else {
            WeatherForecast::Forecast(n.to_string())
        }
    }
}

#[derive(Debug)]
pub struct WeatherRequest {
    pub days: WeatherForecast,
    pub location: String,
}

impl WeatherRequest {
    pub fn new(days: u8, location: String) -> Self {
        Self {
            days: days.into(),
            location,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Weather {
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
struct WeatherSummary {
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
    day: WeatherSummary,
    astro: Astro,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    forecastday: Vec<ForecastDay>,
}
