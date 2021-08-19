mod cli;
mod processor;
mod query;
mod request;
mod types;
mod weather;

use query::QueryParams;
use weather::{Weather, WeatherBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "https://api.weatherapi.com/v1";
    let query_params: QueryParams = cli::extract_args();
    let url = processor::create_url(BASE_URL, &query_params);
    let wb: WeatherBuilder = request::get(&url)?;
    let response: Weather = wb.build(query_params.cels);
    println!("{}", response);
    // TEST
    // let response: Weather = serde_json::from_reader(std::fs::File::open("data/new_data.json")?)?;
    // println!("{}", response);
    // println!("{:#?}", response);
    // println!("{:#?}", response.current.air_quality.get("us-epa-index"));
    Ok(())
}
