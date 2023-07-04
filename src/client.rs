use std::fmt::Debug;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{Client, IntoUrl};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};

pub struct CachedApiClient {
    pub client: ClientWithMiddleware,
}

impl Debug for CachedApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <ClientWithMiddleware as Debug>::fmt(&self.client, f)
    }
}

impl CachedApiClient {
    /// Creates a new [`CachedApiClient`].
    pub fn new() -> Self {
        Self {
            client: ClientBuilder::new(Client::new())
                .with(Cache(HttpCache {
                    mode: CacheMode::Default,
                    manager: CACacheManager::default(),
                    options: None,
                }))
                .build(),
        }
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder {
        self.client.get(url)
    }
}

impl Default for CachedApiClient {
    /// Creates a default [`CachedApiClient`].
    fn default() -> Self {
        Self::new()
    }
}
