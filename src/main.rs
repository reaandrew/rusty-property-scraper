mod rightmove;
mod core;

use crate::core::{scrape};


fn main() {
    let result = scrape::<rightmove::Scraper>("wigan".into())
        .expect("error scraping");

    println!("Properties collected: {}", result.len());

    println!("Property prices: {}", result.iter()
        .map(|property| property.price.to_string())
        .collect::<Vec<String>>().join(","))
}
