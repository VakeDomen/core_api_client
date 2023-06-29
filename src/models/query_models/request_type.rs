use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub(crate) enum QueryRequestType {
    Get,
    Post,
}

impl fmt::Display for QueryRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryRequestType::Get => write!(f, "Get"),
            QueryRequestType::Post => write!(f, "Post"),
        }
    }
}
