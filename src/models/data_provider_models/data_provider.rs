use serde::{Deserialize, Serialize};

use super::{other_identifiers::OtherIdentifiers, location::Location};

/// Represents a provider of open access scientific papers and other resources.
/// More information [here](https://api.core.ac.uk/docs/v3#tag/Data-Providers)
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct DataProvider {
    /// Unique ID of the data provider
    pub id: i32,

    /// OpenDOAR (Directory of Open Access Repositories) ID of the data provider
    #[serde(rename = "openDoarId")]
    pub open_doar_id: Option<i32>,
    
    /// Name of the data provider
    pub name: String,
    
    /// Contact email of the data provider
    pub email: String,
    
    /// URI of the data provider
    pub uri: Option<String>,
    
    /// OAI-PMH (Open Archives Initiative Protocol for Metadata Harvesting) URL of the data provider
    #[serde(rename = "oaiPmhUrl")]
    pub oai_pmh_url: String,
    
    /// Homepage URL of the data provider
    #[serde(rename = "homepageUrl")]
    pub homepage_url: Option<String>,
    
    /// Source of the data
    pub source: Option<String>,
    
    /// Software used by the data provider
    pub software: Option<String>,
    
    /// Metadata format used by the data provider
    #[serde(rename = "metadataFormat")]
    pub metadata_format: String,
    
    /// Date the data provider was created
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
    
    /// Location of the data provider
    pub location: Location,
    
    /// Logo URL of the data provider
    pub logo: String,
    
    /// Type of the data provider
    #[serde(rename = "type")]
    pub data_provider_type: String,
    
    /// Statistical data about the data provider
    pub stats: Option<String>,
    
    /// ROR (Research Organization Registry) ID of the data provider
    #[serde(rename = "rorId")]
    pub ror_id: Option<String>,
    
    /// Name of the institution associated with the data provider
    #[serde(rename = "institutionName")]
    pub institution_name: Option<String>,
    
    /// Aliases of the data provider
    pub aliases: Vec<String>,
    
    /// Other identifiers associated with the data provider
    #[serde(rename = "otherIdentifiers")]
    pub other_identifiers: Option<OtherIdentifiers>,
}



