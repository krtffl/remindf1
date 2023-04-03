use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::config::RECIPIENTS_FILE_PATH;
use crate::utils::is_valid_email;

pub fn read_emails_from_file(file_path: std::path::PathBuf) -> Result<Vec<String>, String> {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "csv" => read_emails_from_csv(file_path),
        "txt" => read_emails_from_txt(file_path),
        "json" => read_emails_from_json(file_path),
        _ => Err("unsupported file type. please provide a .csv, .txt, or .json file.".into()),
    }
}

pub fn display_recipients() -> Result<(), String> {
    if let Ok(recipients_json) = fs::read_to_string(RECIPIENTS_FILE_PATH) {
        let recipients: Vec<String> = serde_json::from_str(&recipients_json)
            .map_err(|e| format!("error parsing recipients json: {}", e))?;

        println!("current recipients:");
        for recipient in recipients {
            println!("{}", recipient);
        }
    } else {
        println!("no recipients set up");
    }

    Ok(())
}

pub fn save_recipients(recipients: Vec<String>) -> Result<Vec<String>, String> {
    let valid_emails: Vec<String> = recipients
        .into_iter()
        .filter(|email| is_valid_email(email))
        .collect();

    if valid_emails.is_empty() {
        return Err("error: no valid email addresses provided".to_string());
    }

    let mut existing_emails: Vec<String> = Vec::new();

    if Path::new(RECIPIENTS_FILE_PATH).exists() {
        let recipients_json = fs::read_to_string(RECIPIENTS_FILE_PATH)
            .map_err(|e| format!("error reading recipients from json: {}", e))?;

        existing_emails = serde_json::from_str(&recipients_json)
            .map_err(|e| format!("error deserializing recipients from json: {}", e))?;
    }

    for email in valid_emails {
        if !existing_emails.contains(&email) {
            existing_emails.push(email);
        }
    }

    let recipients_json = serde_json::to_string(&existing_emails)
        .map_err(|e| format!("error converting recipients to json: {}", e))?;

    fs::write(RECIPIENTS_FILE_PATH, recipients_json)
        .map_err(|e| format!("error writing recipients to json: {}", e))?;

    println!("recipients saved");
    Ok(existing_emails)
}

fn read_emails_from_csv(file_path: std::path::PathBuf) -> Result<Vec<String>, String> {
    let file = File::open(file_path).map_err(|e| format!("error opening file: {}", e))?;
    let reader = BufReader::new(file);
    let mut emails = vec![];

    for line in reader.lines() {
        let line = line.map_err(|e| format!("error reading line: {}", e))?;
        if !line.is_empty() && is_valid_email(&line) {
            emails.push(line);
        }
    }

    Ok(emails)
}

fn read_emails_from_txt(file_path: std::path::PathBuf) -> Result<Vec<String>, String> {
    let file = File::open(file_path).map_err(|e| format!("error opening file: {}", e))?;
    let reader = BufReader::new(file);
    let mut emails = vec![];

    for line in reader.lines() {
        let line = line.map_err(|e| format!("error reading line: {}", e))?;
        if !line.is_empty() && is_valid_email(&line) {
            emails.push(line);
        }
    }

    Ok(emails)
}

fn read_emails_from_json(file_path: std::path::PathBuf) -> Result<Vec<String>, String> {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("error reading file: {}", e))?;

    let data: Value =
        serde_json::from_str(&contents).map_err(|e| format!("error parsing json: {}", e))?;

    let emails: Vec<String> = data
        .as_array()
        .ok_or("error: json file should contain an array of strings.")?
        .iter()
        .filter_map(|v| {
            let email = v.as_str().unwrap();
            if !email.is_empty() && is_valid_email(email) {
                Some(email.to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(emails)
}
