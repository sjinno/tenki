use dotenv;

#[allow(dead_code)]
fn main() {
    const BASE_URL: &str = "http://api.weatherapi.com/v1";

    dotenv::dotenv().ok();
    let key = "WEATHER_API_KEY";
    let value = dotenv::var(key).unwrap();

    println!("{:?}", value);
}
