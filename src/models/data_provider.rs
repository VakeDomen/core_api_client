use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataProvider {
    id: Option<i32>,
    name: Option<String>,
    url: Option<String>, 
    logo: Option<String>,
}