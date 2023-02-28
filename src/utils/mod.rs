use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    time::Duration,
};

use crate::Result;
use lazy_static::lazy_static;
const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

lazy_static! {
    static ref REQ_CLIENT: reqwest::Client = reqwest::Client::builder()
    .user_agent(UA)
    .use_rustls_tls()
    // escape some mysterious tls problem
    .danger_accept_invalid_certs(true)
    // in case big files
    .tcp_keepalive(Duration::from_secs(10))
    .proxy(reqwest::Proxy::all("socks5://172.0.0.1:30808").expect("fail to init proxy"))
    .build()
    .expect("fail to build req client");
}

pub async fn download_to_target(url: &str, path: &str) -> Result<()> {
    let res = REQ_CLIENT.get(url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => {
            let f = OpenOptions::new().create(true).write(true).open(path)?;
            let mut w_buf = BufWriter::new(&f);
            let data = res.bytes().await?;
            w_buf.write_all(&data)?;
            Ok(())
        }
        _ => {
            anyhow::bail!("fail to fetch data from web")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::download_to_target;

    #[test]
    fn test() {
        const URL:&str = "https://files.yande.re/image/d5c188416d08dc1db9805167e5fd5017/yande.re%201064485%20breasts%20genshin_impact%20houkiboshi_%28mmjw7432%29%20nipples%20no_bra%20sangonomiya_kokomi%20see_through%20seifuku%20shirt_lift%20thighhighs%20valentine.jpg";
        let path = "sample.jpg";
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("fail to build rt");

        rt.block_on(download_to_target(URL, path))
            .expect("fail to exec task");
    }
}
