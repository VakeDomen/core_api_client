use core_api_rs::{ApiBuilder, Query, SearchType, SearchQuery, FilterOperator, Work};

fn main() {
    let mut api = ApiBuilder::set_key("DCrZJjaUtFd1KHg3zqbRTYelO9Xs26IM".to_string());
    let setup_query = SearchQuery::default()
        .and(
            FilterOperator::Exists, 
            "doi".to_string(), 
            None
        )
        .and(
            FilterOperator::Eq, 
            "limit".to_string(),
            Some("1".to_string())
        );
    let query = Query::Search(SearchType::Works(setup_query));
    let resp = api.execute_query::<Work>(query);
    println!("{:#?}", resp);
}