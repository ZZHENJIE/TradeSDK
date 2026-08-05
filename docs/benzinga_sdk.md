# Benzinga SDK

`benzinga_sdk` 封装了 [Benzinga](https://www.benzinga.com) 的公开数据接口，目前支持 IPO 日历查询。

> 注意：当前接口为公开接口，无需 API 密钥即可访问。

## 依赖

```toml
[dependencies]
benzinga_sdk = { git = "https://github.com/ZZHENJIE/TradeSDK", package = "benzinga_sdk" }
tokio = { version = "1", features = ["full"] }
```

## 创建客户端

```rust
use benzinga_sdk::Client;

let client = Client::new();
```

Benzinga 客户端无需 API 密钥，可直接创建。

## IPO 日历查询

获取指定时间范围内的 IPO 日历数据。

### 构造查询

```rust
use benzinga_sdk::{IPOQuery, calendar::ipo::IPOType};

let query = IPOQuery {
    page_size: 100,
    date_from: chrono::NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
    date_to: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
    ipo_type: IPOType::SPAC,
};
```

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `page_size` | `i64` | 每页返回数量 |
| `date_from` | `NaiveDate` | 查询起始日期 |
| `date_to` | `NaiveDate` | 查询结束日期 |
| `ipo_type` | `IPOType` | IPO 类型 |

### 请求 URL

```
GET https://www.benzinga.com/api-next/calendar/ipos?pageSize={page_size}&dateFrom={date_from}&dateTo={date_to}[&ipoType=SPAC]
```

### 执行查询

```rust
let items = client.ipo(&query).await?;
```

### `IPOType` 枚举

| 变体 | 说明 |
| --- | --- |
| `OrdinaryShares` | 普通股 |
| `SPAC` | SPAC |

### 响应字段 `Item`

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `currency` | `String` | 货币 |
| `date` | `String` | 日期 |
| `deal_status` | `String` | 交易状态 |
| `description` | `String` | 描述 |
| `exchange` | `String` | 交易所 |
| `id` | `String` | ID |
| `initial_filing_date` | `String` | 初始申请日期 |
| `insider_lockup_date` | `String` | 内部人锁定期日期 |
| `insider_lockup_days` | `i32` | 内部人锁定期天数 |
| `ipo_type` | `String` | IPO 类型 |
| `last_yr_income` | `i64` | 上年收入 |
| `last_yr_income_year` | `i32` | 上年收入年份 |
| `last_yr_revenue` | `i64` | 上年营收 |
| `last_yr_revenue_year` | `i32` | 上年营收年份 |
| `lead_underwriters` | `Vec<String>` | 主承销商 |
| `market_cap_at_offer` | `i64` | 发行市值 |
| `name` | `String` | 公司名称 |
| `notes` | `String` | 备注 |
| `offering_shares` | `i64` | 发行股数 |
| `offering_shares_ord_adr` | `i64` | 普通股/ADR 发行股数 |
| `offering_value` | `i64` | 发行价值 |
| `open_date_verified` | `bool` | 开盘日期是否确认 |
| `ord_shares_out_after_offer` | `i64` | 发行后普通股数量 |
| `other_underwriters` | `Vec<String>` | 其他承销商 |
| `price_max` | `Option<String>` | 最高价格 |
| `price_min` | `Option<String>` | 最低价格 |
| `price_open` | `Option<String>` | 开盘价 |
| `price_public_offering` | `Option<String>` | 公开发行价 |
| `pricing_date` | `String` | 定价日期 |
| `pricing_date_verified` | `bool` | 定价日期是否确认 |
| `sec_accession_number` | `String` | SEC 受理号 |
| `sec_filing_url` | `String` | SEC 申报链接 |
| `shares_outstanding` | `i64` | 流通股数 |
| `sic` | `i32` | SIC 代码 |
| `spac_converted_to_target` | `bool` | SPAC 是否已转换为目标公司 |
| `state_location` | `String` | 所在州 |
| `ticker` | `String` | 股票代码 |
| `time` | `String` | 时间 |
| `underwriter_quiet_expiration_date` | `Option<String>` | 承销静默期到期日 |
| `underwriter_quiet_expiration_days` | `i32` | 承销静默期天数 |
| `updated` | `i64` | 更新时间戳 |

## 完整示例

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
        println!("{} - {} ({})", item.ticker, item.name, item.exchange);
    }

    Ok(())
}
```

## 参考

- [Benzinga](https://www.benzinga.com)
