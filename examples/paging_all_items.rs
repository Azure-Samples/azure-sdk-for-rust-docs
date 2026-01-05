use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let credential = AzureDeveloperCliCredential::new(None)?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let client = SecretClient::new(vault_url.as_str(), credential.clone(), None)?;

    let mut pager = client.list_secret_properties(None)?;

    while let Some(secret) = pager.try_next().await? {
        let name = secret.resource_id()?.name;
        println!("Found secret with name: {}", name);
    }

    Ok(())
}
