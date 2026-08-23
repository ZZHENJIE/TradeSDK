use std::fmt::Write;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub order_by: String,
    pub signal: Option<String>,
    pub parameter: Option<String>,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            order_by: "ticker".to_string(),
            signal: None,
            parameter: None,
        }
    }
}

impl util::API for Query {
    type T = Vec<Item>;
    type P = String;

    fn url(&self, params: Option<Self::P>) -> String {
        let mut result = format!(
            "https://elite.finviz.com/export/screener?v=111&o={}&auth={}",
            self.order_by,
            params.unwrap_or_default()
        );

        if let Some(value) = &self.parameter {
            let _ = write!(result, "&f={value}");
        }

        if let Some(value) = &self.signal {
            let _ = write!(result, "&s={value}");
        }

        result
    }

    async fn fetch(
        &self,
        http_client: &reqwest::Client,
        params: Option<Self::P>,
    ) -> anyhow::Result<Self::T> {
        crate::fetch_csv(&self.url(params), http_client).await
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "No.")]
    pub no: u64,
    #[serde(rename = "Ticker")]
    pub ticker: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "Sector")]
    pub sector: String,
    #[serde(rename = "Industry")]
    pub industry: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Market Cap")]
    pub market_cap: Option<f64>,
    #[serde(rename = "P/E")]
    pub pe_ratio: Option<f64>,
    #[serde(rename = "Price")]
    pub price: Option<f64>,
    #[serde(rename = "Change")]
    pub change: Option<String>,
    #[serde(rename = "Volume")]
    pub volume: Option<u64>,
}
