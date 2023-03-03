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
        let to_path = if let Some(to) = args.location {
            to
        } else {
            TO_DEFAULT.to_string()
        };

        let prefix = ENDPOINT.to_owned();

        match args.by {
            DownloadType::Id => match args.id {
                Some(id) => {
                    let url = format!("{}/post/show/{}", &prefix, id);
                    let path = format!("{}{}{}", to_path, path::MAIN_SEPARATOR, id);
                    return self.util.download_by_show(&url, &path).await;
                }
                None => anyhow::bail!("please provide valid id for download"),
            },
            DownloadType::Day => todo!(),
            DownloadType::Week => todo!(),
            DownloadType::Month => todo!(),
            DownloadType::Tag => {
                if let (Some(tag), Some(pages)) = (args.tag, args.pages) {
                    let mut v: Vec<(String, String)> = Vec::new();
                    for i in 1..pages {
                        let url = format!("{}/post?page={}&tags={}", &prefix, i, tag);
                        let mut current = self.util.extract_hrefs(&url, &prefix).await?;
                        v.append(&mut current);
                    }
                    for (u, id) in v {
                        let path = format!("{}{}{}", to_path, path::MAIN_SEPARATOR, id);
                        self.util.download_by_show(&u, &path).await?
                    }
                    Ok(())
                } else {
                    anyhow::bail!("tag or pages not found")
                }
            }
            DownloadType::Random => match args.pages {
                Some(pages) => {
                    for _ in 1..pages {}
                    Ok(())
                }
                None => anyhow::bail!("do you really need unlimited images?"),
            },
        }
    }
}
