use core_api_rs::{Api, Query, SearchQuery, FilterOperator, work::Work};

fn main() {
    let mut api = Api::from("DCrZJjaUtFd1KHg3zqbRTYelO9Xs26IM");
    let setup_query = SearchQuery::paged(3, 1)
        .and(FilterOperator::Exists("doi".to_string()))
        .and(FilterOperator::Smaller("citations".into(), 10))
        .or(FilterOperator::Bigger("citations".to_owned(), 5));

    let query = Query::SearchWorks(setup_query);
    let resp = api.execute_query::<Work, _, _>(query);
    println!("{:#?}", resp);
}