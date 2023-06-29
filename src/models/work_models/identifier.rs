use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash)]
#[serde(untagged)]
pub enum IdentifierEntry {
    IdentifierWork(Vec<Identifier>),
    IdentifierOuputs(DoiIdentifier),
}

impl Default for IdentifierEntry {
    fn default() -> Self {
        IdentifierEntry::IdentifierWork(vec![])
    }
}

// Struct holds the work identifiers and their type.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Identifier {
    /// Identifier value.
    pub identifier: String,

    /// Type of the identifier.
    #[serde(rename = "type")]
    pub identifier_type: String,
}

// Struct holds the work identifiers and their type.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct DoiIdentifier {
    pub doi: Option<String>,
    pub oai: Option<String>
}

impl fmt::Display for IdentifierEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentifierEntry::IdentifierWork(identifiers) => {
                let ids = identifiers.iter()
                                     .map(|id| id.to_string()) // Assuming Identifier implements ToString
                                     .collect::<Vec<_>>()
                                     .join(", ");
                write!(f, "IdentifierWork: [{}]", ids)
            },
            IdentifierEntry::IdentifierOuputs(doi) => write!(f, "IdentifierOuputs: {}", doi.to_string()), // Assuming DoiIdentifier implements ToString
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Identifier {{ value: {}, type: {} }}", self.identifier, self.identifier_type)
    }
}

impl fmt::Display for DoiIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let doi = self.doi.as_ref().map_or(String::from("None"), |doi| doi.to_string());
        let oai = self.oai.as_ref().map_or(String::from("None"), |oai| oai.to_string());

        write!(f, "DoiIdentifier {{ doi: {}, oai: {} }}", doi, oai)
    }
}