#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub symbol: String,
    pub feed: crate::Feed,
    pub currency: String,
}

impl util::API for Query {
    type T = Response;
    type P = ();

    fn url(&self, _: Option<Self::P>) -> String {
        let feed: &str = self.feed.into();
        format!(
            "https://data.alpaca.markets/v2/stocks/{}/quotes/latest?feed={}&currency={}",
            self.symbol, feed, self.currency
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
    pub symbol: String,
    pub quote: crate::StockQuote,
}
