use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    time::Duration,
};

use crate::Result;

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

pub struct Util {
    client: reqwest::Client,
}

impl Util {
    pub fn new() -> Self {
        let proxy = reqwest::Proxy::all("socks5://127.0.0.1:30808").expect("fail to init proxy");
        let client = reqwest::Client::builder()
            .connection_verbose(true)
            // in case big files
            .connect_timeout(Duration::from_secs(10))
            .user_agent(UA)
            // escape some mysterious tls problem
            .danger_accept_invalid_certs(true)
            .use_rustls_tls()
            // .tcp_keepalive(Duration::from_secs(10))
            .proxy(proxy)
            .build()
            .expect("fail to build req client");

        Self { client }
    }

    pub async fn download_to_target(&self, url: &str, path: &str) -> Result<()> {
        let res = self.client.get(url).send().await?;
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
}

#[cfg(test)]
mod tests {

    use super::Util;

    #[test]
    fn test() {
        const URL:&str = "https://files.yande.re/image/d5c188416d08dc1db9805167e5fd5017/yande.re%201064485%20breasts%20genshin_impact%20houkiboshi_%28mmjw7432%29%20nipples%20no_bra%20sangonomiya_kokomi%20see_through%20seifuku%20shirt_lift%20thighhighs%20valentine.jpg";
        let path = "sample.jpg";
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("fail to build rt");
        let util = Util::new();

        rt.block_on(util.download_to_target(URL, path))
            .expect("fail to exec task");
    }
}
