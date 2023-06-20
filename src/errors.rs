#[derive(Debug)]
pub enum Error {
    InvalidApiKey,
    RequestError(reqwest::Error),
    ParsingError(String),

}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::ParsingError(err.to_string())
    }
}