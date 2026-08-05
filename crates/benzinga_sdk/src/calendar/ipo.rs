use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IPOType {
    OrdinaryShares,
    SPAC,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub page_size: i64,
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
    pub ipo_type: IPOType,
}

impl Query {
    pub fn url(&self) -> String {
        let mut result = format!(
            "https://www.benzinga.com/api-next/calendar/ipos?pageSize={}&dateFrom={}&dateTo={}",
            self.page_size,
            self.date_from.format("%Y-%m-%d").to_string(),
            self.date_to.format("%Y-%m-%d").to_string(),
        );

        match self.ipo_type {
            IPOType::SPAC => {
                result.push_str("&ipoType=SPAC");
            }
            IPOType::OrdinaryShares => {}
        }

        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Item {
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

pub async fn fetch(query: &Query, http_client: &reqwest::Client) -> anyhow::Result<Vec<Item>> {
    let url = query.url();
    let response = http_client.get(&url).send().await?;
    let result = response.json::<Vec<Item>>().await?;
    Ok(result)
}
