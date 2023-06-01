use core_api_rs::{ApiBuilder, Query, SearchType, SearchQuery, FilterOperator, Work};

fn main() {
    let mut api = ApiBuilder::set_key("DCrZJjaUtFd1KHg3zqbRTYelO9Xs26IM".to_string());
    let setup_query = SearchQuery::paged(3, 1)
        .and(
            FilterOperator::Exists, 
            "doi".to_string(), 
            None
        );

    let query = Query::Search(SearchType::Works(setup_query));

    let resp = query.request();

    // let resp = api.execute_query::<Work>(query);
    println!("{:#?}", resp);
}