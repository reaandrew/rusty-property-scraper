use crate::core::{ScrapeResult, Property, PropertyScraper};
use soup::{Soup, QueryBuilderExt, NodeExt};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

pub(crate) struct Scraper {}

#[derive(Serialize, Deserialize)]
struct RightMovePropertyList {
    properties: Vec<RightMoveProperty>
}

impl RightMovePropertyList {
    fn to_property_list(&self) -> Vec<Property> {
        return self.properties.iter().map(|right_move_property: &RightMoveProperty|
            Property {
                price: right_move_property.price.amount,
                currency: right_move_property.price.currency_code.clone(),
                bedrooms: right_move_property.bedrooms.unwrap_or(-1),
                bathrooms: right_move_property.bathrooms.unwrap_or(-1)
            })
            .collect::<Vec<Property>>();
    }
}

#[derive(Serialize, Deserialize)]
struct RightMoveProperty {
    price: RightMovePropertyPrice,
    bedrooms: Option<i16>,
    bathrooms: Option<i16>,
}

#[derive(Serialize, Deserialize)]
struct RightMovePropertyPrice {
    amount: f32,
    #[serde(alias = "currencyCode")]
    currency_code: String,
}

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
        let scripts = soup.tag("script")
            .find_all()
            .filter(|tag| return
                tag.text().contains("jsonModel") &&
                    !tag.text().contains("propertyTypeOptions")
            )
            .collect::<Vec<_>>();

        let data = scripts[0].clone().text().replace("window.jsonModel = ", "");
        let list: RightMovePropertyList = serde_json::from_str(data.as_str())
            .expect("cannot deserialize json");


        Ok(list.to_property_list())
    }

    fn extract_result_count(soup: Soup) -> ScrapeResult<i32> {
        let result_count_element = soup.tag("span")
            .class("searchHeader-resultCount")
            .find()
            .expect("could not find result count");
        let result_count_text = result_count_element.text();
        let result_count: i32 = result_count_text.parse().unwrap_or(-1);
        Ok(result_count)
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

#[cfg(test)]
mod tests {
    use soup::{Soup};

    use crate::rightmove::{Scraper};
    use crate::core::Property;

    fn get_data<'a>() -> &'a str {
        return include_str!("../resources/rightmove/test/search_response_page.html");
    }

    fn get_soup() -> Soup {
        let soup = Soup::new(get_data());
        return soup;
    }

    fn get_property_under_test() -> Property {
        let properties = Scraper::extract_properties(get_soup())
            .expect("error extracting properties");
        let target_property = properties.to_vec().first()
            .cloned()
            .expect("cannot get first poroperty");
        return target_property;
    }

    #[test]
    fn test_get_result_count() {
        let result_count = Scraper::extract_result_count(get_soup())
            .expect("error extracting result count");
        assert_eq!(result_count, 306);
    }

    #[test]
    fn test_extract_properties() {
        let properties = Scraper::extract_properties(get_soup())
            .expect("error extracting properties");
        assert_eq!(properties.len(), 25)
    }

    #[test]
    fn test_extract_property_price() {
        assert_eq!(get_property_under_test().price, 185000 as f32);
    }

    #[test]
    fn test_extract_currency_code() {
        assert_eq!(get_property_under_test().currency, "GBP");
    }

    #[test]
    fn test_extract_property_bedrooms() {
        assert_eq!(get_property_under_test().bedrooms, 3);
    }

    #[test]
    fn test_extract_property_bathrooms(){
        assert_eq!(get_property_under_test().bathrooms, 1);
    }
}
