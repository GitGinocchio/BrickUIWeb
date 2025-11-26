pub fn find_asset(assets: &[serde_json::Value], ext: &str) -> Option<String> {
    for a in assets {
        if let Some(name) = a["name"].as_str() {
            if name.ends_with(ext) {
                return a["browser_download_url"].as_str().map(|s| s.to_string());
            }
        }
    }
    None
}