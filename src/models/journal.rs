use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Journal {
    title: String,
    identifiers: Vec<String>
}