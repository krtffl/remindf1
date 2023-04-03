use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"(?x)^[\w-]+(\.[\w-]+)*@([\w-]+\.)+[a-zA-Z]{2,7}$").unwrap();
    email_regex.is_match(email)
}

pub fn schedule_file_name(year: &str) -> String {
    format!("schedule-{}.json", year)
}

pub fn schedule_api_url(year: &str) -> String {
    format!("https://ergast.com/api/f1/{}/races.json", year)
}
