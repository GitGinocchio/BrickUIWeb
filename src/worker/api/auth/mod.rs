use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub mod register;
pub mod unregister;

pub mod login;
pub mod logout;

pub mod resend;
pub mod confirm;
pub mod refresh;


pub(crate) fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub app_metadata: HashMap<String, Value>,
    pub aud: String,
    pub confirmation_sent_at: Option<String>,
    pub email_confirmed_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub created_at: String,

    #[serde(deserialize_with = "empty_string_as_none")]
    pub email: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub phone: Option<String>,

    pub id: String,
    pub identities: Vec<Identity>,
    pub is_anonymous: bool,
    pub role: String,
    pub updated_at: Option<String>,
    pub user_metadata: HashMap<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub created_at: String,
    pub email: String,
    pub id: String,
    pub identity_data: IdentityData,
    pub identity_id: String,
    pub last_sign_in_at: Option<String>,
    pub provider: String,
    pub updated_at: Option<String>,
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IdentityData {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmailData {
    pub email_otp: String,
    pub hashed_token: String,
    pub redirect_to: String,
    pub verification_type: String,
    pub action_link: String
}