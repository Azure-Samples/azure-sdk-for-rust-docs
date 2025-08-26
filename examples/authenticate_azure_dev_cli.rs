use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // <DOCS_AUTH>
    dotazure::load()?;
    // </DOCS_AUTH>
    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;

    let client = SecretClient::new(&vault_url, credential.clone(), None)?;

    Ok(())
}
