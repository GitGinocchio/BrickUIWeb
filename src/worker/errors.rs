use serde::Serialize;
use serde_json::{Value, json};
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
    let body = json!({
        "code": code,
        "error_code": error_code,
        "msg": msg
    });

    Ok(Response::from_json(&body)?
        .with_status(code))
}

pub fn json_error_with_data(
    code: u16,
    error_code: &'static str,
    msg: &'static str,
    extra: &[(impl AsRef<str>, Value)],
) -> Result<Response> {
    // JSON base
    let mut body = json!({
        "code": code,
        "error_code": error_code,
        "msg": msg
    });

    // Aggiungi campi extra
    for (key, value) in extra {
        body[key.as_ref()] = value.clone();
    }

    Ok(Response::from_json(&body)?
        .with_status(code))
}
