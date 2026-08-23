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
            "https://www.benzinga.com/api-next/calendar/economics?pageSize={}&dateFrom={}&dateTo={}",
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    pub timestamp: i64,
    pub updated: u64,
}

impl From<ResourceItem> for Item {
    fn from(r: ResourceItem) -> Self {
        let timestamp =
            util::parse_eastern_time_to_timestamp(&r.date, &r.time, None, None).unwrap_or(0);

        Self {
            actual: r.actual,
            actual_t: r.actual_t,
            consensus: r.consensus,
            consensus_t: r.consensus_t,
            country: r.country,
            description: r.description,
            event_category: r.event_category,
            event_name: r.event_name,
            event_period: r.event_period,
            id: r.id,
            importance: r.importance,
            notes: r.notes,
            period_year: r.period_year,
            prior: r.prior,
            prior_t: r.prior_t,
            timestamp,
            updated: r.updated,
        }
    }
}
