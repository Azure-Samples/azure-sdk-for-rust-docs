use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    // Example: Types ending with "Headers" contain HTTP header properties
    // These types provide access to HTTP response headers returned by Azure services
    let response = client.get_secret("test-secret", None).await?;

    // Access headers through the response to get metadata like content type, cache info, etc.
    println!("HTTP Status: {}", response.status());
    println!("Response Headers:");
    for (name, value) in response.headers().iter() {
        println!("  {}: {}", name.as_str(), value.as_str());
    }

    Ok(())
}
