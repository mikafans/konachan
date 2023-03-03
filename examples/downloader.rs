use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_file = "";
    let res = reqwest::get(http_file).await?;
    let mut out = File::create("sample.png").await?;
    let content = res.bytes().await?;
    out.write_all(&content).await?;
    Ok(())
}
