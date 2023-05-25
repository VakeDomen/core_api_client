use core_api_rs::{ApiBuilder, Query};

fn main() {
    let mut api = ApiBuilder::set_key("DCrZJjaUtFd1KHg3zqbRTYelO9Xs26IM".to_string());
    let resp = api.execute_query(Query{});
    // println!("{:#?}", resp);
}