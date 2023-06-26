use std::fmt::Display;

use super::{search_query::SearchQuery, request_type::QueryRequestType};

/// The `Query` enum represents various types of API requests that can be executed using the client. 
/// Each variant of the `Query` enum corresponds to a different endpoint of the API.
///
/// `Query` contains a range of operations including but not limited to fetching data from specific data providers,
/// performing search operations across different types of works, outputs, journals and data providers.
/// 
/// # Variants
/// * `DataProviders(String)`: Represents a request to the data-providers endpoint with a given identifier.
/// * `Discovery`: Represents a discovery request.
/// * `ExpertFinder`: Represents an expert finder request.
/// * `Journals(String)`: Represents a request to the journals endpoint with a given identifier.
/// * `Outputs(String)`: Represents a request to the outputs endpoint with a given identifier.
/// * `SearchWorks(SearchQuery<T1, T2>)`: Represents a search request for works.
/// * `SearchOutputs(SearchQuery<T1, T2>)`: Represents a search request for outputs.
/// * `SearchDataProviders(SearchQuery<T1, T2>)`: Represents a search request for data providers.
/// * `SearchJournals(SearchQuery<T1, T2>)`: Represents a search request for journals.
///
/// # Methods
/// `parse_request`: This method processes a `Query` variant and returns the corresponding API endpoint and HTTP method.
///
#[derive(Debug, Clone)]
pub(crate) enum Query<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    DataProviders(T1),
    Discovery,
    ExpertFinder,
    Journals(String),
    Outputs(String),
    SearchWorks(SearchQuery<T1, T2>),
    SearchOutputs(SearchQuery<T1, T2>),
    SearchDataProviders(SearchQuery<T1, T2>),
    SearchJournals(SearchQuery<T1, T2>),
}


impl<T1, T2> Query<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub(crate) fn parse_request(self) -> (QueryRequestType, String) {
        match self {
            Query::DataProviders(id) => (QueryRequestType::Get, format!("data-providers/{}", id.to_string())),
            Query::Discovery => (QueryRequestType::Post, "discover".to_string()),
            Query::ExpertFinder => (QueryRequestType::Post, "labs/expert-finder".to_string()),
            Query::Journals(id) => (QueryRequestType::Get, format!("journals/{}", id)),
            Query::Outputs(id) => (QueryRequestType::Get, format!("outputs/{}", id)),
            Query::SearchWorks(sq) => (QueryRequestType::Get, format!("search/works/{}", sq.parse())),
            Query::SearchOutputs(sq) => (QueryRequestType::Get, format!("search/outputs/{}", sq.parse())),
            Query::SearchDataProviders(sq) => (QueryRequestType::Get, format!("search/data-providers/{}", sq.parse())),
            Query::SearchJournals(sq) => (QueryRequestType::Get, format!("search/journals/{}", sq.parse())),
        }
    }
}

/// `StringDefault` is a helper struct that implements the `Display` trait.
/// It doesn't hold any data and it always prints an empty string.
#[derive(Clone)]
pub struct StringDefault {}
impl Display for StringDefault {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}