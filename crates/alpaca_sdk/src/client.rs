use reqwest::{Method, RequestBuilder};

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    api_secret: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.http_client
            .request(method, url)
            .header("APCA-API-KEY-ID", self.api_key.as_str())
            .header("APCA-API-SECRET-KEY", self.api_secret.as_str())
            .header("accept", "application/json")
    }

    pub async fn snapshot(
        &self,
        query: &crate::SnapshotQuery,
    ) -> anyhow::Result<crate::snapshot::Response> {
        crate::snapshot::fetch(self.request(Method::GET, query.url().as_str())).await
    }
}
