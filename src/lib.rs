mod models;
mod errors;
mod helpers;
mod responses;

pub use models::api::Api;
pub use models::query_models::search_query::SearchQuery;
pub use models::query_models::filter_operator::FilterOperator;
pub use responses::responses::ApiResponseType;
pub use models::work_models::*;


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
