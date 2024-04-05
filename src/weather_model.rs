use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub current_weather: CurrentWeather,
}

#[derive(Debug, Deserialize)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub windspeed: f64,
    pub winddirection: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoData {
    pub lat: String,
    pub lon: String,
}
