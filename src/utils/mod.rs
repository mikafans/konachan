use crate::Result;
use scraper::{Html, Selector};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    time::Duration,
};

const UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36";

pub struct Util {
    client: reqwest::Client,
}

impl Util {
    pub fn new(socks5: Option<String>) -> Self {
        let mut builder = reqwest::Client::builder()
            // .connection_verbose(true)
            // in case big files
            .connect_timeout(Duration::from_secs(10))
            .user_agent(UA)
            .use_rustls_tls()
            // escape some mysterious tls problem
            .danger_accept_invalid_certs(true);

        if let Some(socks5) = socks5 {
            let socks_url = format!("socks5h://{socks5}");
            let proxy = reqwest::Proxy::all(socks_url).expect("fail to init proxy");
            builder = builder.proxy(proxy);
        }

        let client = builder.build().expect("fail to build req client");

        Self { client }
    }

    pub async fn download_by_show(&self, url: &str, file_path: &str) -> Result<()> {
        let body = self.client.get(url).send().await?.text().await?;
        let doc = Html::parse_document(&body);
        let png_selector = Selector::parse("#png").expect("fail to parse selecor");
        let highres_selector = Selector::parse("#highres").expect("fail to parse selecor");

        // try to download png first
        if let Some(png) = doc.select(&png_selector).next() {
            if let Some(file_url) = png.value().attr("href") {
                self.download_to_target(file_url, file_path).await?
            }
        } else if let Some(jpg) = doc.select(&highres_selector).next() {
            if let Some(file_url) = jpg.value().attr("href") {
                self.download_to_target(file_url, file_path).await?
            }
        } else {
            anyhow::bail!("fail to find download entry")
        }

        Ok(())
    }

    async fn download_to_target(&self, url: &str, file_path: &str) -> Result<()> {
        let res = self.client.get(url).send().await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let f = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(file_path)?;
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
        let util = Util::new(Some("127.0.0.1:30808".to_owned()));

        rt.block_on(util.download_to_target(URL, path))
            .expect("fail to exec task");
    }

    #[test]
    fn test_show() {
        // let show_url = "https://yande.re/post/show/1064594";
        let show_url = "https://konachan.com/post/show/353306";
        let path = "sample1.png";
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("fail to build rt");
        let util = Util::new(Some("127.0.0.1:30808".to_owned()));

        rt.block_on(util.download_by_show(show_url, path))
            .expect("fail to exec task");
    }
}
