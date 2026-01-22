use serde::{Deserialize, Serialize};
use serde_json::Value;
use urlencoding::encode;
use std::{collections::HashMap, str::FromStr as _};
use reqwest::Client;
use worker::*;

use crate::{CLIENT, api::users, errors::json_error};

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

pub async fn post_refresh(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: RefreshRequest = req.json().await?;

    // variabili d'ambiente
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

    let payload = serde_json::json!({
        "refresh_token" : body.refresh_token
    });

    let response = CLIENT
        .post(format!("{}/auth/v1/token?grant_type=refresh_token", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
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