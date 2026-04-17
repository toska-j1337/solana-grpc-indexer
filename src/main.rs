mod config;
mod input;

use crate::input::parser;
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
                        //Parsing input below.
                        parser::process_update(update);
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
