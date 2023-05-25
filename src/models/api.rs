use reqwest::{blocking::Client, header, blocking::Response};

use crate::{helpers::response_handler::parse_response, responses::search::SearchResponse};

use super::{query::Query, work::Work};

#[derive(Debug)]
pub struct Api {
    key: String,
    ratelimit_remaining: Option<i32>,
    client: Client,
}


impl Api {
    pub fn get_remainig_rate_limit(&self) -> Option<i32> {
        self.ratelimit_remaining.clone()
    }

    pub fn execute_query(&mut self, query: Query) -> Result<SearchResponse<Work>, crate::errors::Error> {
        let response = match self.client.get("https://api.core.ac.uk/v3/search/works/?limit=100")
            .header(header::AUTHORIZATION, format!("Bearer {}", self.key.clone()))
            .send() {
                Ok(r) => r,
                Err(e) => return Err(crate::errors::Error::RequestError(e)),
            };
        self.ratelimit_remaining = extraxt_rate_limit(&response);
        parse_response(response)
    }
}

impl From<String> for Api {
    fn from(key: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Api { key, ratelimit_remaining: None, client }
    }
}


fn extraxt_rate_limit(resp: &Response) -> Option<i32> {
    if let Some(rate) = resp.headers().get("x-ratelimit-remaining") {
        let rate_str = match rate.to_str() {
            Ok(s) => s,
            Err(_) => return None,
        };
        return rate_str.parse::<i32>().ok();
    }
    None
}