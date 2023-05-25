#[derive(Debug)]
pub enum Error {
    InvalidApiKey,
    RequestError(reqwest::Error),
    ParsingError(String),
}