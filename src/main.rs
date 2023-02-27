use clap::Parser;
use hypothesis::Result;

#[derive(Parser)]
struct Args {
    
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = Args::parse();
    Ok(())
}
