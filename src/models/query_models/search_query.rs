
use crate::FilterOperator;

use super::{logical_operator::LogicalOperator, filter::Filter};

/// `SearchQuery` is a structure that represents a search query to the API. It allows the user to define the criteria
/// to filter data from the API.
///
/// `SearchQuery` contains various search parameters such as `limit`, `offset`, `scroll` and `stats` that can be 
/// optionally set for advanced search operations. 
///
/// A search operation can be composed of multiple `Filter` conditions that can be linked using logical AND/OR 
/// operators. Each `Filter` condition consists of a `FilterOperator` that defines the type of comparison to be made.
///
/// # Example
/// ```
/// use core_api_rs::FilterOperator;
/// use core_api_rs::Api;
/// 
/// let api = Api::from("API_KEY");
/// 
/// let query = api.paged_search(10, 0)
///    .and(FilterOperator::Exists("doi"))
///    .and(FilterOperator::Bigger("citationCount", 20));
/// 
/// let resp = api.search_works(query);
/// ```
/// 
/// # Fields
/// * `filters`: A vector of `Filter` structs that represent the conditions of the search query.
/// * `limit`: The maximum number of results to return. Defaults to None.
/// * `offset`: The number of results to skip before starting to fetch. Defaults to None.
/// * `scroll`: Enable/disable the scrolling functionality. Defaults to None.
/// * `stats`: Enable/disable the statistics functionality. Defaults to None.
///
/// # Methods
/// * `and`: Adds a new filter condition with a logical AND operator.
/// * `or`: Adds a new filter condition with a logical OR operator.
/// * `parse`: Parses the `SearchQuery` object into a string to be used in the API request.
#[derive(Debug, Clone)]
pub struct SearchQuery<T1 = String, T2 = String>
where
    T1: ToString,
    T2: ToString,
{
    filters: Vec<Filter<T1, T2>>,
    limit: Option<i32>,
    offset: Option<i32>,
    scroll: Option<bool>,
    stats: Option<bool>
}

impl<T1, T2> SearchQuery<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    /// Adds a filter to the `SearchQuery` with an AND logical operator.
    ///
    /// # Arguments
    ///
    /// * `operator: FilterOperator<T1, T2>` - The filter operator and its associated values to be added to the search query.
    ///
    /// # Example
    ///
    /// ```
    /// use core_api_rs::{Api, SearchQuery, FilterOperator};
    /// 
    /// let api = Api::from("API_KEY");
    /// let query = api.paged_search(10, 0)
    ///     .and(FilterOperator::Exists("doi"))
    ///     .and(FilterOperator::Bigger("citationCount", 20));
    /// ```
    pub fn and(mut self, operator: FilterOperator<T1, T2>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::And, 
            filter_operator: operator,
        });
        self
    }

    /// Adds a filter to the `SearchQuery` with an OR logical operator.
    ///
    /// # Arguments
    ///
    /// * `operator: FilterOperator<T1, T2>` - The filter operator and its associated values to be added to the search query.
    ///
    /// # Example
    ///
    /// ```
    /// use core_api_rs::{Api, SearchQuery, FilterOperator};
    /// 
    /// let api = Api::from("API_KEY");
    /// let query = api.paged_search(10, 0)
    ///     .or(FilterOperator::Exists("doi"))
    ///     .or(FilterOperator::Bigger("citationCount", 20));
    /// ```
    pub fn or(mut self, operator: FilterOperator<T1, T2>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::Or, 
            filter_operator: operator, 
        });
        self
    }

    /// Converts the `SearchQuery` instance into a string that represents a valid URL query string. 
    ///
    /// This method concatenates all added filters with their corresponding logical operators, and includes
    /// additional parameters like `limit`, `offset`, `scroll` and `stats`, if they are present.
    ///
    /// # Returns
    ///
    /// * `String` - The resulting URL query string.
    ///
    /// # Example
    ///
    /// ```
    /// use core_api_rs::{Api, SearchQuery, FilterOperator};
    /// 
    /// let api = Api::from("API_KEY");
    /// let query = api.paged_search(10, 0)
    ///     .and(FilterOperator::Eq("publisher", "OJS"));
    ///
    /// assert_eq!("?limit=10&offset=0&q=%20AND%20publisher=OJS".to_string(), query.parse());
    /// ```
    pub fn parse(self) -> String {
        let mut final_filter = "?".to_string();
        if let Some(l) = self.limit {
            final_filter = format!("{}limit={}", final_filter, l);
        }
        if let Some(o) = self.offset {
            final_filter = format!("{}&offset={}", final_filter, o);
        }
        if let Some(s) = self.scroll {
            final_filter = format!("{}&scroll={}", final_filter, s);
        }
        if let Some(s) = self.stats {
            final_filter = format!("{}&stats={}", final_filter, s);
        }

        if self.filters.is_empty() {
            return final_filter
        } else {
            final_filter = format!("{}&q=", final_filter);
        }
        for filter in self.filters.into_iter() {
            let query = filter.parse();
            final_filter = format!("{}{}", final_filter, query);
        }
        final_filter
    }
}


impl<T1, T2> SearchQuery <T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub(crate) fn paged(limit: i32, offset: i32) -> Self {
        Self { 
            filters: Default::default(), 
            limit: Some(limit), 
            offset: Some(offset), 
            scroll: None, 
            stats: None ,
        }
    }
}

impl<T1, T2> Default for SearchQuery<T1, T2>
where
    T1: ToString,
    T2: ToString,  
{
    fn default() -> Self {
        Self { 
            filters: Default::default(), 
            limit: None, 
            offset: None, 
            scroll: None, 
            stats: None ,
        }
    }
}

