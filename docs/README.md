# TradeSDK 文档

TradeSDK 是一个基于 Rust 的量化交易数据 SDK 工作区，封装了 **Alpaca**、**Finviz Elite** 与 **Benzinga** 三大数据源，提供类型安全的异步接口。

## 内容

- [Alpaca SDK](alpaca_sdk.md)：Alpaca Markets 历史行情数据（快照 / 交易 / 报价 / K 线）
- [Finviz SDK](finviz_sdk.md)：Finviz Elite 数据（筛选器 / 行情 / 新闻）
- [Benzinga SDK](benzinga_sdk.md)：Benzinga 数据（IPO 日历）

## 工作区结构

```
TradeSDK/
├── crates/
│   ├── alpaca_sdk/          # Alpaca 历史行情 SDK
│   ├── finviz_sdk/          # Finviz Elite 数据 SDK
│   └── benzinga_sdk/        # Benzinga 数据 SDK
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
benzinga_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "benzinga_sdk" }
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
use finviz_sdk::{Client, StockQuery};
use finviz_sdk::stock::{Interval, ValidRanges};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("YOUR_FINVIZ_ELITE_AUTH_TOKEN");

    let query = StockQuery {
        symbol: "AAPL".to_string(),
        interval: Interval::Day,
        valid_ranges: ValidRanges::Month3,
    };

    let bars = client.stock(&query).await?;
    for bar in bars {
        println!("{} open={} close={}", bar.date, bar.open, bar.close);
    }
    Ok(())
}
```

#### Benzinga：查询 IPO 日历

```rust
use benzinga_sdk::{Client, IPOQuery, calendar::ipo::IPOType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let query = IPOQuery {
        page_size: 100,
        date_from: chrono::NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
        date_to: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
        ipo_type: IPOType::SPAC,
    };

    let items = client.ipo(&query).await?;
    for item in items {
        println!("{} ({})", item.ticker, item.name);
    }
    Ok(())
}
```

## License

本项目采用 [GPL-3.0-only](../LICENSE) 许可证。

> 本 SDK 仅用于数据访问，不构成任何投资建议。使用前请遵守各数据源的服务条款。
