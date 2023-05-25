mod models;
mod errors;
mod helpers;
mod responses;

pub use models::api_builder::ApiBuilder;
pub use models::query::Query;

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
