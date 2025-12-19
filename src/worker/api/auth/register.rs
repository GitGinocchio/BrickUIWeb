use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr as _};
use reqwest::Client;
use worker::*;

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
    
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_anon_key = ctx.env.var("SUPABASE_ANON_KEY")?;

    let mut user_metadata = serde_json::Map::new();
    for (k, v) in body.extra.into_iter() {
        user_metadata.insert(k, v);
    }

    let mut payload = serde_json::json!({
        "password": body.password,
        "options": {
            "data": user_metadata
        }
    });

    if let Some(email) = body.email {
        payload["email"] = Value::String(email);
    }
    if let Some(phone) = body.phone {
        payload["phone"] = Value::String(phone);
    }

    let client = Client::new();
    let response = client
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