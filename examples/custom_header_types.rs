//! This example demonstrates two approaches for accessing HTTP headers from Azure service responses.
//!
//! Approach 1: Access headers without consuming the response
//! - Use when you need to inspect headers but still want to use the response object later
//! - Methods: response.status(), response.headers()
//!
//! Approach 2: Deconstruct the response to get all parts
//! - Use when you need direct access to status, headers, and body
//! - More explicit and allows working with each component separately
//! - Method: response.deconstruct()

use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    println!("=== Approach 1: Access headers without consuming the response ===");
    println!();

    // Get the response
    let response = client.get_secret("test-secret", None).await?;

    // Access headers and status without consuming the response
    // This allows you to inspect metadata while keeping the response intact
    println!("HTTP Status: {}", response.status());
    println!("Response Headers:");
    for (name, value) in response.headers().iter() {
        println!("  {}: {}", name.as_str(), value.as_str());
    }

    // The response is still available here if needed
    // But we'll get a fresh one for the second example
    println!();
    println!("=== Approach 2: Deconstruct the response ===");
    println!();

    // Get another response
    let response = client.get_secret("test-secret", None).await?;

    // Deconstruct to get all parts: status, headers, and body
    // This consumes the response and gives you direct access to each component
    let (status, headers, _body) = response.deconstruct();

    println!("HTTP Status: {}", status);
    println!("Response Headers:");
    for (name, value) in headers.iter() {
        println!("  {}: {}", name.as_str(), value.as_str());
    }

    println!();
    println!("=== Summary ===");
    println!("Choose Approach 1 when: You want to inspect headers without consuming the response");
    println!("Choose Approach 2 when: You need explicit access to all response components");

    Ok(())
}
