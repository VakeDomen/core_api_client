
use serde::{Deserialize, Serialize};


/// Struct holds the information of the discovoered resource. More info on the work struct [here](https://api.core.ac.uk/docs/v3#tag/Discovery).
#[derive(Debug, Deserialize, Serialize)]
pub struct Discovery {
    /// Url link to the full text of the discovered source
    #[serde(rename = "fullTextLink")]
    pub full_text_link: String,
        
    /// Discovered rource
    pub source: String,
}