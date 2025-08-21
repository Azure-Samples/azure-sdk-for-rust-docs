use azure_identity::{
    ManagedIdentityCredential,
    ManagedIdentityCredentialOptions,
    UserAssignedId
};
use azure_security_keyvault_secrets::{
    SecretClient, 
    SecretClientOptions
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let user_assigned_id = UserAssignedId::ClientId(
        std::env::var("AZURE_USER_ASSIGNED_IDENTITY")
            .expect("AZURE_USER_ASSIGNED_IDENTITY environment variable is required"),
    );
    
    let credential_options = ManagedIdentityCredentialOptions {
        user_assigned_id: Some(user_assigned_id),
        ..Default::default()
    };

    let credential = ManagedIdentityCredential::new(Some(credential_options))?;

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
    .expect("AZURE_KEY_VAULT_ENDPOINT environment variable is required");

    let secret_client_options = SecretClientOptions {
        api_version: "7.5".to_string(),
        ..Default::default()
    };
    
    let client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential,
        Some(secret_client_options),
    )?;

    Ok(())
}
