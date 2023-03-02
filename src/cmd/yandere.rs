use lazy_static::lazy_static;

use super::{DownloadArgs, DownloadType, Fetcher, TO_DEFAULT};
use crate::{async_trait, Result, Util};

lazy_static! {
    static ref URL: &'static str = "https://yande.re";
}

pub struct YandereFetcher {
    util: Util,
}

impl YandereFetcher {
    pub fn new(util: Util) -> Self {
        Self { util }
    }
}

#[async_trait]
impl Fetcher for YandereFetcher {
    async fn fetch(&self, args: DownloadArgs) -> Result<()> {
        let to_path = if let Some(to) = args.to {
            to
        } else {
            TO_DEFAULT.to_string()
        };
        match args.by {
            DownloadType::Id => match args.id {
                Some(id) => {
                    let url = format!("{}/post/show/{}", URL.to_owned(), id);
                    // self.util.download_by_show(&url, &to_path).await;
                }
                None => anyhow::bail!("please provide valid id for download"),
            },
            DownloadType::Day => todo!(),
            DownloadType::Week => todo!(),
            DownloadType::Month => todo!(),
            DownloadType::Tag => todo!(),
            DownloadType::Random => todo!(),
        }
        Ok(())
    }
}
