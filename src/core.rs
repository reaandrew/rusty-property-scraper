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

#[derive(Clone)]
pub(crate) struct Property {
    pub price: f32,
    pub currency: String,
    pub bedrooms: i16,
    pub bathrooms: i16,
}

pub(crate) trait PropertyScraper {
    fn scrape(location: String) -> ScrapeResult<Vec<Property>>;
}


pub(crate) fn scrape<T: PropertyScraper>(location: String) -> ScrapeResult<Vec<Property>> {
    return T::scrape(location);
}