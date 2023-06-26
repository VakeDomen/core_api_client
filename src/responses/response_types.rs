use serde::{Serialize, Deserialize};

use crate::{work::Work, models::data_provider_models::data_provider::DataProvider, journal::Journal};

use super::{search::SearchResponse, response::ApiResponseTrait};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResponseType {
    DataProviders(Option<DataProvider>),
    Discovery(DiscoveryResponse),
    ExpertFinder(ExpertFinderResponse),
    Journals(JournalsResponse),
    Outputs(Work),
    SearchWorks(SearchResponse<Work>),
    SearchOutputs(SearchResponse<Work>),
    SearchDataProviders(SearchResponse<DataProvider>),
    SearchJournals(SearchResponse<Journal>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse;
impl ApiResponseTrait for DiscoveryResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpertFinderResponse;
impl ApiResponseTrait for ExpertFinderResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalsResponse;
impl ApiResponseTrait for JournalsResponse {}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OutputsResponse;
// impl ApiResponseTrait for OutputsResponse {}



