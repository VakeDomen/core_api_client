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
}

#[derive(Debug, Clone)]
pub enum Query<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    DataProviders(String),
    Discovery,
    ExpertFinder,
    Journals(String),
    Outputs(String),
    SearchWorks(SearchQuery<T1, T2>),
    SearchOutputs(SearchQuery<T1, T2>),
    SearchDataProviders(SearchQuery<T1, T2>),
    SearchJournals(SearchQuery<T1, T2>),
}

#[derive(Debug, Clone)]
pub struct SearchQuery<T1, T2 = String>
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

#[derive(Debug, Clone)]
enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum QueryRequestType {
    Get,
    Post,
}

#[derive(Debug, Clone)]
struct Filter<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    logical_operator: LogicalOperator,
    filter_operator: FilterOperator<T1, T2>,
}

impl<T1, T2> Filter<T1, T2>
where
    T1: ToString,
    T2: ToString,
{
    fn parse(self) -> String {
        format!(
            "{}{}", 
            self.logical_operator.parse(), 
            self.filter_operator.parse()
        )
    }
}

impl LogicalOperator {
    fn parse(self) -> String {
        match self {
            LogicalOperator::And => "%20AND%20".to_string(),
            LogicalOperator::Or => "%20OR%20".to_string(),
        }
    }
}

impl<T1, T2> FilterOperator<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    fn parse(self) -> String {
        match self {
            FilterOperator::Smaller(key, value) => format!("{}<{}", key.to_string(), value.to_string()),
            FilterOperator::Bigger(key, value) => format!("{}>{}", key.to_string(), value.to_string()),
            FilterOperator::Eq(key, value) => format!("{}={}", key.to_string(), value.to_string()),
            FilterOperator::SmallerEq(key, value) => format!("{}<={}", key.to_string(), value.to_string()),
            FilterOperator::BiggerEq(key, value) => format!("{}>={}", key.to_string(), value.to_string()),
            FilterOperator::Exists(key) => format!("exists={}", key.to_string()),
        }
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

impl<T1, T2> SearchQuery<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub fn and(mut self, operator: FilterOperator<T1, T2>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::And, 
            filter_operator: operator, 
        });
        self
    }
    pub fn or(mut self, operator: FilterOperator<T1, T2>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::Or, 
            filter_operator: operator, 
        });
        self
    }
    fn parse(self) -> String {
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

        if self.filters.len() < 1 {
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


impl<T1, T2> Query<T1, T2>
where
    T1: ToString,
    T2: ToString, 
{
    pub fn parse_request(self) -> (QueryRequestType, String) {
        match self {
            Query::DataProviders(id) => (QueryRequestType::Get, format!("data-providers/{}", id)),
            Query::Discovery => (QueryRequestType::Post, "discover".to_string()),
            Query::ExpertFinder => (QueryRequestType::Post, "labs/expert-finder".to_string()),
            Query::Journals(id) => (QueryRequestType::Get, format!("journals/{}", id)),
            Query::Outputs(id) => (QueryRequestType::Get, format!("outputs/{}", id)),
            Query::SearchWorks(sq) => (QueryRequestType::Get, format!("search/works/{}", sq.parse())),
            Query::SearchOutputs(sq) => (QueryRequestType::Get, format!("search/outputs/{}", sq.parse())),
            Query::SearchDataProviders(sq) => (QueryRequestType::Get, format!("search/data-providers/{}", sq.parse())),
            Query::SearchJournals(sq) => (QueryRequestType::Get, format!("search/journals/{}", sq.parse())),
        }
    }
}
