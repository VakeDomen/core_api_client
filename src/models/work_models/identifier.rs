use serde::{Deserialize, Serialize};

/// Struct holds the work identifiers and it's type
#[derive(Debug, Deserialize, Serialize)]
pub struct Identifier {
    pub identifier: String,
    #[serde(rename = "type")]
    pub identifier_type: String,
}