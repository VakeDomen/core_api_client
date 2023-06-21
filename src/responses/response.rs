use serde::{Serialize, Deserialize, de::DeserializeOwned};

use super::responses::ApiResponseType;

pub trait ApiResponseTrait: DeserializeOwned {}

// impl<T: DeserializeOwned> ApiResponseTrait for T {}


#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub ratelimit_remaining: Option<i32>,
    pub response: ApiResponseType
}