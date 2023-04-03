use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonByYearResponse {
    pub MRData: MRData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MRData {
    pub RaceTable: RaceTable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceTable {
    pub Races: Vec<Race>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    pub raceName: String,
    pub Circuit: Circuit,
    pub round: String,
    pub date: String,
    pub time: Option<String>,
    pub FirstPractice: SessionSchedule,
    pub SecondPractice: SessionSchedule,
    pub ThirdPractice: Option<SessionSchedule>,
    pub Qualifying: SessionSchedule,
    pub Sprint: Option<SessionSchedule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circuit {
    pub circuitName: String,
    pub Location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub locality: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionSchedule {
    pub date: String,
    pub time: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Schedule {
    pub races: Vec<RaceSchedule>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceSchedule {
    pub race: SessionSchedule,
    pub circuitName: String,
    pub raceName: String,
    pub round: String,
}
