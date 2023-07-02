use serde::{Serialize, Deserialize};

pub type Results = Vec<Item>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub season: Option<i64>,
    pub number: Option<i64>,
    #[serde(rename = "type")]
    pub item_type: Type,
    pub airdate: String,
    pub airtime: String,
    pub airstamp: String,
    pub runtime: Option<i64>,
    pub rating: Rating,
    pub image: Option<Image>,
    pub summary: Option<String>,
    #[serde(rename = "_links")]
    pub links: ItemLinks,
    pub show: Option<Show>,
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>, // into_show
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedded {
    pub show: Show,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub id: i64,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub show_type: ShowType,
    pub language: String,
    pub genres: Vec<String>,
    pub status: Status,
    pub runtime: Option<i64>,
    pub average_runtime: Option<i64>,
    pub premiered: String,
    pub ended: Option<serde_json::Value>,
    pub official_site: Option<String>,
    pub schedule: Schedule,
    pub rating: Rating,
    pub weight: i64,
    pub network: Option<Network>,
    pub web_channel: Option<Network>,
    pub dvd_country: Option<serde_json::Value>,
    pub externals: Externals,
    pub image: Option<Image>,
    pub summary: Option<String>,
    pub updated: i64,
    #[serde(rename = "_links")]
    pub links: ShowLinks,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Externals {
    pub tvrage: Option<i64>,
    pub thetvdb: Option<i64>,
    pub imdb: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub medium: String,
    pub original: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub previousepisode: Option<Link>,
    pub nextepisode: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub id: i64,
    pub name: String,
    pub country: Option<Country>,
    pub official_site: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub code: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub average: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub time: String,
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
pub enum ShowType {
    Animation,
    #[serde(rename = "Game Show")]
    GameShow,
    Reality,
    Scripted,
    Sports,
    #[serde(rename = "Talk Show")]
    TalkShow,
    Variety,
    Documentary,
    News,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "In Development")]
    InDevelopment,
    Running,
    #[serde(rename = "To Be Determined")]
    ToBeDetermined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub show: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Regular,
    Ended,
    #[serde(rename = "To Be Determined")]
    ToBeDetermined,
    #[serde(rename = "significant_special")]
    SignificantSpecial,
}
