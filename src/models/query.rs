pub enum FilterOperator {
    Smaller,
    Bigger,
    Eq,
    SmallerEq,
    BiggerEq,    
    Exists
}

pub enum Query {
    DataProviders(String),
    Discovery,
    ExpertFinder,
    Journals(String),
    Outputs(String),
    Search(SearchType),
}

pub enum SearchType {
    Works(SearchQuery),
    Outputs(SearchQuery),
    DataProviders(SearchQuery),
    Journals(SearchQuery),
}

pub struct SearchQuery {
    filters: Vec<Filter>
}

enum LogicalOperator {
    And,
    Or,
}
struct Filter {
    logical_operator: LogicalOperator,
    filter_operator: FilterOperator,
    key: String,
    value: Option<String>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self { filters: Default::default() }
    }
}

impl SearchQuery {
    pub fn and(mut self, operator: FilterOperator, key: String, value: Option<String>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::And, 
            filter_operator: operator, 
            key, 
            value 
        });
        self
    }
    pub fn or(mut self, operator: FilterOperator, key: String, value: Option<String>) -> Self {
        self.filters.push(Filter { 
            logical_operator: LogicalOperator::Or, 
            filter_operator: operator, 
            key, 
            value 
        });
        self
    }
}

pub enum QueryRequestType {
    Get,
    Post,
}

impl Query {
    pub fn request(self) -> (QueryRequestType, String) {
        match self {
            Query::DataProviders(id) => (QueryRequestType::Get, format!("data-providers/{}", id)),
            Query::Discovery => (QueryRequestType::Post, "discover".to_string()),
            Query::ExpertFinder => (QueryRequestType::Post, "labs/expert-finder".to_string()),
            Query::Journals(id) => (QueryRequestType::Get, format!("journals/{}", id)),
            Query::Outputs(id) => (QueryRequestType::Get, format!("outputs/{}", id)),
            Query::Search(s) => (QueryRequestType::Get, parse_search_query(s)),
        }
    }
}

fn parse_search_query(st: SearchType) -> String {
    let (route, query) = match st {
        SearchType::Works(sq) => ("works".to_string(), sq),
        SearchType::Outputs(sq) => ("outputs".to_string(), sq),
        SearchType::DataProviders(sq) => ("data-providers".to_string(), sq),
        SearchType::Journals(sq) => ("journals".to_string(), sq),
    };
    "search".to_string()
}