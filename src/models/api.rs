use reqwest::{blocking::Client, header};
use serde::de::DeserializeOwned;

use crate::{helpers::response_handler::parse_response, responses::search::SearchResponse};

use super::{query::Query};

/// Main API struct. API holds your key you acquire from [CORE](https://core.ac.uk/services/api#form). 
/// Also it tracks how many uses of the API you have left based on the rate limit oposed by CORE.
/// Lastly it holds a refernce to a blocking Client it uses to execute queries to the CORE API.
/// 
#[derive(Debug)]
pub struct Api {
    key: String,
    ratelimit_remaining: Option<i32>,
    client: Client,
}




impl Api {
    
    /// Get the currently allowed remaining calls to the api. Note that the limit is acquired when making a request to the api.
    /// This means that the limit is `None` untill the first request is made.
    /// 
    /// # Example
    /// 
    /// ```
    /// let api = core_api_rs::Api::from("MY_API_KEY");
    /// let limit = api.get_remainig_rate_limit();
    /// 
    /// assert_eq!(limit, None);
    /// ```
    /// 
    pub fn get_remainig_rate_limit(&self) -> Option<i32> {
        self.ratelimit_remaining.clone()
    }

    pub fn execute_query<T, T1, T2>(
        &mut self, 
        query: Query<T1, T2>
    ) -> Result<SearchResponse<T>, crate::errors::Error> 
    where 
        T: DeserializeOwned,
        T1: ToString,
        T2: ToString,
    {

        let (req_type, query_uri) = query.parse_request();
        let uri = format!("https://api.core.ac.uk/v3/{}", query_uri);

        let client_builer = match req_type {
            super::query::QueryRequestType::Get => self.client.get(uri),
            super::query::QueryRequestType::Post => self.client.post(uri),
        };
        let response = match client_builer
            .header(header::AUTHORIZATION, format!("Bearer {}", self.key.clone()))
            .send() {
                Ok(r) => r,
                Err(e) => return Err(crate::errors::Error::RequestError(e)),
            };
        let (data, rate_limit) = parse_response::<T>(response)?;
        self.ratelimit_remaining = rate_limit;
        Ok(data)
    }
}

impl<T: Into<String>> From<T> for Api {
    fn from(key: T) -> Self {
        let client = reqwest::blocking::Client::new();
        Api { key: key.into(), ratelimit_remaining: None, client }
    }
}
