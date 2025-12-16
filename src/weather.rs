use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub name: String,
    pub main: MainWeather,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct MainWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub humidity: u32,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
}