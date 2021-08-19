use crate::QueryParams;
use form_urlencoded::Serializer;

pub fn create_url(base_url: &str, query_params: &QueryParams) -> String {
    // 2. Read environment variables from .env
    dotenv::dotenv().ok();
    let key = "WEATHER_API_KEY";
    let value = dotenv::var(key).unwrap();

    // 4. Request data
    // Forecast URL:
    // https://api.weatherapi.com/v1/forecast.json?key={KEY}&q={LOCATION}&days={N}&aqi=yes&alerts=no
    let aqi = if query_params.aqi { "yes" } else { "no" };
    let params = Serializer::new(String::new())
        .append_pair("key", &value)
        .append_pair("q", &query_params.location)
        .append_pair("days", &query_params.days)
        .append_pair("aqi", aqi)
        .append_pair("alerts", "no")
        .finish();
    format!("{}/forecast.json?{}", base_url, params)
}
