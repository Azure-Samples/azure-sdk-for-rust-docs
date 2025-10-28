use azure_core::http::ClientOptions;
use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    // Configure client options for customizing HTTP behavior
    // These settings can help with performance and reliability in different environments
    let options = SecretClientOptions {
        client_options: ClientOptions {
            // Custom HTTP client configuration would go here
            // The Transport wrapper allows different HTTP client implementations
            ..Default::default()
        },
        ..Default::default()
    };

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, Some(options))?;

    println!("Client configured with custom HTTP options");
    println!("Custom settings like timeouts, connection pooling, and compression");
    println!("can be configured through ClientOptions and Transport");

    // Use the client normally - all requests will use your customized HTTP settings
    let response = client.get_secret("test-secret", None).await?;
    println!("Secret retrieved successfully with custom client options");

    Ok(())
}
