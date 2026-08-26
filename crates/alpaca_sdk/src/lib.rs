pub mod client;
pub mod exchange_codes;
pub mod quote;
pub mod quotes;
pub mod snapshot;

use chrono::{DateTime, Utc};
pub use {
    client::Client, exchange_codes::Query as ExchangeCodesQuery, quote::Query as QuoteQuery,
    quotes::Query as QuotesQuery, snapshot::Query as SnapshotQuery,
};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Feed {
    /// all US exchanges
    Sip,
    /// Investors EXchange
    Iex,
    /// SIP with a 15 minute delay
    DelayedSip,
    /// Blue Ocean, overnight US trading data
    Boats,
    /// derived overnight US trading data
    Overnight,
    /// over-the-counter exchanges
    Otc,
}

impl From<Feed> for &'static str {
    fn from(value: Feed) -> Self {
        match value {
            Feed::Sip => "sip",
            Feed::Iex => "iex",
            Feed::DelayedSip => "delayed_sip",
            Feed::Boats => "boats",
            Feed::Overnight => "overnight",
            Feed::Otc => "otc",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StockTape {
    /// New York Stock Exchange
    A,
    /// NYSE Arca, Bats, IEX and other regional exchanges
    B,
    /// NASDAQ
    C,
    /// Overnight
    N,
    /// OTC
    O,
}

/// Quote data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockQuote {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Bid exchange
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// Bid price
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size
    #[serde(rename = "bs")]
    pub bid_size: u32,
    /// Ask price
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size
    #[serde(rename = "as")]
    pub ask_size: u32,
    /// Ask exchange
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    /// Condition codes
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    /// Quote tape
    #[serde(rename = "z")]
    pub tape: StockTape,
}
