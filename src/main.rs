use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: Option<std::path::PathBuf>,
    #[structopt(short = "e", long = "email")]
    emails: Vec<String>,
}

fn main() {
    let args = Cli::from_args();

    let recipients = if let Some(file_path) = args.file {
        read_emails_from_file(file_path)
    } else if !args.emails.is_empty() {
        let valid_emails: Vec<String> = args
            .emails
            .into_iter()
            .filter(|email| is_valid_email(email))
            .collect();
        if !valid_emails.is_empty() {
            Ok(valid_emails)
        } else {
            Err("error: no valid email addresses provided.".into())
        }
    } else {
        Err("error: provide a list of emails or a file ".into())
    };

    match recipients {
        Ok(recipients) => {
            println!("{:?}", recipients)
        }
        Err(error_msg) => {
            eprintln!("{}", error_msg);
        }
    }
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"(?x)^[\w-]+(\.[\w-]+)*@([\w-]+\.)+[a-zA-Z]{2,7}$").unwrap();
    email_regex.is_match(email)
}

fn read_emails_from_file(file_path: std::path::PathBuf) -> Result<Vec<String>, String> {
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
    use serde_json::Value;
    use std::io::Read;

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
