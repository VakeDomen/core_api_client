use serde::{Serialize, Deserialize};

use crate::work::Work;

use super::{search::SearchResponse, response::ApiResponseTrait};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiResponseType {
    DataProviders(DataProvidersResponse),
    Discovery(DiscoveryResponse),
    ExpertFinder(ExpertFinderResponse),
    Journals(JournalsResponse),
    Outputs(OutputsResponse),
    SearchWorks(SearchResponse<Work>),
    SearchOutputs(SearchResponse<Outputs>),
    SearchDataProviders(SearchResponse<DataProviders>),
    SearchJournals(SearchResponse<Journals>),
}

// Assume all of these types implement DeserializeOwned
#[derive(Debug, Serialize, Deserialize)]
pub struct DataProvidersResponse;
impl ApiResponseTrait for DataProvidersResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryResponse;
impl ApiResponseTrait for DiscoveryResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpertFinderResponse;
impl ApiResponseTrait for ExpertFinderResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalsResponse;
impl ApiResponseTrait for JournalsResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputsResponse;
impl ApiResponseTrait for OutputsResponse {}


// TODO: impelement full: ˘˘˘

#[derive(Debug, Serialize, Deserialize)]
pub struct Outputs;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataProviders;

#[derive(Debug, Serialize, Deserialize)]
pub struct Journals;
