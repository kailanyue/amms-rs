use alloy::{primitives::address, providers::ProviderBuilder};
use dotenv::dotenv;
use std::{env, sync::Arc};

use amms::{
    amm::{
        factory::Factory, uniswap_v2::factory::UniswapV2Factory,
        uniswap_v3::factory::UniswapV3Factory,
    },
    sync,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let rpc_endpoint =
        env::var("HTTPS_URL").unwrap_or_else(|_| "https://rpc.mevblocker.io".to_string());

    // Add rpc endpoint here:
    // let rpc_endpoint = env::var("ETHEREUM_RPC_ENDPOINT")?;
    let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse()?));

    let factories = vec![
        // Add UniswapV2
        Factory::UniswapV2Factory(UniswapV2Factory::new(
            address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"),
            2638438,
            300,
        )),
        // Add Sushiswap
        Factory::UniswapV2Factory(UniswapV2Factory::new(
            address!("C0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac"),
            10794229,
            300,
        )),
        // Add UniswapV3
        Factory::UniswapV3Factory(UniswapV3Factory::new(
            address!("1F98431c8aD98523631AE4a59f267346ea31F984"),
            185,
        )),
    ];

    // Sync pairs
    let (amms, _last_synced_block) = sync::sync_amms(factories, provider, None, 500).await?;

    println!("AMMs: {:?}", amms);
    Ok(())
}
