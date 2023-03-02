use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use super::{DownloadArgs, DownloadType, Fetcher, TO_DEFAULT};
use crate::{async_trait, Result, Util};

lazy_static! {
    static ref URL: &'static str = "https://yande.re";
}

struct YandereFetcher {
    util: Arc<Mutex<Util>>,
}

#[async_trait]
impl Fetcher for YandereFetcher {
    async fn fetch(args: DownloadArgs) -> Result<()> {
        let to_path = if let Some(to) = args.to {
            to
        } else {
            TO_DEFAULT.to_string()
        };
        match args.by {
            DownloadType::Id => match args.id {
                Some(id) => {
                    let url = format!("{}/post/show/{}", URL.to_owned(), id);
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
