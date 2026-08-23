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

impl From<Interval> for &'static str {
    fn from(value: Interval) -> Self {
        match value {
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

impl From<ValidRanges> for &'static str {
    fn from(value: ValidRanges) -> Self {
        match value {
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

impl util::API for Query {
    type T = Vec<Item>;
    type P = String;

    fn url(&self, params: Option<Self::P>) -> String {
        let interval: &str = self.interval.into();
        let valid_ranges: &str = self.valid_ranges.into();
        format!(
            "https://elite.finviz.com/export/stock?t={}&p={}&r={}&auth={}",
            self.symbol,
            interval,
            valid_ranges,
            params.unwrap_or_default()
        )
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
