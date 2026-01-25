use std::future::Future;
use serde_json::json;
use worker::*;

pub fn is_dev(env: &Env) -> bool {
    match env.var("WORKER_ENV") {
        Ok(v) => v.to_string() == "dev",
        Err(_) => false, // fallback a production se non definito
    }
}

pub async fn async_rate_limit<F, Fut>(
    req: Request,
    ctx: RouteContext<()>,
    handler: F,
    binding: &str,
    key: &str
) -> Result<Response>
where
    F: Fn(Request, RouteContext<()>) -> Fut,
    Fut: Future<Output = Result<Response>>
{
    if !is_dev(&ctx.env) {
        let api_rt = ctx.env.rate_limiter(binding)?;
        let outcome = api_rt.limit(key.into()).await?;
        if !outcome.success {
            return redirect_to_error(req, ctx, 429).await;
        }
    }

    handler(req, ctx).await
}

pub async fn redirect_to_error(req: Request, _ctx: RouteContext<()>, status_code: u16) -> Result<Response> {
    let wants_json = req
        .headers()
        .get("accept")
        .map_or(false, |ct| {
            if let Some(ct) = ct {
                return ct.to_lowercase().contains("application/json");
            }

            false
        });

    if wants_json {
        let payload = json!({
            "code": status_code,
            "status_code": status_code,
            "msg": format!("An error occurred: {}", status_code)
        });

        Response::from_json(&payload)
            .map(|res| res.with_status(status_code))
    } else {
        // Redirect classico
        let mut url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;
        url.set_path("/"); // redirect alla root
        url.set_query(Some(&format!("status_code={}", status_code)));

        Response::redirect(url)
    }
}