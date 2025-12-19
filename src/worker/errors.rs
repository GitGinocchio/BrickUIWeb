use serde::Serialize;
use worker::*;



#[derive(Serialize)]
pub struct ApiError {
    pub code: u16,
    pub error_code: &'static str,
    pub msg: &'static str,
}

pub fn json_error(
    code: u16,
    error_code: &'static str,
    msg: &'static str,
) -> Result<Response> {
    let body = serde_json::json!({
        "code": code,
        "error_code": error_code,
        "msg": msg
    });

    Ok(Response::from_json(&body)?
        .with_status(code))
}