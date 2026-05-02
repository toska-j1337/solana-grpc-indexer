use prometheus::{Encoder, TextEncoder, Registry, IntCounter, HistogramVec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();
    
    //Basic TX Metrics
    pub static ref TOTAL_TRANSACTIONS: IntCounter = 
        IntCounter::new("total_transactions", "Total transactions proceed").expect("Failed to create TOTAL_TRANSACTIONS counter");

    pub static ref SUCCESSFUL_TRANSACTIONS: IntCounter =
        IntCounter::new("successful_transactions", "Successful transactions").expect("Failed to create SUCCESSFUL_TRANSACTIONS counter");

    pub static ref TOKEN_TRANSFERS: IntCounter =
        IntCounter::new("token_transfers", "Token transfers detected").expect("Failed to create TOKEN_TRANSFERS counter");

    //SOL
    
    pub static ref TOTAL_NATIVE_SOL_MOVED: IntCounter =
        IntCounter::new(
            "total_native_sol_moved",
            "Total Native SOL Moved"
        ).expect("Failed to create TOTAL_NATIVE_SOL_MOVED counter");
    
    pub static ref TOTAL_FEES_PAID: IntCounter =
        IntCounter::new(
            "total_fees_paid",
            "Total Fees Paid"
        ).expect("Failed to create TOTAL_FEES_PAID counter");

    pub static ref TOTAL_WSOL_MOVED: IntCounter =
        IntCounter::new(
            "total_wsol_moved",
            "Total Wrapped SOL Moved"
        ).expect("Failed to create TOTAL_WSOL_MOVED counter");

    pub static ref TOTAL_SOL_MOVED: IntCounter =
        IntCounter::new(
            "total_sol_moved",
            "Total SOL Moved (Lamports, wSOL, Native SOL)"
        ).expect("Failed to create TOTAL_SOL_MOVED");

    //SPL TOKEN(S)
    pub static ref TOTAL_USDC_MOVED: IntCounter =
        IntCounter::new(
            "total_usdc_moved",
            "Total usdc Moved"
        ).expect("Failed to create TOTAL_USDC_MOVED counter");
    
}

pub fn init_metrics() {

    //Basic Metrics
    REGISTRY.register(Box::new(TOTAL_TRANSACTIONS.clone())).unwrap();
    REGISTRY.register(Box::new(SUCCESSFUL_TRANSACTIONS.clone())).unwrap();
    REGISTRY.register(Box::new(TOKEN_TRANSFERS.clone())).unwrap();

    //SOL
    REGISTRY.register(Box::new(TOTAL_NATIVE_SOL_MOVED.clone())).unwrap();
    REGISTRY.register(Box::new(TOTAL_FEES_PAID.clone())).unwrap();
    REGISTRY.register(Box::new(TOTAL_WSOL_MOVED.clone())).unwrap();
    REGISTRY.register(Box::new(TOTAL_SOL_MOVED.clone())).unwrap();

    //SPL
    REGISTRY.register(Box::new(TOTAL_USDC_MOVED.clone())).unwrap();
}

pub fn get_metrics() -> String {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&REGISTRY.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
