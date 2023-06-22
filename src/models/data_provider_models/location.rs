use serde::{Deserialize, Serialize};

/// Represents the geographical location of a data provider.
#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    /// Country code of the location
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    
    /// Latitude of the location
    pub latitude: Option<f32>,
    
    /// Longitude of the location
    pub longitude: Option<f32>,
}