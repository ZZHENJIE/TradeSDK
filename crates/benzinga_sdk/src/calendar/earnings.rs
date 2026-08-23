use chrono::NaiveDate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub page_size: i64,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

impl Query {
    /// Returns the raw, unconverted response data
    pub async fn fetch_raw(
        &self,
        http_client: &reqwest::Client,
    ) -> anyhow::Result<Vec<ResourceItem>> {
        util::fetch_json(util::API::url(self, None), http_client).await
    }
}

impl util::API for Query {
    type T = Vec<Item>;
    type P = ();

    fn url(&self, _: Option<Self::P>) -> String {
        format!(
            "https://www.benzinga.com/api-next/calendar/earnings?pageSize={}&dateFrom={}&dateTo={}",
            self.page_size, self.date_from, self.date_to
        )
    }

    async fn fetch(
        &self,
        http_client: &reqwest::Client,
        _: Option<Self::P>,
    ) -> anyhow::Result<Self::T> {
        Ok(self
            .fetch_raw(http_client)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceItem {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub currency: String,
    pub cusip: String,
    pub timestamp: i64,
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
    pub updated: u64,
}

impl From<ResourceItem> for Item {
    fn from(r: ResourceItem) -> Self {
        let timestamp =
            util::parse_eastern_time_to_timestamp(&r.date, &r.time, None, None).unwrap_or(0);

        Self {
            currency: r.currency,
            cusip: r.cusip,
            timestamp,
            date_confirmed: r.date_confirmed,
            eps: r.eps,
            eps_est: r.eps_est,
            eps_prior: r.eps_prior,
            eps_surprise: r.eps_surprise,
            eps_surprise_percent: r.eps_surprise_percent,
            eps_type: r.eps_type,
            exchange: r.exchange,
            id: r.id,
            importance: r.importance,
            isin: r.isin,
            name: r.name,
            notes: r.notes,
            period: r.period,
            period_year: r.period_year,
            revenue: r.revenue,
            revenue_est: r.revenue_est,
            revenue_prior: r.revenue_prior,
            revenue_surprise: r.revenue_surprise,
            revenue_surprise_percent: r.revenue_surprise_percent,
            revenue_type: r.revenue_type,
            ticker: r.ticker,
            updated: r.updated,
        }
    }
}
