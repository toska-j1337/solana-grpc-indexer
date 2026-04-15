//config.rs for env, or api keys, and other config data
use dotenvy::dotenv;
use std::{env, collections::HashMap};
use helius_laserstream::{LaserstreamConfig, grpc::SubscribeRequest, grpc::SubscribeRequestFilterTransactions};

#[derive(Debug)]
pub struct AppConfig {
    pub laserstream_config: LaserstreamConfig,
    pub subscribe_request: SubscribeRequest,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let api_key = env::var("HELIUS_API_KEY")
            .expect("HELIUS_API_KEY must be set in .env file");

        let laserstream_config = LaserstreamConfig::new(
            "https://laserstream-devnet-ewr.helius-rpc.com".to_string(),
            api_key,
        );

        let mut transactions_filter = HashMap::new();
        transactions_filter.insert(
            "token-txs".to_string(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: None,
                account_include: vec![],
                account_exclude: vec![],
                account_required: vec![],
                signature: None,
            },
        );

        let subscribe_request = SubscribeRequest {
            transactions: Some(transactions_filter).expect("Error in transactions_filter"),
            ..Default::default()
        };

        Ok(AppConfig {
            laserstream_config,
            subscribe_request,
        })
    }
}
