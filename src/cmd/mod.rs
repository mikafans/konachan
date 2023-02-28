mod konachan;
mod yandere;

use chrono::NaiveDate;

pub enum DownloadType {
    // how long (period)
    PopByDay(NaiveDate, NaiveDate),
    // PopByWeek(Instant, Instant),
    // which month
    PopByMonth(NaiveDate, NaiveDate),
    // how many pages
    ByTag(String, u32),
    ById(String),
    Random(u32),
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    #[test]
    fn test() {
        let nd = NaiveDate::parse_from_str("2015/9", "%Y/%m").expect("fail to parse time");

        println!("{}", nd.to_string());
    }
}
