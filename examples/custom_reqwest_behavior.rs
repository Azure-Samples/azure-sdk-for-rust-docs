//! This example demonstrates customizing the reqwest HTTP client with specific settings.
//!
//! The Azure SDK provides a blanket implementation of the HttpClient trait for reqwest::Client,
//! allowing you to customize timeout, compression, connection pooling, and other HTTP behaviors.
//!
//! Key points:
//! - Wrap the customized reqwest::Client in Arc::new()
//! - Pass it via ClientOptions::transport using Transport::new()
//! - This pattern is documented in the official Azure SDK for Rust README
//!
//! References:
//! - https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/azure_core/README.md#reqwest
//! - https://docs.rs/reqwest for reqwest::ClientBuilder options

use azure_core::http::{ClientOptions, Transport};
use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    // Example: Customize the reqwest HTTP client with specific settings
    println!("Creating customized reqwest HTTP client...");

    let http_client = Arc::new(
        reqwest::Client::builder()
            // Set custom timeout (default is 30s, here we set to 60s)
            .timeout(std::time::Duration::from_secs(60))
            // Enable gzip compression for request/response bodies
            .gzip(true)
            // Disable connection pooling (useful for troubleshooting connection issues)
            // Note: This can impact performance but resolves some known hyper issues
            .pool_max_idle_per_host(0)
            .build()?,
    );

    println!("Configured settings:");
    println!("  - Timeout: 60 seconds");
    println!("  - Gzip compression: enabled");
    println!("  - Connection pooling: disabled (pool_max_idle_per_host = 0)");
    println!();

    // Pass the customized HTTP client via ClientOptions
    let options = SecretClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(http_client)),
            ..Default::default()
        },
        ..Default::default()
    };

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, Some(options))?;

    println!("Successfully created SecretClient with customized reqwest HTTP client");
    println!();

    // Use the client normally - the custom HTTP settings apply to all requests
    println!("Testing client with a secret retrieval...");
    let response = client.get_secret("test-secret", None).await?;
    println!("✓ Secret retrieved successfully");

    // Access the secret value
    let secret = response.into_body()?;
    if let Some(value) = secret.value {
        println!("  Secret value retrieved (not displayed for security)");
    }

    Ok(())
}
