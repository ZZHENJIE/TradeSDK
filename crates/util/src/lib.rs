use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use chrono_tz::America::New_York;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

fn to_timestamp(naive: NaiveDateTime) -> anyhow::Result<i64> {
    match New_York.from_local_datetime(&naive) {
        chrono::offset::LocalResult::Single(dt) => Ok(dt.timestamp()),
        chrono::offset::LocalResult::Ambiguous(..) => Err(anyhow::anyhow!(
            "Ambiguous local datetime (due to DST fallback)"
        )),
        chrono::offset::LocalResult::None => Err(anyhow::anyhow!(
            "Invalid local datetime (nonexistent due to DST spring forward)"
        )),
    }
}

pub fn parse_eastern_time_to_timestamp(
    date_str: &str,
    time_str: &str,
    date_fmt: Option<&str>,
    time_fmt: Option<&str>,
) -> anyhow::Result<i64> {
    let date = NaiveDate::parse_from_str(date_str, date_fmt.unwrap_or("%Y-%m-%d"))
        .map_err(|e| anyhow::anyhow!("Failed to parse date '{}': {}", date_str, e))?;
    let time = NaiveTime::parse_from_str(time_str, time_fmt.unwrap_or("%H:%M:%S"))
        .map_err(|e| anyhow::anyhow!("Failed to parse time '{}': {}", time_str, e))?;

    to_timestamp(NaiveDateTime::new(date, time))
}

pub fn parse_eastern_date_time_to_timestamp(
    date_time_str: &str,
    date_time_fmt: Option<&str>,
) -> anyhow::Result<i64> {
    let naive_dt =
        NaiveDateTime::parse_from_str(date_time_str, date_time_fmt.unwrap_or("%Y-%m-%d %H:%M"))
            .map_err(|e| anyhow::anyhow!("Failed to parse date '{}': {}", date_time_str, e))?;

    to_timestamp(naive_dt)
}

pub trait API: Debug + Serialize + DeserializeOwned {
    /// Response type
    type T: Debug + Serialize + DeserializeOwned;
    /// Request parameters
    type P;
    fn url(&self, params: Option<Self::P>) -> String;
    fn fetch(
        &self,
        http_client: &reqwest::Client,
        params: Option<Self::P>,
    ) -> impl Future<Output = anyhow::Result<Self::T>> + Send;
}

/// Sends a GET request and parses the JSON response.
/// On non-success HTTP status or JSON decode failure, the error message
/// includes the response body to aid troubleshooting.
pub async fn fetch_json<T: DeserializeOwned>(
    url: String,
    http_client: &reqwest::Client,
) -> anyhow::Result<T> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let body = response.text().await?;
    serde_json::from_str(&body)
        .map_err(|e| anyhow::anyhow!("Failed to decode JSON response: {e}; body: {body}"))
}
