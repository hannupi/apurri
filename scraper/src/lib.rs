use reqwest::blocking::Client;
use scraper::{Html, Selector};

pub struct Scraper {
    client: Client,
    keywords: Vec<String>,
}

impl Scraper {
    pub fn new(keywords: Vec<String>) -> Scraper {
        Scraper {
            client: Client::new(),
            keywords,
        }
    }

    pub fn scrape_page(&self, url: &str) {
        let res = self.client.get(url).send().expect("GET failed");
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
}

