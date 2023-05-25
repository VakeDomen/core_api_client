use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Identifier {
    identifier: Option<String>,
    #[serde(rename = "type")]
    identifier_type: Option<String>,
}