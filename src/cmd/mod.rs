mod konachan;
mod yandere;

use chrono::NaiveDate;
use clap::{Subcommand, Args};

// #[derive(Subcommand)]
pub enum Commands {
    Yandere(DownloadType),
    Konanchan(DownloadType)
}

// #[derive(Args)]
pub enum DownloadType {
    // how long (period)
    PopByDay(NaiveDate, NaiveDate),
    // PopByWeek(Instant, Instant),
    // which month
    PopByMonth(u8,u8),
    // how many pages
    ByTag(String, u32),
    ById(String),
    Random(u32),
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
