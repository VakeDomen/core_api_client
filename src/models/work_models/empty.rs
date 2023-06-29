use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Hash, Default)]
pub struct Empty {}