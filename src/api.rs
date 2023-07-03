use std::{error, fmt};

pub enum Api {
    Schedule,
    ScheduleWeb,
    ScheduleFull,
}

pub const API_SCHEDULE: &str = "https://api.tvmaze.com/schedule";
pub const API_SCHEDULE_WEB: &str = "https://api.tvmaze.com/schedule/web";
pub const API_SCHEDULE_FULL: &str = "https://api.tvmaze.com/schedule/full";

#[derive(Debug, Clone)]
pub struct ApiURL {}

impl error::Error for ApiURL {}

impl fmt::Display for ApiURL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl ApiURL {
    pub fn get(api: Api) -> Result<&'static str, ()> {
        match api {
            Api::Schedule => Ok(API_SCHEDULE),
            Api::ScheduleFull => Ok(API_SCHEDULE_FULL),
            Api::ScheduleWeb => Ok(API_SCHEDULE_WEB),
        }
    }
}
