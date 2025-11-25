use std::str::FromStr as _;

use worker::*;

pub mod api;


#[event(start)]
fn init() {
    console_error_panic_hook::set_once();
}

// Handler API
pub async fn api_handler(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let url = ctx.env.var("SUPABASE_URL")?;

    console_log!("url: {url}");

    Response::ok("api route!")
}

pub async fn webapp(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let assets = ctx.env.assets("ASSETS").map_err(|e| {
        worker::Error::RustError(format!("Assets binding error: {}", e))
    })?;

    let mut url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;

    // Prova a servire asset statici (JS/CSS/img)
    if let Ok(asset) = assets.fetch(url.path(), None).await {
        return Ok(asset);
    }

    url.set_path("/not-found");

    // Fallback SPA: ritorna index.html
    if let Ok(index) = assets.fetch(url, None).await {
        return Ok(index);
    }

    Response::error("Webapp not found!", 404)
}

pub async fn redirect_to_webapp(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let mut url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;

    url.set_path("/index.html");

    Response::redirect(url)
}

pub async fn method_not_allowed(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::error("Method not allowed", 405)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        // Api routes
        .post_async("/api/auth/register/classic", api::auth::register::classic)
        .get_async("/api/auth/register/google/start", api::auth::register::google_start)
        .get_async("/api/auth/register/google/callback", api::auth::register::google_callback)

        .get_async("/*path", webapp)

        .or_else_any_method_async("/*path", method_not_allowed)
        .run(req, env)
        .await
}
