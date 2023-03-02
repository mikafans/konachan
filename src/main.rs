use clap::Parser;
use hypothesis::{Commands, Result, Util};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    socks: Option<String>,

    #[command(subcommand)]
    cmd: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();
    let _util = Util::new(cli.socks);

    tracing::info!("happy to help you download something");

    // util.download_to_target("https://imgproxy.nanxiongnandi.com/T8_tj5C0WvZr9lCA4GUCVIOtAUqeJJ5tk1NYzUmaiY0/w:1920/q:100/att:1/aHR0cHM6Ly9pbWcu/bmFueGlvbmduYW5k/aS5jb20vMjAyMzAy/L0F0cmFuaUFtYWxm/aS5qcGc.jpg", "sample.jpg").await?;
    Ok(())
}
