use azure_core::{error::ErrorKind, http::StatusCode};
use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotazure::load()?;

    let credential = AzureCliCredential::new(None)?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let client = SecretClient::new(&vault_url, credential.clone(), None)?;

    match client.get_secret("secret-0", "", None).await {
        Ok(secret) => println!("Secret value: {}", secret.into_body().await?.value.unwrap_or_else(|| Default::default())),
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