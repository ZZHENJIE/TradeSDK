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

    pub async fn screener(
        &self,
        query: &crate::ScreenerQuery,
    ) -> anyhow::Result<Vec<crate::screener::Item>> {
        crate::screener::fetch(query, &self.api_key, &self.http_client).await
    }

    pub async fn quote(
        &self,
        query: &crate::QuoteQuery,
    ) -> anyhow::Result<Vec<crate::quote::Item>> {
        crate::quote::fetch(query, &self.api_key, &self.http_client).await
    }

    pub async fn news(&self, query: &crate::NewsQuery) -> anyhow::Result<Vec<crate::news::Item>> {
        crate::news::fetch(query, &self.api_key, &self.http_client).await
    }
}
