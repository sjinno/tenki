use crate::Weather;

pub fn get(url: &str) -> Result<Weather, reqwest::Error> {
    reqwest::blocking::get(url)?.json::<Weather>()
}
