use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use serde_path_to_error::deserialize;

use crate::errors::Error;


pub(crate) fn parse_raw_response(
    resp: Response
) -> Result<(String, Option<i32>), crate::errors::Error> {
    match resp.status() {
        StatusCode::UNAUTHORIZED => return Err(crate::errors::Error::InvalidApiKey),
        StatusCode::INTERNAL_SERVER_ERROR => return Err(extract_error(resp)),
        _ => ()
    };

    let rate_limit = extraxt_rate_limit(&resp);    

    // Get the response text
    match resp.text() {
        Ok(t) => Ok((t, rate_limit)),
        Err(e) => Err(crate::errors::Error::Request(e)),
    }
}

pub(crate) fn parse_json<T>(data: &str) -> Result<T, crate::errors::Error> where T: DeserializeOwned {
    let deserializer = &mut Deserializer::from_str(data);
    let res: Result<T, _> = deserialize(deserializer);
    match res {
        Ok(parsed_data) => Ok(parsed_data),
        Err(e) => Err(Error::Parsing(e.to_string())),
    }
}

fn extract_error(resp: Response) -> crate::errors::Error {
    match resp.text() {
        Ok(t) => crate::errors::Error::Parsing(t),
        Err(e) => crate::errors::Error::Request(e),
    }
}

fn extraxt_rate_limit(resp: &Response) -> Option<i32> {
    if let Some(rate) = resp.headers().get("x-ratelimit-remaining") {
        let rate_str = match rate.to_str() {
            Ok(s) => s,
            Err(_) => return None,
        };
        return rate_str.parse::<i32>().ok();
    }
    None
}