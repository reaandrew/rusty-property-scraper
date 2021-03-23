use std::fmt::Error;

struct Property {
}

trait PropertyScraper {
    fn scrape(location: String) -> Result<Vec<Property>, Error>;
}

struct RightMove {

}

impl PropertyScraper for RightMove {
    fn scrape(_location: String) -> Result<Vec<Property>, Error> {
        return Ok(
            vec![]
        );
    }
}

fn scrape<T: PropertyScraper>(location: String) -> Result<Vec<Property>, Error>{
    return T::scrape(location);
}

fn main() {
    let result = scrape::<RightMove>("somewhere".into())
        .expect("error scraping");
    println!("Properties collected: {}", result.len());
}
