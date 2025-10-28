use azure_security_keyvault_secrets::SecretClient;
use azure_identity::AzureDeveloperCliCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    // Example: Access HTTP headers from responses
    // These headers contain important metadata about the operation
    let response = client.get_secret("test-secret", None).await?;

    // Access headers through the response to get metadata like content type, cache info, etc.
    let (status, headers, _body) = response.deconstruct();
    
    println!("HTTP Status: {}", status);
    println!("Response Headers:");
    for (name, value) in headers.iter() {
        println!("  {}: {}", name.as_str(), value.as_str());
    }

    Ok(())
}