
/// `FilterOperator` enum defines the types of comparison operations
/// that can be used in a `SearchQuery` filter.
///
/// This enum is parametrized over two types `T1` and `T2` where `T1` is
/// the key to be filtered upon and `T2` is the value against which the key 
/// will be compared. Both `T1` and `T2` must implement the `ToString` trait.
///
/// The operators provided by this enum include:
/// 
/// * `Smaller`: Checks if the value of a key is smaller than the provided value.
/// * `Bigger`: Checks if the value of a key is greater than the provided value.
/// * `Eq`: Checks if the value of a key equals the provided value.
/// * `SmallerEq`: Checks if the value of a key is smaller than or equal to the provided value.
/// * `BiggerEq`: Checks if the value of a key is greater than or equal to the provided value.
/// * `Exists`: Checks if a key exists in the data.
/// * `HasValue`: Checks if the value of a key equals the provided value.
///
/// ```rust
/// use core_api_rs::FilterOperator;
/// 
/// let filter = FilterOperator::Eq("age", 30);
/// ```
///
/// The string representation follows the convention of `key<operator>value` for most operators,
/// `_exists_:key` for the `Exists` operator, and `key:value` for the `HasValue` operator.

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
