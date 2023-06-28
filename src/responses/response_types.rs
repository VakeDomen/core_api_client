use serde::{Serialize, Deserialize};

use crate::{work::Work, models::{data_provider_models::data_provider::DataProvider, discovery_models::discovery::Discovery}, journal::Journal};

use super::{search::SearchResponse, response::ApiResponseTrait};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResponseType {
    DataProviders(Option<DataProvider>),
    Discovery(Discovery),
    ExpertFinder(ExpertFinderResponse),
    Journals(Journal),
    Outputs(Work),
    SearchWorks(SearchResponse<Work>),
    SearchOutputs(SearchResponse<Work>),
    SearchDataProviders(SearchResponse<DataProvider>),
    SearchJournals(SearchResponse<Journal>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpertFinderResponse;
impl ApiResponseTrait for ExpertFinderResponse {}

