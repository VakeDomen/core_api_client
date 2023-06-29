use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub(crate) enum LogicalOperator {
    And,
    Or,
}

impl LogicalOperator {
    pub(crate) fn parse(self) -> String {
        match self {
            LogicalOperator::And => "%20AND%20".to_string(),
            LogicalOperator::Or => "%20OR%20".to_string(),
        }
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "And"),
            LogicalOperator::Or => write!(f, "Or"),
        }
    }
}
