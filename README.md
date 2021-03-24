# A Property Scraper Tool Written in Rust

## For the purposes of learning and demonstrating rust

- Using an interface.
- Chose the reqwest library for HTTP Client.
- Chose the soup library for scraping.
- Using a generic function to select the scraper implementation e.g. Right Move.
- Picking an initial implementation of right move.
  - Need to supply a user agent or Right Move blocks your request (proxies will be required later).
- A route to the data which I selected:
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

## Scraping

A more specific description of this project would be:

- Non-commercial research and private study
- Text and data mining for non-commercial research

https://www.gov.uk/guidance/exceptions-to-copyright#text-and-data-mining-for-non-commercial-research