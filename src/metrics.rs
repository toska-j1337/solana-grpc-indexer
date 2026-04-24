//metrics.rs is for updating my counters with my structs
use std::sync::atomic::{AtomicU64, Ordering};
use crate::parser::*;

//My Structs
//
//ParsedTransaction
//signature string
//is_successful bool
//fee u64
//compute_units_consumed Option<u64>
//token_transfers vector of TokenTransfer
//
//TokenTransfer
//mint string
//owner string
//pre_amount f64
//post_amount f64
//delta f64

#[derive(Debug)]
pub struct Metrics {
    //Basic transaction metrics
    pub total_transactions: AtomicU64,
    pub successful_transactions: AtomicU64,
    pub failed_transactions: AtomicU64,

    //Token movement metrics
    pub token_transfers_detected: AtomicU64,
    //pub total_sol_moved: AtomicU64, //in lamports
    //pub total_usdc_moved: AtomicU64,

    //Performance metrics
    pub total_compute_units: AtomicU64,
}


impl Metrics {
    pub fn new() -> Self {
        Self {
            //Basic TX Metrics
            total_transactions: AtomicU64::new(0),
            successful_transactions: AtomicU64::new(0),
            failed_transactions: AtomicU64::new(0),

            //Token Movement Metrics
            token_transfers_detected: AtomicU64::new(0),
            //total_sol_moved: AtomicU64::new(0),
            //total_usdc_moved: AtomicU64::new(0),

            //Performance Metrics
            total_compute_units: AtomicU64::new(0),
        }
    }

    pub fn record_transaction(&self, parsed: &ParsedTransaction) {
        self.total_transactions.fetch_add(1, Ordering::Relaxed);

        if parsed.is_successful {
            self.successful_transactions.fetch_add(1, Ordering::Relaxed);
        }
        else {
            self.failed_transactions.fetch_add(1, Ordering::Relaxed);
        }

        self.total_compute_units.fetch_add(parsed.compute_units_consumed.unwrap_or(0), Ordering::Relaxed);

        //Token Transfers
        let transfer_count = parsed.token_transfers.len() as u64;
        if transfer_count > 0 {
            self.token_transfers_detected.fetch_add(transfer_count, Ordering::Relaxed);

            //Add logic here to track specific token volume (native, USDC, etc)
            //for transfer in &parsed.token_transfers {
            //LOGIC FOR TRACKING MINTS HERE
            //}

        }
    }
}


