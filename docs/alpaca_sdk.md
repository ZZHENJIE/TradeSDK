# Alpaca SDK

`alpaca_sdk` 封装了 [Alpaca Markets](https://alpaca.markets) 的[历史行情数据接口](https://docs.alpaca.markets/us/docs/historical-stock-data-1)，目前支持股票快照（Snapshot）查询。

## 依赖

```toml
[dependencies]
alpaca_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "alpaca_sdk" }
tokio = { version = "1", features = ["full"] }
```

## 创建客户端

```rust
use alpaca_sdk::Client;

let client = Client::new("YOUR_API_KEY", "YOUR_API_SECRET");
```

Alpaca API 密钥需要在 [Alpaca](https://app.alpaca.markets) 控制台创建。建议通过环境变量注入：

```rust
let client = Client::new(
    &std::env::var("ALPACA_API_KEY")?,
    &std::env::var("ALPACA_API_SECRET")?,
);
```

## 快照查询（Snapshot）

获取股票的最新交易、报价与 K 线数据。

### 构造查询

```rust
use alpaca_sdk::{SnapshotQuery, Feed};

let query = SnapshotQuery {
    symbol: "AAPL".to_string(),
    feed: Feed::Sip,
    currency: "USD".to_string(),
};
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `symbol` | `String` | 股票代码 |
| `feed` | `Feed` | 数据源（见下表） |
| `currency` | `String` | 货币单位，如 `"USD"` |

### 请求 URL

```
GET https://data.alpaca.markets/v2/stocks/{symbol}/snapshot?feed={feed}&currency={currency}
```

### 执行查询

```rust
let snapshot = client.snapshot(&query).await?;
```

### Feed 枚举

| 变体 | URL 值 | 说明 |
| --- | --- | --- |
| `Feed::Sip` | `sip` | 综合行情（SIP，推荐） |
| `Feed::Iex` | `iex` | IEX 数据 |
| `Feed::DelayedSip` | `delayed_sip` | 延时 SIP 数据 |
| `Feed::Boats` | `boats` | BOATS 数据 |
| `Feed::Overnight` | `overnight` | 隔夜交易数据 |
| `Feed::Otc` | `otc` | 场外交易数据 |

## 响应数据结构

### `Response`

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `symbol` | `String` | 股票代码 |
| `currency` | `Option<String>` | 货币单位 |
| `snapshot` | `StockSnapshot` | 快照内容（`serde(flatten)`） |

### `StockSnapshot`

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `latest_trade` | `Option<StockTrade>` | 最新一笔成交 |
| `latest_quote` | `Option<StockQuote>` | 最新盘口报价 |
| `minute_bar` | `Option<StockBar>` | 最新分钟 K 线 |
| `daily_bar` | `Option<StockBar>` | 当日日 K 线 |
| `prev_daily_bar` | `Option<StockBar>` | 前一交易日日 K 线 |

### `StockBar`（OHLC 数据）

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `timestamp` | `DateTime<Utc>` | 时间戳 |
| `open` | `f64` | 开盘价 |
| `high` | `f64` | 最高价 |
| `low` | `f64` | 最低价 |
| `close` | `f64` | 收盘价 |
| `volume` | `i64` | 成交量 |
| `trade_count` | `i64` | 成交笔数 |
| `vwap` | `f64` | 成交量加权平均价 |

### `StockTrade`

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `timestamp` | `DateTime<Utc>` | 时间戳 |
| `id` | `u64` | 交易 ID |
| `exchange` | `String` | 交易所代码 |
| `price` | `f64` | 成交价 |
| `size` | `u32` | 成交量 |
| `conditions` | `Vec<String>` | 条件标识 |
| `tape` | `StockTape` | 交易 tape |
| `update` | `Option<String>` | 更新状态 |

### `StockQuote`

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `timestamp` | `DateTime<Utc>` | 时间戳 |
| `bid_exchange` | `String` | 买方交易所 |
| `bid_price` | `f64` | 买价 |
| `bid_size` | `u32` | 买单数量 |
| `ask_price` | `f64` | 卖价 |
| `ask_size` | `u32` | 卖单数量 |
| `ask_exchange` | `String` | 卖方交易所 |
| `conditions` | `Vec<String>` | 条件标识 |
| `tape` | `StockTape` | 报价 tape |

### `StockTape`

| 变体 | 说明 |
| --- | --- |
| `A` | 纽约证券交易所 |
| `B` | NYSE Arca、Bats、IEX 及其他区域性交易所 |
| `C` | NASDAQ |
| `N` | 隔夜交易 |
| `O` | 场外交易 |

## 完整示例

```rust
use alpaca_sdk::{Client, Feed, SnapshotQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(
        &std::env::var("ALPACA_API_KEY")?,
        &std::env::var("ALPACA_API_SECRET")?,
    );

    let query = SnapshotQuery {
        symbol: "TSLA".to_string(),
        feed: Feed::Sip,
        currency: "USD".to_string(),
    };

    let snapshot = client.snapshot(&query).await?;
    println!("symbol: {}", snapshot.symbol);

    if let Some(trade) = snapshot.snapshot.latest_trade {
        println!("最新成交: {} @ {}", trade.price, trade.timestamp);
    }

    if let Some(bar) = snapshot.snapshot.daily_bar {
        println!("当日: open={} high={} low={} close={}", bar.open, bar.high, bar.low, bar.close);
    }

    Ok(())
}
```

## 扩展 `request`

`Client::request` 暴露了底层 `reqwest::RequestBuilder`，可用于调用其他 Alpaca 接口（需自行构造 URL 与反序列化）：

```rust
use reqwest::Method;

let builder = client.request(Method::GET, "https://data.alpaca.markets/v2/stocks/AAPL/bars?timeframe=1D");
```

## 参考

- [Alpaca 官方文档](https://docs.alpaca.markets/us/docs/historical-stock-data-1)
