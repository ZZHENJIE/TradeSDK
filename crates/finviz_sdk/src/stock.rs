use chrono::TimeZone;

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
        let items: Vec<ResourceItem> = crate::fetch_csv(&self.url(params), http_client).await?;
        items
            .iter()
            .map(|item| item.try_into())
            .collect::<anyhow::Result<Vec<Item>>>()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceItem {
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

fn parse_eastern_time(s: &str) -> anyhow::Result<chrono::NaiveDateTime> {
    let s = s.trim();

    // 12-hour format (04:00 AM)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%m/%d/%Y %I:%M %p") {
        return Ok(dt);
    }

    // Strip trailing AM/PM, parse as 24-hour format (13:00 PM -> 13:00)
    let without_ampm = s
        .trim_end_matches(|c: char| c.is_ascii_alphabetic() || c == ' ')
        .trim();
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(without_ampm, "%m/%d/%Y %H:%M") {
        return Ok(dt);
    }

    anyhow::bail!("failed to parse date: {}", s)
}

impl TryInto<Item> for &ResourceItem {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Item> {
        let naive_dt = parse_eastern_time(&self.date)
            .map_err(|e| anyhow::anyhow!("failed to parse '{}': {}", self.date, e))?;

        let local_dt = chrono_tz::America::New_York
            .from_local_datetime(&naive_dt)
            .single()
            .ok_or_else(|| anyhow::anyhow!("invalid US Eastern time: {}", self.date))?;

        let utc_dt = local_dt.with_timezone(&chrono::Utc);

        Ok(Item {
            timestamp: utc_dt,
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "Date", with = "chrono::serde::ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
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
