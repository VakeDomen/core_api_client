use reqwest::{blocking::Client, header};

use crate::{helpers::response_handler::parse_response, responses::search::SearchResponse};

use super::{query::Query, work::Work};

pub struct Api {
    key: String,
    ratelimit_remaining: Option<i32>,
    client: Client,
}


impl Api {
    pub fn get_remainig_rate_limit(&self) -> Option<i32> {
        self.ratelimit_remaining.clone()
    }

    pub fn execute_query(&self, query: Query) -> Result<SearchResponse<Work>, crate::errors::Error> {
        let response = match self.client.get("https://api.core.ac.uk/v3/search/works/?limit=1")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.key.clone()))
            .send() {
                Ok(r) => r,
                Err(e) => return Err(crate::errors::Error::RequestError(e)),
            };
        parse_response(response)
    }
}

impl From<String> for Api {
    fn from(key: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Api { key, ratelimit_remaining: None, client }
    }
}