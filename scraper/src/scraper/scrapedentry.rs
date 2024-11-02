use scraper::{ElementRef, Selector};

pub struct ScrapedEntry {
    pub name: String,
    pub img: String,
    pub price: String,
    pub url: String,
}

pub fn extract_entry(item: &ElementRef) -> Result<ScrapedEntry, String> {
    let title_element = select_element(item, "h2")?;
    let img_element = select_element(item, "img")?;
    let price_element = select_element(item, "div > div > span")?;
    let url_element = select_element(item, "div > h2 > a")?;

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

fn select_element<'a>(item: &ElementRef<'a>, selector_str: &str) -> Result<ElementRef<'a>, String> {
    let selector = Selector::parse(selector_str).unwrap();
    item.select(&selector)
        .next()
        .ok_or_else(|| format!("Error with selector {:?}", selector))
}
