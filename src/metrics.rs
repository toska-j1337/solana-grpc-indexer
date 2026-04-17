//for my counters
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct Metrics {
    pub total_transactions: AtomicU64,
    pub successful_transactions: AtomicU64,
    pub token_transfers_detected: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            total_transactions: AtomicU64::new(0),
            successful_transactions: AtomicU64::new(0),
            token_transfers_detected: AtomicU64::new(0),
        }
    }

    pub fn record_transaction(&self, is_successful: bool, token_transfer_count: u64) {
        self.total_transactions.fetch_add(1, Ordering::Relaxed);
        if is_successful {
            self.successful_transactions.fetch_add(1, Ordering::Relaxed);
        }
        if token_transfer_count > 0 {
            self.token_transfers_detected.fetch_add(token_transfer_count, Ordering::Relaxed);
        }
    }

    pub fn print_summary(&self) {
        println!("Summary:");
        println!("    Total Transactions: {}", self.total_transactions.load(Ordering::Relaxed));
        println!("    Successful Transactions: {}", self.successful_transactions.load(Ordering::Relaxed));
        println!("    Token Transfers Detected: {}", self.token_transfers_detected.load(Ordering::Relaxed));
    }
}


