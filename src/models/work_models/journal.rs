use serde::{Deserialize, Serialize};

/// Struct holds data of the [journal](https://api.core.ac.uk/docs/v3#tag/Journals)
#[derive(Debug, Deserialize, Serialize)]
pub struct Journal {
    title: String,
    identifiers: Vec<String>
}