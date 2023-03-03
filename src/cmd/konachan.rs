use lazy_static::lazy_static;

use super::{DownloadArgs, DownloadType, TO_DEFAULT};
use crate::{Result, Util};

lazy_static! {
    static ref URL: &'static str = "https://konachan.com";
}

pub struct KonachanFetcher {
    util: Util,
}

impl KonachanFetcher {
    pub fn new(util: Util) -> Self {
        Self { util }
    }
    pub async fn fetch(&self, args: DownloadArgs) -> Result<()> {
        let to_path = if let Some(to) = args.to {
            to
        } else {
            TO_DEFAULT.to_string()
        };
        match args.by {
            DownloadType::Id => match args.id {
                Some(id) => {
                    let url = format!("{}/post/show/{}", URL.to_owned(), id);
                    return self.util.download_by_show(&url, &to_path).await;
                }
                None => anyhow::bail!("please provide valid id for download"),
            },
            DownloadType::Day => todo!(),
            DownloadType::Week => todo!(),
            DownloadType::Month => todo!(),
            DownloadType::Tag => todo!(),
            DownloadType::Random => todo!(),
        }
    }
}
