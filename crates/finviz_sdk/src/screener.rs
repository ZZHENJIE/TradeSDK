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

impl Query {
    pub fn url(&self, auth: &str) -> String {
        let mut result = format!(
            "https://elite.finviz.com/export?v=111&o={}&auth={}",
            self.order_by, auth
        );

        if let Some(value) = &self.parameter {
            result.push_str(format!("&f={}", value).as_str());
        }

        if let Some(value) = &self.signal {
            result.push_str(format!("&s={}", value).as_str());
        }

        result
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

pub async fn fetch(
    query: &Query,
    auth: &str,
    http_client: &reqwest::Client,
) -> anyhow::Result<Vec<Item>> {
    let url = query.url(auth);
    let response = http_client.get(&url).send().await?;
    let csv_data = response.text().await?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());

    let mut result: Vec<Item> = Vec::new();

    for record in reader.deserialize() {
        let item: Item = record?;
        result.push(item);
    }

    Ok(result)
}
