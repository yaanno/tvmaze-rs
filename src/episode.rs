// https://api.tvmaze.com/episodes/9

use crate::{
    other::{Image, Rating, Type},
    show::Show,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
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
    pub summary: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedded {
    pub show: Show,
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
