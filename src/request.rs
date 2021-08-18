use crate::WeatherBuilder;

pub fn get(url: &str) -> Result<WeatherBuilder, reqwest::Error> {
    reqwest::blocking::get(url)?.json::<WeatherBuilder>()
}
