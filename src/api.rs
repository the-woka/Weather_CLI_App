use crate::weather::WeatherResponse;

const BASE_URL : &str = "https://api.openweathermap.org/data/2.5/weather";

pub async fn fetch_weather(
    city : &str,
    api_key : &str
) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let url = format!("{}?q={}&appid={}&units=metric", BASE_URL, city, api_key);

    let response = reqwest::get(&url).await?;

    if !response.status().is_success(){
        return Err(format!("Failed to fetch weather: {}", response.status()).into());
    }

    let weather = response.json::<WeatherResponse>().await?;

    Ok(weather)
}