use serde::Serialize;
use serde_json::{Value, json};
use worker::*;



#[derive(Serialize)]
pub struct ApiError {
    pub code: u16,
    pub error_code: String,
    pub msg: String,
}

impl ApiError {
    pub fn try_from_value(value: &Value) -> Option<Self> {
        let code = value.get("code")?.as_u64()? as u16;

        let error_code = value
            .get("error_code")
            .and_then(|v| v.as_str())
            .unwrap_or("unhandled_error")
            .to_string();

        let msg = value
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("unhandled error")
            .to_string();

        Some(Self {
            code,
            error_code,
            msg,
        })
    }

    pub fn into_response(self) -> Result<Response> {
        json_error(self.code, &self.error_code, &self.msg)
    }
}


pub fn json_error(
    code: u16,
    error_code: &str,
    msg: &str,
) -> Result<Response> {
    let body = json!({
        "code": code,
        "error_code": error_code,
        "msg": msg
    });

    Ok(Response::from_json(&body)?.with_status(code))
}

pub fn json_error_with_data(
    code: u16,
    error_code: &str,
    msg: &str,
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

    Ok(Response::from_json(&body)?.with_status(code))
}
