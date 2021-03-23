use soup::{Soup, QueryBuilderExt, NodeExt};
use reqwest::header::{HeaderMap};

#[derive(Debug)]
enum ScrapeError {
    Parse(reqwest::Error)
}

impl From<reqwest::Error> for ScrapeError {
    fn from(err: reqwest::Error) -> Self {
        return ScrapeError::Parse(err);
    }
}

type ScrapeResult<T> = std::result::Result<T, ScrapeError>;

struct Property { price: f32 }

trait PropertyScraper {
    fn scrape(location: String) -> ScrapeResult<Vec<Property>>;
}

fn parse_price(input: String) -> f32 {
    let result = input
        .replace("£", "")
        .replace(",", "")
        .trim().parse().unwrap_or(-1 as f32);

    return result;
}

struct RightMove {}

impl RightMove {

    fn get_search_code(client: &reqwest::blocking::Client, location: String)
        -> ScrapeResult<String> {
        let search_code_url = include_str!("../resources/rightmove/search_code_url.txt")
            .replace("{}", location.as_str());

        let response_text = client
            .get(search_code_url)
            .send()?.text()?;

        let soup = Soup::new(response_text.as_str());
        let recommended_locations = soup.tag("select")
            .attr("id", "locationIdentifier")
            .find()
            .expect("Couldn't find element with an 'id'");
        let first_recomended_location =
            Soup::new(recommended_locations.display().as_str())
            .tag("option")
            .find()
            .expect("Cannot find an option in the select box");
        let search_value = first_recomended_location.get("value");
        Ok(search_value.unwrap())
    }

    fn get_properties(client: &reqwest::blocking::Client, search_code: String) -> ScrapeResult<Vec<Property>> {
        let url = include_str!("../resources/rightmove/search_url.txt")
            .replace("{}", search_code.as_str());
        let response_text = client
            .get(url)
            .send()?.text()?;

        let soup = Soup::new(response_text.as_str());

        return RightMove::extract_properties(soup);
    }

    fn extract_properties(soup: Soup) -> ScrapeResult<Vec<Property>> {
        let properties = soup.tag("div")
            .class("l-searchResult")
            .class("is-list")
            .find_all()
            .map(|tag| {
                let inner_soup = Soup::new(tag.display().as_str())
                    .tag("div")
                    .class("propertyCard-priceValue")
                    .find()
                    .expect("Can't find the price for the property");
                return Property {
                    price: parse_price(inner_soup.text())
                };
            })
            .collect::<Vec<Property>>();
        Ok(properties)
    }
}

impl PropertyScraper for RightMove {
    fn scrape(location: String) -> ScrapeResult<Vec<Property>> {
        let user_agent_desktop = include_str!("../resources/useragent.txt");
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", user_agent_desktop.parse().unwrap());
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        let search_code = RightMove::get_search_code(&client, location)?;

        return RightMove::get_properties(&client, search_code);
    }
}


fn scrape<T: PropertyScraper>(location: String) -> ScrapeResult<Vec<Property>> {
    return T::scrape(location);
}

fn main() {
    let result = scrape::<RightMove>("wigan".into())
        .expect("error scraping");
    println!("Properties collected: {}", result.len());
    println!("Property prices: {}", result.iter()
        .map(|property| property.price.to_string())
        .collect::<Vec<String>>().join(","))
}

#[cfg(test)]
mod tests {
    use crate::parse_price;

    #[test]
    fn test_must_parse_price() {
        let input = "       £160,000";
        let expected = 160000 as f32;
        let result = parse_price(input.into());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_must_return_negative_1_when_unable_to_parse() {
        let input = "       £160,,,,000 isuhauhdiuahda";
        let expected = -1 as f32;
        let result = parse_price(input.into());
        assert_eq!(result, expected);
    }
}

impl RightMove {

}

impl RightMove {

}
