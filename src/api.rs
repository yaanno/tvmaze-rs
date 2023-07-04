#[derive(Debug, Clone)]
pub enum Api {
    Schedule,
    ScheduleWeb,
    ScheduleFull,
}

impl Default for Api {
    fn default() -> Self {
        Api::Schedule
    }
}

pub const API_SCHEDULE: &str = "https://api.tvmaze.com/schedule";
pub const API_SCHEDULE_WEB: &str = "https://api.tvmaze.com/schedule/web";
pub const API_SCHEDULE_FULL: &str = "https://api.tvmaze.com/schedule/full";

#[derive(Debug, Clone)]
pub struct ApiURL {
    pub api: Api,
}

impl ApiURL {
    /// Creates a new [`ApiURL`].
    pub fn new(api: Api) -> Self {
        Self { api }
    }

    pub fn get(&self) -> Result<&'static str, ()> {
        match &self.api {
            Api::Schedule => Ok(API_SCHEDULE),
            Api::ScheduleFull => Ok(API_SCHEDULE_FULL),
            Api::ScheduleWeb => Ok(API_SCHEDULE_WEB),
        }
    }
}

impl Default for ApiURL {
    fn default() -> Self {
        let api = Api::default();
        Self::new(api)
    }
}
