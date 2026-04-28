use prometheus::{Encoder, TextEncoder, Registry, IntCounter, HistogramVec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    pub static ref TOTAL_TRANSACTIONS: IntCounter = 
        IntCounter::new("total_transactions", "Total transactions proceed").expect("Failed to create TOTAL_TRANSACTIONS counter");

    pub static ref SUCCESSFUL_TRANSACTIONS: IntCounter =
        IntCounter::new("successful_transactions", "Successful transactions").expect("Failed to create SUCCESSFUL_TRANSACTIONS counter");

    pub static ref TOKEN_TRANSFERS: IntCounter =
        IntCounter::new("token_transfers", "Token transfers detected").expect("Failed to create TOKEN_TRANSFERS counter");

    pub static ref TPS: HistogramVec =
        HistogramVec::new(
            prometheus::opts!("tps", "Transaction per second").into(),
            &["window"]
        ).expect("Failed to create TPS counter");
}

pub fn init_metrics() {
    REGISTRY.register(Box::new(TOTAL_TRANSACTIONS.clone())).unwrap();
    REGISTRY.register(Box::new(SUCCESSFUL_TRANSACTIONS.clone())).unwrap();
    REGISTRY.register(Box::new(TOKEN_TRANSFERS.clone())).unwrap();
    REGISTRY.register(Box::new(TPS.clone())).unwrap();
}

pub fn get_metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
