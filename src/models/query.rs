use std::fmt::format;

#[derive(Debug)]
pub enum FilterOperator {
    Smaller,
    Bigger,
    Eq,
    SmallerEq,
    BiggerEq,    
    Exists
}

#[derive(Debug)]
pub enum Query {
    DataProviders(String),
    Discovery,
    ExpertFinder,
    Journals(String),
    Outputs(String),
    Search(SearchType),
}

#[derive(Debug)]
pub enum SearchType {
    Works(SearchQuery),
    Outputs(SearchQuery),
    DataProviders(SearchQuery),
    Journals(SearchQuery),
}

#[derive(Debug)]
pub struct SearchQuery {
    filters: Vec<Filter>,
    limit: Option<i32>,
    offset: Option<i32>,
    scroll: Option<bool>,
    stats: Option<bool>
}

#[derive(Debug)]
enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum QueryRequestType {
    Get,
    Post,
}

#[derive(Debug)]
struct Filter {
    logical_operator: LogicalOperator,
    filter_operator: FilterOperator,
    key: String,
    value: Option<String>,
}

impl LogicalOperator {
    fn parse(self) -> String {
        match self {
            LogicalOperator::And => "%20AND%20".to_string(),
            LogicalOperator::Or => "%20OR%20".to_string(),
        }
    }
}

impl Filter {
    fn parse(self) -> (LogicalOperator, String) {
        let filter_query = match self.filter_operator {
            FilterOperator::Smaller => format!("{}<{}", self.key, self.value.unwrap_or("".to_string())),
            FilterOperator::Bigger => format!("{}>{}", self.key, self.value.unwrap_or("".to_string())),
            FilterOperator::Eq => format!("{}={}", self.key, self.value.unwrap_or("".to_string())),
            FilterOperator::SmallerEq => format!("{}<={}", self.key, self.value.unwrap_or("".to_string())),
            FilterOperator::BiggerEq => format!("{}>={}", self.key, self.value.unwrap_or("".to_string())),
            FilterOperator::Exists => format!("exists={}", self.key),
        };
        (self.logical_operator, filter_query)
    }
}

impl SearchQuery {
    pub fn paged(limit: i32, offset: i32) -> Self {
        Self { 
            filters: Default::default(), 
            limit: Some(limit), 
            offset: Some(offset), 
            scroll: None, 
            stats: None ,
        }
    }
}

impl Default for SearchQuery {
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
            let (op, query) = filter.parse();
            if final_filter.eq("?") {
                final_filter = format!("{}{}", final_filter, query);
            } else {
                final_filter = format!("{}{}{}", final_filter, op.parse(), query)
            }
        }
        final_filter
    }
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
    let query_string = query.parse();
    format!("search/{}/{}", route, query_string)
}