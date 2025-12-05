use std::io::{self, Write};
use serde::Deserialize;
use colored::*;

// Structs to deserialize JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to fetch weather info from API
fn get_weather_info(city: &str, country_code: &str, api_key: &str)
    -> Result<WeatherResponse, reqwest::Error>
{
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to get emoji based on temperature
fn get_temp_emoji(temp: f64) -> &'static str {
    match temp {
        t if t < 0.0 => "❄️",
        t if t < 10.0 => "🧥",
        t if t < 20.0 => "🌤️",
        t if t < 30.0 => "☀️",
        _ => "🔥",
    }
}

// Function to display weather info
fn display_weather_info(response: &WeatherResponse) {
    let description = &response.weather[0].description;
    let temp = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
> Temperature: {:.1}°C
> Humidity: {:.1}%
> Pressure: {:.1} hPa
> Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temp),
        temp,
        humidity,
        pressure,
        wind_speed
    );

    // Color text based on weather description
    let weather_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" => weather_text.bright_white(),
        "rain" | "shower rain" => weather_text.bright_blue(),
        "thunderstorm" => weather_text.bright_magenta(),
        "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_colored);
}

fn main() {
    println!("{}", "Welcome to the Weather Station!".bright_green().bold());

    let api_key = "YOUR_API_KEY_HERE"; // Replace with your OpenWeatherMap API key

    loop {
        // Read city
        print!("{}", "Enter city name: ".bright_blue());
        io::stdout().flush().unwrap();
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        // Read country code
        print!("{}", "Enter country code (e.g., US, KE): ".bright_blue());
        io::stdout().flush().unwrap();
        let mut country = String::new();
        io::stdin().read_line(&mut country).expect("Failed to read input");
        let country = country.trim();

        // Fetch weather
        match get_weather_info(city, country, api_key) {
            Ok(response) => display_weather_info(&response),
            Err(err) => eprintln!("Error fetching weather: {}", err),
        }

        // Ask if user wants to continue
        print!("{}", "Search another city? (yes/no): ".bright_green());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("{}", "Thank you for using Weather Station!".bright_yellow());
            break;
        }
    }
}
