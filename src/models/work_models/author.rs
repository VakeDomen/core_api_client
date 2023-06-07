use serde::{Deserialize, Serialize};

/// Struct holds the name of the author
#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
}