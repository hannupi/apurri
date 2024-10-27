use reqwest::blocking::Client;
use scraper::{ElementRef, Html, Selector};

pub struct Scraper {
    client: Client,
    pub keywords: Vec<String>,
}

struct ScrapedEntry {
    name: String,
    img: String,
    price: String,
}

impl Scraper {
    pub fn new(keywords: Vec<String>) -> Scraper {
        Scraper {
            client: Client::new(),
            keywords,
        }
    }
    fn create_selector(&self, selector: &str) -> Selector {
        Selector::parse(selector).unwrap()
    }

    fn select_element<'a>(
        &self,
        item: &ElementRef<'a>,
        selector: &Selector,
    ) -> Result<ElementRef<'a>, String> {
        item.select(selector)
            .next()
            .ok_or_else(|| format!("Error with selector {:?}", selector))
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
        let item_selector = self.create_selector("section > div > article");
        let title_selector = self.create_selector("h2");
        let img_selector = self.create_selector("img");
        let price_selector = self.create_selector("div > div > span");

        for item in doc.select(&item_selector) {
            let title_element = self.select_element(&item, &title_selector).unwrap();
            let img_element = self.select_element(&item, &img_selector).unwrap();
            let price_element = self.select_element(&item, &price_selector).unwrap();

            let scraped_entry = ScrapedEntry {
                name: title_element.text().collect(),
                img: img_element
                    .value()
                    .attr("src")
                    .expect("failed to get img attr")
                    .to_string(),
                price: price_element.inner_html(),
            };

            println!(
                "Name: {} \nPrice: {}\nImg: {}\n",
                scraped_entry.name, scraped_entry.price, scraped_entry.img
            );
        }
    }
}
