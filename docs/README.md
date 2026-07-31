# TradeSDK 文档

TradeSDK 是一个基于 Rust 的量化交易数据 SDK 工作区，封装了 **Alpaca** 与 **Finviz Elite** 两大数据源，提供类型安全的异步接口。

## 内容

- [Alpaca SDK](alpaca_sdk.md)：Alpaca Markets 历史行情数据（快照 / 交易 / 报价 / K 线）
- [Finviz SDK](finviz_sdk.md)：Finviz Elite 数据（筛选器 / 行情 / 新闻）

## 工作区结构

```
TradeSDK/
├── crates/
│   ├── alpaca_sdk/          # Alpaca 历史行情 SDK
│   └── finviz_sdk/          # Finviz Elite 数据 SDK
├── docs/                    # 本文档站点（docsify）
├── Cargo.toml               # 工作区配置
└── README.md                # 项目 README
```

## 快速开始

### 环境要求

- Rust 1.85+（`edition = "2024"`）

### 添加依赖

```toml
[dependencies]
alpaca_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "alpaca_sdk" }
finviz_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "finviz_sdk" }
tokio = { version = "1", features = ["full"] }
```

### 示例

#### Alpaca：获取股票快照

```rust
use alpaca_sdk::{Client, Feed, SnapshotQuery};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("YOUR_API_KEY", "YOUR_API_SECRET");

    let query = SnapshotQuery {
        symbol: "AAPL".to_string(),
        feed: Feed::Sip,
        currency: "USD".to_string(),
    };

    let snapshot = client.snapshot(&query).await?;
    if let Some(bar) = snapshot.snapshot.daily_bar {
        println!("{} 收盘价: {}", snapshot.symbol, bar.close);
    }
    Ok(())
}
```

#### Finviz：获取股票日线行情

```rust
use finviz_sdk::{Client, QuoteQuery};
use finviz_sdk::quote::{Interval, ValidRanges};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("YOUR_FINVIZ_ELITE_AUTH_TOKEN");

    let query = QuoteQuery {
        symbol: "AAPL".to_string(),
        interval: Interval::Day,
        valid_ranges: ValidRanges::Month,
    };

    let bars = client.quote(&query).await?;
    for bar in bars {
        println!("{} open={} close={}", bar.date, bar.open, bar.close);
    }
    Ok(())
}
```

## License

本项目采用 [GPL-3.0-only](../LICENSE) 许可证。

> 本 SDK 仅用于数据访问，不构成任何投资建议。使用前请遵守 Alpaca 与 Finviz 的服务条款。
