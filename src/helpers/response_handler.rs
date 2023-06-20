use reqwest::{blocking::Response, StatusCode};


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
        Err(e) => Err(crate::errors::Error::RequestError(e)),
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