//parser.rs handles protobuf data,
use helius_laserstream::grpc::{subscribe_update, SubscribeUpdate};
use helius_laserstream::solana::storage::confirmed_block::TransactionStatusMeta;
use bs58;

pub fn process_update(update: SubscribeUpdate) -> (bool, u64) {
    let mut is_successful = false;
    let mut token_transfer_count = 0;

    if let Some(subscribe_update::UpdateOneof::Transaction(tx_update)) = update.update_oneof {
        if let Some(tx_info) = tx_update.transaction {
            let signature = extract_signature(&tx_info.signature);
            println!("Received Transaction: {}", signature);

            if let Some(meta) = &tx_info.meta {
                is_successful = meta.err.is_none();

                print_transaction_metadata(meta);

                //Count and print token balances
                token_transfer_count = print_and_count_token_changes(meta);
            }
        }
    }
    //Returning
    (is_successful, token_transfer_count)
}




//Extract b58 signature from byte array
fn extract_signature(signature_bytes: &[u8]) -> String {
    bs58::encode(signature_bytes).into_string()
}

//Prints fee and compute units
fn print_transaction_metadata(meta: &TransactionStatusMeta) {
    println!("    Fee: {} lamports", meta.fee);

    if let Some(compute) = meta.compute_units_consumed {
        println!("    Compute Units Consumed: {}", compute);
    }
}

//Prints token balance changes and returns count of meaningful transfers
fn print_and_count_token_changes(meta: &TransactionStatusMeta) -> u64 {
    let mut transfer_count = 0;

    if meta.pre_token_balances.is_empty() && meta.post_token_balances.is_empty() {
        return 0;
    }

    println!("    Token Balances Changed:");

    for (pre, post) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()) {
        if pre.mint == post.mint {
            let pre_amount = pre.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
            let post_amount = post.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
            let delta = post_amount - pre_amount;
            let delta_abs = delta.abs();

            if delta_abs > 0.000001 {
                println!("    Mint: {} | Owner: {} | {:6} ➡ {:6} (delta: {:6})",
                    pre.mint,
                    pre.owner,
                    pre_amount,
                    post_amount,
                    delta);

                transfer_count += 1;
            }  
        }
    }
    transfer_count
}
