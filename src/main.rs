use chrono::Datelike;
use commands::recipients::read_emails_from_file;
use commands::recipients::save_recipients;
use config::ConfigCmd;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

mod commands;
mod config;
mod models;
mod utils;

use commands::config::display_config;
use commands::config::prompt_and_save_config;
use commands::recipients::display_recipients;
use config::Cli;
use models::api::SeasonByYearResponse;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Config(cmd) => match (cmd.set, cmd.display) {
            (true, _) => {
                if let Err(e) = prompt_and_save_config() {
                    eprintln!("error saving config: {}", e);
                }
            }
            (_, true) => {
                if let Err(e) = display_config() {
                    eprintln!("error displaying config: {}", e);
                }
            }
            _ => {
                eprintln!("provide either --set or --display flag");
            }
        },
        Cli::Recipients(cmd) => match (cmd.display, cmd.file.as_ref(), cmd.add) {
            (true, None, None) => {
                if let Err(e) = display_recipients() {
                    eprintln!("{}", e);
                }
            }
            (false, Some(file), None) => {
                if let Err(e) = read_emails_from_file(PathBuf::from(file)).and_then(save_recipients)
                {
                    eprintln!("{}", e);
                }
            }

            (false, None, Some(recipients)) => {
                if let Err(e) = save_recipients(recipients) {
                    eprintln!("{}", e);
                }
            }
            _ => {
                eprintln!("provide -d, -f or -a flags")
            }
        },
    }

    Ok(())
}
