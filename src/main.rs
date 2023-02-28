use clap::Parser;
use hypothesis::Result;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Args {
    
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("happy to help you download something");
    Ok(())
}
