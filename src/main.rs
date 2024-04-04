use anyhow::{anyhow, Result};
use reqwest::{header, Client};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct WeatherData {
    current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    winddirection: i32,
}

#[derive(Debug, Deserialize, Clone)]
struct GeoData {
    lat: String,
    lon: String,
}

async fn fetch_geo_data(city: &str) -> Result<GeoData> {
    let client: Client = Client::new();
    let encoded_city = urlencoding::encode(city);
    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json",
        encoded_city
    );
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "weather_app")
        .send()
        .await?;

    if response.status().is_success() {
        let geo_data: Vec<GeoData> = response.json().await?;
        if let Some(data) = geo_data.first() {
            Ok(data.clone())
        } else {
            Err(anyhow!("City not found"))
        }
    } else if response.status() == reqwest::StatusCode::FORBIDDEN {
        Err(anyhow!(
            "Request forbidden. Check Nominatim documentation for usage limits."
        ))
    } else {
        Err(anyhow!("Unexpected status code: {}", response.status()))
    }
}

async fn fetch_weather(latitude: f64, longitude: f64) -> Result<WeatherData> {
    let client: Client = Client::new();
    let url = format!("https://api.open-meteo.com/v1/forecast/?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,relativehumidity_2m,windspeed_10m", latitude, longitude);
    let response = client.get(&url).send().await?;
    let weather_data = response.json().await?;
    Ok(weather_data)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <city>", args[0]);
        return;
    }
    let city = &args[1];

    match fetch_geo_data(city).await {
        Ok(geo_data) => {
            println!("City: {}", city);
            println!("Latitude: {}", geo_data.lat);
            println!("Longitude: {}", geo_data.lon);

            if let Ok(lat) = geo_data.lat.parse::<f64>() {
                if let Ok(lon) = geo_data.lon.parse::<f64>() {
                    match fetch_weather(lat, lon).await {
                        Ok(weather_data) => {
                            println!(
                                "Current Temperature: {}°C",
                                weather_data.current_weather.temperature
                            );
                            println!(
                                "Current Wind Speed: {} km/h",
                                weather_data.current_weather.windspeed
                            );
                            println!(
                                "Current Wind Direction: {}°",
                                weather_data.current_weather.winddirection
                            );
                        }
                        Err(e) => {
                            eprintln!("Error fetching weather: {}", e);
                        }
                    }
                } else {
                    eprintln!("Error parsing longitude to f64");
                }
            } else {
                eprintln!("Error parsing latitude to f64");
            }
        }
        Err(e) => {
            eprintln!("Error fetching city data: {}", e);
        }
    }
}
