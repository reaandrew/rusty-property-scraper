# A Property Scraper Tool Written in Rust

## For fun, learning and demonstration purposes.

- Using an interface
- Chose the reqwest library for HTTP Client
- Chose the soup library for scraping
- Using a generic function to select the scraper implementation e.g. Right Move
- Picking an initial implementation of right move
  - Need to supply a user agent or Right Move blocks your request (proxies will be required later).
  - To search for property you need to obtain a specific lookup code which we select the first option.
  - When there are no other suggested locations for the one you searched for the lookup code is a hidden field.


## Ideas to extend

- [ ]  Complete the population of Property struct for Right Move - one field at a time.
- [ ]  Paginate all the results for a Right Move Search.
- [ ]  Pass the search location in from the command line using the `clap` library.
- [ ]  Support different output formats for the CLI e.g. JSON, CSV etc... - one format at a time.
- [ ]  Support using a proxy to prevent target sites blocking you e.g. https://www.scraperapi.com/.
- [ ]  Support other property websites e.g. Zoopla.
- [ ]  Support list of property websites to use as CLI argument.
- [ ]  Support common filters applicable to all supported property websites.
- [ ]  Support deduping and merging of results, attiributing information to the origin property website.
- [ ]  Change to non-blocking HTTP Client.
- [ ]  Support storing the result in database, passing configuration file as a CLI argument.
- [ ]  Support natural language queries e.g. 'find cheapest 3 bedroom detached in Preston'.