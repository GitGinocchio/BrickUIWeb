use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr as _};
use reqwest::Client;
use worker::*;

#[derive(Deserialize, Serialize)]
struct ClassicRegisterRequest {
    email: String,
    password: String,
    phone: Option<String>,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub async fn classic(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: ClassicRegisterRequest = req.json().await?;
    
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

    let mut payload = serde_json::json!({
        "email": body.email,
        "password": body.password,
        // false = l'utente deve verificarsi, attualmente non supportiamo
        // la verifica, questo pero' ci permette eventualmente di farlo in futuro
        "phone": body.phone,
        "email_confirm": false,
        "phone_confirm": false,
        "user_metadata": {}
    });

    if let Some(phone) = body.phone {
        payload["phone"] = Value::String(phone);
    }

    for (key, value) in body.extra.into_iter() {
        payload["user_metadata"][key] = value;
    }

    let client = Client::new();
    let response = client
        .post(format!("{}/auth/v1/admin/users", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?;

    let status_code = response.status().as_u16();
    let body = response
        .text()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    Ok(Response::from_body(ResponseBody::Body(body.into_bytes()))
        .map_err(|e| format!("Error creating response: {e}"))?
        .with_status(status_code))
}

pub async fn google_start(mut _req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let redirect_uri = ctx.env.var("GOOGLE_REDIRECT_URI")?;

    let auth_url = Url::from_str(&format!("{supabase_url}/auth/v1/authorize?provider=google&redirect_to={redirect_uri}&response_type=code"))?;

    console_log!("auth_url:{auth_url}");

    Response::redirect(auth_url)
}

pub async fn google_callback(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::empty()
}