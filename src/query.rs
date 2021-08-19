#[derive(Debug)]
pub struct QueryParams {
    pub days: String,
    pub location: String,
    pub aqi: bool,
    pub cels: bool,
}

impl QueryParams {
    pub fn new(days: u8, location: String, aqi: bool, cels: bool) -> Self {
        Self {
            days: days.to_string(),
            location,
            aqi,
            cels,
        }
    }
}
