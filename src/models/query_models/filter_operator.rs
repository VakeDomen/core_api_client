#[derive(Debug, Clone)]
pub enum FilterOperator<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    Smaller(T1, T2),
    Bigger(T1, T2),
    Eq(T1, T2),
    SmallerEq(T1, T2),
    BiggerEq(T1, T2),
    Exists(T1),
    HasValue(T1, T2),
}


impl<T1, T2> FilterOperator<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub(crate) fn parse(self) -> String {
        match self {
            FilterOperator::Smaller(key, value) => format!("{}<{}", key.to_string(), value.to_string()),
            FilterOperator::Bigger(key, value) => format!("{}>{}", key.to_string(), value.to_string()),
            FilterOperator::Eq(key, value) => format!("{}={}", key.to_string(), value.to_string()),
            FilterOperator::SmallerEq(key, value) => format!("{}<={}", key.to_string(), value.to_string()),
            FilterOperator::BiggerEq(key, value) => format!("{}>={}", key.to_string(), value.to_string()),
            FilterOperator::Exists(key) => format!("_exists_:{}", key.to_string()),
            FilterOperator::HasValue(key, value) => format!("{}:{}", key.to_string(), value.to_string()),
        }
    }
}
