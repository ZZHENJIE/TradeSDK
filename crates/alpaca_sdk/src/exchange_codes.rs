use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {}

impl util::API for Query {
    type T = HashMap<String, String>;
    type P = ();

    fn url(&self, _: Option<Self::P>) -> String {
        "https://data.alpaca.markets/v2/stocks/meta/exchanges".to_string()
    }

    async fn fetch(
        &self,
        http_client: &reqwest::Client,
        _: Option<Self::P>,
    ) -> anyhow::Result<Self::T> {
        util::fetch_json(self.url(None), http_client).await
    }
}
