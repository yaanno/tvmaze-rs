use reqwest_middleware::Error;
use tvmaze_rs::api::{Api, ApiURL};
use tvmaze_rs::client;
use tvmaze_rs::results;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: &str = ApiURL::get(Api::Schedule).unwrap();
    let api_client: client::ApiClient = client::ApiClient::new().unwrap();
    let results: results::Results = api_client.client.get(url).send().await?.json().await?;
    println!("{:#?}", results.first());
    Ok(())
}
