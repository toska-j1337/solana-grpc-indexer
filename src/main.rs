mod config;

use config::AppConfig;
use helius_laserstream::{subscribe, grpc::subscribe_update};
use futures_util::StreamExt;
use tracing_subscriber;
use tokio::signal;
use bs58;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let config = AppConfig::load()?;
    
    println!("Starting indexer..");
    
    //Opening the gRPC connection to Helius LaserStream below.
    let (stream, _handle) = subscribe(
        config.laserstream_config,
        config.subscribe_request,
    );

    println!("Connected to Helius LaserStream");
    
    //Needed for type safety.
    tokio::pin!(stream);

    //Looping over the stream of protobufs here.
    loop {
        tokio::select! { //select! for graceful termination.
            Some(message) = stream.next() => {
                match message {
                    Ok(update) => {
                        if let Some(subscribe_update::UpdateOneof::Transaction(tx_update)) = update.update_oneof {
                            
                            if let Some(tx_info) = tx_update.transaction {
                                let signature_bytes = tx_info.signature;
                                let signature = bs58::encode(&signature_bytes).into_string();

                                println!("Received Transaction: {}", signature);

                                if let Some(meta) = tx_info.meta {
                                    println!("  Fee: {} lamports", meta.fee);
                                    if let Some(compute) = meta.compute_units_consumed {
                                        println!("   Compute Units Consumed: {}", compute);
                                    }
                                    if !meta.pre_token_balances.is_empty() || !meta.post_token_balances.is_empty() {
                                        println!("    Token balances changed: ");//to get Pre/Post
                                                                                 //token account
                                                                                 //balances
                                        for (pre, post) in meta.pre_token_balances.iter().zip(meta.post_token_balances.iter()) {
                                            if pre.mint == post.mint && pre.ui_token_amount != post.ui_token_amount {
                                                println!("    Mint: {} | Change: {:?} -> {:?}",
                                                    pre.mint,
                                                    pre.ui_token_amount,
                                                    post.ui_token_amount);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                       // println!("Received tx protobuf; {:?}", update);
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                    }
                }
            }
            //Connection termination below
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C command, shutting down connection to Helius");
                break;
            }
        }
    }
    
    Ok(())
}
