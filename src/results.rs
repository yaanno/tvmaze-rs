use crate::other::{Image, Link, Rating};
use crate::show::Show;
use serde::{Deserialize, Serialize};

// result item depends on the api call type
// esp the type & show field is prolematic

pub type Results = Vec<Item>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<i64>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub season: Option<i64>,
    pub number: Option<i64>,
    #[serde(rename = "type")]
    pub item_type: Type,
    pub airdate: Option<String>,
    pub airtime: Option<String>,
    pub airstamp: Option<String>,
    pub runtime: Option<i64>,
    pub rating: Rating,
    pub image: Option<Image>,
    pub summary: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub show: Option<Show>,
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>, // into_show
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedded {
    pub show: Show,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub previousepisode: Option<Link>,
    pub nextepisode: Option<Link>,
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
    #[serde(rename = "Panel Show")]
    PanelShow,
    #[serde(rename = "Award Show")]
    AwardShow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
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
    #[serde(rename = "insignificant_special")]
    InSignificantSpecial,
}
