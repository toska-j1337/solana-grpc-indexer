//parser.rs handles protobuf data,
use helius_laserstream::grpc::{subscribe_update, SubscribeUpdate};
use bs58;

pub fn process_update(update: SubscribeUpdate) {
    if let Some (subscribe_update::UpdateOneof::Transaction(tx_update)) = update.update_oneof {
        
        if let Some(tx_info) = tx_update.transaction {
            let signature_bytes = tx_info.signature;
            let signature = bs58::encode(&signature_bytes).into_string();
            
            println!("Received Transaction: {}", signature);

            if let Some(meta) = tx_info.meta {

                println!("    Fee: {} lamports", meta.fee);

                if let Some(compute) = meta.compute_units_consumed {
                    println!("    Compute Units Consumed: {}", compute);
                }
                //Parsing token balance changes.
                if !meta.pre_token_balances.is_empty() || !meta.post_token_balances.is_empty() {

                    println!("    Token Balances Changed: ");

                    for (pre, post) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()) {
                        
                        if pre.mint == post.mint {//extracting f64 from UiTokeAmount struct in
                                                  //pre/post_amount.
                            let pre_amount = pre.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
                            let post_amount = post.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
                            let delta = post_amount - pre_amount;
                            let delta_abs = delta.abs();
                            //Abs to filter noise.
                            if delta_abs > 0.000001 {
                                println!("    Mint: {} | Owner: {} | {:.6} → {:.6} (delta: {:.6})",
                                    pre.mint,
                                    pre.owner,
                                    pre_amount,
                                    post_amount,
                                    delta);
                            }
                        }
                    }
                }
            }
        }
    }
}
