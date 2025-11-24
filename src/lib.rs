use worker::*;

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

    let url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;

    // Prova a servire asset statici (JS/CSS/img)
    if let Ok(asset) = assets.fetch(url.path(), None).await {
        return Ok(asset);
    }

    // Fallback SPA: ritorna index.html
    if let Ok(index) = assets.fetch("index.html", None).await {
        return Ok(index);
    }

    Response::error("Webapp not found!", 404)
}

pub async fn not_found_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::error("Not Found", 404)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/", webapp)
        .get_async("/api", api_handler)
        .on_async("/*path", not_found_handler)
        .run(req, env)
        .await
}
