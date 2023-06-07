use serde::{Deserialize, Serialize};

/// struct holds the avalible data on the [data providers](https://api.core.ac.uk/docs/v3#tag/Data-Providers)
#[derive(Debug, Deserialize, Serialize)]
pub struct DataProvider {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub logo: String,
}