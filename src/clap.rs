pub use clap::{Parser, ValueEnum};
use solana_sdk::commitment_config::CommitmentLevel;
use yellowstone_grpc_client::{GeyserGrpcBuilder, GeyserGrpcClient, Interceptor};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(short, long, default_value_t = String::from("http://127.0.0.1:10000"))]
    pub grpc_endpoint: String,

    #[clap(long)]
    pub commitment: Option<ArgsCommitment>,

    // 最大解码消息大小，完整的块可能会特别大，单位是B，默认值是1GB
    #[clap(long, default_value_t = 1024*1024*1024)]
    max_decoding_message_size: usize,
}
// 交易确认级别
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
}
// 为ArgsCommitment实现向CommitmentLevel的转换
impl From<ArgsCommitment> for CommitmentLevel {
    fn from(commitment: ArgsCommitment) -> Self {
        match commitment {
            ArgsCommitment::Processed => CommitmentLevel::Processed,
            ArgsCommitment::Confirmed => CommitmentLevel::Confirmed,
            ArgsCommitment::Finalized => CommitmentLevel::Finalized,
        }
    }
}

impl Args {
    // 确认交易确认级别
    fn get_commitment(&self) -> Option<CommitmentLevel> {
        Some(self.commitment.unwrap_or_default().into())
    }
    // 连接
    async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
        let mut builder = GeyserGrpcClient::build_from_shared(self.grpc_endpoint.clone())?
            .max_decoding_message_size(self.max_decoding_message_size);
        builder.connect().await.map_err(Into::into)
    }
}
