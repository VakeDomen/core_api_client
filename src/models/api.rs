use reqwest::{blocking::Client, header};
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use crate::{
    helpers::response_handler::{parse_raw_response}, 
    responses::{response::ApiResponse, responses::ApiResponseType}, 
    SearchQuery, errors::Error
};
use super::query::Query;

/// Main API struct. API holds your key you acquire from [CORE](https://core.ac.uk/services/api#form). 
/// Lastly it holds a refernce to a blocking Client it uses to execute queries to the CORE API.
/// 
#[derive(Debug)]
pub struct Api {
    key: String,
    client: Client,
}


impl Api {
    pub fn execute_query<T1, T2>(
        &self, 
        query: Query<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        let query_retained = query.clone();
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
        let (data, rate_limit) = parse_raw_response(response)?;
        let deserialized_response = match query_retained {
            Query::DataProviders(_) => ApiResponseType::DataProviders(parse_json(&data)?),
            Query::Discovery => ApiResponseType::Discovery(parse_json(&data)?),
            Query::ExpertFinder => ApiResponseType::ExpertFinder(parse_json(&data)?),
            Query::Journals(_) => ApiResponseType::Journals(parse_json(&data)?),
            Query::Outputs(_) => ApiResponseType::Outputs(parse_json(&data)?),
            Query::SearchWorks(_) => ApiResponseType::SearchWorks(parse_json(&data)?),
            Query::SearchOutputs(_) => ApiResponseType::SearchOutputs(parse_json(&data)?),
            Query::SearchDataProviders(_) => ApiResponseType::SearchDataProviders(parse_json(&data)?),
            Query::SearchJournals(_) => ApiResponseType::SearchJournals(parse_json(&data)?),
        };
    
        Ok(ApiResponse {
            ratelimit_remaining: rate_limit,
            response: deserialized_response,
        })
    }

    pub fn paged<T>(&self, limit: i32, offset: i32) -> SearchQuery<String, T> where T:ToString {
        SearchQuery::paged(limit, offset)
    }
}

impl<T: Into<String>> From<T> for Api {
    fn from(key: T) -> Self {
        let client = reqwest::blocking::Client::new();
        Api { key: key.into(), client }
    }
}

fn parse_json<T>(data: &str) -> Result<T, crate::errors::Error> where T: DeserializeOwned {
    let deserializer = &mut Deserializer::from_str(&data);
    let res: Result<T, _> = deserialize(deserializer);
    match res {
        Ok(parsed_data) => Ok(parsed_data),
        Err(e) => return Err(Error::ParsingError(e.to_string())),
    }
}