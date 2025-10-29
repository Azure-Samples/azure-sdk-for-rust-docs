use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    // Example: Demonstrating how to work with request and response types
    // Azure SDK uses strongly-typed models for API operations

    let secret_name = "example-secret-with-attributes";

    // Retrieve a secret - the response contains structured data
    let response = client.get_secret(secret_name, None).await?;

    // Extract the secret information from the response
    let secret_info = response.into_body()?;

    println!("Secret '{}' retrieved successfully", secret_name);
    println!("Secret ID: {:?}", secret_info.id);
    println!(
        "Enabled: {:?}",
        secret_info.attributes.and_then(|a| a.enabled)
    );

    Ok(())
}
