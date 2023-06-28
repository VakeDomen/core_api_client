use super::{search_query::SearchQuery, request_type::QueryRequestType};

/// The `Query` enum represents various types of API requests that can be executed using the client. 
/// Each variant of the `Query` enum corresponds to a different endpoint of the API.
///
/// `Query` contains a range of operations including but not limited to fetching data from specific data providers,
/// performing search operations across different types of works, outputs, journals and data providers.
/// 
/// # Variants
/// * `DataProviders(T1)`: Represents a request to the data-providers endpoint with a given identifier.
/// * `Discovery`: Represents a discovery request.
/// * `ExpertFinder`: Represents an expert finder request.
/// * `Journals(T1)`: Represents a request to the journals endpoint with a given identifier.
/// * `Outputs(T1)`: Represents a request to the outputs endpoint with a given identifier.
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
    Discovery(T1),
    Journals(T1),
    Outputs(T1),
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
    pub(crate) fn parse_request(self) -> (QueryRequestType, String, Option<String>) {
        match self {
            Query::DataProviders(id) => (QueryRequestType::Get, format!("data-providers/{}", id.to_string()), None),
            Query::Discovery(doi) => (QueryRequestType::Post, "discover".to_string(), Some(create_discovery_body(doi))),
            Query::Journals(id) => (QueryRequestType::Get, format!("journals/{}", id.to_string()), None),
            Query::Outputs(id) => (QueryRequestType::Get, format!("outputs/{}", id.to_string()), None),
            Query::SearchWorks(sq) => (QueryRequestType::Get, format!("search/works/{}", sq.parse()), None),
            Query::SearchOutputs(sq) => (QueryRequestType::Get, format!("search/outputs/{}", sq.parse()), None),
            Query::SearchDataProviders(sq) => (QueryRequestType::Get, format!("search/data-providers/{}", sq.parse()), None),
            Query::SearchJournals(sq) => (QueryRequestType::Get, format!("search/journals/{}", sq.parse()), None),
        }
    }
}

fn create_discovery_body<T>(doi: T) -> String where T: ToString {
    format!("{{\"doi\": \"{}\"}}", doi.to_string())
}