use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Journal {
    title: Option<String>,
    identifiers: Option<Vec<String>>
}