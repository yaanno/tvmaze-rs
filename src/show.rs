// https://api.tvmaze.com/shows/1?embed=episodes

use crate::episode::Episode;
use crate::other::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub average_runtime: Option<i64>,
    pub dvd_country: Option<serde_json::Value>,
    #[serde(rename = "_embedded")]
    pub embedded: Option<Embedded>,
    pub ended: Option<String>,
    pub externals: Externals,
    pub genres: Vec<String>,
    pub id: Option<i64>,
    pub image: Option<Image>,
    pub language: Option<String>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub name: String,
    pub network: Option<Network>,
    pub official_site: Option<String>,
    pub premiered: Option<String>,
    pub rating: Rating,
    pub runtime: Option<i64>,
    pub schedule: Schedule,
    #[serde(rename = "type")]
    pub show_type: String,
    pub status: Option<String>,
    pub summary: Option<String>,
    pub updated: Option<i64>,
    pub url: String,
    pub web_channel: Option<serde_json::Value>,
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedded {
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Regular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Link,
    pub previousepisode: Option<Link>,
}
