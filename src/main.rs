mod clap;

use {
    crate::clap::Action,
    backoff::{future::retry, Error as BackoffError, ExponentialBackoff},
    clap::Parser,
    futures::future::TryFutureExt,
    log::{debug, error, info},
    std::{env, sync::Arc},
    tokio::{fs, sync::Mutex},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();
    info!("Starting A8 bot...");
    // 加载.env变量
    dotenv::dotenv().ok();
    // 读取传入参数
    let mut args = clap::Args::parse();

    args.grpc_endpoint = if env::var("GRPC_ENDPOINT").is_ok() {
        env::var("GRPC_ENDPOINT").unwrap()
    } else {
        args.grpc_endpoint
    };
    debug!("GRPC_ENDPOINT: {}", args.grpc_endpoint);
    let zero_attempts = Arc::new(Mutex::new(true));

    // The default exponential backoff strategy intervals:
    // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    retry(ExponentialBackoff::default(), move || {
        let args = args.clone();
        let zero_attempts = Arc::clone(&zero_attempts);

        async move {
            let mut zero_attempts = zero_attempts.lock().await;
            if *zero_attempts {
                *zero_attempts = false;
            } else {
                info!("Retrying...")
            }
            drop(zero_attempts);
            let commitment = args.get_commitment();
            info!("Attempting to connect to gRPC...");
            let mut client = args.connect().await.map_err(backoff::Error::transient)?;
            let _ = client.ping(1).await.map_err(anyhow::Error::new)?;
            info!("Connected.");

            match &args.action {
                Action::Ping { count } => client
                    .ping(*count)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("Ping: {response:?}")),
            }
            .map_err(backoff::Error::transient)?;

            Ok::<(), backoff::Error<anyhow::Error>>(())
        }
        .inspect_err(|error| error!("Connection failed: {error}"))
    })
    .await
    .map_err(Into::into)
}
