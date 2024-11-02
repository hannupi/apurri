use reqwest::blocking::Client;

use scraper::{Html, Selector};
pub mod scrapedentry;

pub struct Scraper {
    client: Client,
    pub keywords: Vec<String>,
}

impl Scraper {
    pub fn new(keywords: Vec<String>) -> Scraper {
        Scraper {
            client: Client::new(),
            keywords,
        }
    }

    pub fn scrape_page(&self, keyword: &str) {
        let url = format!(
            "https://www.tori.fi/recommerce/forsale/search?computeracc_type=3&product_category=2.93.3215.46&q={}&sort=PUBLISHED_DESC",
            keyword
        );
        println!("\nSearching keyword '{}' URL: {}", keyword, url);
        let res = self.client.get(url).send().expect("GET failed");
        let body = res.text().expect("response -> text failed");
        let doc = Html::parse_document(&body);
        let entry_selector =
            Selector::parse("section > div > article").expect("Failed to create entry selector");

        for item in doc.select(&entry_selector) {
            match scrapedentry::extract_entry(&item) {
                Ok(entry) => {
                    println!(
                        "Name: {} \nPrice: {}\nImg: {}\nUrl: {}\n",
                        entry.name, entry.price, entry.img, entry.url
                    );
                }
                Err(err) => eprint!("Failed to extract entry: {}", err),
            }
        }
    }
}
