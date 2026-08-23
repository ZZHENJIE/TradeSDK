pub mod client;
pub mod news;
pub mod screener;
pub mod stock;
pub mod calendar {
    pub mod earnings;
    pub mod economic;
}

pub use {
    calendar::earnings::Query as CalendarEarningsQuery,
    calendar::economic::Query as CalendarEconomicsQuery, client::Client, news::Query as NewsQuery,
    screener::Query as ScreenerQuery, stock::Query as StockQuery,
};

pub(crate) async fn fetch_csv<T: serde::de::DeserializeOwned>(
    url: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<Vec<T>> {
    let response = http_client.get(url).send().await?.error_for_status()?;
    let text = response.text().await?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(text.as_bytes());
    Ok(reader
        .deserialize()
        .collect::<Result<Vec<_>, csv::Error>>()?)
}
