use std::collections::HashMap;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use worker::*;

#[derive(Deserialize)]
struct ClassicLoginRequest {
    email: String,
    password: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>, // campi extra, se in futuro servono
}

pub async fn post_login(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: ClassicLoginRequest = req.json().await?;

    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

    let mut payload = serde_json::json!({
        "email": body.email,
        "password": body.password,

        "user_metadata": {}
    });

    for (key, value) in body.extra.into_iter() {
        payload["user_metadata"][key] = value;
    }

    let client = Client::new();

    let response = client
        .post(format!("{}/auth/v1/token?grant_type=password", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?;

    let status_code = response.status().as_u16();
    let body = response.text().await.unwrap_or_else(|_| "Unknown error".into());

    let headers = Headers::new();
    headers.append("Content-Type", "application/json")?;

    Ok(Response::from_body(ResponseBody::Body(body.into_bytes()))
        .map_err(|e| format!("Error creating response: {e}"))?
        .with_status(status_code)
        .with_headers(headers))
}
