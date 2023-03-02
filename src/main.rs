use clap::Parser;
use hypothesis::{
    cmd::{Fetcher, YandereFetcher, KonachanFetcher},
    Commands, Result, Util,
};
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

    tracing::info!("happy to help you download something");

let util= Util::new(cli.socks);
    match cli.cmd {
        Commands::Yandere(args) => {
            let yf = YandereFetcher::new(util);
            yf.fetch(args).await?;
        }
        Commands::Konanchan(args) => {
            let cf = KonachanFetcher::new(util);
            cf.fetch(args).await?;
        }
    }
    Ok(())
}
