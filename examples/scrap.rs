#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let res = reqwest::get("https://bing.com").await?;
    let body = res.text().await?;
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("#sb_form_c").expect("fail to select element");

    for elm in document.select(&selector) {
        println!("{:?}", elm.value());
    }

    Ok(())
}
