use serde::{Deserialize, Serialize};

/// struct holds the avalible data on the [data providers](https://api.core.ac.uk/docs/v3#tag/Data-Providers)
#[derive(Debug, Deserialize, Serialize)]
pub struct DataProvider {
    /// ID of the data provider.
    pub id: i32,

    /// Name of the data provider.
    pub name: String,

    /// URL of the data provider.
    pub url: String,

    /// Logo of the data provider.
    pub logo: String,
}