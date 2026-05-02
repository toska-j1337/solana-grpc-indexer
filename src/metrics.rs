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
    //pub total_usdc_volume: AtomicU64,
    //pub total_wsol_volume: AtomicU64,

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
            //total_usdc_volume: AtomicU64::new(0),
            //total_wsol_volume: AtomicU64::new(0),

            //Performance Metrics
            total_compute_units: AtomicU64::new(0),
        }
    }
    //Main Function To Record Metrics With ParsedTransaction Structc
    pub fn record_transaction(&mut self, parsed: &ParsedTransaction) {
        self.record_basic_transaction_metrics(parsed);
        self.record_compute_units(parsed);
        self.record_token_metrics(parsed);
    }

    //Basic Transaction Metrics: TOTAL_TRANSACTIONS and SUCCESSFUL_TRANSACTIONS
    fn record_basic_transaction_metrics(&self, parsed: &ParsedTransaction) {
        //Total Transactions
        self.total_transactions.fetch_add(1, Ordering::Relaxed);
        prometheus_metrics::TOTAL_TRANSACTIONS.inc();
        
        //Successful Transactions
        if parsed.is_successful {
            self.successful_transactions.fetch_add(1, Ordering::Relaxed);
            prometheus_metrics::SUCCESSFUL_TRANSACTIONS.inc();
        } else {
            self.failed_transactions.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    //Compute Units
    fn record_compute_units(&mut self, parsed: &ParsedTransaction) {
        if let Some(cu) = parsed.compute_units_consumed {
            self.total_compute_units.fetch_add(cu, Ordering::Relaxed);
        }
    }

    //Track Token Metrics: TOKEN_TRANSFERS, TOTAL_SOL_MOVED, 
    //TOTAL_WSOL_MOVED, TOTAL_USDC_MOVED, and TOTAL_FEES_PAID
    fn record_token_metrics(&mut self, parsed: &ParsedTransaction) {
        let transfer_count = parsed.token_transfers.len() as u64;
        if transfer_count == 0 {
            return;
        }  
        
        self.token_transfers_detected.fetch_add(transfer_count, Ordering::Relaxed);
        prometheus_metrics::TOKEN_TRANSFERS.inc_by(transfer_count);

        for transfer in &parsed.token_transfers {
            let amount = transfer.delta.abs() as u64; //as lamports
            
            //Updates Dynamic Volume For All Tokens On Network
            self.token_volume
                .entry(transfer.mint.clone())
                .or_insert_with(|| AtomicU64::new(0))
                .fetch_add(amount, Ordering::Relaxed);

            //Track Specific Tokens
            match transfer.mint.as_str() {
                "So11111111111111111111111111111111111111112" => {
                    prometheus_metrics::TOTAL_WSOL_MOVED.inc_by(amount);
                    prometheus_metrics::TOTAL_SOL_MOVED.inc_by(amount);
                }
                "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" |
                "z23BZbAiFRb6u5CBH64XjZPUud6dP6y2ZuKoYSM4LCY" |
                "BhyCjj4aLaLgQygpp9rGMjkFSafk7WAcs19dhCvKCvr4" => {
                    prometheus_metrics::TOTAL_USDC_MOVED.inc_by(amount);
                }
                _ => {}
            }            
        }
        
        //Track Native Tokens
        for native in &parsed.native_sol_transfers {
            if native.delta > 0 {
                let amount = native.delta as u64;
                //placed here instead
                prometheus_metrics::TOTAL_NATIVE_SOL_MOVED.inc_by(amount);
                prometheus_metrics::TOTAL_SOL_MOVED.inc_by(amount);
            }
        }

        prometheus_metrics::TOTAL_FEES_PAID.inc_by(parsed.fee);
        prometheus_metrics::TOTAL_SOL_MOVED.inc_by(parsed.fee);
    }
}
