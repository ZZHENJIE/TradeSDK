pub mod calendar {
    pub mod ipo;
}
pub mod client;

pub use {calendar::ipo::Query as IPOQuery, client::Client};
