use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IdentifierEntry {
    IdentifierWork(Vec<Identifier>),
    IdentifierOuputs(DoiIdentifier),
}

// Struct holds the work identifiers and their type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    /// Identifier value.
    pub identifier: String,

    /// Type of the identifier.
    #[serde(rename = "type")]
    pub identifier_type: String,
}

// Struct holds the work identifiers and their type.
#[derive(Debug, Deserialize, Serialize)]
pub struct DoiIdentifier {
    pub doi: Option<String>,
    pub oai: Option<String>
}