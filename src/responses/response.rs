use serde::{Serialize, Deserialize, de::DeserializeOwned};

use super::response_types::ApiResponseType;

pub trait ApiResponseTrait: DeserializeOwned {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub ratelimit_remaining: Option<i32>,
    pub response: ApiResponseType
}