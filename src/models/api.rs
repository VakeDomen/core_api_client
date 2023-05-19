use super::query::Query;

pub struct Api {
    key: String,
    ratelimit_remaining: Option<i32>
}


impl Api {
    pub fn get_remainig_rate_limit(&self) -> Option<i32> {
        self.ratelimit_remaining.clone()
    }

    pub fn execute_query(query: Query) {
        
    }
}

impl From<String> for Api {
    fn from(key: String) -> Self {
        Api { key, ratelimit_remaining: None }
    }
}