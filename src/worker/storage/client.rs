use worker::*;

pub struct StorageClient {
    pub base_url: String,
    pub api_key: String,
}

impl StorageClient {
    pub fn from_env(env: &Env) -> Result<Self> {
        let base_url = env
            .var("SUPABASE_URL")?
            .to_string();

        let api_key = env
            .var("SUPABASE_KEY")?
            .to_string();

        Ok(Self { base_url, api_key })
    }

    pub async fn request(
        &self,
        path: &str,
        method: Method,
        body: Option<Vec<u8>>,
        content_type: Option<&str>,
    ) -> Result<Response> {
        let mut init = RequestInit::new();
        init.method = method;

        if let Some(b) = body {
            init.body = Some(wasm_bindgen::JsValue::from(js_sys::Uint8Array::from(b.as_slice())));
        }

        let url = format!("{}/storage/v1/{}", self.base_url, path);

        let headers = Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_key))?;
        headers.set("apikey", &self.api_key)?;

        if let Some(ct) = content_type {
            headers.set("Content-Type", ct)?;
        }

        init.headers = headers;

        let req = Request::new_with_init(&url, &init)?;
        Fetch::Request(req).send().await
    }
}
