use scraper::{ElementRef, Selector};

pub struct ScrapedEntry {
    pub name: String,
    pub img: String,
    pub price: String,
    pub url: String,
}

pub fn extract_entry(item: &ElementRef) -> Result<ScrapedEntry, String> {
    let title_selector = create_selector("h2");
    let img_selector = create_selector("img");
    let price_selector = create_selector("div > div > span");
    let url_selector = create_selector("div > h2 > a");

    let title_element = select_element(item, &title_selector).unwrap();
    let img_element = select_element(item, &img_selector).unwrap();
    let price_element = select_element(item, &price_selector).unwrap();
    let url_element = select_element(item, &url_selector).unwrap();

    let scraped_entry = ScrapedEntry {
        name: title_element.text().collect(),
        img: img_element
            .value()
            .attr("src")
            .expect("failed to get img attr")
            .to_string(),
        price: price_element.inner_html(),
        url: url_element
            .value()
            .attr("href")
            .expect("failed to get item entry URL")
            .to_string(),
    };
    Ok(scraped_entry)
}

fn create_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

fn select_element<'a>(
    item: &ElementRef<'a>,
    selector: &Selector,
) -> Result<ElementRef<'a>, String> {
    item.select(selector)
        .next()
        .ok_or_else(|| format!("Error with selector {:?}", selector))
}
