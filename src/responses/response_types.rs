use serde::{Serialize, Deserialize};

use crate::{
    Work, 
    DataProvider, 
    Journal, 
    Discovery
};

use super::search::SearchResponse;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ApiResponseType {
    DataProviders(DataProvider),
    Discovery(Discovery),
    Journals(Journal),
    Outputs(Work),
    SearchWorks(SearchResponse<Work>),
    SearchOutputs(SearchResponse<Work>),
    SearchDataProviders(SearchResponse<DataProvider>),
    SearchJournals(SearchResponse<Journal>),
}