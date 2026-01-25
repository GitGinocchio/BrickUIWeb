use resend_rs::{Resend, types::{CreateEmailBaseOptions, EmailTemplate}};
use std::{collections::HashMap, str::FromStr as _};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use serde_json::Value;
use urlencoding::encode;
use reqwest::Client;
use worker::*;


use crate::{CLIENT, api::{auth::{EmailData, UserIdentity}, users}, errors::{ApiError, json_error}};

#[derive(Deserialize, Serialize)]
struct ClassicRegisterRequest {
    email: Option<String>,
    phone: Option<String>,

    password: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub fn generate_username(identifier: &str, is_email: bool) -> String {
    let id_trimmed = identifier.trim();

    // Calcola hash SHA-256
    let mut hasher = Sha256::new();
    hasher.update(id_trimmed.as_bytes());
    let result = hasher.finalize();
    let hash_str = hex::encode(result);
    let short_hash = &hash_str[..8]; // primi 8 caratteri

    if is_email {
        // Prendi la parte prima della chiocciola
        let local_part = id_trimmed.split('@').next().unwrap_or("user");
        // Username: localpart o localpart_{hash} per sicurezza/unicità
        format!("{}_{}", local_part, short_hash)
    } else {
        // Registrazione con telefono
        format!("user_{}", short_hash)
    }
}

pub async fn post_classic(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: ClassicRegisterRequest = req.json().await?;
    let origin = req.url()?.origin().unicode_serialization();

    // variabili d'ambiente
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;
    let resend_key = ctx.env.var("RESEND_KEY")?;
    //let supabase_anon_key = ctx.env.var("SUPABASE_ANON_KEY")?;

    let username = if let Some(email) = &body.email {
        generate_username(email, true)
    }
    else if let Some(phone) = &body.phone {
        generate_username(phone, false)
    }
    else {
        return json_error(400, "missing_fields", "Email or phone required");
    };

    // user_metadata
    let mut user_metadata = serde_json::Map::new();
    user_metadata.insert("username".into(),Value::String(username.clone()));
    user_metadata.insert("display_name".into(),Value::String(username));
    for (k, v) in body.extra.into_iter() {
        user_metadata.insert(k, v);
    }

    // payload base per signup
    let mut payload = serde_json::json!({
        "type": "signup",
        "password": body.password,
        "data": user_metadata,
        "redirect_to": format!("{origin}/auth/confirmed")
    });

    // check email/phone
    let users_json: Value = CLIENT
        .get(format!("{}/auth/v1/admin/users", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send()
        .await
        .map_err(|e| format!("Error querying users: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Error obtaining text response: {e}"))?;

    if let Some(err) = ApiError::try_from_value(&users_json) {
        return err.into_response();
    }

    let users = users_json["users"].as_array().ok_or("Unexpected response format")?;

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

    if let Some(err) = ApiError::try_from_value(&users_json) {
        return err.into_response();
    }

    let email_data: EmailData = serde_json::from_value(response.clone())
        .map_err(|e| format!("Error deserializing json: {e}"))?;

    let user_data: UserIdentity = serde_json::from_value(response)
        .map_err(|e| format!("Error deserializing json: {e}"))?;

    if let Some(email_address) = &user_data.email {
        let resend = Resend::new(&resend_key.to_string());

        let mut variables = HashMap::<String, Value>::new();
        variables.insert("USER_NAME".into(), Value::String(email_address.clone()));
        variables.insert(
            "VERIFY_URL".into(), 
            Value::String(format!("{}/api/auth/confirm?token={}&redirect_to={}", email_data.redirect_to, email_data.hashed_token, "https://brickui.app/auth/confirmed"))
        );

        let template = EmailTemplate::new("confirm-registration").with_variables(variables);
        let opts = CreateEmailBaseOptions::new(
            "BrickUI <brickui@mail.brickui.app>", 
            vec![email_address], 
            "Almost done! Confirm your BrickUI account"
        ).with_template(template);

        let _email = resend.emails
            .send(opts)
            .await
            .map_err(|e| format!("Error sending email: {e}"))?;
    }

    Ok(Response::from_body(ResponseBody::Body(serde_json::to_vec(&user_data)?))
        .map_err(|e| format!("Error creating response: {e}"))?)

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