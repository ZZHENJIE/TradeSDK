use std::collections::BTreeMap;

use alpaca_sdk::{Feed, QuoteQuery, SnapshotQuery};
use benzinga_sdk::calendar::{EarningsQuery, EconomicsQuery, IPOQuery, ipo::IPOType};
use finviz_sdk::{NewsQuery, ScreenerQuery, StockQuery, news::StocksParameter};

const ALPACA_API_KEY: &str = "xxx";
const ALPACA_API_SECRET: &str = "xxx";
const FINVIZ_AUTH: &str = "xxx";

/// Environment variable takes precedence, falling back to the source constant.
fn secret(env_var: &str, fallback: &'static str) -> String {
    std::env::var(env_var).unwrap_or_else(|_| fallback.to_string())
}

fn alpaca_api_key() -> String {
    secret("ALPACA_API_KEY", ALPACA_API_KEY)
}

fn alpaca_api_secret() -> String {
    secret("ALPACA_API_SECRET", ALPACA_API_SECRET)
}

fn finviz_auth() -> String {
    secret("FINVIZ_ELITE_AUTH", FINVIZ_AUTH)
}

#[tokio::main]
async fn main() {
    // Load .env from the project root; already-set env vars take precedence.
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let registry = registry();

    if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "list" {
        println!("Usage: test <endpoint-name>\n\nAvailable endpoints:");
        for name in registry.keys() {
            println!("  {name}");
        }
        println!("\n  all  (run every endpoint)");
        return;
    }

    if args[0] == "all" {
        let mut failed = Vec::new();
        for (name, runner) in &registry {
            println!("=== {name} ===");
            if let Err(e) = runner().await {
                eprintln!("FAILED: {e:#}");
                failed.push(*name);
            }
        }
        if !failed.is_empty() {
            eprintln!("{} endpoint(s) failed: {}", failed.len(), failed.join(", "));
            std::process::exit(1);
        }
        return;
    }

    let Some(runner) = registry.get(args[0].as_str()) else {
        eprintln!(
            "Unknown endpoint: {}\nUse `test list` to see available endpoints",
            args[0]
        );
        std::process::exit(1);
    };

    match runner().await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Test failed: {e:#}");
            std::process::exit(1);
        }
    }
}

type Runner = fn() -> futures::future::BoxFuture<'static, anyhow::Result<()>>;

fn registry() -> BTreeMap<&'static str, Runner> {
    BTreeMap::from([
        (
            "alpaca:snapshot",
            (|| Box::pin(run_alpaca_snapshot()) as _) as Runner,
        ),
        (
            "alpaca:quote",
            (|| Box::pin(run_alpaca_quote()) as _) as Runner,
        ),
        (
            "benzinga:earnings",
            (|| Box::pin(run_benzinga_earnings()) as _) as Runner,
        ),
        (
            "benzinga:economics",
            (|| Box::pin(run_benzinga_economics()) as _) as Runner,
        ),
        (
            "benzinga:ipo",
            (|| Box::pin(run_benzinga_ipo()) as _) as Runner,
        ),
        (
            "finviz:screener",
            (|| Box::pin(run_finviz_screener()) as _) as Runner,
        ),
        (
            "finviz:stock",
            (|| Box::pin(run_finviz_stock()) as _) as Runner,
        ),
        (
            "finviz:news",
            (|| Box::pin(run_finviz_news()) as _) as Runner,
        ),
        (
            "finviz:economic",
            (|| Box::pin(run_finviz_economic()) as _) as Runner,
        ),
        (
            "finviz:earnings",
            (|| Box::pin(run_finviz_earnings()) as _) as Runner,
        ),
    ])
}

fn alpaca_client() -> alpaca_sdk::Client {
    alpaca_sdk::Client::new(&alpaca_api_key(), &alpaca_api_secret())
}

fn benzinga_client() -> benzinga_sdk::Client {
    benzinga_sdk::Client::new()
}

fn finviz_client() -> finviz_sdk::Client {
    finviz_sdk::Client::new(&finviz_auth())
}

fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

async fn run_alpaca_snapshot() -> anyhow::Result<()> {
    let response = alpaca_client()
        .api(&SnapshotQuery {
            symbol: "AAPL".to_string(),
            feed: Feed::DelayedSip,
            currency: "USD".to_string(),
        })
        .await?;
    println!("{response:#?}");
    Ok(())
}

async fn run_alpaca_quote() -> anyhow::Result<()> {
    let response = alpaca_client()
        .api(&QuoteQuery {
            symbol: "AAPL".to_string(),
            feed: Feed::DelayedSip,
            currency: "USD".to_string(),
        })
        .await?;
    println!("{response:#?}");
    Ok(())
}

async fn run_benzinga_earnings() -> anyhow::Result<()> {
    let items = benzinga_client()
        .api(&EarningsQuery {
            page_size: 10,
            date_from: date(2026, 8, 1),
            date_to: date(2026, 8, 8),
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_benzinga_economics() -> anyhow::Result<()> {
    let items = benzinga_client()
        .api(&EconomicsQuery {
            page_size: 10,
            date_from: date(2026, 8, 1),
            date_to: date(2026, 8, 8),
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_benzinga_ipo() -> anyhow::Result<()> {
    let items = benzinga_client()
        .api(&IPOQuery {
            page_size: 10,
            date_from: date(2026, 8, 1),
            date_to: date(2026, 9, 1),
            ipo_type: IPOType::SPAC,
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_finviz_screener() -> anyhow::Result<()> {
    let items = finviz_client().api(&ScreenerQuery::default()).await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_finviz_stock() -> anyhow::Result<()> {
    let items = finviz_client()
        .api(&StockQuery {
            symbol: "SPY".to_string(),
            interval: finviz_sdk::stock::Interval::Minutes5,
            valid_ranges: finviz_sdk::stock::ValidRanges::Day,
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_finviz_news() -> anyhow::Result<()> {
    let items = finviz_client()
        .api(&NewsQuery::Stocks(StocksParameter {
            symbol: vec!["SPY".to_string()],
            category: finviz_sdk::news::StocksParameterCategory::ETF,
        }))
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_finviz_economic() -> anyhow::Result<()> {
    let items = finviz_client()
        .api(&finviz_sdk::CalendarEconomicsQuery {
            date_from: date(2026, 8, 17),
            date_to: date(2026, 8, 24),
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}

async fn run_finviz_earnings() -> anyhow::Result<()> {
    let items = finviz_client()
        .api(&finviz_sdk::CalendarEarningsQuery {
            date_from: date(2026, 8, 17),
            date_to: date(2026, 8, 24),
        })
        .await?;
    println!("{items:#?}");
    Ok(())
}
