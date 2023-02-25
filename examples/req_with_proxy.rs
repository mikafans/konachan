#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // should enable socks feature before-hand
    let proxy = reqwest::Proxy::http("socks5://127.0.0.1:30808")?;
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(header)
        .proxy(proxy)
        .build()?;

    let ret = client.get("https://youtube.com").send().await?;
    println!("{}", ret.status());
    Ok(())
}
