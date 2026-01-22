use chrono::{DateTime, TimeDelta, Utc};
use serde::Deserialize;
use serde_json::Value;
use urlencoding::encode;
use reqwest::Client;
use worker::*;

use crate::{CLIENT, errors::{json_error, json_error_with_data}};

#[derive(Deserialize)]
pub struct ResendRequest {
    pub email: String,

    #[serde(default)]
    pub redirect_to: Option<String>,
}

pub async fn post_resend(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: ResendRequest = req.json().await?;

    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;
    let supabase_anon_key = ctx.env.var("SUPABASE_ANON_KEY")?;

    // 1. Recupera utente
    let query = format!(
        "{}/auth/v1/admin/users?email=eq.{}&select=confirmation_sent_at,confirmed_at",
        supabase_url,
        encode(&body.email)
    );

    let user_response = CLIENT
        .get(&query)
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send()
        .await
        .map_err(|e| format!("Error querying users: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Error obtaining text response: {e}"))?;

    let json_user_response: Value = serde_json::from_str(&user_response)?;

    let user = json_user_response["users"]
        .as_array()
        .and_then(|a| a.first())
        .ok_or("user_not_found")?;

    // 2. Se già confermato → errore
    if !user["confirmed_at"].is_null() {
        return json_error(400, "already_confirmed", "Email already verified");
    }

    let cooldown = TimeDelta::minutes(2);

    // 3. Cooldown (es. 2 minuti)
    if let Some(sent_at) = user["confirmation_sent_at"].as_str() {
        let sent = DateTime::parse_from_rfc3339(sent_at)
            .map_err(|e| format!("Error parsing date: {e}"))?
            .with_timezone(&Utc);
        
        let delta = cooldown - (Utc::now() - sent);
        let can_resend_in_seconds = if delta > TimeDelta::zero() {
            delta.num_seconds() as u64
        } else {
            0
        };

        if can_resend_in_seconds > 0 {
            return json_error_with_data(
                429,
                "too_many_requests",
                "Please wait before requesting another email",
                &[("cooldown", serde_json::Value::Number(can_resend_in_seconds.into()))],
            );
        }
    }

    // 4. Resend ufficiale
    let payload = serde_json::json!({
        "type": "signup",
        "email": body.email,
        "email_redirect_to": body.redirect_to
    });

    let resend_response = CLIENT
        .post(format!("{}/auth/v1/resend", supabase_url))
        .header("apikey", supabase_anon_key.to_string())
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|e| format!("Error: {e}"))?;

    let status_code = resend_response.status().as_u16();
    let response_body = resend_response
        .text()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    Ok(Response::from_body(ResponseBody::Body(response_body.into_bytes()))
        .map_err(|e| format!("Error creating response: {e}"))?
        .with_status(status_code))
}
