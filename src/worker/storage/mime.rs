

pub const META_MIME_TYPES: &[&str] = &[
    "text/yaml",
    "application/x-yaml",
    "application/json",
];

pub const MEDIA_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/webp",
    "image/svg+xml"
];

pub const CODE_MIME_TYPES: &[&str] = &[
    "application/zip",
    "application/x-zip-compressed"
];


pub fn is_allowed_mime_for_meta(mime: &str) -> bool {
    META_MIME_TYPES.contains(&mime)
}

pub fn is_allowed_mime_for_media(mime: &str) -> bool {
    MEDIA_MIME_TYPES.contains(&mime)
}

pub fn is_allowed_mime_for_code(mime: &str) -> bool {
    CODE_MIME_TYPES.contains(&mime)
}

fn get_mime_type_from_ext(file_name: &str) -> Result<&'static str, String> {
    let ext = file_name
        .rsplit('.')
        .next()
        .ok_or("File has no extension")?
        .to_ascii_lowercase();

    match ext.as_str() {
        // MEDIA
        "png" => Ok("image/png"),
        "jpg" | "jpeg" => Ok("image/jpeg"),
        "webp" => Ok("image/webp"),
        "svg" => Ok("image/svg+xml"),

        // META
        "json" => Ok("application/json"),
        "yml" | "yaml" => Ok("application/x-yaml"),

        // CODE
        "zip" => Ok("application/zip"),

        _ => Err(format!("Unsupported file extension: .{}", ext)),
    }
}

fn normalize_mime(mime: &str) -> &str {
    match mime {
        "text/yaml" | "application/x-yaml" => "application/x-yaml",
        other => other,
    }
}

pub fn get_mime_type(file_name: &str, content: &[u8]) -> Result<&'static str, String> {
    let expected = get_mime_type_from_ext(file_name)?;
    let detected = infer::get(content);

    if let Some(t) = detected && normalize_mime(t.mime_type()) != normalize_mime(expected) {
        return Err("MIME mismatch".into())
    }
    
    match detected.map(|t| t.mime_type()).unwrap_or(expected) {
        "application/json" => {
            serde_json::from_slice::<serde_json::Value>(content)
                .map_err(|_| "Invalid JSON")?;
        }

        "application/x-yaml" | "text/yaml" => {
            serde_yaml::from_slice::<serde_yaml::Value>(content)
                .map_err(|_| "Invalid YAML")?;
        }

        "image/svg+xml" => {
            std::str::from_utf8(content)
                .map_err(|_| "Invalid UTF-8")?;
            // opzionale: XML parse
        }

        _ => return Err("Unsupported MIME".into()),
    }

    Ok(expected)
}