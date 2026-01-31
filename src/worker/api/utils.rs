use worker::*;

use crate::errors::json_error;

pub fn extract_bearer_token(req: &Request) -> Result<String, Result<Response, Error>> {
    let authorization = req.headers().get("Authorization").unwrap_or(None);

    match authorization {
        Some(h) if h.trim().starts_with("Bearer ") => {
            Ok(h.trim()[7..].to_string())
        }
        _ => Err(json_error(401, "unauthorized", "You must be authorized to use this route")),
    }
}