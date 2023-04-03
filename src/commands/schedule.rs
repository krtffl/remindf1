use chrono::{Datelike, NaiveDate, Utc};
use serde_json::{from_str, json, Value};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::models::api::{Schedule, SeasonByYearResponse};
use crate::utils::{schedule_api_url, schedule_file_name};

pub async fn fetch_and_save_schedule(year: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_url = schedule_api_url(year);
    let response = reqwest::get(&api_url)
        .await?
        .json::<SeasonByYearResponse>()
        .await?;

    let schedule: Vec<_> = response
      .MRData
      .RaceTable
      .Races
      .into_iter()
      .map(|race| {
          json!({
              "raceName": race.raceName,
              "circuitName": race.Circuit.circuitName,
              "location": race.Circuit.Location,
              "round": race.round,
              "freePractice1": race.FirstPractice,
              "freePractice2": race.SecondPractice,
              "freePractice3": race.ThirdPractice,
              "qualy": race.Qualifying,
              "sprint": race.Sprint,
              "race": { "date": race.date.to_string(), "time": race.time.unwrap_or_else(|| "TBA".to_string())}
          })
      })
      .collect();

    let schedule_json = json!({ "schedule": schedule });
    let schedule_string = serde_json::to_string_pretty(&schedule_json)?;
    let schedule_file_path = schedule_file_name(year);

    if let Ok(local_schedule_content) = fs::read_to_string(schedule_file_path.clone()) {
        if let Ok(local_schedule_json) = serde_json::from_str::<Value>(&local_schedule_content) {
            if schedule_json == local_schedule_json {
                println!("local schedule is up-to-date");
                return Ok(());
            } else {
                let local_schedule = local_schedule_json["schedule"]
                    .as_array()
                    .ok_or("error parsing local schedule")?;

                for (index, race) in schedule.iter().enumerate() {
                    let local_race = &local_schedule[index];
                    let mut changes = HashMap::new();

                    for key in race.as_object().unwrap().keys() {
                        if race[key] != local_race[key] {
                            changes
                                .insert(key, (local_race[key].to_string(), race[key].to_string()));
                        }
                    }

                    if !changes.is_empty() {
                        println!(
                            "changes for round {}: {}",
                            race["round"].as_str().unwrap(),
                            race["raceName"].as_str().unwrap()
                        );

                        for (field, (old_value, new_value)) in changes {
                            println!("  {}: {} -> {}", field, old_value, new_value);
                        }
                    }
                }
            }
        }
    }

    let mut file = File::create(schedule_file_path)?;
    file.write_all(schedule_string.as_bytes())?;

    Ok(())
}

pub async fn fetch_and_display_next_race() -> Result<(), String> {
    let now = Utc::now().date_naive();
    let schedule_file_path = schedule_file_name(&now.year().to_string());

    if !Path::new(&schedule_file_path).exists() {
        fetch_and_save_schedule(&schedule_file_path)
            .await
            .map_err(|e| format!("error fetching schedule: {}", e))?;
    }

    let schedule_data = fs::read_to_string(schedule_file_path)
        .map_err(|e| format!("error reading schedule file: {}", e))?;

    let schedule: Schedule = from_str(&schedule_data)
        .map_err(|e| format!("error deserializing schedule data: {}", e))?;

    let next_race = schedule
        .races
        .into_iter()
        .find(|race| {
            NaiveDate::parse_from_str(&race.race.date, "%Y-%m-%d")
                .expect("failed to parse date string")
                > now
        })
        .ok_or("no upcoming races found")?;

    println!("next race: {}", next_race.raceName);
    println!("round: {}", next_race.round);
    println!("circuit: {}", next_race.circuitName);
    println!("date: {}", next_race.race.date);

    Ok(())
}
