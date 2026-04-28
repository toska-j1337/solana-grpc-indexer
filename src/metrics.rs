//metrics.rs is for updating my counters with my ParsedTransaction struct
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

use crate::api::prometheus_metrics;
use crate::parser::*;

#[derive(Debug)]
pub struct Metrics {
    //Basic TX Metrics
    pub total_transactions: AtomicU64,
    pub successful_transactions: AtomicU64,
    pub failed_transactions: AtomicU64,

    //Token Movement Metrics
    pub token_transfers_detected: AtomicU64,
    pub token_volume: HashMap<String, AtomicU64>, //<Mint, Total Moved>

    //Performance Metrics
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
            token_volume: HashMap::new(),

            //Performance Metrics
            total_compute_units: AtomicU64::new(0),
        }
    }
    
    //Record Transaction Metrics from ParsedTransaction Struct
    pub fn record_transaction(&mut self, parsed: &ParsedTransaction) {
        self.total_transactions.fetch_add(1, Ordering::Relaxed);
        prometheus_metrics::TOTAL_TRANSACTIONS.inc();

        if parsed.is_successful {
            self.successful_transactions.fetch_add(1, Ordering::Relaxed);
            prometheus_metrics::SUCCESSFUL_TRANSACTIONS.inc();
        }
        else {
            self.failed_transactions.fetch_add(1, Ordering::Relaxed);
        }
        
        //Average TPS
        prometheus_metrics::TPS
            .with_label_values(&["30s"])
            .observe(1.0);

        self.total_compute_units.fetch_add(parsed.compute_units_consumed.unwrap_or(0), Ordering::Relaxed);

        //Token Transfers
        let transfer_count = parsed.token_transfers.len() as u64;
        if transfer_count > 0 {
            self.token_transfers_detected.fetch_add(transfer_count, Ordering::Relaxed);
            prometheus_metrics::TOKEN_TRANSFERS.inc_by(transfer_count);

            for transfer in &parsed.token_transfers {
                let amount_moved = transfer.delta.abs() as u64;
                
                //Tracking SPL Tokens Here.
                self.token_volume
                    .entry(transfer.mint.clone())
                    .or_insert_with(|| AtomicU64::new(0))
                    .fetch_add(amount_moved, Ordering::Relaxed);

                self.token_transfers_detected.fetch_add(1, Ordering::Relaxed);
                
            }
        }
    }
}


