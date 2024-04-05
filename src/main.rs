// main.rs

mod api;
mod weather_model;

use anyhow::{anyhow, Result};
use prettytable::{row, Table};
use reqwest::{header, Client};
use std::env;
use weather_model::{GeoData, WeatherData};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <city>", args[0]);
        return Ok(());
    }
    let city = &args[1];

    let geo_data = fetch_geo_data(city).await?;
    let weather_data = fetch_weather(&geo_data).await?;

    print_weather_table(city, &geo_data, &weather_data);

    Ok(())
}

async fn fetch_geo_data(city: &str) -> Result<GeoData> {
    let encoded_city = urlencoding::encode(city);
    let url = format!("{}?q={}&format=json", api::NOMINATIM_API, encoded_city);
    let client = Client::new();
    let response = client
        .get(&url)
        .header(header::USER_AGENT, "WhetherFetchingApp")
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

async fn fetch_weather(geo_data: &GeoData) -> Result<WeatherData> {
    let url = format!(
        "{}?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,relativehumidity_2m,windspeed_10m",
        api::OPEN_METEO_API, geo_data.lat, geo_data.lon
    );
    let response = reqwest::get(&url).await?;
    let weather_data = response.json().await?;
    Ok(weather_data)
}

fn print_weather_table(city: &str, geo_data: &GeoData, weather_data: &WeatherData) {
    let mut table = Table::new();

    //Adding headers to table
    table.add_row(row![bFg->"City", bFg->"Latitude", bFg->"Longitude"]);
    table.add_row(row![city, geo_data.lat, geo_data.lon]);

    // Adding weather data to the table
    table.add_row(row![bFg->"Current Temperature (°C)", bFg->"Current Wind Speed (km/h)", bFg->"Current Wind Direction (°)"]);
    table.add_row(row![
        weather_data.current_weather.temperature,
        weather_data.current_weather.windspeed,
        weather_data.current_weather.winddirection
    ]);

    table.printstd();
}
