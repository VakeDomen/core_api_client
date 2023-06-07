use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    id: Option<i32>,
    authors: Option<Vec<String>>,
    cites: Option<String>,
    date: Option<String>,
    doi: Option<String>,
    raw: Option<String>,
    title: Option<String>,
}