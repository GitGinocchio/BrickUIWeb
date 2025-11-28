
pub fn find_asset(assets: &[serde_json::Value], ext: &str) -> Option<(u64, String)> {
    for asset in assets {
        if let Some(name) = asset["name"].as_str() {
            if name.ends_with(ext) {
                if let Some(id) = asset["id"].as_u64() {
                    return Some((id, name.to_string()));
                }
            }
        }
    }
    None
}