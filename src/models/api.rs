use reqwest::{blocking::Client, header};
use crate::{
    helpers::response_handler::{parse_raw_response, parse_json}, 
    responses::{response::ApiResponse, responses::ApiResponseType}, 
    SearchQuery,
};
use super::query::Query;

/// Main API struct. API holds your key you acquire from [CORE](https://core.ac.uk/services/api#form). 
/// Lastly it holds a refernce to a blocking Client it uses to execute queries to the CORE API.
/// 
#[derive(Debug)]
pub struct Api {
    key: String,
    client: Client,
    log_target: bool,
    log_raw_response: bool,
}


impl Api {

    /// Executes a search on the API for works based on the query.
    /// 
    /// ```
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// 
    /// let query = api.paged_search(10, 0)
    ///    .and(FilterOperator::Exists("doi"))
    ///    .and(FilterOperator::Bigger("citationCount", 20));
    /// 
    /// let resp = api.search_works(query);
    /// ```
    /// 
    pub fn search_works<T1, T2>(
        &self, 
        query: SearchQuery<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        self.execute_query(Query::SearchWorks(query))
    }
    
    /// Executes a search on the API for works based on the query.
    /// 
    /// ```
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// 
    /// let query = api.paged_search(10, 0)
    ///    .and(FilterOperator::Exists("software"))
    ///    .and(FilterOperator::HasValue("type", "JOURNAL"));
    /// let resp = api.search_data_providers(query);
    /// ```
    /// 
    pub fn search_data_providers<T1, T2>(
        &self, 
        query: SearchQuery<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        self.execute_query(Query::SearchDataProviders(query))
    }

    pub fn search_journals<T1, T2>(
        &self, 
        query: SearchQuery<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        self.execute_query(Query::SearchJournals(query))
    }

    pub fn search_outputs<T1, T2>(
        &self, 
        query: SearchQuery<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        self.execute_query(Query::SearchOutputs(query))
    }
    

    
    pub fn paged_search<T1, T2>(
        &self, limit: i32, 
        offset: i32
    ) -> SearchQuery<T1, T2> 
    where 
        T1:ToString, 
        T2:ToString 
    {
        SearchQuery::paged(limit, offset)
    }

    /// Method allows the user to override the default (flase) logging of the target URI that is being fetched
    /// for data retrieval from the api
    /// ```
    /// use core_api_rs::Api;
    /// let api = Api::from("API_KEY").log_target(true);
    /// ```
    pub fn log_target(self, log_target: bool) -> Self {
        Self { key: self.key, client: self.client, log_target, log_raw_response: self.log_raw_response }
    }


    /// Method allows the user to override the default (flase) logging of the raw responses that are returned
    /// from the API. 
    /// ```
    /// use core_api_rs::Api;
    /// let api = Api::from("API_KEY").log_raw_response(true);
    /// ```
    pub fn log_raw_response(self, log_raw_response: bool) -> Self {
        Self { key: self.key, client: self.client, log_target: self.log_target, log_raw_response }
    }

    fn execute_query<T1, T2>(
        &self, 
        query: Query<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        let query_retained = query.clone();
        let (req_type, query_uri) = query.parse_request();
        if self.log_target {
            println!("{}", query_uri);
        }
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
        if self.log_raw_response {
            println!("{}", data);
        }
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

}

impl<T: Into<String>> From<T> for Api {
    fn from(key: T) -> Self {
        let client = reqwest::blocking::Client::new();
        Api { key: key.into(), client, log_target: false, log_raw_response: false }
    }
}
