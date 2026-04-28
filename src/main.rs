mod config;
mod input;
mod metrics;
mod output;
mod api;

use crate::input::parser;
use crate::output::console;
use crate::api::prometheus_metrics;

use config::AppConfig;
use metrics::*;

use actix_web::{web, App, HttpResponse, HttpServer};
use helius_laserstream::{subscribe};
use futures_util::StreamExt;
use tracing_subscriber;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    let config = AppConfig::load()?;
    let mut metrics = Metrics::new();

    //Starting prometheus metrics registry
    prometheus_metrics::init_metrics();

    //Start HTTP server for metrics endpoint
    let server = HttpServer::new(|| {
        App::new()
            .route("/metrics", web::get().to(metrics_handler))
    })
    .bind("0.0.0.0:8080")?
    .run();

    //Run HTTP server in background
    tokio::spawn(server);
    println!("Metrics resource at http://192.168.50.113:8080/metrics");
    
    println!("Starting indexer..");
    
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
                        //console::print_transaction(&parsed);

                        //TODO: Add async PostgreSQL bulk inserts 
                        //      for historical data storage and analysis/time-range querying
                        
                    }
                    Err(e) => {
                        println!("Error receiving message: {:?}", e);
                    }
                }
            }
            //Connection termination below, printing summary to console with src/output/console.rs.
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C command, shutting down connection to Helius");
                console::print_summary(&metrics); 
                break;
            }
        }
    }
    
    Ok(())
}

async fn metrics_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(prometheus_metrics::get_metrics())
}
