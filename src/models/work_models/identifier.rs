use serde::{Deserialize, Serialize};

// Struct holds the work identifiers and their type.
#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    /// Identifier value.
    pub identifier: String,

    /// Type of the identifier.
    #[serde(rename = "type")]
    pub identifier_type: String,
}