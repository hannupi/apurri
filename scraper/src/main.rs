use std::env;

use apurri_scraper::Scraper;

fn main() -> () {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let scraper = Scraper::new(args);

    // url for keebs
    let url = "https://www.tori.fi/recommerce/forsale/search?computeracc_type=3&product_category=2.93.3215.46&sort=PUBLISHED_DESC";
    scraper.scrape_page(url);
}
