use std::env;

use apurri_scraper::Scraper;

fn main() -> () {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.is_empty() {
        eprint!("Give at least one keyword as args");
        std::process::exit(1);
    }

    let scraper = Scraper::new(args);

    for keyword in scraper.keywords.iter() {
        scraper.scrape_page(keyword);
    }
}
