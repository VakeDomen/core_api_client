#[derive(Debug)]
pub enum Error {
    InvalidApiKey,
    Request(reqwest::Error),
    Parsing(String),

}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Parsing(err.to_string())
    }
}