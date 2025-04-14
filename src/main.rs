mod clap;

use core::option::Option;
use futures::join;
use jupiter_swap_api_client::quote::{QuoteRequest, QuoteResponse, SwapMode};
use jupiter_swap_api_client::{quote, ClientError, JupiterSwapApiClient};
use serde::__private::de::Content::U64;
use serde_json::Value;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use std::arch::aarch64::float64x1_t;
use std::env;
use std::io::Error;
use std::rc::Rc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Starting A8 Bot ...");
    // 加载.env变量
    dotenv::dotenv().ok();
    
}
