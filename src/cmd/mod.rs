mod konachan;
mod yandere;

pub use konachan::KonachanFetcher;
pub use yandere::YandereFetcher;

use crate::lazy_static;
use clap::{Args, Subcommand, ValueEnum};

// default download location
lazy_static! {
    pub(crate) static ref TO_DEFAULT: &'static str = ".";
    pub(crate) static ref TIME_FMT: &'static str = "%Y%m%d";
}

/**
 * Fetcher Trait, defines the general concept of downloading
 */

#[derive(Clone, Subcommand)]
pub enum Commands {
    // Download from Yandere
    Y(DownloadArgs),
    // Download from Konanchan
    K(DownloadArgs),
}

#[derive(Args, Clone)]
pub struct DownloadArgs {
    // by which way
    #[arg(value_enum)]
    pub by: DownloadType,
    // download to where
    #[arg(short, long)]
    pub location: Option<String>,

    // show id
    pub id: Option<String>,

    // tag
    #[arg(short, long)]
    pub tag: Option<String>,
    // download pages
    #[arg(short, long)]
    pub pages: Option<u16>,

    // start end period (yyyyMMdd)
    #[arg(short, long)]
    pub start: Option<String>,
    #[arg(short, long)]
    pub end: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DownloadType {
    // download image by Id
    Id,
    // download by daily popular
    Day,
    // download by weekly popular
    Week,
    // download by monthly popular
    Month,
    // download by specific tag
    Tag,
    // download in random style
    Random,
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};
    #[test]
    fn test() {
        let nd = NaiveDate::parse_from_str("2015/9/30", "%Y/%m/%d").expect("fail to parse time");
        nd.week(chrono::Weekday::Wed);
        let m = NaiveDate::from_ymd_opt(2023, 2, 28)
            .expect("fail to parse")
            .month();
        println!("{}, {m}", nd.to_string());
    }
}
