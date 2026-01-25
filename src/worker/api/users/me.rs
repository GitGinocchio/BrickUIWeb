use worker::*;

use serde_json::Value;

use crate::{CLIENT, api::{auth::UserIdentity, utils::extract_bearer_token}, errors::{ApiError, json_error}};

pub async fn get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let supabase_url = ctx.env.var("SUPABASE_URL")?.to_string();
    let supabase_key = ctx.env.var("SUPABASE_KEY")?.to_string();

    let token = match extract_bearer_token(&req) {
        Ok(token) => token,
        Err(error) => return error
    };

    let response: Value = CLIENT
        .get(format!("{supabase_url}/auth/v1/user"))
        .header("Authorization", format!("Bearer {token}"))
        .header("apikey", &supabase_key)
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    if let Some(err) = ApiError::try_from_value(&response) {
        return err.into_response();
    }

    let identity: UserIdentity= serde_json::from_value(response)
        .map_err(|e| format!("Error deserializing autority: {e}"))?;

    let response: Value = CLIENT
        .get(format!("{}/rest/v1/users?id=eq.{}", supabase_url, identity.id))
        .header("apikey", supabase_key)
        .send()
        .await
        .map_err(|e| format!("Error sending request: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Error obtaining response body: {e}"))?;

    if let Some(err) = ApiError::try_from_value(&response) {
        return err.into_response();
    }

    let array = match response.as_array() {
        Some(array) => array,
        None => return json_error(404, "not_found", "User not found")
    };

    let user = match array.get(0) {
        Some(user) => user,
        None => return json_error(404, "not_found", "User not found")
    };

    // Esito positivo: restituisci il primo elemento
    Ok(Response::from_json(user)?.with_status(200))
}