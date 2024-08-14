use colored::*;
use serde::Deserialize;
use std::io;

// Struct to deserialize the JSON response from openWeatherMap API

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather desc

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent the main weather params

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get the weather information from the API

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;

    let response_json: WeatherResponse = response.json()?;
    Ok(response_json)
}

fn display_weather_info(response: &WeatherResponse) {
    // Extract the weather info from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    // Formatting weather info into a string
    let weather_text = format!(
        "weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    );

    // Coloring the weather based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow().to_string(),
        "few clouds" | "scattered clouds" | "broken clouds" => {
            weather_text.bright_blue().to_string()
        }
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed().to_string()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan().to_string(),
        _ => weather_text.normal().to_string(),
    };

    println!("{}", weather_text_colored);
}

// Function to get emoji

fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️ "
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️ "
    } else if temperature >= 20.0 && temperature < 30.0 {
        "⛅"
    } else if temperature >= 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    println!("{}", "Welcome to the Weather Station!".bright_yellow());

    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("faild to read input");

        let city: &str = city.trim();

        println!(
            "{}",
            "Please enter the name of the country code (e.g, FR for France):".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("faild to read input");

        let country_code: &str = country_code.trim();

        let api_key = "db47082bfa67a1c1948235895ecb830c";

        match get_weather_info(city, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprint!("Error: {}", err);
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        );

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Only yes or no answer please");

        let input = input.trim();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}
