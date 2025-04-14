use std::env;
use std::time::Duration;
use futures::join;
use jupiter_swap_api_client::{ClientError, JupiterSwapApiClient};
use jupiter_swap_api_client::quote::{QuoteRequest, QuoteResponse, SwapMode};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use tokio::time::sleep;
use crate::{SOL_MINT, USDC_MINT};

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
// 套利
async fn arbitrate() {
    let jup_api_url = env::var("JUPITER_API_URL").expect("JUPITER_API_URL Not Define！");
    let solana_rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL Not Defined!");
    let wall_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS Not Defined!");
    // 获取报价
    let jupiter_client = JupiterSwapApiClient::new(jup_api_url);

    loop {
        let (res1, res2) =
            get_arbitrage_opportunity(jupiter_client.clone(), SOL_MINT, USDC_MINT, 5000000000)
                .await;

        if res1.is_ok() && res2.is_ok() {
            let q1 = res1.unwrap();
            let q2 = res2.unwrap();
            println!("计算套利空间:");
            println!("Q1:{},{}", q1.in_amount, q1.out_amount);
            println!("Q2:{},{}", q2.in_amount, q2.out_amount);
            let profit = q1.out_amount as f64 / q2.in_amount as f64 * q2.out_amount as f64
                - q1.in_amount as f64;
            println!("套利空间: {}", profit);
            if profit > 150000.0 {
                println!("套利空间: {}", profit)
            } else {
                println!("不存在套利空间！")
            }
        } else {
            println!("请求失败！");
        }
        println!("-------------------------");
        sleep(Duration::from_secs(1)).await;
    }
}
// 获取套利机会
async fn get_arbitrage_opportunity(
    client: JupiterSwapApiClient,
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
) -> (
    Result<QuoteResponse, ClientError>,
    Result<QuoteResponse, ClientError>,
) {
    let (q1, q2) = join!(
        get_quote(
            &client,
            input_mint,
            output_mint,
            amount,
            Option::from(SwapMode::ExactIn)
        ),
        get_quote(
            &client,
            output_mint,
            input_mint,
            amount,
            Option::from(SwapMode::ExactOut)
        )
    );
    return (q1, q2);
}
// 获取报价
async fn get_quote(
    client: &JupiterSwapApiClient,
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: u64,
    swap_mode: Option<SwapMode>,
) -> Result<QuoteResponse, ClientError> {
    let quote_request = QuoteRequest {
        input_mint,
        output_mint,
        amount,
        swap_mode,
        slippage_bps: 0,
        ..QuoteRequest::default()
    };
    client.quote(&quote_request).await
}
