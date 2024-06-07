mod helpers;
mod models;
pub mod errors;
pub mod responses;

pub use models::api::Api;
pub use models::query_models::search_query::SearchQuery;
pub use models::query_models::filter_operator::FilterOperator;
pub use models::work_models::work::Work;
pub use models::data_provider_models::data_provider::DataProvider;
pub use models::discovery_models::discovery::Discovery;
pub use models::journal_models::journal::Journal;

#[cfg(test)]
mod tests {
    use crate::{models::{
        discovery_models::discovery::Discovery, 
        data_provider_models::data_provider::DataProvider,
        journal_models::journal::Journal, 
        query_models::query::Query,
        work_models::work::Work
    }, responses::response::ApiResponse};

    use static_assertions::assert_impl_all;

    #[test]
    fn test_send_sync_discovery() {
        assert_impl_all!(Discovery: Send, Sync);
    }

    #[test]
    fn test_send_sync_data_provider() {
        assert_impl_all!(DataProvider: Send, Sync);
    }

    #[test]
    fn test_send_sync_journal() {
        assert_impl_all!(Journal: Send, Sync);
    }

    #[test]
    fn test_send_sync_query() {
        assert_impl_all!(Query<String, String>: Send, Sync);
    }

    #[test]
    fn test_send_sync_work() {
        assert_impl_all!(Work: Send, Sync);
    }

    #[test]
    fn test_send_sync_api_response() {
        assert_impl_all!(ApiResponse<Work>: Send, Sync);
    }
}
