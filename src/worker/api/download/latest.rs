use worker::*;

use super::utils::find_asset;

pub async fn get(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let token = ctx.env.var("GITHUB_TOKEN")?.to_string();
    let owner = ctx.env.var("GITHUB_OWNER")?.to_string();
    let repo  = ctx.env.var("GITHUB_REPO")?.to_string();

    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;
    headers.set("User-Agent", "BrickUI-Updater")?;

    let mut release_response = Fetch::Request(Request::new_with_init(
        api_url.as_str(),
        &RequestInit {
            method: Method::Get,
            headers: headers,
            ..Default::default()
        },
    )?)
    .send()
    .await?;

    if release_response.status_code() != 200 {
        return Ok(release_response)
    }

    let release: serde_json::Value = release_response.json().await?;

    let default_assets = vec![];

    let assets = release["assets"].as_array().unwrap_or(&default_assets);

    let ua = req.headers().get("User-Agent")?.unwrap_or_default();

    let asset_info = if ua.contains("Win") {
        find_asset(assets, ".exe")
    } else if ua.contains("Mac") {
        find_asset(assets, ".dmg")
    } else {
        find_asset(assets, ".AppImage")
    };

    let (id, file_name) = match asset_info {
        Some(v) => v,
        None => return Response::error("No asset available", 400),
    };

    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/assets/{}",
        owner, repo, id
    );

    let headers = Headers::new();
    headers.set("Authorization", &format!("Bearer {}", token))?;
    headers.set("Accept", "application/octet-stream")?;
    headers.set("User-Agent", "BrickUI-Updater")?;

    let mut release_response = Fetch::Request(Request::new_with_init(
        api_url.as_str(),
        &RequestInit {
            method: Method::Get,
            headers: headers,
            redirect: RequestRedirect::Follow,
            ..Default::default()
        },
    )?)
    .send()
    .await?;

    if release_response.status_code() != 200 {
        return Ok(release_response)
    }

    let stream = release_response.stream()?;

    let mut stream_response = Response::from_stream(stream)?;

    stream_response
        .headers_mut()
        .set("Content-Type", "application/octet-stream")?;

    stream_response
        .headers_mut()
        .set("Content-Disposition", &format!("attachment; filename={file_name}"))?;

    Ok(stream_response)
}