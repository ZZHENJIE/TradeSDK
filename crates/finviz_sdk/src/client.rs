#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn api<Q: util::API>(&self, api_query: &Q) -> anyhow::Result<Q::T>
    where
        Q: util::API<P = String>,
    {
        api_query
            .fetch(&self.http_client, Some(self.api_key.clone()))
            .await
    }
}
