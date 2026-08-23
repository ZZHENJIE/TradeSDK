use chrono::{DateTime, Utc};

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
            "https://data.alpaca.markets/v2/stocks/{}/snapshot?feed={}&currency={}",
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

/// Bar data (OHLC)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockBar {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume
    #[serde(rename = "v")]
    pub volume: i64,
    /// Trade count
    #[serde(rename = "n")]
    pub trade_count: i64,
    /// Volume-weighted average price
    #[serde(rename = "vw")]
    pub vwap: f64,
}

/// Trade data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockTrade {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Trade ID
    #[serde(rename = "i")]
    pub id: u64,
    /// Exchange code
    #[serde(rename = "x")]
    pub exchange: String,
    /// Trade price
    #[serde(rename = "p")]
    pub price: f64,
    /// Volume
    #[serde(rename = "s")]
    pub size: u32,
    /// Condition codes
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    /// Trade tape
    #[serde(rename = "z")]
    pub tape: crate::StockTape,
    /// Update status (optional)
    #[serde(rename = "u")]
    pub update: Option<String>,
}

/// Stock snapshot data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockSnapshot {
    /// Latest trade data
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<StockTrade>,
    /// Latest quote data
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<crate::StockQuote>,
    /// Latest minute bar
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<StockBar>,
    /// Current day daily bar
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<StockBar>,
    /// Previous daily bar
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<StockBar>,
}

/// Stock snapshot response (single symbol)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
    /// Symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Currency
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub snapshot: StockSnapshot,
}
