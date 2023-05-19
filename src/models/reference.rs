use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reference {
    id: i32,
    authors: Vec<String>,
    cites: String,
    date: String,
    doi: String,
    raw: String,
    title: String,
}