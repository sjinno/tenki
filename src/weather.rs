use crate::types::Aqi;

use ansi_term::{Colour, Style};
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::Deserialize;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table,
};

use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum WeatherForecast {
    Forecast(String),
}

impl From<u8> for WeatherForecast {
    fn from(n: u8) -> WeatherForecast {
        WeatherForecast::Forecast(n.to_string())
    }
}

#[derive(Debug)]
pub struct WeatherRequest {
    pub days: WeatherForecast,
    pub location: String,
    pub aqi: bool,
    pub cels: bool,
}

impl WeatherRequest {
    pub fn new(days: u8, location: String, aqi: bool, cels: bool) -> Self {
        Self {
            days: days.into(),
            location,
            aqi,
            cels,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WeatherBuilder {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

impl WeatherBuilder {
    pub fn build(self, cels: bool) -> Weather {
        Weather {
            location: self.location,
            current: self.current,
            forecast: self.forecast,
            cels,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
    pub cels: bool,
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
pub struct Hour {
    time: String,
    temp_c: f32,
    temp_f: f32,
    chance_of_rain: u8,
    chance_of_snow: u8,
    uv: f32,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
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
    air_quality: Option<Aqi>,
}

#[derive(Deserialize, Debug)]
struct WeatherSummary {
    maxtemp_c: f32,
    maxtemp_f: f32,
    mintemp_c: f32,
    mintemp_f: f32,
    avghumidity: f32,
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
    hour: Vec<Hour>,
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
        self.format_todays_forecast(&mut table);
        self.format_forecast(&mut table);
        table.render()
    }

    //# LOCATION
    fn format_location(&self, table: &mut Table) {
        let bold = Style::new().bold();
        let purple = Colour::Purple.bold();
        let location = self.get_location();
        let last_updated = self.get_last_updated();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            format!("{} | {}", bold.paint(location), purple.paint(last_updated)),
            6,
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
        self.set_weather_title(table);
        self.set_weather_header_1(table);
        self.set_weather_data_1(table);
        self.set_weather_header_2(table);
        self.set_weather_data_2(table);
    }

    fn set_weather_title(&self, table: &mut Table) {
        let style = Colour::Green.bold();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            style.paint("CURRENT WEATHER INFO"),
            6,
            Alignment::Center,
        )]));
    }

    fn set_weather_header_1(&self, table: &mut Table) {
        let bold = Style::new().bold();
        let temp_unit = if self.cels { "TEMP (C)" } else { "TEMP (F)" };
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(bold.paint(temp_unit), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("FEELS LIKE"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("CONDITION"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("WIND (KPH)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("PRECIP (MM)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("HUMIDITY (%)"), 1, Alignment::Center),
        ]));
    }

    fn set_weather_data_1(&self, table: &mut Table) {
        let temp = if self.cels {
            self.current.temp_c
        } else {
            self.current.temp_f
        };
        let feelslike = if self.cels {
            self.current.feelslike_c
        } else {
            self.current.feelslike_f
        };
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(temp, 1, Alignment::Center),
            TableCell::new_with_alignment(feelslike, 1, Alignment::Center),
            TableCell::new_with_alignment(&self.current.condition.text, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.wind_kph, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.precip_mm, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.humidity, 1, Alignment::Center),
        ]));
    }

    fn set_weather_header_2(&self, table: &mut Table) {
        let bold = Style::new().bold();
        // temp (c), condition, wind (kph), precip (mm), humidity, cloud, feels like (c), uv
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(bold.paint("CLOUD (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("UV"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("US EPA IDX"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("PM 2.5"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("PM 10"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("OZONE"), 1, Alignment::Center),
        ]));
    }

    fn set_weather_data_2(&self, table: &mut Table) {
        let mut data = vec![
            TableCell::new_with_alignment(self.current.cloud, 1, Alignment::Center),
            TableCell::new_with_alignment(self.current.uv, 1, Alignment::Center),
        ];
        if let Some(aqi) = &self.current.air_quality {
            data.extend_from_slice(&[
                TableCell::new_with_alignment(
                    aqi.get("us-epa-index").unwrap(),
                    1,
                    Alignment::Center,
                ),
                TableCell::new_with_alignment(
                    format!("{:.2}", aqi.get("pm2_5").unwrap().as_f64().unwrap()),
                    1,
                    Alignment::Center,
                ),
                TableCell::new_with_alignment(
                    format!("{:.2}", aqi.get("pm10").unwrap().as_f64().unwrap()),
                    1,
                    Alignment::Center,
                ),
                TableCell::new_with_alignment(
                    format!("{:.2}", aqi.get("o3").unwrap().as_f64().unwrap()),
                    1,
                    Alignment::Center,
                ),
            ]);
        }
        table.add_row(Row::new(data));
    }
    //# CURRENT ENDS

    //# TODAY"S FORECAST
    fn format_todays_forecast(&self, table: &mut Table) {
        let fd = &self.forecast.forecastday[0];
        self.set_forecast_title(fd, table);
        self.set_forecast_header(table);
        self.set_forecast_data(fd, table);
        self.set_forecast_hour(&fd.hour, table);
    }

    //# FORECAST
    fn format_forecast(&self, table: &mut Table) {
        self.set_forecast(table);
    }

    fn set_forecast(&self, table: &mut Table) {
        let yellow = Colour::Yellow.bold();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            yellow.paint(format!(
                "WEATHER FORECAST FOR THE NEXT {} DAYS",
                self.forecast.forecastday.len() - 1
            )),
            6,
            Alignment::Center,
        )]));

        self.forecast.forecastday[1..].iter().for_each(|fd| {
            self.set_forecast_title(fd, table);
            self.set_forecast_header(table);
            self.set_forecast_data(fd, table);
            self.set_forecast_hour(&fd.hour, table);
        });
    }

    fn set_forecast_title(&self, fd: &ForecastDay, table: &mut Table) {
        let purple = Colour::Purple.bold();
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            purple.paint(format!(
                "{date}, {day}",
                date = &fd.date,
                day = NaiveDate::parse_from_str(&fd.date, "%Y-%m-%d")
                    .unwrap()
                    .format("%A")
                    .to_string()
                    .to_ascii_uppercase()
            )),
            6,
            Alignment::Center,
        )]));
    }

    fn set_forecast_header(&self, table: &mut Table) {
        let bold = Style::new().bold();
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(
                format!("{}\n{}", bold.paint("MAX TEMP"), bold.paint("MIN TEMP")),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(
                format!("{}\n{}", bold.paint("SUNRISE"), bold.paint("SUNSET")),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(bold.paint("CONDITION"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("RAIN (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("SNOW (%)"), 1, Alignment::Center),
            TableCell::new_with_alignment(bold.paint("UV"), 1, Alignment::Center),
        ]));
    }

    fn set_forecast_data(&self, fd: &ForecastDay, table: &mut Table) {
        let max = if self.cels {
            fd.day.maxtemp_c
        } else {
            fd.day.maxtemp_f
        };
        let min = if self.cels {
            fd.day.mintemp_c
        } else {
            fd.day.mintemp_f
        };

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(format!("{}\n{}", max, min), 1, Alignment::Center),
            TableCell::new_with_alignment(
                format!("{}\n{}", &fd.astro.sunrise, &fd.astro.sunset),
                1,
                Alignment::Center,
            ),
            TableCell::new_with_alignment(&fd.day.condition.text, 1, Alignment::Center),
            TableCell::new_with_alignment(fd.day.daily_chance_of_rain, 1, Alignment::Center),
            TableCell::new_with_alignment(fd.day.daily_chance_of_snow, 1, Alignment::Center),
            TableCell::new_with_alignment(fd.day.uv, 1, Alignment::Center),
        ]));
    }

    fn set_forecast_hour(&self, hrs: &[Hour], table: &mut Table) {
        let chunks = hrs.chunks(6);
        chunks.for_each(|hr| {
            // println!("{:#?}", h);
            self.set_forecast_hour_header(hr, table);
            self.set_forecast_hour_data(hr, table);
        })
    }

    fn set_forecast_hour_header(&self, hr: &[Hour], table: &mut Table) {
        let bold = Style::new().bold();
        let header = hr
            .iter()
            .map(|h| {
                TableCell::new_with_alignment(
                    bold.paint(
                        NaiveDateTime::parse_from_str(&h.time, "%Y-%m-%d %H:%M")
                            .unwrap()
                            .format("%H:%M")
                            .to_string(),
                    ),
                    1,
                    Alignment::Center,
                )
            })
            .collect::<Vec<_>>();
        table.add_row(Row::new(header));
    }

    fn set_forecast_hour_data(&self, hr: &[Hour], table: &mut Table) {
        let green = Colour::Green;

        let data = hr
            .iter()
            .map(|h| {
                let temp = if self.cels {
                    h.temp_c.to_string()
                } else {
                    h.temp_f.to_string()
                };
                TableCell::new_with_alignment(
                    format!(
                        "TEMP: {}\nRAIN: {}\nSNOW: {}\nUV: {}",
                        green.paint(temp),
                        h.chance_of_rain,
                        h.chance_of_snow,
                        h.uv
                    ),
                    1,
                    Alignment::Center,
                )
            })
            .collect::<Vec<_>>();
        table.add_row(Row::new(data));
    }

    //# FORECAST ENDS
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.create_table_format())
    }
}
