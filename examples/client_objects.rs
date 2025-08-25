use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let credential = AzureCliCredential::new(None)?;

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
        .map_err(|_| "AZURE_KEY_VAULT_ENDPOINT environment variable is required")?;
   
    let client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential,
        None,
    )?;

    Ok(())
}
