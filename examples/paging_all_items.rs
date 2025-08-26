use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // <DOCS_AUTH>
    dotazure::load()?;
    // </DOCS_AUTH>
    let credential = AzureDeveloperCliCredential::new(None)?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let client = SecretClient::new(vault_url.as_str(), credential.clone(), None)?;

    // get a stream of items
    let mut pager = client.list_secret_properties(None)?;

    // poll the pager until there are no more SecretListResults
    while let Some(secret) = pager.try_next().await? {
        // get the secret name from the ID
        let name = secret.resource_id()?.name;
        println!("Found secret with name: {}", name);
    }

    Ok(())
}
