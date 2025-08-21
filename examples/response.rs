use azure_core::{error::ErrorKind, http::{Response, StatusCode}};
use azure_identity::AzureCliCredential;
use azure_security_keyvault_secrets::{models::Secret, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let key_vault_endpoint = std::env::var("AZURE_KEY_VAULT_ENDPOINT")
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "AZURE_KEY_VAULT_ENDPOINT environment variable is required",
            )
        })?;

    let credential = AzureCliCredential::new(None)?;

    let key_vault_secret_client = SecretClient::new(
        key_vault_endpoint.as_str(),
        credential.clone(),
        None,
    )?;

    // call a service method, which returns Response<T>
    let response = key_vault_secret_client.get_secret("secret-name", "", None).await;

    match response {
        Ok(secret) => println!("{}", secret.into_body().await?.value.unwrap_or_else(|| Default::default())),
        Err(e) => match e.kind() {
            ErrorKind::HttpResponse { status, error_code, .. } if *status == StatusCode::NotFound => {
                if let Some(code) = error_code {
                    println!("ErrorCode: {}", code);
                } else {
                    println!("Secret not found, but no error code provided.");
                }
            }
            _ => println!("An error occurred: {e:?}"),
        },
    }

    // get response again because it was moved in above statement
    let response: Response<Secret> = key_vault_secret_client.get_secret("secret-name", "", None).await?;

    // 2. The deconstruct() method for accessing all the details of the HTTP response
    let (status, headers, body) = response.deconstruct();

    // for example, you can access HTTP status
    println!("Status: {}", status);

    // or the headers
    for (header_name, header_value) in headers.iter() {
        println!("{}: {}", header_name.as_str(), header_value.as_str());
    }

    Ok(())
}