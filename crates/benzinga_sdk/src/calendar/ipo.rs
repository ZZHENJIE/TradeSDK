use chrono::NaiveDate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum IPOType {
    OrdinaryShares,
    SPAC,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub page_size: i64,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub ipo_type: IPOType,
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
        let mut result = format!(
            "https://www.benzinga.com/api-next/calendar/ipos?pageSize={}&dateFrom={}&dateTo={}",
            self.page_size,
            self.date_from.format("%Y-%m-%d"),
            self.date_to.format("%Y-%m-%d"),
        );

        if matches!(self.ipo_type, IPOType::SPAC) {
            result.push_str("&ipoType=SPAC");
        }

        result
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceItem {
    pub currency: String,
    pub date: String,
    pub deal_status: String,
    pub description: String,
    pub exchange: String,
    pub id: String,
    pub initial_filing_date: String,
    pub insider_lockup_date: String,
    pub insider_lockup_days: i32,
    pub ipo_type: String,
    pub last_yr_income: i64,
    pub last_yr_income_year: i32,
    pub last_yr_revenue: i64,
    pub last_yr_revenue_year: i32,
    pub lead_underwriters: Vec<String>,
    pub market_cap_at_offer: i64,
    pub name: String,
    pub notes: String,
    pub offering_shares: i64,
    pub offering_shares_ord_adr: i64,
    pub offering_value: i64,
    pub open_date_verified: bool,
    pub ord_shares_out_after_offer: i64,
    pub other_underwriters: Vec<String>,
    #[serde(default)]
    pub price_max: Option<String>,
    #[serde(default)]
    pub price_min: Option<String>,
    #[serde(default)]
    pub price_open: Option<String>,
    #[serde(default)]
    pub price_public_offering: Option<String>,
    pub pricing_date: String,
    pub pricing_date_verified: bool,
    pub sec_accession_number: String,
    pub sec_filing_url: String,
    pub shares_outstanding: i64,
    pub sic: i32,
    pub spac_converted_to_target: bool,
    pub state_location: String,
    pub ticker: String,
    pub time: String,
    #[serde(default)]
    pub underwriter_quiet_expiration_date: Option<String>,
    pub underwriter_quiet_expiration_days: i32,
    pub updated: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    pub currency: String,
    pub timestamp: i64,
    pub deal_status: String,
    pub description: String,
    pub exchange: String,
    pub id: String,
    pub initial_filing_date: String,
    pub insider_lockup_date: String,
    pub insider_lockup_days: i32,
    pub ipo_type: String,
    pub last_yr_income: i64,
    pub last_yr_income_year: i32,
    pub last_yr_revenue: i64,
    pub last_yr_revenue_year: i32,
    pub lead_underwriters: Vec<String>,
    pub market_cap_at_offer: i64,
    pub name: String,
    pub notes: String,
    pub offering_shares: i64,
    pub offering_shares_ord_adr: i64,
    pub offering_value: i64,
    pub open_date_verified: bool,
    pub ord_shares_out_after_offer: i64,
    pub other_underwriters: Vec<String>,
    #[serde(default)]
    pub price_max: Option<String>,
    #[serde(default)]
    pub price_min: Option<String>,
    #[serde(default)]
    pub price_open: Option<String>,
    #[serde(default)]
    pub price_public_offering: Option<String>,
    pub pricing_date: String,
    pub pricing_date_verified: bool,
    pub sec_accession_number: String,
    pub sec_filing_url: String,
    pub shares_outstanding: i64,
    pub sic: i32,
    pub spac_converted_to_target: bool,
    pub state_location: String,
    pub ticker: String,
    #[serde(default)]
    pub underwriter_quiet_expiration_date: Option<String>,
    pub underwriter_quiet_expiration_days: i32,
    pub updated: i64,
}

impl From<ResourceItem> for Item {
    fn from(r: ResourceItem) -> Self {
        let timestamp =
            util::parse_eastern_time_to_timestamp(&r.date, &r.time, None, None).unwrap_or(0);

        Self {
            currency: r.currency,
            timestamp,
            deal_status: r.deal_status,
            description: r.description,
            exchange: r.exchange,
            id: r.id,
            initial_filing_date: r.initial_filing_date,
            insider_lockup_date: r.insider_lockup_date,
            insider_lockup_days: r.insider_lockup_days,
            ipo_type: r.ipo_type,
            last_yr_income: r.last_yr_income,
            last_yr_income_year: r.last_yr_income_year,
            last_yr_revenue: r.last_yr_revenue,
            last_yr_revenue_year: r.last_yr_revenue_year,
            lead_underwriters: r.lead_underwriters,
            market_cap_at_offer: r.market_cap_at_offer,
            name: r.name,
            notes: r.notes,
            offering_shares: r.offering_shares,
            offering_shares_ord_adr: r.offering_shares_ord_adr,
            offering_value: r.offering_value,
            open_date_verified: r.open_date_verified,
            ord_shares_out_after_offer: r.ord_shares_out_after_offer,
            other_underwriters: r.other_underwriters,
            price_max: r.price_max,
            price_min: r.price_min,
            price_open: r.price_open,
            price_public_offering: r.price_public_offering,
            pricing_date: r.pricing_date,
            pricing_date_verified: r.pricing_date_verified,
            sec_accession_number: r.sec_accession_number,
            sec_filing_url: r.sec_filing_url,
            shares_outstanding: r.shares_outstanding,
            sic: r.sic,
            spac_converted_to_target: r.spac_converted_to_target,
            state_location: r.state_location,
            ticker: r.ticker,
            underwriter_quiet_expiration_date: r.underwriter_quiet_expiration_date,
            underwriter_quiet_expiration_days: r.underwriter_quiet_expiration_days,
            updated: r.updated,
        }
    }
}
