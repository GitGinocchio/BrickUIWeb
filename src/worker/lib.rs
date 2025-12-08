use std::future::Future;
use worker::*;

pub mod api;

#[event(start)]
fn init() {
    console_error_panic_hook::set_once();
}

async fn async_rate_limit<F, Fut>(
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
    let is_dev = match ctx.env.var("WORKER_ENV") {
        Ok(v) => v.to_string() == "dev",
        Err(_) => false, // fallback a production se non definito
    };

    if !is_dev {
        let api_rt = ctx.env.rate_limiter(binding)?;
        let outcome = api_rt.limit(key.into()).await?;
        if !outcome.success {
            return redirect_to_error(req, ctx, 429).await;
        }
    }

    handler(req, ctx).await
}

async fn redirect_to_error(req: Request, _ctx: RouteContext<()>, status_code: u16) -> Result<Response> {
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

        /* Register */
        .post_async(
            "/api/auth/register/classic",
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::classic, "API_RL", "api")
        )
        .get_async(
            "/api/auth/register/google/start", 
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::google_start, "API_RL", "api")
        )
        .get_async(
            "/api/auth/register/google/callback",
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::google_callback, "API_RL", "api")
        )

        /* Unregister */
        .delete_async(
            "/api/auth/unregister",
            |req, ctx| async_rate_limit(req, ctx, api::auth::unregister::unregister, "API_RL", "api")
        )

        /* Login */
        .post_async(
            "/api/auth/login",
            |req, ctx| async_rate_limit(req, ctx, api::auth::login::login, "API_RL", "api")
        )

        /* Logout */
        .post_async(
            "/api/auth/logout",
            |req, ctx| async_rate_limit(req, ctx, api::auth::logout::logout, "API_RL", "api")
        )

        /* Download */
        .get_async(
            "/api/download/latest",
            |req, ctx| async_rate_limit(req, ctx, api::download::latest::get, "API_RL", "api")
        )
        
        .or_else_any_method_async("/api/*path",|req, ctx| redirect_to_error(req, ctx, 404))
        .on_async("/*path", |req, ctx| redirect_to_error(req, ctx, 405))

        .run(req, env)
        .await
}
