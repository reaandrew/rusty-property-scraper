#[derive(Debug)]
pub enum ScrapeError {
    Parse(reqwest::Error)
}

impl From<reqwest::Error> for ScrapeError {
    fn from(err: reqwest::Error) -> Self {
        return ScrapeError::Parse(err);
    }
}

pub(crate) type ScrapeResult<T> = std::result::Result<T, ScrapeError>;

pub(crate) struct Property {
    pub price: f32
}

pub(crate) trait PropertyScraper {
    fn scrape(location: String) -> ScrapeResult<Vec<Property>>;
}

pub(crate) fn parse_price(input: String) -> f32 {
    let result = input
        .replace("£", "")
        .replace(",", "")
        .trim().parse().unwrap_or(-1 as f32);

    return result;
}


pub(crate) fn scrape<T: PropertyScraper>(location: String) -> ScrapeResult<Vec<Property>> {
    return T::scrape(location);
}

#[cfg(test)]
mod tests {
    use crate::core::parse_price;

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