use std::collections::HashMap;

use resend_rs::{Resend, types::{CreateEmailBaseOptions, CreateEmailResponse, EmailTemplate}};
use serde_json::Value;


pub async fn send_confirmation_email(
    resend_key: &String, 
    origin: String,
    email_address: &String,
    hashed_token: String
) -> Result<CreateEmailResponse, String> {
    let resend = Resend::new(resend_key);

    let mut variables = HashMap::<String, Value>::new();
    variables.insert("USER_NAME".into(), Value::String(email_address.clone()));
    variables.insert(
        "VERIFY_URL".into(), 
        Value::String(format!(
            "{}/api/auth/confirm?token={}&redirect_to={}", 
            origin,
            hashed_token, 
            format!("{origin}/auth/confirmed")
        ))
    );

    let template = EmailTemplate::new("confirm-registration").with_variables(variables);
    let opts = CreateEmailBaseOptions::new(
        "BrickUI <brickui@mail.brickui.app>", 
        vec![email_address], 
        "Almost done! Confirm your BrickUI account"
    ).with_template(template);

    resend.emails
        .send(opts)
        .await
        .map_err(|e| format!("Error sending email: {e}"))
}