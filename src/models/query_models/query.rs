use std::fmt::Display;

use super::{search_query::SearchQuery, request_type::QueryRequestType};

#[derive(Debug, Clone)]
pub(crate) enum Query<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    DataProviders(String),
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
            Query::DataProviders(id) => (QueryRequestType::Get, format!("data-providers/{}", id)),
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

#[derive(Clone)]
pub struct StringDefault {}
impl Display for StringDefault {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}