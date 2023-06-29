use serde::{Serialize, Deserialize};
use std::fmt;

use super::{logical_operator::LogicalOperator, filter_operator::FilterOperator};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Hash)]
pub(crate) struct Filter<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub(crate) logical_operator: LogicalOperator,
    pub(crate) filter_operator: FilterOperator<T1, T2>,
}

impl<T1, T2> Filter<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    pub(crate) fn parse(self) -> String {
        format!(
            "{}{}", 
            self.logical_operator.parse(), 
            self.filter_operator.parse()
        )
    }
}


impl<T1, T2> fmt::Display for Filter<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Assuming LogicalOperator and FilterOperator both implement Display or ToString.
        write!(f, "Filter {{ logical_operator: {}, filter_operator: {} }}", 
               self.logical_operator.to_string(), 
               self.filter_operator.to_string())
    }
}