use crate::storage::client::StorageClient;
use crate::storage::limits::*;
use crate::storage::mime::*;
use worker::Method;
use worker::Response;

/// Upload generico a Supabase Storage.
///
/// `bucket` = nome del bucket (users-media, bricks-code, ecc)
/// `path`   = percorso interno dentro al bucket
async fn upload_to_bucket(
    client: &StorageClient,
    bucket: &str,
    path: &str,
    bytes: Vec<u8>,
    mime: &str,
) -> Result<Response, String> {
    let full_path = format!("object/{}/{}", bucket, path);

    let res = client
        .request(&full_path, Method::Post, Some(bytes), Some(mime))
        .await
        .map_err(|e| format!("Upload request failed: {:?}", e))?;

    Ok(res)
}

// TODO: Ricontrollare i metodi upload in quanto non rispettano la struttura ottimale
//       dei file all'interno del bucket

// Brick

pub async fn upload_brick_media(
    client: &StorageClient,
    user_id: &str,
    brick_id: &str,
    version: &str,
    file_name: &str,
    bytes: Vec<u8>,
) -> Result<Response, String> {
    if bytes.len() > MAX_BRICK_MEDIA_SIZE {
        return Err(format!("Brick media too large (max {MAX_BRICK_MEDIA_SIZE})").into());
    }

    let path = format!("{}/{}/{}/{}", user_id, brick_id, version, file_name);
    let mime = get_mime_type(file_name, bytes.as_slice())?;

    if !is_allowed_mime_for_media(mime) {
        return Err(format!("Brick media mime type not allowed: {mime}").into());
    }

    upload_to_bucket(client, "bricks-media", &path, bytes, mime).await
}

pub async fn upload_brick_code(
    client: &StorageClient,
    user_id: &str,
    brick_id: &str,
    version: &str,
    file_name: &str,
    bytes: Vec<u8>,
) -> Result<Response, String> {
    if bytes.len() > MAX_BRICK_CODE_SIZE {
        return Err(format!("Brick code too large (max {} MB)", MAX_BRICK_CODE_SIZE / 1024 / 1024).into());
    }

    let path = format!("{user_id}/{brick_id}/{version}/brick.zip");
    let mime = get_mime_type(file_name, bytes.as_slice())?;

    if !is_allowed_mime_for_code(mime) {
        return Err("Brick code mime type not allowed".into());
    }

    upload_to_bucket(client, "bricks-code", &path, bytes, mime).await
}

// TODO: In futuro supportare anche file .json oltre a file .yml
pub async fn upload_brick_meta(
    client: &StorageClient,
    user_id: &str,
    brick_id: &str,
    version: &str,
    file_name: &str,
    bytes: Vec<u8>,
) -> Result<Response, String> {
    if bytes.len() > MAX_BRICK_META_SIZE {
        return Err(format!("Brick meta too large (max {} KB)", MAX_BRICK_META_SIZE / 1024).into());
    }

    let path = format!("{}/{}/{}/brick.yml", user_id, brick_id, version);
    let mime = get_mime_type(file_name, bytes.as_slice())?;

    if !is_allowed_mime_for_meta(mime) {
        return Err(format!("Brick meta mime type not allowed: {mime}").into());
    }

    upload_to_bucket(client, "bricks-meta", &path, bytes, mime).await
}

// User

pub async fn upload_user_media(
    client: &StorageClient,
    user_id: &str,
    brick_id: &str,
    version: &str,
    file_name: &str,
    bytes: Vec<u8>,
) -> Result<Response, String> {
    if bytes.len() > MAX_USER_MEDIA_SIZE {
        return Err(format!("User media too large (max {} MB)", MAX_USER_MEDIA_SIZE / 1024 / 1024).into());
    }

    let path = format!("{}/{}/{}/{}", user_id, brick_id, version, file_name);
    let mime = get_mime_type(file_name, bytes.as_slice())?;

    if !is_allowed_mime_for_media(mime) {
        return Err(format!("User media mime type not allowed: {mime}").into());
    }

    upload_to_bucket(client, "users-media", &path, bytes, mime).await
}