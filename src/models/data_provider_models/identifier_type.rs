use serde::{Deserialize, Serialize};

use crate::helpers::string_number_deserializer::deserialize_as_string;

/// Represents a type of identifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    /// Preferred form of the identifier
    #[serde(deserialize_with = "deserialize_as_string", default)]
    pub preferred: Option<String>,
    
    /// All forms of the identifier
    pub all: Option<String>,
}
