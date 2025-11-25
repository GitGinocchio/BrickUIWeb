use worker::*;

pub mod api;

#[event(start)]
fn init() {
    console_error_panic_hook::set_once();
}

pub async fn redirect_to_error(req: Request, _ctx: RouteContext<()>, status_code: u16) -> Result<Response> {
    let mut url = req.url().map_err(|e| worker::Error::RustError(format!("Url error: {e}")))?;

    let query = format!("status_code={status_code}");
    url.set_query(Some(&query));
    url.set_path("/");

    Response::redirect(url)
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        // Api routes
        .post_async("/api/auth/register/classic", api::auth::register::classic)
        .get_async("/api/auth/register/google/start", api::auth::register::google_start)
        .get_async("/api/auth/register/google/callback", api::auth::register::google_callback)
        
        .or_else_any_method_async("/api/*path",|req, ctx| redirect_to_error(req, ctx, 404))
        .on_async("/*path", |req, ctx| redirect_to_error(req, ctx, 405))

        .run(req, env)
        .await
}
