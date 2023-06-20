use reqwest::{blocking::Client, header};
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use serde_path_to_error::deserialize;
use crate::{
    helpers::response_handler::{parse_raw_response}, 
    responses::{response::ApiResponse, responses::ApiResponseEnum, search::SearchResponse}, 
    SearchQuery, work::Work, errors::Error
};
use super::query::Query;

/// Main API struct. API holds your key you acquire from [CORE](https://core.ac.uk/services/api#form). 
/// Also it tracks how many uses of the API you have left based on the rate limit oposed by CORE.
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
            Query::DataProviders(_) => ApiResponseEnum::DataProviders(parse_json(&data)?),
            Query::Discovery => ApiResponseEnum::Discovery(parse_json(&data)?),
            Query::ExpertFinder => ApiResponseEnum::ExpertFinder(parse_json(&data)?),
            Query::Journals(_) => ApiResponseEnum::Journals(parse_json(&data)?),
            Query::Outputs(_) => ApiResponseEnum::Outputs(parse_json(&data)?),
            Query::SearchWorks(_) => ApiResponseEnum::SearchWorks(parse_json(&data)?),
            Query::SearchOutputs(_) => ApiResponseEnum::SearchOutputs(parse_json(&data)?),
            Query::SearchDataProviders(_) => ApiResponseEnum::SearchDataProviders(parse_json(&data)?),
            Query::SearchJournals(_) => ApiResponseEnum::SearchJournals(parse_json(&data)?),
        };
    

        // self
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