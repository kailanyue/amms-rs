use alloy::providers::ProviderBuilder;
use amms::discovery::factory::{discover_factories, DiscoverableFactory};
use dotenv::dotenv;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let rpc_endpoint =
        env::var("HTTPS_URL").unwrap_or_else(|_| "https://rpc.mevblocker.io".to_string());

    // let rpc_endpoint = std::env::var("ETHEREUM_RPC_ENDPOINT")?;
    let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse()?));

    // Find all UniswapV2 and UniswapV3 compatible factories and filter out matches with less than 1000 AMMs
    let number_of_amms_threshold = 1000;
    let factories = discover_factories(
        vec![
            DiscoverableFactory::UniswapV2Factory,
            DiscoverableFactory::UniswapV3Factory,
        ],
        number_of_amms_threshold,
        provider,
        50000,
    )
    .await?;

    println!("Factories: {:?}", factories);

    Ok(())
}
