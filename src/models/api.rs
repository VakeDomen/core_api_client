use reqwest::{blocking::Client, header};
use crate::{
    helpers::response_handler::{parse_raw_response, parse_json}, 
    responses::{response::ApiResponse, response_types::ApiResponseType}, 
    SearchQuery,
};

use super::query_models::{query::Query, request_type::QueryRequestType};

/// Main API struct. API holds your key you acquire from [CORE](https://core.ac.uk/services/api#form). 
/// Lastly it holds a refernce to a blocking Client it uses to execute queries to the CORE API.
/// The `Api` struct provides a set of methods to interact with a specific API service.
/// It includes methods to search for works, data providers, journals, and outputs based on various queries.
/// The struct uses an API key and an HTTP client for requests, and optionally logs the request target and raw response.
///
/// Key methods include:
/// * `get_output`: Get a single Output based on CORE id.
/// * `get_journal`: Get a single journal based on its identifier in CORE
/// * `discover`: Allows you to find links to full texts based on a DOI. The system will search through the CORE data and other external sources to provide you the best match.
/// * `search_works`: Executes a search for research works.
/// * `search_data_providers`: Executes a search for data providers.
/// * `search_journals`: Executes a search for journal titles.
/// * `search_outputs`: Executes a search for work outputs.
/// * `paged_search`: Initiates a paginated search. (SearchQuery builder)
/// * `log_target`: Enables/disables logging of the target URI.
/// * `log_raw_response`: Enables/disables logging of the raw response.
///
/// An instance of `Api` can be created using an API key and provides an easy way to interact with the API service.
#[derive(Debug)]
pub struct Api {
    key: String,
    client: Client,
    log_target: bool,
    log_raw_response: bool,
}


impl Api {
    /// allows you to find links to full texts based on a DOI. The system will search through the CORE 
    /// data and other external sources to provide you the best match.
    /// 
    /// # Parameters
    /// 
    /// * 'doi' - Doi of the target discover resource 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// api.discover("10.1016/0370-2693(96)00910-0");
    /// ```
    pub fn discover<T>(
        &self,
        doi: T
    ) -> Result<ApiResponse, crate::errors::Error> 
    where
        T: ToString + Clone
    {
        self.execute_query::<T, String>(Query::Discovery(doi))
    }


    /// Fetches a single output from CORE using the provided output id.
    ///
    /// # Parameters
    ///
    /// * `id` - The Journal id in CORE. Use issn:ISSN to search by ISSN instead of the CORE identifier.
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// api.get_journal("issn:1179-1497");
    /// ```
    pub fn get_journal<T>(
        &self,
        id: T
    ) -> Result<ApiResponse, crate::errors::Error>
    where 
        T: ToString + Clone
    {
        self.execute_query::<T, String>(Query::Journals(id))
    }

    /// Fetches a single output from CORE using the provided output id.
    ///
    /// # Parameters
    ///
    /// * `id` - The CORE ID of the output to be fetched.
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// api.get_output(0);
    /// ```
    pub fn get_output<T>(
        &self,
        id: T
    ) -> Result<ApiResponse, crate::errors::Error>
    where 
        T: ToString + Clone
    {
        self.execute_query::<T, String>(Query::Outputs(id))
    }

    /// Fetches a specific data provider from CORE using the provided data provider identifier.
    ///
    /// The function makes use of the CORE API's capability to fetch data provider details using their identifiers.
    /// The identifiers can be either:
    /// 1. CORE's data provider identifier.
    /// 2. OpenDOAR's identifier, prefixed by "opendoar:" (e.g., "opendoar:123").
    ///
    /// A call to this function executes a query against the CORE API and returns the results wrapped in an `ApiResponse`.
    ///
    /// # Parameters
    /// 
    /// * `id`: Identifier of the data provider. Can be a CORE data provider identifier (integer) or an OpenDOAR identifier prefixed with "opendoar:".
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// api.get_data_provider(86);
    /// api.get_data_provider("opendoar:300");
    /// 
    /// ```
    pub fn get_data_provider<T>(
        &self,
        id: T
    ) -> Result<ApiResponse, crate::errors::Error>
    where 
        T: ToString + Clone
    {
        self.execute_query::<T, String>(Query::DataProviders(id))
    }

    /// Executes a search on the API for works based on the query.
    /// These are the entities that represent a piece of research, .e.g research articles, theses, etc. 
    /// In total, it is a deduplicated and enriched version of records.
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
    /// It gives you access to the collection of entities that offer data to CORE. 
    /// It contains repositories (institutional and disciplinary), preprint servers, journals and publishers.
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

    /// Executes a search on the API for journals based on the query.
    /// This dataset contains all journal titles included in the CORE collection. 
    /// Moreover, you can search and retrieve any journal even if it is not a CORE data provider.
    /// 
    /// ```
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// 
    /// let query = api.paged_search(10, 0)
    ///     .and(FilterOperator::Eq("publisher", "OJS"));
    /// let resp = api.search_journals(query);
    /// ```
    /// 
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


