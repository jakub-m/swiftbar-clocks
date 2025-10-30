use chrono::{Local, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    cities: Vec<CityConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CityConfig {
    name: String,
    timezone: String,
}

fn get_accurate_clock_icon(hour: u32, minute: u32) -> &'static str {
    // Round to nearest 30 minutes for clock face selection
    let rounded_minute = if minute < 15 {
        0
    } else if minute < 45 {
        30
    } else {
        0
    };

    let display_hour = if minute >= 45 { (hour + 1) % 12 } else { hour % 12 };

    match (display_hour, rounded_minute) {
        (12, 0) | (0, 0) => "🕐",
        (12, 30) | (0, 30) => "🕜",
        (1, 0) => "🕑",
        (1, 30) => "🕝",
        (2, 0) => "🕒",
        (2, 30) => "🕞",
        (3, 0) => "🕓",
        (3, 30) => "🕟",
        (4, 0) => "🕔",
        (4, 30) => "🕠",
        (5, 0) => "🕕",
        (5, 30) => "🕡",
        (6, 0) => "🕖",
        (6, 30) => "🕢",
        (7, 0) => "🕗",
        (7, 30) => "🕣",
        (8, 0) => "🕘",
        (8, 30) => "🕤",
        (9, 0) => "🕙",
        (9, 30) => "🕥",
        (10, 0) => "🕚",
        (10, 30) => "🕦",
        (11, 0) => "🕛",
        (11, 30) => "🕧",
        _ => "🕐",
    }
}

fn load_config() -> Config {
    // Check for config file from command line argument
    let config_path = env::args().nth(1).or_else(|| env::var("CLOCK_CONFIG").ok());

    if let Some(path) = config_path {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = serde_yaml::from_str::<Config>(&content) {
                return config;
            }
        }
    }

    // Default configuration
    Config {
        cities: vec![
            CityConfig {
                name: "New York".to_string(),
                timezone: "America/New_York".to_string(),
            },
            CityConfig {
                name: "London".to_string(),
                timezone: "Europe/London".to_string(),
            },
            CityConfig {
                name: "Tokyo".to_string(),
                timezone: "Asia/Tokyo".to_string(),
            },
        ],
    }
}

fn main() {
    let config = load_config();
    let local_time = Local::now();

    // Get clock icon based on current local minutes
    let clock_icon = get_accurate_clock_icon(
        local_time.hour(),
        local_time.minute()
    );

    let mut output = String::new();
    output.push_str(&format!("{}\n", clock_icon));

    for city in &config.cities {
        if let Ok(tz) = city.timezone.parse::<Tz>() {
            let city_time = local_time.with_timezone(&tz);
            output.push_str(&format!(
                "{:02}:{:02} {}\n",
                city_time.hour(),
                city_time.minute(),
                city.name
            ));
        } else {
            eprintln!("Warning: Invalid timezone '{}' for {}", city.timezone, city.name);
        }
    }

    // Write to stderr as specified
    let stderr = io::stderr();
    let mut handle = stderr.lock();
    let _ = handle.write_all(output.as_bytes());
}
