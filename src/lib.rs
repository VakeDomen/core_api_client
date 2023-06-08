mod models;
mod errors;
mod helpers;
mod responses;

pub use models::api::Api;
pub use models::query::Query;
pub use models::query::SearchQuery;
pub use models::query::FilterOperator;
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
