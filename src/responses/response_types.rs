use serde::{Serialize, Deserialize};

use crate::{work::Work, models::{data_provider_models::data_provider::DataProvider, discovery_models::discovery::Discovery}, journal::Journal};

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