mod config;

use config::AppConfig;
use helius_laserstream::{subscribe};
use futures_util::StreamExt;
use tracing_subscriber;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let config = AppConfig::load()?;
    
    println!("Starting indexer..");

    let (stream, _handle) = subscribe(
        config.laserstream_config,
        config.subscribe_request,
    );

    println!("Connected to Helius LaserStream");

    tokio::pin!(stream);

    loop {
        tokio::select! {
            Some(message) = stream.next() => {
                match message {
                    Ok(update) => {
                        println!("Received tx protobuf; {:?}", update);
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                    }
                }
            }
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C command, shutting down connection to Helius");
                break;
            }
        }
    }


    /*
    while let Some(message) = stream.next().await {
        match message {
            Ok(update) => {
                println!("Received tx: {:?}", update);
            }
            Err(e) => {
                println!("Error receiving message: {}", e);
            }
        }
    }
    */
    
    Ok(())
}
