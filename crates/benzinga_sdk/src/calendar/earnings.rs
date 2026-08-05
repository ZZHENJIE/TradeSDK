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
            "https://www.benzinga.com/api-next/calendar/earnings?pageSize={}&dateFrom={}&dateTo={}",
            self.page_size, self.date_from, self.date_to
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub currency: String,
    pub cusip: String,
    pub date: String,
    pub date_confirmed: u8,
    pub eps: String,
    pub eps_est: String,
    pub eps_prior: String,
    pub eps_surprise: String,
    pub eps_surprise_percent: String,
    pub eps_type: String,
    pub exchange: String,
    pub id: String,
    pub importance: u8,
    pub isin: String,
    pub name: String,
    pub notes: String,
    pub period: String,
    pub period_year: u16,
    pub revenue: String,
    pub revenue_est: String,
    pub revenue_prior: String,
    pub revenue_surprise: String,
    pub revenue_surprise_percent: String,
    pub revenue_type: String,
    pub ticker: String,
    pub time: String,
    pub updated: u64,
}

pub async fn fetch(query: &Query, http_client: &reqwest::Client) -> anyhow::Result<Vec<Item>> {
    let url = query.url();
    let response = http_client.get(&url).send().await?;
    let result = response.json::<Vec<Item>>().await?;
    Ok(result)
}
