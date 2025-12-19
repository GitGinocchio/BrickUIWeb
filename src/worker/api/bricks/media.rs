use worker::*;

use crate::{
    errors::json_error, 
    storage::{
        client::StorageClient,
        methods::upload::upload_brick_media
    }
};



pub async fn post(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let brick_id = match ctx.param("brick_id") {
        Some(id) => id,
        None => return json_error(400, "MISSING_PARAM", "brick_id parameter is missing")
    };

    let version = match ctx.param("version") {
        Some(v) => v,
        None => return json_error(400, "MISSING_PARAM", "version parameter is missing")
    };

    let client = StorageClient::from_env(&ctx.env)?;

    //upload_brick_media(&client, user_id, brick_id, version, file_name, bytes).await?;

    Response::empty()
}

pub async fn get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    Response::empty()
}