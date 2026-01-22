use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding::encode;
use std::{collections::HashMap, str::FromStr as _};
use reqwest::Client;
use worker::*;

use crate::{CLIENT, api::users, errors::json_error};

#[derive(Deserialize, Serialize)]
struct ClassicRegisterRequest {
    email: Option<String>,
    phone: Option<String>,

    password: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub async fn post_classic(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: ClassicRegisterRequest = req.json().await?;
    let user_agent = req.headers().get("user-agent").unwrap_or_default();

    // variabili d'ambiente
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;
    let supabase_anon_key = ctx.env.var("SUPABASE_ANON_KEY")?;

    // user_metadata
    let mut user_metadata = serde_json::Map::new();
    for (k, v) in body.extra.into_iter() {
        user_metadata.insert(k, v);
    }

    // redirect email in base al client
    let email_redirect_to = if let Some(ua) = user_agent.as_ref() {
        if ua.starts_with("BrickUIApp") {
            "brickui://"
        } else {
            "https://brickui.app"
        }
    } else {
        "https://brickui.app"
    };

    // payload base per signup
    let mut payload = serde_json::json!({
        "password": body.password,
        "options": {
            "data": user_metadata,
            "emailRedirectTo": email_redirect_to,
            "email_redirect_to": email_redirect_to
        }
    });

    // check email/phone
    let users_json = CLIENT
        .get(format!("{}/auth/v1/admin/users", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send()
        .await
        .map_err(|e| format!("Error querying users: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Error obtaining text response: {e}"))?;

    let users_map: Value = serde_json::from_str(&users_json)?;
    let users = users_map["users"].as_array().ok_or("Unexpected response format")?;

    // filtro lato Rust
    if let Some(email) = body.email.as_ref() {
        let email = email.trim();
        payload["email"] = Value::String(email.to_string());
        if users.iter().any(|u| u["email"].as_str() == Some(email)) {
            return json_error(409, "user_exists", "An account with this email already exists");
        }
    } else if let Some(phone) = body.phone.as_ref() {
        let phone = phone.trim();
        payload["phone"] = Value::String(phone.to_string());
        if users.iter().any(|u| u["phone"].as_str() == Some(phone)) {
            return json_error(409, "user_exists", "An account with this phone number already exists");
        }
    } else {
        return json_error(400, "missing_fields", "Email or phone required");
    }

    // signup vero e proprio
    let response = CLIENT
        .post(format!("{}/auth/v1/signup", supabase_url))
        .header("apikey", supabase_anon_key.to_string())
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?;

    let status_code = response.status().as_u16();
    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    Ok(Response::from_body(ResponseBody::Body(response_body.into_bytes()))
        .map_err(|e| format!("Error creating response: {e}"))?
        .with_status(status_code))
}

pub async fn get_google_start(mut _req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let redirect_uri = ctx.env.var("GOOGLE_REDIRECT_URI")?;

    let auth_url = Url::from_str(&format!("{supabase_url}/auth/v1/authorize?provider=google&redirect_to={redirect_uri}&response_type=code"))?;

    console_log!("auth_url:{auth_url}");

    Response::redirect(auth_url)
}

pub async fn get_google_callback(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::empty()
}