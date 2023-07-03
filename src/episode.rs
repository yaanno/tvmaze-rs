// https://api.tvmaze.com/episodes/9

use crate::other::{Image, Rating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub airdate: String,
    pub airstamp: String,
    pub airtime: String,
    pub id: Option<i64>,
    pub image: Image,
    #[serde(rename = "_links")]
    pub links: Links,
    pub name: String,
    pub number: Option<i64>,
    pub rating: Rating,
    pub runtime: Option<i64>,
    pub season: Option<i64>,
    pub summary: String,
    pub url: String,
    #[serde(rename = "type")]
    pub welcome_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub show: Link,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub href: String,
}
