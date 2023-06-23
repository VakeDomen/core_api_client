use super::{logical_operator::LogicalOperator, filter_operator::FilterOperator};


#[derive(Debug, Clone)]
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
