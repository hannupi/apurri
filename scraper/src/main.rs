use std::env;

mod scraper;

fn main() {
    let keywords = env::args().skip(1).collect::<Vec<String>>();
    if keywords.is_empty() {
        eprint!("Give at least one keyword as args");
        std::process::exit(1);
    }

    let scraper = scraper::Scraper::new(keywords);

    for keyword in scraper.keywords.iter() {
        scraper.scrape_page(keyword);
    }
}
