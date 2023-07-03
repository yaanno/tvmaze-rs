use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub average: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub medium: String,
    pub original: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Externals {
    pub imdb: Option<String>,
    pub thetvdb: Option<i64>,
    pub tvrage: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub country: Country,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub official_site: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub time: Option<String>,
    pub days: Vec<Day>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Day {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Ended,
    #[serde(rename = "In Development")]
    InDevelopment,
    Running,
    #[serde(rename = "To Be Determined")]
    ToBeDetermined,
}
