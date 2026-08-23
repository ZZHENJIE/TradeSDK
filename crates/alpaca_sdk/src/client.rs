use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "APCA-API-KEY-ID",
            HeaderValue::from_str(api_key).expect("invalid api key"),
        );
        headers.insert(
            "APCA-API-SECRET-KEY",
            HeaderValue::from_str(api_secret).expect("invalid api secret"),
        );
        headers.insert("accept", HeaderValue::from_static("application/json"));
        Self {
            http_client: reqwest::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn api<Q: util::API>(&self, api_query: &Q) -> anyhow::Result<Q::T> {
        api_query.fetch(&self.http_client, None).await
    }
}
