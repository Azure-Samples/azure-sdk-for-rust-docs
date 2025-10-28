use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let credential = AzureDeveloperCliCredential::new(None)?;
    let client = SecretClient::new(&vault_url, credential, None)?;

    let secret_name = "response-handling-test";

    // Option 1: Extract just the body (consumes the response)
    // This is the most common usage when you only need the data
    let response = client.get_secret(secret_name, None).await?;
    let secret = response.into_body()?;
    println!("Option 1 - Secret value: {:?}", secret.value);

    // Option 2: Get a new response to access HTTP details
    // Useful when you need both the data and HTTP metadata
    let response = client.get_secret(secret_name, None).await?;

    println!("Option 2 - HTTP Status: {}", response.status());
    println!(
        "Headers available: {} total headers",
        response.headers().iter().count()
    );

    // Access the parsed response data
    let secret = response.into_body()?;
    println!("Secret ID: {:?}", secret.id);

    Ok(())
}
