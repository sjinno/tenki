use ansi_term::{Colour, Style};
use serde_derive::Deserialize;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table,
};

use std::fmt::{Display, Formatter, Result as FmtResult};

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
    pub location: Location,
    pub current: Current,
    pub forecast: Option<Forecast>,
}

#[derive(Deserialize, Debug)]
struct Condition {
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
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
pub struct Forecast {
    forecastday: Vec<ForecastDay>,
}

impl Weather {
    fn create_table_format(&self) -> String {
        let mut table = term_table::Table::new();
        table.max_column_width = 42;
        table.style = term_table::TableStyle::simple();
        self.format_location(&mut table);
        self.format_current_weather(&mut table);
        if self.forecast.is_some() {
            self.format_forecast(&mut table);
        }
        table.render()
    }

    //# LOCATION
    fn format_location(&self, table: &mut Table) {
        let style = Style::new().bold();
        let location = self.get_location();
        let last_updated = self.get_last_updated();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            style.paint(format!("{} --- {}", location, last_updated)),
            8,
            Alignment::Center,
        )]));
    }

    fn get_location(&self) -> String {
        format!(
            "{city}, {region} {localtime}",
            city = self.location.name.to_ascii_uppercase(),
            region = self.location.region.to_ascii_uppercase(),
            localtime = self.location.localtime
        )
    }

    fn get_last_updated(&self) -> String {
        format!(
            "LAST UPDATED: {last_updated}",
            last_updated = self.current.last_updated
        )
    }
    //# LOCATION ENDS

    //# CURRENT
    fn format_current_weather(&self, table: &mut Table) {
        // 1. Set title
        self.set_weather_title(table);
        // 2. Set header
        // temp (c), condition, wind (kph), precip (mm), humidity, cloud, feels like (c), uv
        // Total 8 columns
        self.set_weather_header(table);
        // 3. Set data
        self.set_weather_data(table);
    }

    fn set_weather_title(&self, table: &mut Table) {
        let style = Colour::Green.bold();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            style.paint("CURRENT WEATHER INFO"),
            8,
            Alignment::Center,
        )]));
    }

    fn set_weather_header(&self, table: &mut Table) {
        let bold = Style::new().bold();
        // temp (c), condition, wind (kph), precip (mm), humidity, cloud, feels like (c), uv
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(bold.paint("temp_c"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("cond"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("wind_kph"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("precip_mm"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("humidity (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("cloud"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("feelslike_c"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("uv"), 1, Alignment::Center),
        ]));
    }

    fn set_weather_data(&self, table: &mut Table) {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(self.current.temp_c, 1, Alignment::Center),
            TableCell::new_with_alignment(&self.current.condition.text, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.wind_kph, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.precip_mm, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.humidity, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.cloud, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.feelslike_c, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.uv, 1, Alignment::Center),
        ]));
    }
    //# CURRENT ENDS

    //# FORECAST
    fn format_forecast(&self, table: &mut Table) {
        // 1. Set title
        self.set_forecast_title(table);
        // 2. Set header
        self.set_forecast_header(table);
        // date, temp (c), condition, wind (kph), precip (mm), humidity, cloud, feels like (c), uv
        // Total 9 columns
        // 3. Set data
        self.set_forecast_data(table);
    }

    fn set_forecast_title(&self, table: &mut Table) {
        let style = Colour::Yellow.bold();
        if let Some(forecast) = &self.forecast {
            table.add_row(Row::new(vec![TableCell::new_with_alignment(
                style.paint(format!(
                    "WEATHER FORECAST FOR THE NEXT {} DAYS",
                    forecast.forecastday.len()
                )),
                8,
                Alignment::Center,
            )]));
        }
    }

    fn set_forecast_header(&self, table: &mut Table) {
        let bold = Style::new().bold();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(bold.paint("date"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("maxmin_temp_c"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("cond"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("rain (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("snow (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("sunrise"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("sunset"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("uv"), 1, Alignment::Center),
        ]));
    }

    fn set_forecast_data(&self, table: &mut Table) {
        if let Some(forecast) = &self.forecast {
            let bold = Style::new().bold();
            forecast.forecastday.iter().for_each(|fd| {
                table.add_row(Row::new(vec![
                    TableCell::new_with_alignment(bold.paint(&fd.date), 1, Alignment::Center),
                    TableCell::new_with_alignment(
                        format!("{} / {}", fd.day.maxtemp_c, fd.day.mintemp_c),
                        1,
                        Alignment::Center,
                    ),
                    TableCell::new_with_alignment(&fd.day.condition.text, 1, Alignment::Center),
                    TableCell::new_with_alignment(
                        fd.day.daily_chance_of_rain,
                        1,
                        Alignment::Center,
                    ),
                    TableCell::new_with_alignment(
                        fd.day.daily_chance_of_snow,
                        1,
                        Alignment::Center,
                    ),
                    TableCell::new_with_alignment(&fd.astro.sunrise, 1, Alignment::Center),
                    TableCell::new_with_alignment(&fd.astro.sunset, 1, Alignment::Center),
                    TableCell::new_with_alignment(fd.day.uv, 1, Alignment::Center),
                ]));
            });
        }
    }
    //# FORECAST ENDS
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.create_table_format())
    }
}
