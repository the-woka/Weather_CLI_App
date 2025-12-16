mod weather;
mod api;

use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
#[command(name = "Weather API App")]
#[command(version = "0.1")]
#[command(about = "Get current weather for a city", long_about = None)]
struct Cli{
    city: String,
}

fn display_weather(weather: &weather::WeatherResponse) {
    println!("Weather in {}", weather.name);
    println!("-------------------");
    println!("Temperature: {:.1}°C", weather.main.temp);
    println!("Feels like: {:.1}°C", weather.main.feels_like);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Wind Speed: {} m/s", weather.wind.speed);

    if let Some(condition) = weather.weather.first() {
        println!("Condition: {}", condition.description);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let api_key = match env::var("API_KEY") {
        Ok(key) => key, 
        Err(_) => {
            eprintln!("Error: API_KEY not found in .env file");
            eprintln!("Please create a .env file with: API_KEY=your_api_key");
            return;
        }
    };

    match api::fetch_weather(&cli.city, &api_key).await {
        Ok(weather) => {
            display_weather(&weather);
        } Err(e) => {
            eprintln!("Error: {}", e)
        }
    }


}
