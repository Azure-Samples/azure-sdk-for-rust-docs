use azure_identity::{ManagedIdentityCredential, ManagedIdentityCredentialOptions, UserAssignedId};
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    let user_assigned_id: Option<UserAssignedId> = std::env::var("AZURE_USER_ASSIGNED_IDENTITY")
        .ok()
        .map(|id| UserAssignedId::ClientId(id.clone()));

    let credential_options = ManagedIdentityCredentialOptions {
        user_assigned_id,
        ..Default::default()
    };

    let credential = ManagedIdentityCredential::new(Some(credential_options))?;

    let client = SecretClient::new(vault_url.as_str(), credential.clone(), None)?;

    Ok(())
}
