use std::time::Duration;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

// should enable socks feature before-hand
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    // proxy all traffic to the passed URL
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:30808")?;
    let client = reqwest::Client::builder()
        // .default_headers(header)
        .user_agent(UA)
        .danger_accept_invalid_certs(true)
        .connection_verbose(true)
        .use_rustls_tls()
        .tcp_keepalive(Duration::from_secs(10))
        .proxy(proxy)
        .build()?;
    // let url = "https://files.yande.re/image/d5c188416d08dc1db9805167e5fd5017/yande.re%201064485%20breasts%20genshin_impact%20houkiboshi_%28mmjw7432%29%20nipples%20no_bra%20sangonomiya_kokomi%20see_through%20seifuku%20shirt_lift%20thighhighs%20valentine.jpg";
    let url = "https://youtube.com";
    let ret = client.get(url).send().await?;
    println!("{}", ret.status());
    println!("Body: {}", ret.text().await?);
    Ok(())
}
