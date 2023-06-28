# core_api_rs - CORE API Rust Client

A Rust library for interacting with [CORE](https://core.ac.uk/services/api/), a service that provides access to metadata and full texts of research papers from thousands of data providers.

## Table of Contents

1. [Introduction](#introduction)
2. [Features](#features)
3. [Installation](#installation)
4. [Documentation](#documentation)
5. [Examples](#examples)
6. [Contributing](#contributing)
7. [License](#license)

## Introduction

The CORE API Rust Client library allows for easy interaction with the [CORE API](https://api.core.ac.uk/docs/v3#section/Welcome!), a unique service providing real-time machine access to vast quantities of metadata and full text research papers. 

## Features

- Fetch metadata of research papers
- Download full text of research papers
- Real-time access to CORE's continuously growing corpus

## Installation

To add the library to your project, add the following to your Cargo.toml:

```toml
[dependencies]
core_api_client = "0.1.0"
```
Then run `cargo build` to download and compile the library.

## Documentation

Yout can access the full documentation on here [here]()
## Examples

### Works
Executes a search on the API for works based on the query.
These are the entities that represent a piece of research, .e.g research articles, theses, etc. 
In total, it is a deduplicated and enriched version of records.

```rust
use core_api_rs::FilterOperator;
use core_api_rs::Api;

let api = Api::from("API_KEY");

let query = api.paged_search(10, 0)
   .and(FilterOperator::Exists("doi"))
   .and(FilterOperator::Bigger("citationCount", 20));

match api.search_works(query) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```


### Data Providers
Executes a search on the API for works based on the query.
It gives you access to the collection of entities that offer data to CORE. 
It contains repositories (institutional and disciplinary), preprint servers, journals and publishers.

```rust
use core_api_rs::FilterOperator;
use core_api_rs::Api;

let api = Api::from("API_KEY");

let query = api.paged_search(10, 0)
   .and(FilterOperator::Exists("software"))
   .and(FilterOperator::HasValue("type", "JOURNAL"));

match api.search_data_providers(query) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```

Fetches a specific data provider from CORE using the provided data provider identifier.
The function makes use of the CORE API's capability to fetch data provider details using their identifiers.
The identifiers can be either:
1. CORE's data provider identifier.
2. OpenDOAR's identifier, prefixed by "opendoar:" (e.g., "opendoar:123").
A call to this function executes a query against the CORE API and returns the results wrapped in an `ApiResponse`.

* `id`: Identifier of the data provider. Can be a CORE data provider identifier (integer) or an OpenDOAR identifier prefixed with "opendoar:".

```rust
use core_api_rs::FilterOperator;
use core_api_rs::Api;

let api = Api::from("API_KEY");

match api.get_data_provider(86) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};

match api.get_data_provider("opendoar:300") {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```
### Journals
Executes a search on the API for journals based on the query.
This dataset contains all journal titles included in the CORE collection. 
Moreover, you can search and retrieve any journal even if it is not a CORE data provider.

```rust
use core_api_rs::FilterOperator;
use core_api_rs::Api;

let api = Api::from("API_KEY");

let query = api.paged_search(10, 0)
    .and(FilterOperator::Eq("publisher", "OJS"));

match api.search_journals(query) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```
Fetches a single output from CORE using the provided output id.

* `id` - The Journal id in CORE. Use issn:ISSN to search by ISSN instead of the CORE identifier.

```rust
use core_api_rs::Api;

let api = Api::from("API_KEY");

match api.get_journal("issn:1179-1497") {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```

### Outputs
Executes a search on the API for otuputs (works) based on the query.
Outputs are a representation of a Work in a data provider. 
The data is not enriched and it mirrors exactly the content harvested from the data provider.

```rust
use core_api_rs::FilterOperator;
use core_api_rs::Api;

let api = Api::from("API_KEY");

let query = api.paged_search(10, 0)
    .and(FilterOperator::Eq("publisher", "OJS"));

match api.search_outputs(query) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```
Fetches a single output from CORE using the provided output id.
* `id` - The CORE ID of the output to be fetched.

```rust
use core_api_rs::Api;

let api = Api::from("API_KEY");

match api.get_output(0) {
    Ok(data) => println!("{:#?}", data),
    Err(e) => println!("{:#?}", e),
};
```


## Contributing

We would love for you to contribute to `core_api_rust_client` and help make it even better than it is today! As a contributor, here are the guidelines we would like you to follow:

### Issues and Bugs

If you find a bug in the source code or a mistake in the documentation, you can help us by submitting an issue to the [core_api_client GitHub Repository](https://github.com/VakeDomen/core_api_rs). Even better, you can submit a Pull Request with a fix.

### Feature Requests

You can also request a new feature by submitting an issue to our GitHub Repository. If you would like to implement a new feature, please submit an issue with a proposal for your work first, to be sure that we can use it.

### Pull Requests

We actively welcome your pull requests. Here's a quick guide:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

### Commit Messages

Please follow a simple rule for commit messages: Keep them short, but expressive. The first line of the commit message should be less than 50 characters and should start with a capital letter. An empty line should follow it, followed by a more detailed explanation if necessary.

## License

This project is licensed under the Creative Commons License. See the [LICENSE](LICENSE) file for details.