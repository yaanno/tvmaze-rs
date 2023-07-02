use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, Result};

#[path = "lib.rs"]
mod lib;
use lib::Results;

#[tokio::main]
async fn main() -> Result<()>{
    let client = ClientBuilder::new(Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();

    let shows: Results = client
    .get("https://api.tvmaze.com/schedule")
    .send()
    .await?
    .json()
    .await?;

    println!("{:#?}", shows.last());
    Ok(())
}
