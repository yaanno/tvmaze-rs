use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub struct ApiClient {
    pub client: ClientWithMiddleware,
}

impl ApiClient {
    pub fn new() -> Result<ApiClient, ()> {
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();
        Ok(ApiClient { client })
    }
}
