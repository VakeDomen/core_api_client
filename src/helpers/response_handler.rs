use reqwest::{blocking::Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::responses::search::SearchResponse;

pub(crate) fn parse_response<T: DeserializeOwned>(
    resp: Response
) -> Result<(SearchResponse<T>, Option<i32>), crate::errors::Error> {
    match resp.status() {
        StatusCode::UNAUTHORIZED => return Err(crate::errors::Error::InvalidApiKey),
        StatusCode::INTERNAL_SERVER_ERROR => return Err(extract_error(resp)),
        _ => ()
    };

    let rate_limit = extraxt_rate_limit(&resp);    

    // Get the response text
    let resp_text = match resp.text() {
        Ok(t) => t,
        Err(e) => return Err(crate::errors::Error::RequestError(e)),
    };

    // Try to parse the response text into a Work struct
    match serde_json::from_str::<SearchResponse<T>>(&resp_text) {
        Ok(data) => Ok((data, rate_limit)),
        Err(e) => Err(crate::errors::Error::ParsingError(e.to_string())),
    }
}

fn extract_error(resp: Response) -> crate::errors::Error {
    match resp.text() {
        Ok(t) => crate::errors::Error::ParsingError(t),
        Err(e) => crate::errors::Error::RequestError(e),
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