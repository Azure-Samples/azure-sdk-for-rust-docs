use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key_vault_name = std::env::var("AZURE_KEYVAULT_NAME")
        .map_err(|_| "AZURE_KEYVAULT_NAME environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;

    let client = SecretClient::new(&key_vault_name, credential.clone(), None)?;

    Ok(())
}
