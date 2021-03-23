use crate::core::{ScrapeResult, Property, parse_price, PropertyScraper};
use soup::{Soup, QueryBuilderExt, NodeExt};
use reqwest::header::HeaderMap;

pub(crate) struct Scraper {}

impl Scraper {

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

        return Scraper::extract_properties(soup);
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

impl PropertyScraper for Scraper {
    fn scrape(location: String) -> ScrapeResult<Vec<Property>> {
        let user_agent_desktop = include_str!("../resources/useragent.txt");
        let mut headers = HeaderMap::new();
        headers.append("User-Agent", user_agent_desktop.parse().unwrap());
        let client = reqwest::blocking::ClientBuilder::new()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        let search_code = Scraper::get_search_code(&client, location)?;

        return Scraper::get_properties(&client, search_code);
    }
}
