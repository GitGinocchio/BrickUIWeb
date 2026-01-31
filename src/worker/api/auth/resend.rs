use chrono::{DateTime, TimeDelta, Utc};
use serde::Deserialize;
use serde_json::Value;
use urlencoding::encode;
use worker::*;

use crate::{CLIENT, api::auth::{EmailData, UserIdentity}, errors::{ApiError, json_error, json_error_with_data}};
use crate::api::auth::utils::send_confirmation_email;

#[derive(Deserialize)]
pub struct ResendRequest {
    pub email: String,

    #[serde(default)]
    pub redirect_to: Option<String>,
}

// TODO: sostituire il resend con quello custom utilizzando resend
pub async fn post_resend(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let origin = req.url()?.origin().unicode_serialization();
    let body: ResendRequest = req.json().await?;
    
    let resend_key = ctx.env.var("RESEND_KEY")?;
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

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

    // payload base per resend
    let payload = serde_json::json!({
        "type": "signup",
        "email": body.email,
        "redirect_to": format!("{origin}/auth/confirmed")
    });

    let response: Value = CLIENT
        .post(format!("{}/auth/v1/admin/generate_link", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    if let Some(err) = ApiError::try_from_value(&response) {
        return err.into_response();
    }

    let email_data: EmailData = serde_json::from_value(response.clone())
        .map_err(|e| format!("Error deserializing json: {e}"))?;

    let user_data: UserIdentity = serde_json::from_value(response)
        .map_err(|e| format!("Error deserializing json: {e}"))?;
    
    let _email = send_confirmation_email(
        &resend_key.to_string(),
        origin, 
        &body.email, 
        email_data.hashed_token
    ).await?;

    Ok(Response::from_body(ResponseBody::Body(serde_json::to_vec(&user_data)?))
        .map_err(|e| format!("Error creating response: {e}"))?)
}
