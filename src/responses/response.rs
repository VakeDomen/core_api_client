use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub trait ApiResponseTrait: DeserializeOwned {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub ratelimit_remaining: Option<i32>,
    pub response: T,
}