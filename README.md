# A Property Scraper Tool Written in Rust

## For learning and demonstrattion purposes...probably.

- Using an interface
- Chose the reqwest library for HTTP Client
- Chose the soup library for scraping
- Using a generic function to select the scraper implementation e.g. Right Move
- Picking an initial implementation of right move
  - Need to supply a user agent or Right Move blocks your request (proxies will be required later).
  - To search for property you need to obtain a specific lookup code which we select the first option.
  - When there are no other suggested locations for the one you searched for the lookup code is a hidden field.
