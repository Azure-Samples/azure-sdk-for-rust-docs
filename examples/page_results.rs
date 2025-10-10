use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::SecretClient;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotazure::load()?;

    let credential = AzureCliCredential::new(None)?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let client = SecretClient::new(&vault_url, credential.clone(), None)?;

    let mut pager = client.list_secret_properties(None)?.into_pages();

    while let Some(page) = pager.try_next().await? {

        let page = page.into_body()?;
        println!("items_in_page: {}", page.value.len());
    }

    Ok(())
}
