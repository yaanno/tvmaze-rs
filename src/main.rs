use reqwest_middleware::Error;
use tvmaze_rs::client;
use tvmaze_rs::results;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = client::RequestClient::new();

    let results: results::Results = client
        .get("https://github.com/yaanno/tvmaze-schedule-flat-data/raw/master/tvmaze-schedule.json")
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", results.len());
    Ok(())
}
