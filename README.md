# Weather Fetching App

## Overview

This Rust application fetches weather information based on the provided city name. It uses external APIs to retrieve the latitude and longitude of the city and then uses this information to fetch weather data from another API.

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

## How to run

Replace <city_name> with the name of the city for which you want to fetch weather information.

```sh
$ cargo run -- <city_name>
```

Example :

```sh
$ cargo run -- Tokyo
```

## Output looks like below :

```sh
City: Tokyo
Latitude: 35.6821936
Longitude: 139.762221
Current Temperature: 15.2°C
Current Wind Speed: 2.6 km/h
Current Wind Direction: 236°

```
