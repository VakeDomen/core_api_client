use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reference {
    id: Option<i32>,
    authors: Option<Vec<String>>,
    cites: Option<String>,
    date: Option<String>,
    doi: Option<String>,
    raw: Option<String>,
    title: Option<String>,
}