use serde_json;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::config::{Config, CONFIG_FILE_PATH};

pub fn display_config() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(CONFIG_FILE_PATH).exists() {
        println!("provide an initial config. you may change it later on with `config`");
        prompt_and_save_config()?;
        return Ok(());
    }

    let config_contents = fs::read_to_string(CONFIG_FILE_PATH)?;
    let config: Config = serde_json::from_str(&config_contents)?;

    println!("remindf1 current config");
    println!("  free practice: {}", config.remind_free_practice);
    println!("  qualifying: {}", config.remind_quali);
    println!("  sprint: {}", config.remind_sprint);
    println!("  race: {}", config.remind_race);
    println!("  days before: {}", config.days_before_reminder);

    Ok(())
}

pub fn prompt_and_save_config() -> Result<(), Box<dyn std::error::Error>> {
    let remind_free_practice = prompt_bool("free practice sessions reminders? (y/n)")?;
    let remind_quali = prompt_bool("qualifying reminders? (y/n)")?;
    let remind_sprint = prompt_bool("sprint races reminders? (y/n)")?;
    let remind_race = prompt_bool("race reminders? (y/n)")?;

    let days_before_reminder =
        prompt_u32("how many days before the race do you want to be reminded?")?;

    let config = Config {
        remind_free_practice,
        remind_quali,
        remind_sprint,
        remind_race,
        days_before_reminder,
    };

    save_config(&config)?;

    Ok(())
}

fn prompt_bool(prompt: &str) -> Result<bool, Box<dyn std::error::Error>> {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("{}", prompt),
        }
    }
}

fn prompt_u32(prompt: &str) -> Result<u32, Box<dyn std::error::Error>> {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<u32>() {
            Ok(num) => return Ok(num),
            Err(_) => println!("please enter a valid number"),
        }
    }
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(CONFIG_FILE_PATH)?;
    let config_string = serde_json::to_string_pretty(config)?;
    file.write_all(config_string.as_bytes())?;
    println!("configuration saved successfully");
    Ok(())
}
