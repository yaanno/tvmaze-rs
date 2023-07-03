use crate::other::{Image, Link, Rating};
use crate::show::Show;
use serde::{Deserialize, Serialize};

// result item depends on the api call type
// esp the type & show field is prolematic

pub type Results = Vec<Item>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub airdate: Option<String>,
    pub airstamp: Option<String>,
    pub airtime: Option<String>,
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>,
    pub id: Option<i64>,
    pub image: Option<Image>,
    #[serde(rename = "type")]
    pub item_type: Type,
    #[serde(rename = "_links")]
    pub links: Links,
    pub name: Option<String>,
    pub number: Option<i64>,
    pub rating: Rating,
    pub runtime: Option<i64>,
    pub season: Option<i64>,
    pub show: Option<Show>,
    pub summary: Option<String>,
    pub url: Option<String>, // into_show
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedded {
    pub show: Show,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub nextepisode: Option<Link>,
    pub previousepisode: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShowType {
    Animation,
    #[serde(rename = "Award Show")]
    AwardShow,
    Documentary,
    #[serde(rename = "Game Show")]
    GameShow,
    News,
    #[serde(rename = "Panel Show")]
    PanelShow,
    Reality,
    Scripted,
    Sports,
    #[serde(rename = "Talk Show")]
    TalkShow,
    Variety,
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
    Ended,
    #[serde(rename = "insignificant_special")]
    InSignificantSpecial,
    Regular,
    #[serde(rename = "significant_special")]
    SignificantSpecial,
    #[serde(rename = "To Be Determined")]
    ToBeDetermined,
}
