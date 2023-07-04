use reqwest_middleware::Error;
use tvmaze_rs::api::{Api, ApiURL};
use tvmaze_rs::client::CachedApiClient;
use tvmaze_rs::results;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: &str = ApiURL::new(Api::ScheduleWeb).get().unwrap();
    let tv_url: &str = ApiURL::new(Api::Schedule).get().unwrap();
    let client = CachedApiClient::default();
    let web_cached: results::WebResults = client.get(url).send().await?.json().await?;
    let tv_cached: results::TVResults = client.get(tv_url).send().await?.json().await?;
    println!("{:#?}", web_cached.len());
    println!("{:#?}", tv_cached.len());
    Ok(())
}
