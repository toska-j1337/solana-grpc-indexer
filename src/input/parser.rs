//parser.rs parses protobuf data,
use helius_laserstream::grpc::{subscribe_update, SubscribeUpdate, SubscribeUpdateTransactionInfo};
use helius_laserstream::solana::storage::confirmed_block::TransactionStatusMeta;
use bs58;

pub struct ParsedTransaction {
    pub signature: String,
    pub is_successful: bool,
    pub fee: u64,
    pub compute_units_consumed: Option<u64>,
    pub token_transfers: Vec<TokenTransfer>,
    pub native_sol_transfers: Vec<NativeSolTransfer>,
}

pub struct NativeSolTransfer {
    pub account: String,
    pub pre_amount: u64,
    pub post_amount: u64,
    pub delta: i64,
}

pub struct TokenTransfer { 
    pub mint: String,
    pub owner: String,
    pub pre_amount: f64,
    pub post_amount: f64,
    pub delta: f64,
}

pub fn parse_update(update: SubscribeUpdate) -> ParsedTransaction {
    let mut is_successful = false;
    let mut fee = 0u64;
    let mut compute_units_consumed = None;
    let mut signature = String::new();
    let mut token_transfers = Vec::new();
    let mut native_sol_transfers = Vec::new();

    if let Some(subscribe_update::UpdateOneof::Transaction(tx_update)) = update.update_oneof {
        if let Some(tx_info) = tx_update.transaction {
            signature = extract_signature(&tx_info.signature);

            if let Some(meta) = &tx_info.meta {
                is_successful = meta.err.is_none();

                fee = meta.fee;

                compute_units_consumed = meta.compute_units_consumed;

                token_transfers = parse_token_balances(meta);

                native_sol_transfers = parse_native_sol_balances(meta, &tx_info);
            }
        }
    }
    //Returning
    ParsedTransaction {
        signature,
        is_successful,
        fee,
        compute_units_consumed,
        token_transfers,
        native_sol_transfers,
    }
}


//Extract b58 signature from byte array
fn extract_signature(signature_bytes: &[u8]) -> String {
    bs58::encode(signature_bytes).into_string()
}


//Parse token balances from metadata
fn parse_token_balances(meta: &TransactionStatusMeta) -> Vec<TokenTransfer> {

    let mut transfers = Vec::new();

    for (pre, post) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()) {

        if pre.mint == post.mint {
            let pre_amount = pre.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
            let post_amount = post.ui_token_amount.as_ref().map(|x| x.ui_amount).unwrap_or(0.0);
            let delta = post_amount - pre_amount;
            let delta_abs = delta.abs();

            if delta_abs > 0.000001 {
                transfers.push(TokenTransfer {
                    mint: pre.mint.clone(),
                    owner: pre.owner.clone(),
                    pre_amount,
                    post_amount,
                    delta,
                });
            }
        }
    }
    transfers
}


fn parse_native_sol_balances(meta: &TransactionStatusMeta, tx_info: &SubscribeUpdateTransactionInfo) -> Vec<NativeSolTransfer> {
    let mut transfers = Vec::new();

    //pre_balances and post balances are Vec<u64>
    for (i, (pre, post)) in meta.pre_balances.iter().zip(meta.post_balances.iter()).enumerate() {
        let delta = *post as i64 - *pre as i64;

        if delta != 0 {
            //Account Address
            let account = if let Some (keys) = &tx_info.transaction.as_ref().and_then(|x| x.message.as_ref()) {
                if let Some(key) = keys.account_keys.get(i) {
                    bs58::encode(key).into_string()
                } else {
                    format!("account_{}", i)
                }
            } else {
                format!("account_{}", i)
            };

            transfers.push(NativeSolTransfer {
                account,
                pre_amount: *pre,
                post_amount: *post,
                delta,
            });
        }
    }
    transfers
}   
