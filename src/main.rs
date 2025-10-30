use chrono::{Local, Timelike};
use chrono_tz::Tz;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const DEFAULT_SWIFTBAR_CLOCK_CONFIG: &str = "~/.config/swiftbar_clock_config.yaml";

// Clock face constants (1F55B-1F567)
const CLOCK_1200: &str = "\u{1F55B}"; // twelve o'clock
const CLOCK_1230: &str = "\u{1F567}"; // twelve-thirty
const CLOCK_0100: &str = "\u{1F550}"; // one o'clock
const CLOCK_0130: &str = "\u{1F55C}"; // one-thirty
const CLOCK_0200: &str = "\u{1F551}"; // two o'clock
const CLOCK_0230: &str = "\u{1F55D}"; // two-thirty
const CLOCK_0300: &str = "\u{1F552}"; // three o'clock
const CLOCK_0330: &str = "\u{1F55E}"; // three-thirty
const CLOCK_0400: &str = "\u{1F553}"; // four o'clock
const CLOCK_0430: &str = "\u{1F55F}"; // four-thirty
const CLOCK_0500: &str = "\u{1F554}"; // five o'clock
const CLOCK_0530: &str = "\u{1F560}"; // five-thirty
const CLOCK_0600: &str = "\u{1F555}"; // six o'clock
const CLOCK_0630: &str = "\u{1F561}"; // six-thirty
const CLOCK_0700: &str = "\u{1F556}"; // seven o'clock
const CLOCK_0730: &str = "\u{1F562}"; // seven-thirty
const CLOCK_0800: &str = "\u{1F557}"; // eight o'clock
const CLOCK_0830: &str = "\u{1F563}"; // eight-thirty
const CLOCK_0900: &str = "\u{1F558}"; // nine o'clock
const CLOCK_0930: &str = "\u{1F564}"; // nine-thirty
const CLOCK_1000: &str = "\u{1F559}"; // ten o'clock
const CLOCK_1030: &str = "\u{1F565}"; // ten-thirty
const CLOCK_1100: &str = "\u{1F55A}"; // eleven o'clock
const CLOCK_1130: &str = "\u{1F566}"; // eleven-thirty

#[derive(Parser, Debug)]
#[command(name = "swiftbar_clocks")]
#[command(about = "Display world clocks with unicode clock icons", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, env = "SWIFTBAR_CLOCK_CONFIG", default_value=DEFAULT_SWIFTBAR_CLOCK_CONFIG)]
    config: String,

    /// List all available timezones
    ///
    /// See also: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
    #[arg(short = 'l', long = "list-timezones")]
    list_timezones: bool,
}

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

    let display_hour = if minute >= 45 {
        (hour + 1) % 12
    } else {
        hour % 12
    };

    match (display_hour, rounded_minute) {
        (12, 0) | (0, 0) => CLOCK_1200,
        (12, 30) | (0, 30) => CLOCK_1230,
        (1, 0) => CLOCK_0100,
        (1, 30) => CLOCK_0130,
        (2, 0) => CLOCK_0200,
        (2, 30) => CLOCK_0230,
        (3, 0) => CLOCK_0300,
        (3, 30) => CLOCK_0330,
        (4, 0) => CLOCK_0400,
        (4, 30) => CLOCK_0430,
        (5, 0) => CLOCK_0500,
        (5, 30) => CLOCK_0530,
        (6, 0) => CLOCK_0600,
        (6, 30) => CLOCK_0630,
        (7, 0) => CLOCK_0700,
        (7, 30) => CLOCK_0730,
        (8, 0) => CLOCK_0800,
        (8, 30) => CLOCK_0830,
        (9, 0) => CLOCK_0900,
        (9, 30) => CLOCK_0930,
        (10, 0) => CLOCK_1000,
        (10, 30) => CLOCK_1030,
        (11, 0) => CLOCK_1100,
        (11, 30) => CLOCK_1130,
        _ => CLOCK_1200,
    }
}

fn list_timezones() {
    // chrono-tz provides TZ_VARIANTS constant with all timezones
    for tz in chrono_tz::TZ_VARIANTS {
        println!("{}", tz.name());
    }
}

fn load_config(path: String) -> Config {
    // Try provided path first
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(config) = serde_yaml::from_str::<Config>(&content) {
            return config;
        }
    }

    // If loading failed and path starts with ~/, expand it and try again
    if path.starts_with("~/") {
        if let Some(home) = env::var_os("HOME") {
            let mut expanded_path = PathBuf::from(home);
            expanded_path.push(&path[2..]);

            if let Ok(content) = fs::read_to_string(&expanded_path) {
                if let Ok(config) = serde_yaml::from_str::<Config>(&content) {
                    return config;
                }
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
    let args = Args::parse();

    // If list-timezones flag is set, list timezones and exit
    if args.list_timezones {
        list_timezones();
        return;
    }

    let config = load_config(args.config);
    let local_time = Local::now();

    // Get clock icon based on current local minutes
    let clock_icon = get_accurate_clock_icon(local_time.hour(), local_time.minute());

    let mut output = String::new();
    output.push_str(&format!("{}\n", clock_icon));
    output.push_str("---\n");

    output.push_str(&format!("{}\n", local_time.to_rfc2822()));
    for city in config.cities {
        if let Ok(tz) = city.timezone.parse::<Tz>() {
            let city_time = local_time.with_timezone(&tz);
            output.push_str(&format!(
                "{:02}:{:02} {}\n",
                city_time.hour(),
                city_time.minute(),
                city.name
            ));
        } else {
            eprintln!(
                "Warning: Invalid timezone '{}' for {}",
                city.timezone, city.name
            );
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let _ = handle.write_all(output.as_bytes());
}
