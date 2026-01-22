use base64::Engine;
use base64::engine::general_purpose::STANDARD as b64;
use serde_json::Value;
use worker::*;

use crate::CLIENT;

fn get_user_id_from_jwt(token: &str) -> Option<String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 { return None; }

    let decoded = b64.decode(parts[1]).ok()?;
    let json: Value = serde_json::from_slice(&decoded).ok()?;
    json.get("sub")?.as_str().map(|s| s.to_string())
}

pub async fn delete_unregister(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

    // Legge Authorization: Bearer <token>
    let auth_header = match req.headers().get("Authorization")? {
        Some(h) => h,
        None => return Response::error("Missing Authorization header", 401),
    };

    let token = auth_header.replace("Bearer ", "");
    let user_id = match get_user_id_from_jwt(&token) {
        Some(id) => id,
        None => return Response::error("Invalid JWT", 400),
    };

    let response = CLIENT
        .delete(format!("{}/auth/v1/admin/users/{}", supabase_url, user_id))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key)) // service key
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?;

    let status = response.status().as_u16();
    let body = response.text().await.unwrap_or_else(|_| "Unknown error".into());

    let headers = Headers::new();
    headers.append("Content-Type", "application/json")?;

    Ok(Response::from_body(ResponseBody::Body(body.into_bytes()))
        .unwrap()
        .with_status(status)
        .with_headers(headers))
}
