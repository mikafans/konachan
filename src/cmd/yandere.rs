use std::path;

use super::{DownloadArgs, DownloadType, TO_DEFAULT};
use crate::{Result, Util};
use lazy_static::lazy_static;

lazy_static! {
    static ref ENDPOINT: &'static str = "https://yande.re";
}

pub struct YandereFetcher {
    util: Util,
}

impl YandereFetcher {
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
                    let url = format!("{}/post/show/{}", ENDPOINT.to_owned(), id);
                    let path = format!("{}{}{}", to_path, path::MAIN_SEPARATOR, id);
                    return self.util.download_by_show(&url, &path).await;
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
