use benzinga_sdk::{IPOQuery, calendar::ipo::IPOType};
use finviz_sdk::{NewsQuery, ScreenerQuery, StockQuery, news::StocksParameter};

#[tokio::main]
async fn main() {
    // finviz().await;
    benzinga().await;
}

async fn benzinga() {
    let client = benzinga_sdk::Client::new();

    let ipo_result = client
        .ipo(&IPOQuery {
            page_size: 100,
            date_from: chrono::NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
            date_to: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).unwrap(),
            ipo_type: IPOType::SPAC,
        })
        .await;

    println!("{:#?}", ipo_result);
}

async fn finviz() {
    let client = finviz_sdk::Client::new("xxx");

    let screener = client.screener(&ScreenerQuery::default()).await;

    let stock_result = client
        .stock(&StockQuery {
            symbol: "SPY".to_string(),
            interval: finviz_sdk::stock::Interval::Minutes5,
            valid_ranges: finviz_sdk::stock::ValidRanges::Day,
        })
        .await;

    let news_result = client
        .news(&NewsQuery::Stocks(StocksParameter {
            symbol: vec!["SPY".to_string()],
            category: finviz_sdk::news::StocksParameterCategory::ETF,
        }))
        .await;

    println!("{:#?}", stock_result.unwrap()[0]);
    println!("{:#?}", news_result.unwrap()[0]);
    println!("{:#?}", screener.unwrap()[0]);
}
