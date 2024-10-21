use anyhow::Result;
use heart_beater_rust::cli;

#[tokio::main]
async fn main() -> Result<()> {
    cli::main().await
}
