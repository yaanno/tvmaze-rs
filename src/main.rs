use reqwest_middleware::Error;
use tvmaze_rs::api::ApiURL;
use tvmaze_rs::client::CachedApiClient;
use tvmaze_rs::results;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url: &str = ApiURL::default().get().unwrap();
    let cached: results::WebResults = CachedApiClient::default()
        .get(url)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", cached.first());
    Ok(())
}
