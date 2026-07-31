# TradeSDK

基于 Rust 的量化交易数据 SDK 工作区（workspace），封装了 **Alpaca** 与 **Finviz Elite** 两大行情数据源，提供类型安全的异步 API 接口。

## 特性

- **类型安全**：请求参数与响应数据全部使用强类型结构体，配合 `serde` 自动解析 JSON / CSV。
- **异步**：基于 `reqwest` 的异步 HTTP 客户端，非阻塞 I/O。
- **模块化**：每个数据源为独立 crate，可按需引入。
- **零 unsafe**：所有 crate 均设置 `#![forbid(unsafe_code)]`。

## 工作区结构

```
TradeSDK/
├── crates/
│   ├── alpaca_sdk/          # Alpaca 历史行情 SDK
│   └── finviz_sdk/          # Finviz Elite 数据 SDK
├── docs/                    # 文档站点（docsify）
├── Cargo.toml               # 工作区配置
└── README.md
```

### 子 crate 一览

| Crate | 说明 | 依赖 |
| --- | --- | --- |
| [`alpaca_sdk`](docs/alpaca_sdk.md) | Alpaca Markets 历史行情（快照 / 交易 / 报价 / K 线） | `reqwest` `serde` `chrono` |
| [`finviz_sdk`](docs/finviz_sdk.md) | Finviz Elite 数据（筛选器 / 行情 / 新闻） | `reqwest` `serde` `csv` |

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

### 使用 Alpaca SDK

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

### 使用 Finviz SDK

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

> 注意：Finviz Elite 接口需要登录后在浏览器 URL 中获取的 `auth` 参数作为 API 密钥。

## 环境变量

建议将密钥通过环境变量注入，避免硬编码：

```rust
let api_key = std::env::var("ALPACA_API_KEY")?;
let api_secret = std::env::var("ALPACA_API_SECRET")?;
```

## 文档

- 在线文档：[docs](docs/README.md)（docsify 构建）
- 本地文档：`cargo doc --no-deps` 生成 rustdoc，位于 `target/doc/`

## License

本项目采用 [GPL-3.0-only](LICENSE) 许可证。

## 声明

本 SDK 仅用于数据访问，不构成任何投资建议。使用前请遵守 Alpaca 与 Finviz 的服务条款。
