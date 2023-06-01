mod models;
mod errors;
mod helpers;
mod responses;

pub use models::api_builder::ApiBuilder;
pub use models::query::Query;
pub use models::query::SearchQuery;
pub use models::query::SearchType;
pub use models::query::FilterOperator;
pub use models::work::Work;

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
