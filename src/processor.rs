use crate::{weather::WeatherRequest, WeatherForecast};
use form_urlencoded::Serializer;

pub fn create_url(base_url: &str, wr: &WeatherRequest) -> String {
    // 2. Read environment variables from .env
    dotenv::dotenv().ok();
    let key = "WEATHER_API_KEY";
    let value = dotenv::var(key).unwrap();

    // 4. Request data
    // Current forecast URL:
    // https://api.weatherapi.com/v1/current.json?key={KEY}&q={LOCATION}&aqi=no

    // Forecast URL:
    // https://api.weatherapi.com/v1/forecast.json?key={KEY}&q={LOCATION}&days={N}&aqi=no&alerts=no

    let aqi = if wr.aqi { "yes" } else { "no" };
    let params;
    match &wr.days {
        WeatherForecast::Forecast(n) => {
            params = Serializer::new(String::new())
                .append_pair("key", &value)
                .append_pair("q", &wr.location)
                .append_pair("days", &n)
                .append_pair("aqi", aqi)
                .append_pair("alerts", "no")
                .finish();
            format!("{}/forecast.json?{}", base_url, params)
        }
    }
}
