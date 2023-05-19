use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Identifier {
    identifier: String,
    identifier_type: String,
}