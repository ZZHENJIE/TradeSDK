#[derive(Debug, Clone)]
pub struct Client {
    pub http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::builder()
                .user_agent("benzinga_sdk/1.0.0")
                .build()
                .expect("Build reqwest Client failed."),
        }
    }

    pub async fn api<Q: util::API>(&self, api_query: &Q) -> anyhow::Result<Q::T> {
        api_query.fetch(&self.http_client, None).await
    }
}
