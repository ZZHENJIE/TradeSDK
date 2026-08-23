use chrono::NaiveDate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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
            "https://elite.finviz.com/export/calendar/earnings?dateFrom={}&dateTo={}&auth={}",
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceItem {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Ticker")]
    pub ticker: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Market Cap")]
    pub market_cap: f64,
    #[serde(rename = "EPS Estimate")]
    pub eps_estimate: Option<f64>,
    #[serde(rename = "EPS Actual")]
    pub eps_actual: Option<f64>,
    #[serde(rename = "EPS Surprise")]
    pub eps_surprise: Option<f64>,
    #[serde(rename = "EPS GAAP Estimate")]
    pub eps_gaap_estimate: Option<f64>,
    #[serde(rename = "EPS GAAP Actual")]
    pub eps_gaap_actual: Option<f64>,
    #[serde(rename = "EPS GAAP Surprise")]
    pub eps_gaap_surprise: Option<f64>,
    #[serde(rename = "Revenue Estimate")]
    pub revenue_estimate: Option<f64>,
    #[serde(rename = "Revenue Actual")]
    pub revenue_actual: Option<f64>,
    #[serde(rename = "Revenue Surprise")]
    pub revenue_surprise: Option<f64>,
    #[serde(rename = "1-Day Price Reaction")]
    pub day_price_reaction: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "Timestamp")]
    pub timestamp: i64,
    #[serde(rename = "Ticker")]
    pub ticker: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Market Cap")]
    pub market_cap: f64,
    #[serde(rename = "EPS Estimate")]
    pub eps_estimate: Option<f64>,
    #[serde(rename = "EPS Actual")]
    pub eps_actual: Option<f64>,
    #[serde(rename = "EPS Surprise")]
    pub eps_surprise: Option<f64>,
    #[serde(rename = "EPS GAAP Estimate")]
    pub eps_gaap_estimate: Option<f64>,
    #[serde(rename = "EPS GAAP Actual")]
    pub eps_gaap_actual: Option<f64>,
    #[serde(rename = "EPS GAAP Surprise")]
    pub eps_gaap_surprise: Option<f64>,
    #[serde(rename = "Revenue Estimate")]
    pub revenue_estimate: Option<f64>,
    #[serde(rename = "Revenue Actual")]
    pub revenue_actual: Option<f64>,
    #[serde(rename = "Revenue Surprise")]
    pub revenue_surprise: Option<f64>,
    #[serde(rename = "1-Day Price Reaction")]
    pub day_price_reaction: Option<f64>,
}

impl From<ResourceItem> for Item {
    fn from(item: ResourceItem) -> Self {
        let timestamp = util::parse_eastern_date_time_to_timestamp(&item.date, None).unwrap_or(0);

        Self {
            timestamp,
            ticker: item.ticker,
            company: item.company,
            market_cap: item.market_cap,
            eps_estimate: item.eps_estimate,
            eps_actual: item.eps_actual,
            eps_surprise: item.eps_surprise,
            eps_gaap_estimate: item.eps_gaap_estimate,
            eps_gaap_actual: item.eps_gaap_actual,
            eps_gaap_surprise: item.eps_gaap_surprise,
            revenue_estimate: item.revenue_estimate,
            revenue_actual: item.revenue_actual,
            revenue_surprise: item.revenue_surprise,
            day_price_reaction: item.day_price_reaction,
        }
    }
}
