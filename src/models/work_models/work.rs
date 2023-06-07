use serde::{Deserialize, Serialize};

use crate::empty::Empty;

use super::{identifier::Identifier, reference::Reference, journal::Journal, author::Author, data_provider::DataProvider, link::LinkType};


/// Struct holds the work information. More info on the work struct [here](https://api.core.ac.uk/docs/v3#tag/Works/null) 
#[derive(Debug, Deserialize, Serialize)]
pub struct Work {
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<String>, // date
    
    #[serde(rename = "arxivId")]
    pub arxiv_id: Option<String>,
    
    pub authors: Option<Vec<Author>>,
    
    #[serde(rename = "citationCount")]
    pub citation_count: Option<i32>,

    pub contributors: Option<Vec<String>>,
    
    pub outputs: Option<Vec<String>>,

    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
    
    #[serde(rename = "dataProviders")]
    pub data_providers: Option<Vec<DataProvider>>,
    
    #[serde(rename = "depositedDate")]
    pub deposited_date: Option<String>, // date
    
    #[serde(rename = "abstractText")]
    pub abstract_text: Option<String>,
    
    #[serde(rename = "documentType")]
    pub document_type: Option<String>,

    pub doi: Option<String>,
    
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    
    #[serde(rename = "fieldOfStudy")]
    pub field_of_study: Option<String>,
    
    #[serde(rename = "fullText")]
    pub full_text: Option<String>,
    
    pub id: Option<i32>,
    
    pub identifiers: Option<Vec<Identifier>>,
    
    pub title: Option<String>,
    
    pub language: Option<Empty>,
    
    #[serde(rename = "magId")]
    pub mag_id: Option<String>,
    
    #[serde(rename = "oaiIds")]
    pub oai_ids: Option<Vec<String>>,
    
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>, // date
    
    pub publisher: Option<String>,
    
    #[serde(rename = "pubmedId")]
    pub pubmed_id: Option<String>,
    
    pub references: Option<Vec<Reference>>,
    
    #[serde(rename = "sourceFulltextUrls")]
    pub source_fulltext_urls: Option<Vec<String>>,
    
    pub journals: Option<Vec<Journal>>,
    
    #[serde(rename = "updatedDate")]
    pub updated_date: Option<String>, // date
    
    #[serde(rename = "yearPublished")]
    pub year_published: Option<i32>,
    
    pub links: Option<Vec<LinkType>>,
}