pub enum Api {
    Schedule,
    ScheduleWeb,
    ScheduleFull,
}

pub const API_SCHEDULE: &str = "https://api.tvmaze.com/schedule";
pub const API_SCHEDULE_WEB: &str = "https://api.tvmaze.com/schedule/web";
pub const API_SCHEDULE_FULL: &str = "https://api.tvmaze.com/schedule/full";

pub struct ApiURL {}

impl ApiURL {
    pub fn get(api: Api) -> Result<&'static str, ()> {
        match api {
            Api::Schedule => Ok(API_SCHEDULE),
            Api::ScheduleFull => Ok(API_SCHEDULE_FULL),
            Api::ScheduleWeb => Ok(API_SCHEDULE_WEB),
        }
    }
}
