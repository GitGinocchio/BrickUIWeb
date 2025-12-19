use worker::*;

pub async fn post_logout(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let supabase_url = ctx.env.var("SUPABASE_URL")?;
    let supabase_key = ctx.env.var("SUPABASE_KEY")?;

    // Legge Authorization: Bearer <token>
    let auth_header = match req.headers().get("Authorization")? {
        Some(h) => h,
        None => return Response::error("Missing Authorization header", 401),
    };

    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/auth/v1/logout", supabase_url))
        .header("apikey", supabase_key.to_string())
        .header("Authorization", auth_header)   // token dell’utente
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