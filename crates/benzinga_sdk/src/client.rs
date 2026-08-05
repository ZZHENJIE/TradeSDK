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
        query: &crate::calendar::IPOQuery,
    ) -> anyhow::Result<Vec<crate::calendar::ipo::Item>> {
        crate::calendar::ipo::fetch(query, &self.http_client).await
    }

    pub async fn economics(
        &self,
        query: &crate::calendar::EconomicsQuery,
    ) -> anyhow::Result<Vec<crate::calendar::economics::Item>> {
        crate::calendar::economics::fetch(query, &self.http_client).await
    }

    pub async fn earnings(
        &self,
        query: &crate::calendar::EarningsQuery,
    ) -> anyhow::Result<Vec<crate::calendar::earnings::Item>> {
        crate::calendar::earnings::fetch(query, &self.http_client).await
    }
}
