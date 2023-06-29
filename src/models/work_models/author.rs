use serde::{Deserialize, Serialize};

/// Struct holds the name of the author.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Author {
    /// Name of the author.
    pub name: String,
}