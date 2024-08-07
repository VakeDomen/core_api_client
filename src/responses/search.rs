use serde::{Deserialize, Serialize, de::DeserializeOwned};

use super::response::ApiResponseTrait;

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResponse<T> {
    
    #[serde(rename = "totalHits")]
    #[serde(deserialize_with = "deserialize_limit")]
    pub total_hits: Option<i32>,

	#[serde(deserialize_with = "deserialize_limit")]
    pub limit: Option<i32>,
	
    #[serde(deserialize_with = "deserialize_limit")]
    pub offset: Option<i32>,
	
    #[serde(rename = "scrollId")]
    pub scroll_id: Option<i32>,

	pub results: Option<Vec<T>>,
    
    pub tooks: Option<String>,
	
    #[serde(rename = "esTook")]
    pub es_took: Option<String>,
}



impl<T: DeserializeOwned> ApiResponseTrait for SearchResponse<T> {}

fn deserialize_limit<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntOrStringOrNull { Int(i32), String(String), Null }

    match IntOrStringOrNull::deserialize(deserializer)? {
        IntOrStringOrNull::Int(i) => Ok(Some(i)),
        IntOrStringOrNull::String(s) => s.parse().map(Some).map_err(serde::de::Error::custom),
        IntOrStringOrNull::Null => Ok(None),
    }
}