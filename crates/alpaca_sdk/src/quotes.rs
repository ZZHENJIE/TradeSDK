use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub symbol: Vec<String>,
    pub feed: crate::Feed,
    pub currency: String,
}

impl util::API for Query {
    type T = Response;
    type P = ();

    fn url(&self, _: Option<Self::P>) -> String {
        let feed: &str = self.feed.into();
        format!(
            "https://data.alpaca.markets/v2/stocks/quotes/latest?symbols={}&feed={}&currency={}",
            self.symbol.join(","),
            feed,
            self.currency
        )
    }

    async fn fetch(
        &self,
        http_client: &reqwest::Client,
        _: Option<Self::P>,
    ) -> anyhow::Result<Self::T> {
        util::fetch_json(self.url(None), http_client).await
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub quotes: HashMap<String, crate::StockQuote>,
}
