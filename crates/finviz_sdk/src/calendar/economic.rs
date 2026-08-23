use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

impl Query {
    /// Returns the raw, unconverted response data
    pub async fn fetch_raw(
        &self,
        http_client: &reqwest::Client,
        auth: Option<&str>,
    ) -> anyhow::Result<Vec<ResourceItem>> {
        crate::fetch_csv(&util::API::url(self, auth.map(String::from)), http_client).await
    }
}

impl util::API for Query {
    type T = Vec<Item>;
    type P = String;

    fn url(&self, params: Option<Self::P>) -> String {
        format!(
            "https://elite.finviz.com/export/calendar/economic?dateFrom={}&dateTo={}&auth={}",
            self.date_from,
            self.date_to,
            params.unwrap_or_default()
        )
    }

    async fn fetch(
        &self,
        http_client: &reqwest::Client,
        params: Option<Self::P>,
    ) -> anyhow::Result<Self::T> {
        Ok(self
            .fetch_raw(http_client, params.as_deref())
            .await?
            .into_iter()
            .map(Item::from)
            .collect())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceItem {
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Impact")]
    pub impact: u8,
    #[serde(rename = "For")]
    #[serde(default)]
    pub for_field: String,
    #[serde(rename = "Actual")]
    #[serde(default)]
    pub actual: String,
    #[serde(rename = "Expected")]
    #[serde(default)]
    pub expected: String,
    #[serde(rename = "Prior")]
    #[serde(default)]
    pub prior: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Impact")]
    pub impact: u8,
    #[serde(rename = "For")]
    #[serde(default)]
    pub for_field: String,
    #[serde(rename = "Actual")]
    #[serde(default)]
    pub actual: String,
    #[serde(rename = "Expected")]
    #[serde(default)]
    pub expected: String,
    #[serde(rename = "Prior")]
    #[serde(default)]
    pub prior: String,
}

impl From<ResourceItem> for Item {
    fn from(item: ResourceItem) -> Self {
        let timestamp =
            util::parse_eastern_time_to_timestamp(&item.date, &item.time, None, Some("%H:%M"))
                .unwrap_or(0);

        Self {
            timestamp,
            event: item.event,
            impact: item.impact,
            for_field: item.for_field,
            actual: item.actual,
            expected: item.expected,
            prior: item.prior,
        }
    }
}
