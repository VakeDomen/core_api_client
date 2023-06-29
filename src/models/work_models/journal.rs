use serde::{Deserialize, Serialize};

/// Struct holds data of the [journal](https://api.core.ac.uk/docs/v3#tag/Journals)
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Journal {
    /// Title of the journal.
    title: Option<String>,
    
    /// List of identifiers associated with the journal.
    identifiers: Vec<String>,
}