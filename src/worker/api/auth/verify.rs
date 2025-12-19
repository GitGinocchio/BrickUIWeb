use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use worker::*;

#[derive(Deserialize, Serialize)]
struct VerifyRequestBody {
    token: String,

    redirect_to: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

pub async fn get_verify(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: VerifyRequestBody = req.query()?;

    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_anon_key = ctx.env.var("SUPABASE_ANON_KEY")?;

    let redirect_to = urlencoding::encode(&body.redirect_to);

    let verify_url = format!(
        "{}/auth/v1/verify?token={}&type=signup&redirect_to={}",
        supabase_url,
        body.token,
        redirect_to
    );

    console_log!("Verify URL: {}", verify_url);

    let client = Client::new();
    let resp = client
        .get(verify_url)
        .header("apikey", supabase_anon_key.to_string())
        .send()
        .await
        .map_err(|e| format!("Error calling Supabase: {e}"))?;

    let status_code = resp.status();

    console_log!("status_code: {}", status_code);

    Response::redirect(body.redirect_to.parse()?)
}
