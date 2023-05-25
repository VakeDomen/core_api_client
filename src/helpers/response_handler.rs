use reqwest::{blocking::Response, StatusCode};

use crate::{responses::search::SearchResponse, models::work::Work};

pub(crate) fn parse_response(resp: Response) -> Result<SearchResponse<Work>, crate::errors::Error> {
    match resp.status() {
        StatusCode::UNAUTHORIZED => return Err(crate::errors::Error::InvalidApiKey),
        StatusCode::INTERNAL_SERVER_ERROR => return Err(extract_text(resp)),
        _ => ()
    };
    // Get the response text
    let resp_text = match resp.text() {
        Ok(t) => t,
        Err(e) => return Err(crate::errors::Error::RequestError(e)),
    };

    // // Print the response text
    // println!("Response text: {}", &resp_text);

    // Try to parse the response text into a Work struct
    match serde_json::from_str(&resp_text) {
        Ok(data) => Ok(data),
        Err(e) => Err(crate::errors::Error::ParsingError(e.to_string())),
    }


}


fn extract_text(resp: Response) -> crate::errors::Error {
    match resp.text() {
        Ok(t) => crate::errors::Error::ParsingError(t),
        Err(e) => crate::errors::Error::RequestError(e),
    }
}