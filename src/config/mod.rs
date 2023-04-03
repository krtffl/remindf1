use serde::{Deserialize, Serialize};
use structopt::StructOpt;

pub const CONFIG_FILE_PATH: &str = "config.json";
pub const RECIPIENTS_FILE_PATH: &str = "recipients.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub remind_free_practice: bool,
    pub remind_quali: bool,
    pub remind_sprint: bool,
    pub remind_race: bool,
    pub days_before_reminder: u32,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "remindf1",
    about = "a rust cli tool that reminds you of f1 schedules on race week"
)]
pub enum Cli {
    #[structopt(name = "config", about = "manage global reminder config")]
    Config(ConfigCmd),
    #[structopt(name = "recipients", about = "manage reminder recipients")]
    Recipients(RecipientsCmd),
    #[structopt(name = "schedule", about = "update and print schedule informaton")]
    Schedule(ScheduleCmd),
}

#[derive(StructOpt, Debug)]
pub struct ConfigCmd {
    #[structopt(short = "s", long = "set", help = "set up remindf1 config")]
    pub set: bool,
    #[structopt(
        short = "d",
        long = "display",
        help = "display current remindf1 config"
    )]
    pub display: bool,
}

#[derive(StructOpt, Debug)]
pub struct RecipientsCmd {
    #[structopt(short = "a", long = "add", help = "add reminder recipients")]
    pub add: Option<Vec<String>>,
    #[structopt(
        short = "f",
        long = "file",
        help = "add reminder recipient from a file"
    )]
    pub file: Option<String>,
    #[structopt(
        short = "d",
        long = "display",
        help = "display current remindf1 config"
    )]
    pub display: bool,
}

#[derive(StructOpt, Debug)]
pub struct ScheduleCmd {
    #[structopt(short = "n", long = "next", help = "show next race information")]
    pub next: bool,
    #[structopt(
        short = "u",
        long = "update",
        help = "manually check for schedule updates"
    )]
    pub update: bool,
}
