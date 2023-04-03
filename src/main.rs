use std::path::PathBuf;
use structopt::StructOpt;

mod commands;
mod config;
mod models;
mod utils;

use commands::config::display_config;
use commands::config::prompt_and_save_config;
use commands::recipients::display_recipients;
use commands::recipients::read_emails_from_file;
use commands::recipients::save_recipients;
use commands::schedule::fetch_and_display_next_race;
use commands::schedule::fetch_and_save_schedule;
use config::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    match args {
        Cli::Config(cmd) => match (cmd.set, cmd.display) {
            (true, false) => {
                if let Err(e) = prompt_and_save_config() {
                    eprintln!("error saving config: {}", e);
                }
            }
            (false, true) => {
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
        Cli::Schedule(cmd) => match (cmd.update, cmd.next) {
            (true, false) => {
                if let Err(e) = fetch_and_save_schedule("2023").await {
                    eprintln!("error updating schedule: {}", e)
                }
            }
            (false, true) => {
                if let Err(e) = fetch_and_display_next_race().await {
                    eprintln!("{}", e);
                }
            }
            _ => {
                eprintln!("-u to manually update, -n to see next race");
            }
        },
    }

    Ok(())
}
