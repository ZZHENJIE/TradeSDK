use chrono::{DateTime, Utc};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Query {
    pub symbol: String,
    pub feed: crate::Feed,
    pub currency: String,
}

impl Query {
    pub fn url(&self) -> String {
        let feed: &str = self.feed.into();
        format!(
            "https://data.alpaca.markets/v2/stocks/{}/snapshot?feed={}&currency={}",
            self.symbol, feed, self.currency
        )
    }
}

pub async fn fetch(request: reqwest::RequestBuilder) -> anyhow::Result<Response> {
    let response = request.send().await?;
    let result = response.json::<Response>().await?;
    Ok(result)
}

/// K线数据（OHLC）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockBar {
    /// 时间戳
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// 开盘价
    #[serde(rename = "o")]
    pub open: f64,
    /// 最高价
    #[serde(rename = "h")]
    pub high: f64,
    /// 最低价
    #[serde(rename = "l")]
    pub low: f64,
    /// 收盘价
    #[serde(rename = "c")]
    pub close: f64,
    /// 成交量
    #[serde(rename = "v")]
    pub volume: i64,
    /// 成交笔数
    #[serde(rename = "n")]
    pub trade_count: i64,
    /// 成交量加权平均价格
    #[serde(rename = "vw")]
    pub vwap: f64,
}

/// 逐笔交易数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockTrade {
    /// 时间戳
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// 交易ID
    #[serde(rename = "i")]
    pub id: u64,
    /// 交易所代码
    #[serde(rename = "x")]
    pub exchange: String,
    /// 成交价
    #[serde(rename = "p")]
    pub price: f64,
    /// 成交量
    #[serde(rename = "s")]
    pub size: u32,
    /// 条件标识
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    /// 交易 tape
    #[serde(rename = "z")]
    pub tape: StockTape,
    /// 更新状态（可选）
    #[serde(rename = "u")]
    pub update: Option<String>,
}

/// 报价数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockQuote {
    /// 时间戳
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// 买方交易所
    #[serde(rename = "bx")]
    pub bid_exchange: String,
    /// 买价
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// 买单数量
    #[serde(rename = "bs")]
    pub bid_size: u32,
    /// 卖价
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// 卖单数量
    #[serde(rename = "as")]
    pub ask_size: u32,
    /// 卖方交易所
    #[serde(rename = "ax")]
    pub ask_exchange: String,
    /// 条件标识
    #[serde(rename = "c")]
    pub conditions: Vec<String>,
    /// 报价 tape
    #[serde(rename = "z")]
    pub tape: StockTape,
}

/// 交易 tape 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StockTape {
    /// 纽约证券交易所
    A,
    /// NYSE Arca, Bats, IEX 和其他区域性交易所
    B,
    /// NASDAQ
    C,
    /// 隔夜交易
    N,
    /// 场外交易
    O,
}

/// 股票快照数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockSnapshot {
    /// 最新的逐笔交易数据
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<StockTrade>,
    /// 最新的报价数据
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<StockQuote>,
    /// 最新的分钟K线
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<StockBar>,
    /// 当日日K线
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<StockBar>,
    /// 前一个交易日的日K线
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<StockBar>,
}

/// 股票快照响应（单只股票）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response {
    /// 股票代码
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// 货币单位
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(flatten)]
    pub snapshot: StockSnapshot,
}
