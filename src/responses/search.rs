use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse<T> {
    
    #[serde(rename = "totalHits")]
    #[serde(deserialize_with = "deserialize_limit")]
    total_hits: Option<i32>,

	#[serde(deserialize_with = "deserialize_limit")]
    limit: Option<i32>,
	
    offset: Option<i32>,
	
    #[serde(rename = "scrollId")]
    scroll_id: Option<i32>,

	results: Option<Vec<T>>,
    
    tooks: Option<String>,
	
    #[serde(rename = "esTook")]
    es_took: Option<String>,
}




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