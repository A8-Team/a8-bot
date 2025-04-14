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
use clap::Parser;
use std::io::Error;
use std::rc::Rc;
use log::{debug, info};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();
    info!("Starting A8 Bot ...");
    // 加载.env变量
    dotenv::dotenv().ok();
    // 读取传入参数
    let mut args = clap::Args::parse();

    args.grpc_endpoint = if env::var("GRPC_ENDPOINT").is_ok() {
        env::var("GRPC_ENDPOINT").unwrap()
    } else {
        args.grpc_endpoint
    };
    debug!("GRPC_ENDPOINT: {}", args.grpc_endpoint)
}
