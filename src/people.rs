use serde::{Deserialize, Serialize};

use crate::other::{Country, Image, Link};

pub type People = Vec<Person>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub country: Option<Country>,
    pub birthday: Option<String>,
    pub deathday: Option<String>,
    pub gender: Option<Gender>,
    pub image: Option<Image>,
    pub updated: i64,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub links_self: Link,
}
