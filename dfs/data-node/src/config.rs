use clap::Parser;
use serde::Deserialize;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Parser)]
pub struct Config {
    // Address of main server
    #[arg(short, long)]
    main_server_address: String,
    /// Port
    #[arg(short, long, default_value_t = 40000)]
    port: u16,
    /// Block size
    #[arg(short, long)]
    block_size: u32,
    /// Volume of disk space to use in KB. If not set service will use all disk space
    #[arg(short, long)]
    disk_usage: Option<u128>,
}

impl Config {
    pub async fn try_from_file(path: impl AsRef<std::path::Path>) -> Self {
        Self::from_file(path).await.unwrap_or(Self::parse())
    }

    pub async fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let mut config_file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)
            .await?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer).await?;
        let config: Config = toml::from_str(&buffer)?;

        Ok(config)
    }
}

//Fields
impl Config {
    pub fn main_server_addr(&self) -> &str {
        &self.main_server_address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    pub fn disk_usage(&self) -> Option<u128> {
        self.disk_usage
    }
}
