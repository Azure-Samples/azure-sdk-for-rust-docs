use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{models::SecretAttributes, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    // Example: Using request types to configure operations
    // Request types bundle all the inputs needed for a specific Azure service operation

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential.clone(), None)?;

    // Create attributes with type safety and IDE autocompletion
    let attributes = SecretAttributes {
        enabled: Some(true),
        not_before: None,
        ..Default::default()
    };

    println!("Created SecretAttributes: {:?}", attributes);
    println!("\nRequest types provide:");
    println!("- Type safety at compile time");
    println!("- IDE autocompletion for all available options");
    println!("- Clear documentation of required vs optional fields");
    println!("- Default values through Rust's Default trait");

    // Fetch a secret to demonstrate working with response types
    let secret_name = "test-secret";
    let response = client.get_secret(secret_name, None).await?;
    let secret_info = response.into_body()?;

    println!("\n✅ Successfully retrieved secret '{}'", secret_name);
    println!("Secret ID: {:?}", secret_info.id);
    println!(
        "Enabled: {:?}",
        secret_info.attributes.and_then(|a| a.enabled)
    );

    Ok(())
}
