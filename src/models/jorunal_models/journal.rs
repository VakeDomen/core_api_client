use serde::{Deserialize, Serialize};

/// Represents a Journal with various associated data.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Journal {
    /// The various identifiers of the Journal.
    pub identifiers: Vec<String>,

    /// The language of the Journal.
    pub language: String,

    /// The publisher of the Journal.
    pub publisher: String,

    /// The subjects of the Journal.
    pub subjects: Vec<String>,

    /// The data provider ID associated with the Journal.
    #[serde(rename = "dataProviderId")]
    pub data_provider_id: i32,

    /// The title of the Journal.
    pub title: String,
}
