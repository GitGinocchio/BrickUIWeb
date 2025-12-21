use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;
use worker::*;

#[derive(Deserialize, Serialize)]
struct VerifyRequestBody {
    token: String,

    redirect_to: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// TODO: Creare una lista di "valid redirects"

pub async fn get_confirm(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: VerifyRequestBody = req.query()?;

    let supabase_url = ctx.env.var("SUPABASE_URL")?;

    let verify_url = format!(
        "{}/auth/v1/verify?token={}&type=signup&redirect_to={}",
        supabase_url,
        body.token,
        urlencoding::encode(&body.redirect_to)
    );

    console_log!("Redirecting to verify URL: {}", verify_url);

    // Reindirizzamento 302
    Response::redirect(verify_url.parse()?)
}
