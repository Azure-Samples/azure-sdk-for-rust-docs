use azure_core::http::ClientOptions;
use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};

fn create_client_options() -> ClientOptions {
    // Configure client options for resilient cloud operations
    // Note: Specific retry policies and timeouts are typically configured
    // through the ClientOptions or service-specific options

    ClientOptions {
        ..Default::default()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;

    // Use the same options across multiple clients
    // This ensures consistent behavior across all Azure services in your application
    let options = create_client_options();

    let secret_client = SecretClient::new(
        &vault_url,
        credential.clone(),
        Some(SecretClientOptions {
            client_options: options.clone(),
            ..Default::default()
        }),
    )?;

    // Note: In a real application, you would create other service clients here
    // using the same client options to ensure consistent behavior

    println!("Secret client created with global configuration");
    println!("Configuration can be shared across multiple Azure service clients");

    // Test the client configuration
    let response = secret_client.get_secret("test-secret", None).await?;
    println!("Successfully retrieved secret with client options");

    Ok(())
}
