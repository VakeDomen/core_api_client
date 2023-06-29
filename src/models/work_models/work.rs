use serde::{Deserialize, Serialize};

use crate::{empty::Empty, identifier::IdentifierEntry, helpers::string_number_deserializer::deserialize_as_string};

use super::{reference::Reference, journal::Journal, author::Author, data_provider::DataProvider, link::LinkType};


/// Struct holds the work information. More info on the work struct [here](https://api.core.ac.uk/docs/v3#tag/Works/null) 
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Work {
    /// Date the work was accepted
    #[serde(rename = "acceptedDate")]
    pub accepted_date: Option<String>,
    
    /// ARXIV identifier of the work
    #[serde(rename = "arxivId")]
    pub arxiv_id: Option<String>,

    /// List of authors of the work
    pub authors: Option<Vec<Author>>,

    /// Number of citations the work has received
    #[serde(rename = "citationCount")]
    pub citation_count: Option<i32>,

    /// List of contributors to the work
    pub contributors: Option<Vec<String>>,

    /// List of outputs associated with the work
    pub outputs: Option<Vec<String>>,

    /// Date the work was created
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,

    /// List of data providers associated with the work
    #[serde(rename = "dataProviders")]
    pub data_providers: Option<Vec<DataProvider>>,

    /// Date the work was deposited
    #[serde(rename = "depositedDate")]
    pub deposited_date: Option<String>,

    /// Abstract text of the work
    #[serde(rename = "abstractText")]
    pub abstract_text: Option<String>,

    /// Type of the document (e.g., article, thesis, etc.)
    #[serde(rename = "documentType")]
    pub document_type: Option<String>,

    /// Digital Object Identifier (DOI) of the work
    pub doi: Option<String>,

    /// URL for downloading the work
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,

    /// Field of study associated with the work
    #[serde(rename = "fieldOfStudy")]
    pub field_of_study: Option<String>,

    /// Full text of the work
    #[serde(rename = "fullText")]
    pub full_text: Option<String>,

    /// ID of the work
    pub id: Option<i32>,

    /// List of identifiers associated with the work
    pub identifiers: Option<IdentifierEntry>,

    /// Title of the work
    pub title: Option<String>,

    /// Language of the work
    pub language: Option<Empty>,

    /// MAG (Microsoft Academic Graph) identifier of the work
    #[serde(rename = "magId")]
    pub mag_id: Option<String>,

    /// List of OAI (Open Archives Initiative) identifiers of the work
    #[serde(rename = "oaiIds")]
    pub oai_ids: Option<Vec<String>>,

    /// Date the work was published
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,

    /// Publisher of the work
    pub publisher: Option<String>,

    /// PubMed identifier of the work
    #[serde(rename = "pubmedId")]
    pub pubmed_id: Option<String>,

    /// List of references cited by the work
    pub references: Option<Vec<Reference>>,

    /// List of URLs to the full text of the work from different sources
    #[serde(rename = "sourceFulltextUrls")]
    pub source_fulltext_urls: Option<Vec<String>>,

    /// List of journals associated with the work
    pub journals: Option<Vec<Journal>>,

    /// Date the work was last updated
    #[serde(rename = "updatedDate")]
    pub updated_date: Option<String>,

    /// Year the work was published
    #[serde(rename = "yearPublished", deserialize_with="deserialize_as_string", default)]
    pub year_published: Option<String>,
        
    // List of links associated with the work
    pub links: Option<Vec<LinkType>>,
}