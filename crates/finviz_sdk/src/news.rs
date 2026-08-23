use std::fmt::Write;

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
            let _ = write!(result, "&t={}", self.symbol.join(","));
        }

        result
    }
}

impl util::API for Query {
    type T = Vec<Item>;
    type P = String;

    fn url(&self, params: Option<Self::P>) -> String {
        let base_url = format!(
            "https://elite.finviz.com/export/news?auth={}&",
            params.unwrap_or_default()
        );
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
