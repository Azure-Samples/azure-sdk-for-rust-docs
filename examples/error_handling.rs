use azure_core::{error::ErrorKind, http::StatusCode};
use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let credential = AzureCliCredential::new(None)?;

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
    .expect("AZURE_KEY_VAULT_ENDPOINT environment variable is required");

    let key_vault_secret_client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential.clone(),
        None,
    )?;

    match key_vault_secret_client.get_secret("secret-name", "", None).await {
        Ok(secret) => println!("{}", secret.into_body().await?.value.unwrap_or_default()),
        Err(e) => match e.kind() {
            ErrorKind::HttpResponse { status, error_code, .. } if *status == StatusCode::NotFound => {

                if let Some(code) = error_code {
                    println!("ErrorCode: {}", code);
                } else {
                    println!("Secret not found, but no error code provided.");
                }
            },
            _ => println!("An error occurred: {e:?}"),
        },
    }

    Ok(())
}