    /// Executes a search on the API for otuputs (works) based on the query.
    /// Outputs are a representation of a Work in a data provider. 
    /// The data is not enriched and it mirrors exactly the content harvested from the data provider.
    /// 
    /// ```
    /// use core_api_rs::FilterOperator;
    /// use core_api_rs::Api;
    /// 
    /// let api = Api::from("API_KEY");
    /// 
    /// let query = api.paged_search(10, 0)
    ///     .and(FilterOperator::Eq("publisher", "OJS"));
    /// let resp = api.search_outputs(query);
    /// ```
    /// 
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
    

    /// The `paged_search` method initiates a paginated search on the API.
    /// It takes a limit and an offset as arguments, representing the number of results to return per page and the starting point for the results respectively.
    /// This method returns a `SearchQuery` object that can be further manipulated to define the search criteria.
    ///
    /// Due to generic implementation the search requres 2 types. In case you do not use any filters the types can not 
    /// be infered and therefore requre you to use any generic type that implements ToString.
    /// 
    /// Example:
    /// ```
    /// use core_api_rs::Api;
    /// use core_api_rs::FilterOperator;
    /// 
    /// let api = Api::from("API_KEY");
    /// let query1 = api.paged_search::<String, String>(10, 0); // Initiates a paginated search for 10 items starting from the first result
    /// let query2 = api.paged_search::<_, String>(10, 0)
    ///     .and(FilterOperator::Exists("software"));
    /// let query3 = api.paged_search(10, 0)
    ///     .and(FilterOperator::HasValue("type", "JOURNAL"));
    /// ```
    pub fn paged_search<T1 , T2>(
        &self, limit: i32, 
        offset: i32
    ) -> SearchQuery<T1, T2> 
    where 
        T1:ToString, 
        T2:ToString, 
    {
        SearchQuery::paged(limit, offset)
    }

    /// Method allows the user to override the default (false) logging of the target URI that is being fetched
    /// for data retrieval from the api
    /// ```
    /// use core_api_rs::Api;
    /// let api = Api::from("API_KEY").log_target(true);
    /// ```
    pub fn log_target(self, log_target: bool) -> Self {
        Self { key: self.key, client: self.client, log_target, log_raw_response: self.log_raw_response }
    }


    /// Method allows the user to override the default (false) logging of the raw responses that are returned
    /// from the API. 
    /// ```
    /// use core_api_rs::Api;
    /// let api = Api::from("API_KEY").log_raw_response(true);
    /// ```
    pub fn log_raw_response(self, log_raw_response: bool) -> Self {
        Self { key: self.key, client: self.client, log_target: self.log_target, log_raw_response }
    }


    /// The `execute_query` method performs the actual API request based on the query provided.
    /// It accepts a `Query` object that represents the search criteria and returns an `ApiResponse` object 
    /// which contains the API response and the remaining rate limit.
    /// 
    /// This method is primarily used internally by other public methods and might not be directly called by the user.
    ///
    /// Note: This method is private and not exposed to the user directly.
    fn execute_query<T1, T2>(
        &self, 
        query: Query<T1, T2>
    ) -> Result<ApiResponse, crate::errors::Error> 
    where 
        T1: ToString + Clone,
        T2: ToString + Clone,
    {
        let retained_query = query.clone(); // for later refernce
        let (req_type, query_uri, body) = query.parse_request();
        
        let target = format!("https://api.core.ac.uk/v3/{}", query_uri);
        if self.log_target {
            println!("{}", query_uri);
        }
        
        let client_builer = match req_type {
            QueryRequestType::Get   => self.client.get(target),
            QueryRequestType::Post  => self.client.post(target),
        };

        let client_builer = client_builer.header(
            header::AUTHORIZATION,
            format!("Bearer {}", self.key.clone())
        );

        let client_builder = match (req_type, body) {
            (QueryRequestType::Get,     None)                   => client_builer,
            (QueryRequestType::Get,     Some(_))                => client_builer,
            (QueryRequestType::Post,    None)                   => client_builer,
            (QueryRequestType::Post,    Some(content))  => client_builer.body(content),
        };

        let response = match client_builder.send() {
            Ok(r) => r,
            Err(e) => return Err(crate::errors::Error::Request(e)),
        };
        
        let (data, rate_limit) = parse_raw_response(response)?;
        
        if self.log_raw_response {
            println!("{}", data);
        }
        
        let deserialized_response = match retained_query {
            Query::DataProviders(_)         => ApiResponseType::DataProviders(parse_json(&data)?),
            Query::Discovery(_)             => ApiResponseType::Discovery(parse_json(&data)?),
            Query::Journals(_)              => ApiResponseType::Journals(parse_json(&data)?),
            Query::Outputs(_)               => ApiResponseType::Outputs(parse_json(&data)?),
            Query::SearchWorks(_)           => ApiResponseType::SearchWorks(parse_json(&data)?),
            Query::SearchOutputs(_)         => ApiResponseType::SearchOutputs(parse_json(&data)?),
            Query::SearchDataProviders(_)   => ApiResponseType::SearchDataProviders(parse_json(&data)?),
            Query::SearchJournals(_)        => ApiResponseType::SearchJournals(parse_json(&data)?),
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
