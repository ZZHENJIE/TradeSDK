/*
 * Doc: https://docs.alpaca.markets/us/docs/historical-stock-data-1
 */
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Feed {
    Sip,
    Iex,
    DelayedSip,
    Boats,
    Overnight,
    Otc,
}

impl Into<&str> for Feed {
    fn into(self) -> &'static str {
        match self {
            Feed::Sip => "sip",
            Feed::Iex => "iex",
            Feed::DelayedSip => "delayed_sip",
            Feed::Boats => "boats",
            Feed::Overnight => "overnight",
            Feed::Otc => "otc",
        }
    }
}

pub mod client;
pub mod snapshot;

pub use {client::Client, snapshot::Query as SnapshotQuery};
