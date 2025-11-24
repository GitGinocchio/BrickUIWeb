use worker::*;

#[event(start)]
fn init() {
    console_error_panic_hook::set_once();
}

// Handler API
pub async fn get(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::empty()
}

pub async fn webapp(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let assets = ctx.env.assets("ASSETS").map_err(|e| {
        worker::Error::RustError(format!("Assets binding error: {}", e))
    })?;

    let url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;

    console_debug!("{url}");

    // Prova a servire asset statici (JS/CSS/img)
    if let Ok(asset) = assets.fetch(url.path(), None).await {
        return Ok(asset);
    }

    // Fallback SPA: ritorna index.html
    if let Ok(index) = assets.fetch("index.html", None).await {
        return Ok(index);
    }

    Response::error("Asset non trovato", 404)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/api", get)
        .get_async("/*path", webapp)
        .run(req, env)
        .await
}
