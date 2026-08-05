pub mod calendar {
    pub use {
        earnings::Query as EarningsQuery, economics::Query as EconomicsQuery,
        ipo::Query as IPOQuery,
    };

    pub mod earnings;
    pub mod economics;
    pub mod ipo;
}
pub mod client;

pub use client::Client;
