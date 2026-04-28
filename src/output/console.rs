use crate::parser::ParsedTransaction;
use crate::metrics::Metrics;
use std::sync::atomic::Ordering;

pub fn print_transaction(parsed: &ParsedTransaction) {
    println!("    TX: {}", parsed.signature);
    println!("    Success: {}", if parsed.is_successful {"Yes"} else {"No"});
    println!("    Fee: {} lamports", parsed.fee);

    if let Some(compute) = parsed.compute_units_consumed {
        println!("    Compute Units: {}", compute);
    }

    if !parsed.token_transfers.is_empty() {
        println!("    Token Transfers ({}):", parsed.token_transfers.len());
        for transfer in &parsed.token_transfers {
            println!("    Mint: {} | Owner: {} | {:.6} ➡ {:.6} (Δ {:.6})",
                transfer.mint,
                transfer.owner,
                transfer.pre_amount,
                transfer.post_amount,
                transfer.delta);
        }
    }
    println!("{}", "-".repeat(80));
}

pub fn print_summary(metrics: &Metrics) {
    println!("\n-------INDEXER SUMMARY-------");
    println!("Total TX                    : {}", metrics.total_transactions.load(Ordering::Relaxed));
    println!("Successful TX               : {}", metrics.successful_transactions.load(Ordering::Relaxed));
    println!("Failed TX                   : {}", metrics.failed_transactions.load(Ordering::Relaxed));
    println!("Token Transfers Detected    : {}", metrics.token_transfers_detected.load(Ordering::Relaxed));
    println!("Total Compute Units         : {}", metrics.total_compute_units.load(Ordering::Relaxed));
    println!("HASHMAP OVERVIEW            : {:?}", metrics.token_volume);
    println!("-----------------------------");
}
