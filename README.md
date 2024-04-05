# Weather Fetching App

## Overview

This Rust application fetches weather information based on the provided city name. It uses external APIs to retrieve the latitude and longitude of the city and then uses this information to fetch weather data from another API.

## Rust and Cargo Setup
First, complete the basic Rust and Cargo setup instructions.


## Building and Running Locally
If you want to learn how to use weather_app, please follow below:
```sh
# Clone to repository
git clone https://github.com/NadigerAmit/weather-fetching-app.git
cd weather_app

# Build the code in release mode 
# Replace <city_name> with the name of the city for which you want to fetch weather information.
cargo build --release -p weather_app

# How to run the executable
cd target/release
./weather_app <city_name>

# Example 
./weather_app --Tokyo

```

## How to run locally 

Replace <city_name> with the name of the city for which you want to fetch weather information.

```sh
$ cargo run -- <city_name>
```

Example :

```sh
$ cargo run -- Tokyo
```

## output looks like below 
```sh
+--------------------------+---------------------------+----------------------------+
| City                     | Latitude                  | Longitude                  |
+--------------------------+---------------------------+----------------------------+
| --Tokyo                  | 35.6821936                | 139.762221                 |
+--------------------------+---------------------------+----------------------------+
| Current Temperature (°C) | Current Wind Speed (km/h) | Current Wind Direction (°) |
+--------------------------+---------------------------+----------------------------+
| 9                        | 2.2                       | 9                          |
+--------------------------+---------------------------+----------------------------+
```


## APIs Used
Nominatim API: Used to retrieve the latitude and longitude of a city based on its name.
Example below :
```sh
https://nominatim.openstreetmap.org/search?q={Bangalore}&format=json
```

Open Meteo API: Used to fetch weather information based on latitude and longitude.
Example below :
```sh
https://api.open-meteo.com/v1/forecast/?latitude=52.52&longitude=13.41&current_weather=true
```

