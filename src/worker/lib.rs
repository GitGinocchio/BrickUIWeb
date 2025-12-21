use worker::*;

pub mod errors;
pub mod api;
pub mod utils;
pub mod storage;

use crate::utils::async_rate_limit;
use crate::utils::redirect_to_error;

#[event(start)]
fn init() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        // Api routes

        // Register
        .post_async(
            "/api/auth/register/classic",
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::post_classic, "API_RL", "api")
        )
        .get_async(
            "/api/auth/register/google/start", 
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::get_google_start, "API_RL", "api")
        )
        .get_async(
            "/api/auth/register/google/callback",
            |req, ctx| async_rate_limit(req, ctx, api::auth::register::get_google_callback, "API_RL", "api")
        )
        
        // Confirm

        .get_async(
            "/api/auth/confirm",
            |req, ctx| async_rate_limit(req, ctx, api::auth::confirm::get_confirm, "API_RL", "api")
        )

        // Resend

        .post_async(
            "/api/auth/resend",
            |req, ctx| async_rate_limit(req, ctx, api::auth::resend::post_resend, "API_RL", "api")
        )

        // Bricks - Meta (GET/POST)

        .get_async(
            "/api/bricks/:brick_id/:version/meta",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::meta::get, "API_RL", "api")
        )

        .post_async(
            "/api/bricks/:brick_id/:version/meta",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::meta::post, "API_RL", "api")
        )

        // Bricks - Media (GET/POST)

        .get_async(
            "/api/bricks/:brick_id/:version/media",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::media::get, "API_RL", "api")
        )

        .post_async(
            "/api/bricks/:brick_id/:version/media",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::media::post, "API_RL", "api")
        )

        // Bricks - Code (GET/POST)

        .get_async(
            "/api/bricks/:brick_id/:version/code",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::code::get, "API_RL", "api")
        )

        .post_async(
            "/api/bricks/:brick_id/:version/code",
            |req, ctx| async_rate_limit(req, ctx, api::bricks::code::post, "API_RL", "api")
        )

        // Unregister
        .delete_async(
            "/api/auth/unregister",
            |req, ctx| async_rate_limit(req, ctx, api::auth::unregister::delete_unregister, "API_RL", "api")
        )

        // Login
        .post_async(
            "/api/auth/login",
            |req, ctx| async_rate_limit(req, ctx, api::auth::login::post_login, "API_RL", "api")
        )

        // Logout
        .post_async(
            "/api/auth/logout",
            |req, ctx| async_rate_limit(req, ctx, api::auth::logout::post_logout, "API_RL", "api")
        )

        // Download
        .get_async(
            "/api/download/latest",
            |req, ctx| async_rate_limit(req, ctx, api::download::latest::get, "API_RL", "api")
        )
        
        // fallback per path esistenti ma metodo sbagliato → 405
        .or_else_any_method_async("/api/*path", |req, ctx| redirect_to_error(req, ctx, 405))

        // fallback generico per path inesistenti → 404
        .on_async("/*path", |req, ctx| redirect_to_error(req, ctx, 404))

        .run(req, env)
        .await
}
