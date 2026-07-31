#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Query {
    Market(MarketParameter),
    Stocks(StocksParameter),
    Crypto(Vec<String>),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MarketParameter {
    pub ordered: MarketParameterOrdered,
    pub category: Option<MarketParameterCategory>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum MarketParameterCategory {
    News,
    Blogs,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum MarketParameterOrdered {
    Time,
    Source,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StocksParameter {
    pub symbol: Vec<String>,
    pub category: StocksParameterCategory,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum StocksParameterCategory {
    ETF,
    NoETF,
}

impl MarketParameter {
    pub fn url(&self) -> String {
        let mut result = "v=".to_string();
        match self.ordered {
            MarketParameterOrdered::Time => result.push('1'),
            MarketParameterOrdered::Source => result.push('2'),
        };
        if let Some(category) = &self.category {
            result.push_str("&c=");
            match category {
                MarketParameterCategory::News => result.push('1'),
                MarketParameterCategory::Blogs => result.push('2'),
            };
        }
        result
    }
}

impl StocksParameter {
    pub fn url(&self) -> String {
        let mut result = "v=".to_string();
        match self.category {
            StocksParameterCategory::NoETF => result.push('3'),
            StocksParameterCategory::ETF => result.push('4'),
        };

        if !self.symbol.is_empty() {
            result.push_str(&format!("&t={}", self.symbol.join(",")));
        }

        result
    }
}

impl Query {
    pub fn url(&self, auth: &str) -> String {
        let base_url = format!("https://elite.finviz.com/news_export?auth={}&", auth);
        match self {
            Query::Market(value) => format!("{}{}", base_url, value.url()),
            Query::Stocks(value) => format!("{}{}", base_url, value.url()),
            Query::Crypto(value) => {
                if value.is_empty() {
                    format!("{}v=5", base_url)
                } else {
                    format!("{}v=5&t={}", base_url, value.join(","))
                }
            }
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Ticker")]
    pub ticker: Option<String>,
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
