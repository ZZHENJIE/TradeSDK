use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub page_size: i64,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

impl Query {
    pub fn url(&self) -> String {
        format!(
            "https://www.benzinga.com/api-next/calendar/economics?pageSize={}&dateFrom={}&dateTo={}",
            self.page_size, self.date_from, self.date_to
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(default)]
    pub actual: String,
    #[serde(rename = "actual_t", default)]
    pub actual_t: String,
    #[serde(default)]
    pub consensus: String,
    #[serde(rename = "consensus_t", default)]
    pub consensus_t: String,
    pub country: String,
    pub date: String,
    pub description: String,
    #[serde(rename = "event_category")]
    pub event_category: String,
    #[serde(rename = "event_name")]
    pub event_name: String,
    #[serde(rename = "event_period")]
    pub event_period: String,
    pub id: String,
    pub importance: u8,
    #[serde(default)]
    pub notes: String,
    #[serde(rename = "period_year")]
    pub period_year: u16,
    #[serde(default)]
    pub prior: String,
    #[serde(rename = "prior_t", default)]
    pub prior_t: String,
    pub time: String,
    pub updated: u64,
}

pub async fn fetch(query: &Query, http_client: &reqwest::Client) -> anyhow::Result<Vec<Item>> {
    let url = query.url();
    let response = http_client.get(&url).send().await?;
    let result = response.json::<Vec<Item>>().await?;
    Ok(result)
}
