use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Author {
    name: String,
}