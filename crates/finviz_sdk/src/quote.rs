#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Interval {
    Minute,
    Minutes2,
    Minutes3,
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
    Hour,
    Hour2,
    Hour4,
    Day,
    Week,
    Month,
}

impl Into<&str> for Interval {
    fn into(self) -> &'static str {
        match self {
            Interval::Minute => "i1",
            Interval::Minutes2 => "i2",
            Interval::Minutes3 => "i3",
            Interval::Minutes5 => "i5",
            Interval::Minutes10 => "i10",
            Interval::Minutes15 => "i15",
            Interval::Minutes30 => "i30",
            Interval::Hour => "h",
            Interval::Hour2 => "h2",
            Interval::Hour4 => "h4",
            Interval::Day => "d",
            Interval::Week => "w",
            Interval::Month => "m",
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ValidRanges {
    Day,
    Day5,
    Month,
    Month3,
    Month6,
    YearToDate,
    Year,
    Year2,
    Year5,
    Max,
}

impl Into<&str> for ValidRanges {
    fn into(self) -> &'static str {
        match self {
            ValidRanges::Day => "d1",
            ValidRanges::Day5 => "d5",
            ValidRanges::Month => "m1",
            ValidRanges::Month3 => "m3",
            ValidRanges::Month6 => "m6",
            ValidRanges::YearToDate => "ytd",
            ValidRanges::Year => "y1",
            ValidRanges::Year2 => "y2",
            ValidRanges::Year5 => "y5",
            ValidRanges::Max => "max",
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub symbol: String,
    pub interval: Interval,
    pub valid_ranges: ValidRanges,
}

impl Query {
    pub fn url(&self, auth: &str) -> String {
        let interval: &str = self.interval.into();
        let valid_ranges: &str = self.valid_ranges.into();
        format!(
            "https://elite.finviz.com/quote_export?t={}&p={}&r={}&auth={}",
            self.symbol, interval, valid_ranges, auth
        )
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Open")]
    pub open: f64,
    #[serde(rename = "High")]
    pub high: f64,
    #[serde(rename = "Low")]
    pub low: f64,
    #[serde(rename = "Close")]
    pub close: f64,
    #[serde(rename = "Volume")]
    pub volume: u64,
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
