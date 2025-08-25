use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::{ResourceExt, SecretClient};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create a client
    let credential = AzureCliCredential::new(None)?;

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
        .map_err(|_| "AZURE_KEY_VAULT_ENDPOINT environment variable is required")?;

    let client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential.clone(),
        None,
    )?;

    // get a stream of pages
    let mut pager = client.list_secret_properties(None)?.into_pages();

    // poll the pager until there are no more SecretListResults
    while let Some(page) = pager.try_next().await? {
        // print the raw page (SecretsListResults)
        let page = page.into_body().await?;
        println!("items_in_page: {}", page.value.len());
    }

    Ok(())
}