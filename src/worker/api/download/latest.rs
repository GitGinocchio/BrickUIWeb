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

    let asset_url;

    if ua.contains("Win") {
        asset_url = find_asset(assets, ".exe");
    } else if ua.contains("Mac") {
        asset_url = find_asset(assets, ".dmg");
    } else {
        asset_url = find_asset(assets, ".AppImage");
    }

    let url = match asset_url {
        Some(url) => url,
        None => return Response::error("No asset available", 400),
    };

    // Redirect al download diretto
    Response::redirect(url.parse()?)
}