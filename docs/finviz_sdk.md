# Finviz SDK

`finviz_sdk` 封装了 [Finviz Elite](https://elite.finviz.com) 的 CSV 导出接口，支持筛选器（Screener）、行情（Quote）与新闻（News）三种数据。

> 注意：这些接口需要 **Finviz Elite 付费账号**。登录后浏览器地址栏中 URL 携带的 `auth` 参数即为 API 密钥。

## 依赖

```toml
[dependencies]
finviz_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "finviz_sdk" }
tokio = { version = "1", features = ["full"] }
```

## 创建客户端

```rust
use finviz_sdk::Client;

let client = Client::new("YOUR_FINVIZ_ELITE_AUTH_TOKEN");
```

建议通过环境变量注入密钥：

```rust
let client = Client::new(&std::env::var("FINVIZ_ELITE_AUTH")?);
```

## 筛选器查询（Screener）

按条件筛选股票并导出结果。

### 构造查询

```rust
use finviz_sdk::ScreenerQuery;

let query = ScreenerQuery {
    order_by: "market cap".to_string(),
    signal: Some("topgainers".to_string()),
    parameter: Some("cap_midover".to_string()),
};

// 或使用默认值（order_by = "ticker"，signal/parameter 为 None）
let default_query = ScreenerQuery::default();
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `order_by` | `String` | 排序字段，如 `ticker`、`market cap` 等 |
| `signal` | `Option<String>` | 技术/信号筛选，如 `topgainers` |
| `parameter` | `Option<String>` | 基本面参数筛选，如 `cap_midover` |

### 请求 URL

```
GET https://elite.finviz.com/export?v=111&o={order_by}&auth={auth}[&f={parameter}][&s={signal}]
```

### 执行查询

```rust
let items = client.screener(&query).await?;
```

### 响应字段 `Item`

| 字段 | CSV 列 | 类型 | 说明 |
| --- | --- | --- | --- |
| `no` | `No.` | `u64` | 序号 |
| `ticker` | `Ticker` | `String` | 股票代码 |
| `company` | `Company` | `String` | 公司名称 |
| `sector` | `Sector` | `String` | 板块 |
| `industry` | `Industry` | `String` | 行业 |
| `country` | `Country` | `String` | 国家 |
| `market_cap` | `Market Cap` | `Option<f64>` | 市值 |
| `pe_ratio` | `P/E` | `Option<f64>` | 市盈率 |
| `price` | `Price` | `Option<f64>` | 现价 |
| `change` | `Change` | `Option<String>` | 涨跌幅 |
| `volume` | `Volume` | `Option<u64>` | 成交量 |

## 行情查询（Quote）

获取单个股票的历史 OHLC 行情。

### 构造查询

```rust
use finviz_sdk::{QuoteQuery, Interval, ValidRanges};

let query = QuoteQuery {
    symbol: "AAPL".to_string(),
    interval: Interval::Day,
    valid_ranges: ValidRanges::Month3,
};
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `symbol` | `String` | 股票代码 |
| `interval` | `Interval` | K 线周期 |
| `valid_ranges` | `ValidRanges` | 历史时间范围 |

### 请求 URL

```
GET https://elite.finviz.com/quote_export?t={symbol}&p={interval}&r={valid_ranges}&auth={auth}
```

### 执行查询

```rust
let bars = client.quote(&query).await?;
```

### `Interval` 枚举

| 变体 | URL 值 | 说明 |
| --- | --- | --- |
| `Minute` | `i1` | 1 分钟 |
| `Minutes2` | `i2` | 2 分钟 |
| `Minutes3` | `i3` | 3 分钟 |
| `Minutes5` | `i5` | 5 分钟 |
| `Minutes10` | `i10` | 10 分钟 |
| `Minutes15` | `i15` | 15 分钟 |
| `Minutes30` | `i30` | 30 分钟 |
| `Hour` | `h` | 1 小时 |
| `Hour2` | `h2` | 2 小时 |
| `Hour4` | `h4` | 4 小时 |
| `Day` | `d` | 日线 |
| `Week` | `w` | 周线 |
| `Month` | `m` | 月线 |

### `ValidRanges` 枚举

| 变体 | URL 值 | 说明 |
| --- | --- | --- |
| `Day` | `d1` | 1 天 |
| `Day5` | `d5` | 5 天 |
| `Month` | `m1` | 1 个月 |
| `Month3` | `m3` | 3 个月 |
| `Month6` | `m6` | 6 个月 |
| `YearToDate` | `ytd` | 年初至今 |
| `Year` | `y1` | 1 年 |
| `Year2` | `y2` | 2 年 |
| `Year5` | `y5` | 5 年 |
| `Max` | `max` | 全部历史 |

### 响应字段 `Item`

| 字段 | CSV 列 | 类型 | 说明 |
| --- | --- | --- | --- |
| `date` | `Date` | `String` | 日期 |
| `open` | `Open` | `f64` | 开盘价 |
| `high` | `High` | `f64` | 最高价 |
| `low` | `Low` | `f64` | 最低价 |
| `close` | `Close` | `f64` | 收盘价 |
| `volume` | `Volume` | `u64` | 成交量 |

## 新闻查询（News）

获取市场、个股或加密货币的新闻，支持三种变体。

### 构造查询

```rust
use finviz_sdk::news::{
    MarketParameter, MarketParameterCategory, MarketParameterOrdered,
    StocksParameter, StocksParameterCategory,
};
use finviz_sdk::NewsQuery;

// 市场新闻（按时间排序）
let market = NewsQuery::Market(MarketParameter {
    ordered: MarketParameterOrdered::Time,
    category: Some(MarketParameterCategory::News),
});

// 个股新闻（指定代码）
let stocks = NewsQuery::Stocks(StocksParameter {
    symbol: vec!["AAPL".to_string(), "MSFT".to_string()],
    category: StocksParameterCategory::NoETF,
});

// 加密货币新闻
let crypto = NewsQuery::Crypto(vec!["BTC".to_string(), "ETH".to_string()]);
```

### `Query` 变体

| 变体 | 说明 |
| --- | --- |
| `Query::Market(MarketParameter)` | 市场新闻/博客，按时间或来源排序，可选分类 |
| `Query::Stocks(StocksParameter)` | 指定股票代码的新闻，可选是否包含 ETF |
| `Query::Crypto(Vec<String>)` | 加密货币新闻，空列表表示全部 |

### 参数说明

**`MarketParameter`**

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `ordered` | `MarketParameterOrdered` | 排序方式：`Time` 或 `Source` |
| `category` | `Option<MarketParameterCategory>` | 分类：`News` 或 `Blogs` |

**`StocksParameter`**

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `symbol` | `Vec<String>` | 股票代码列表（CSV 拼接） |
| `category` | `StocksParameterCategory` | `ETF` 或 `NoETF` |

### 执行查询

```rust
let items = client.news(&news_query).await?;
```

### 响应字段 `Item`

| 字段 | CSV 列 | 类型 | 说明 |
| --- | --- | --- | --- |
| `title` | `Title` | `String` | 标题 |
| `source` | `Source` | `String` | 来源 |
| `date` | `Date` | `String` | 日期 |
| `url` | `Url` | `String` | 链接 |
| `category` | `Category` | `String` | 分类 |
| `ticker` | `Ticker` | `Option<String>` | 相关股票代码（可能为空） |

## 完整示例

```rust
use finviz_sdk::{Client, QuoteQuery};
use finviz_sdk::quote::{Interval, ValidRanges};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(&std::env::var("FINVIZ_ELITE_AUTH")?);

    // 获取 AAPL 近 3 个月日线
    let quote = QuoteQuery {
        symbol: "AAPL".to_string(),
        interval: Interval::Day,
        valid_ranges: ValidRanges::Month3,
    };

    for bar in client.quote(&quote).await? {
        println!("{} O={} H={} L={} C={} V={}", bar.date, bar.open, bar.high, bar.low, bar.close, bar.volume);
    }

    Ok(())
}
```

## 参考

- [Finviz Elite](https://elite.finviz.com)
