use reqwest::blocking::Client;
use scraper::{Html, Selector};

fn main() -> () {
    let client = Client::new();
    // url for keebs
    let url = "https://www.tori.fi/recommerce/forsale/search?computeracc_type=3&product_category=2.93.3215.46&sort=PUBLISHED_DESC";
    let res = client.get(url).send().expect("GET failed");
    let body = res.text().expect("response -> text failed");

    let doc = Html::parse_document(&body);
    let item_selector = Selector::parse("section > div > article").unwrap();
    let title_selector = Selector::parse("h2").unwrap();

    for item in doc.select(&item_selector) {
        if let Some(title_element) = item.select(&title_selector).next() {
            let title: String = title_element.text().collect();
            println!("{}", title);
        }
    }
}
