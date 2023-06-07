use serde::{Deserialize, Serialize};

/// Struct representing a reference.
#[derive(Debug, Deserialize, Serialize)]
pub struct Reference {
    /// ID of the reference.
    pub id: Option<i32>,
    /// List of authors of the reference.
    pub authors: Option<Vec<String>>,

    /// Cites information of the reference.
    pub cites: Option<String>,

    /// Date associated with the reference.
    pub date: Option<String>,

    /// Digital Object Identifier (DOI) of the reference.
    pub doi: Option<String>,

    /// Raw content of the reference.
    pub raw: Option<String>,

    /// Title of the reference.
    pub title: Option<String>,

}