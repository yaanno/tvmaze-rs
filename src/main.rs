use reqwest_middleware::Error;
use tvmaze_rs::api::{Api, ApiURL};
use tvmaze_rs::client;
use tvmaze_rs::results;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = ApiURL::get(Api::Schedule).unwrap();
    let client = client::ApiClient::new();
    let results: results::Results = client.get(url).send().await?.json().await?;
    println!("{:#?}", results.first());
    Ok(())
}
