#[derive(Debug, Clone)]
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