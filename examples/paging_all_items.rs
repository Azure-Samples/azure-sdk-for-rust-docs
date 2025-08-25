use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let credential = AzureCliCredential::new(None)?;

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
        .map_err(|_| "AZURE_KEY_VAULT_ENDPOINT environment variable is required")?;

    let key_vault_secret_client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential.clone(),
        None,
    )?;

    // get a stream of items
    let mut pager = key_vault_secret_client.list_secret_properties(None)?;

    // poll the pager until there are no more SecretListResults
    while let Some(secret) = pager.try_next().await? {
        // get the secret name from the ID
        let name = secret.resource_id()?.name;
        println!("Found secret with name: {}", name);
    }

    Ok(())
}