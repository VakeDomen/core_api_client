use serde::Deserialize;

use super::{identifier::Identifier, empty::Empty, reference::Reference, journal::Journal, author::Author, data_provider::DataProvider, link::LinkType};

#[derive(Debug, Deserialize)]
pub struct Work {
    #[serde(rename = "acceptedDate")]
    accepted_date: Option<String>, // date
    
    #[serde(rename = "arxivId")]
    arxiv_id: Option<String>,
    
    authors: Option<Vec<Author>>,
    
    #[serde(rename = "citationCount")]
    citation_count: Option<i32>,

    contributors: Option<Vec<String>>,
    
    outputs: Option<Vec<String>>,

    #[serde(rename = "createdDate")]
    created_date: Option<String>,
    
    #[serde(rename = "dataProviders")]
    data_providers: Option<Vec<DataProvider>>,
    
    #[serde(rename = "depositedDate")]
    deposited_date: Option<String>, // date
    
    #[serde(rename = "abstractText")]
    abstract_text: Option<String>,
    
    #[serde(rename = "documentType")]
    document_type: Option<String>,

    doi: Option<String>,
    
    #[serde(rename = "downloadUrl")]
    download_url: Option<String>,
    
    #[serde(rename = "fieldOfStudy")]
    field_of_study: Option<String>,
    
    #[serde(rename = "fullText")]
    full_text: Option<String>,
    
    id: Option<i32>,
    
    identifiers: Option<Vec<Identifier>>,
    
    title: Option<String>,
    
    language: Option<Empty>,
    
    #[serde(rename = "magId")]
    mag_id: Option<String>,
    
    #[serde(rename = "oaiIds")]
    oai_ids: Option<Vec<String>>,
    
    #[serde(rename = "publishedDate")]
    published_date: Option<String>, // date
    
    publisher: Option<String>,
    
    #[serde(rename = "pubmedId")]
    pubmed_id: Option<Empty>,
    
    references: Option<Vec<Reference>>,
    
    #[serde(rename = "sourceFulltextUrls")]
    source_fulltext_urls: Option<Vec<String>>,
    
    journals: Option<Vec<Journal>>,
    
    #[serde(rename = "updatedDate")]
    updated_date: Option<String>, // date
    
    #[serde(rename = "yearPublished")]
    year_published: Option<i32>,
    
    links: Option<Vec<LinkType>>,
}