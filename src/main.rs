mod config;
mod input;
mod metrics;
mod output;

use crate::input::parser;
use crate::output::console;
use config::AppConfig;
use metrics::*;
use helius_laserstream::{subscribe};
use futures_util::StreamExt;
use tracing_subscriber;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let config = AppConfig::load()?;
    let metrics = Metrics::new();
    
    println!("Starting indexer..");
   
    //Remember to move this to input/stream.rs later.
    //Opening the gRPC connection to Helius LaserStream below.
    let (stream, _handle) = subscribe(
        config.laserstream_config,
        config.subscribe_request,
    );

    println!("Connected to Helius LaserStream");
    
    tokio::pin!(stream);

    //Looping over the stream of protobufs here.
    loop {
        tokio::select! { //select! for graceful termination.
            Some(message) = stream.next() => {
                match message {
                    Ok(update) => {
                        
                        let parsed = parser::parse_update(update);

                        metrics.record_transaction(&parsed);
                        console::print_transaction(&parsed);
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                    }
                }
            }
            //Connection termination below, printing summary to console.
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C command, shutting down connection to Helius");
                //metrics.print_summary();
                console::print_summary(&metrics); 
                break;
            }
        }
    }
    
    Ok(())
}
