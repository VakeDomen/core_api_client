use serde::Deserialize;

use super::{identifier::Identifier, empty::Empty, reference::Reference, journal::Journal};

#[derive(Debug, Deserialize)]
pub struct Work {
    accepted_date: String, // date
    arxiv_id: String,
    authors: Vec<String>,
    citation_count: i32,
    contributors: Vec<String>,
    outputs: Vec<String>,
    created_date: String,
    data_providers: Vec<String>,
    deposited_date: String, // date
    abstract_text: String,
    document_type: String,
    doi: String,
    download_url: String,
    field_of_study: String,
    full_text: String,
    id: i32,
    identifiers: Vec<Identifier>,
    title: String,
    language: Option<Empty>,
    mag_id: String,
    oai_ids: Vec<String>,
    published_date: String, // date
    publisher: Empty,
    pubmed_id: Empty,
    references: Vec<Reference>,
    source_fulltext_urls: Vec<String>,
    journals: Vec<Journal>,
    updated_date: String, // date
    year_published: i32,
    links: Vec<String>
}