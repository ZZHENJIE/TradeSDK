#[derive(Debug, Clone)]
pub struct Client {
    pub http_client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Client {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn ipo(
        &self,
        query: &crate::IPOQuery,
    ) -> anyhow::Result<Vec<crate::calendar::ipo::Item>> {
        crate::calendar::ipo::fetch(query, &self.http_client).await
    }
}
