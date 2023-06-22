use serde::{Deserialize, Serialize, Deserializer};

/// Represents a type of identifier.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifierType {
    /// Preferred form of the identifier
    #[serde(deserialize_with = "deserialize_as_string", default)]
    pub preferred: Option<String>,
    
    /// All forms of the identifier
    pub all: Option<String>,
}

/// Custom deserialization function that always deserializes as a string
fn deserialize_as_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        _ => Ok(None),
    }
}
