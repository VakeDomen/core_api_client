use serde::{Deserialize, Serialize};

use super::identifier_type::IdentifierType;

/// Represents other identifiers associated with a data provider.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct OtherIdentifiers {
    /// GRID (Global Research Identifier Database) ID of the data provider
    #[serde(rename = "GRID")]
    pub grid: Option<IdentifierType>,
    
    /// ISNI (International Standard Name Identifier) of the data provider
    #[serde(rename = "ISNI")]
    pub isni: Option<IdentifierType>,
    
    /// FundRef ID of the data provider
    #[serde(rename = "FundRef")]
    pub fund_ref: Option<IdentifierType>,
    
    /// Wikidata ID of the data provider
    #[serde(rename = "Wikidata")]
    pub wikidata: Option<IdentifierType>,
}