use super::api::Api;

pub struct ApiBuilder {}

impl ApiBuilder {
    pub fn set_key(key: String) -> Api {
        Api::from(key)
    }
}


