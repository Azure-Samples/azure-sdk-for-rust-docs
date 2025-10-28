use azure_security_keyvault_secrets::models::*;

fn main() {
    // Example: Demonstrating request type structures
    // Request types bundle all the inputs needed for a specific Azure service operation

    // Example structure showing how secret attributes can be configured
    let _secret_attributes = SecretAttributes {
        enabled: Some(true),
        not_before: None,
        ..Default::default()
    };

    println!("Request types help structure your Azure API calls");
    println!("They provide type safety and IDE autocompletion for all available options");
}
