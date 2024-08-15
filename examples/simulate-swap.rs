use alloy::{
    primitives::{address, Address, Uint, U256},
    providers::{ProviderBuilder, RootProvider},
    transports::http::Http,
};
use amms::amm::{uniswap_v2::UniswapV2Pool, uniswap_v3::UniswapV3Pool, AutomatedMarketMaker};
use dotenv::dotenv;
use reqwest::Client;
use std::{env, sync::Arc};

const UINT_10_18: u128 = 1000000000000000000_u128;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let rpc_endpoint =
        env::var("HTTPS_URL").unwrap_or_else(|_| "https://rpc.mevblocker.io".to_string());

    let provider = Arc::new(ProviderBuilder::new().on_http(rpc_endpoint.parse()?));
    let pool_address = address!("B4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc"); // WETH/USDC
    let token_in = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

    let amount_out = simulate_swap_v2(provider.clone(), pool_address, token_in, 0.01).await?;
    println!("Amount out: {amount_out}");

    Ok(())
}

async fn simulate_swap_v2(
    provider: Arc<RootProvider<Http<Client>>>,
    pool_address: Address,
    token_in: Address,
    amount_in: f64,
) -> eyre::Result<Uint<256, 4>> {
    // Initialize the pool
    let pool = UniswapV2Pool::new_from_address(pool_address, 300, provider.clone()).await?;

    // Simulate a swap
    let amount_out = pool.simulate_swap(
        token_in,
        U256::from((UINT_10_18 as f64 * amount_in) as u128),
    )?;

    Ok(amount_out)
}

/*
let v3_address = address!("CBCdF9626bC03E24f779434178A73a0B4bad62eD");
let pool_v3 = UniswapV3Pool::new_from_address(v3_address, 300, provider.clone()).await?;
*/
