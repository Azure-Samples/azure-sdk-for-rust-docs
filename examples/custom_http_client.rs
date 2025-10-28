use azure_core::http::{ClientOptions, Transport};
use azure_core::{
    error::ErrorKind,
    http::{BufResponse, HttpClient, Request},
};
use azure_identity::AzureDeveloperCliCredential;
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use std::sync::Arc;

#[derive(Debug)]
struct UreqClient(ureq::Agent);

impl Default for UreqClient {
    fn default() -> Self {
        Self(ureq::Agent::new())
    }
}

#[async_trait::async_trait]
impl HttpClient for UreqClient {
    async fn execute_request(&self, request: &Request) -> azure_core::Result<BufResponse> {
        // Convert azure_core::Request to ureq::Request
        let mut ureq_request = self
            .0
            .request(request.method().as_str(), request.url().as_str());

        // Add headers
        for (name, value) in request.headers().iter() {
            ureq_request = ureq_request.set(name.as_str(), value.as_str());
        }

        // For demonstration purposes, we'll use a simple implementation
        // In a real implementation, you would properly convert the response
        println!(
            "Custom HTTP client would execute request to: {}",
            request.url()
        );

        // Return an error for demonstration since full implementation is complex
        Err(azure_core::Error::new(
            ErrorKind::Other,
            "Custom HTTP client implementation not complete - this is a demonstration",
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let vault_url = std::env::var("AZURE_KEYVAULT_URL")
        .map_err(|_| "AZURE_KEYVAULT_URL environment variable is required")?;

    // Create an instance of your custom HTTP client
    let http_client = Arc::new(UreqClient::default());

    // Configure the Azure client to use your custom HTTP client
    // The Transport wrapper allows any HttpClient implementation to be used
    let options = SecretClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(http_client)),
            ..Default::default()
        },
        ..Default::default()
    };

    let credential = AzureDeveloperCliCredential::new(None)?;
    // The client will now use your custom HTTP implementation for all requests
    let client = SecretClient::new(&vault_url, credential, Some(options))?;

    println!("Custom HTTP client configured successfully");

    Ok(())
}
