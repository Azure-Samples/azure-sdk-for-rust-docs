# Azure SDK for Rust Examples

Examples used in Microsoft Learn content.

## Prerequisites

* [Rust](https://www.rust-lang.org)
* [Azure Developer CLI](https://aka.ms/azure-dev)

## Authenticate for local development

Before you can run (test) the examples, you need to log into both `az` and `azd` to test different authentication examples,
and to provision resources with `azd`:

```bash
az login
azd auth login
```

## Create Azure resources

This samples use Azure Identity and Azure Key Vault. Use `azd up` from Azure Developer CLI to create these resources.

```
azd up
```

## Run the examples

Running the examples will build them automatically. For whichever example file name you want to run:

```bash
cargo run --example authenticate_azure_dev_cli
```

You can also build all examples if you just want to make sure they are compilable:

```bash
cargo build --examples
```

## Code Formatting

This project uses `rustfmt` to maintain consistent code style. A `rustfmt.toml` configuration file is included.

### Format all code (stable Rust)

```bash
cargo fmt
```

This will format all code using stable Rust features (basic formatting, line width, etc.).

### Advanced formatting (requires nightly Rust)

For advanced features like automatic import ordering and grouping:

```bash
# Install nightly toolchain (one-time setup)
rustup toolchain install nightly

# Format with nightly features
cargo +nightly fmt
```

**Note:** Import ordering and some advanced formatting features require the nightly Rust toolchain. The stable `cargo fmt` command will work but won't reorder imports.

## Troubleshooting

### Files showing errors in VS Code after successful compilation

If `cargo build --examples` succeeds but files still show red/errors in VS Code:

1. **Restart rust-analyzer**: Press `Ctrl+Shift+P` → type "rust-analyzer: Restart server"
2. **Reload VS Code window**: Press `Ctrl+Shift+P` → type "Developer: Reload Window"
3. **Check cargo**: Run `cargo check` to verify compilation
4. **Clear rust-analyzer cache**: Press `Ctrl+Shift+P` → type "rust-analyzer: Clear flycheck diagnostics"

The rust-analyzer language server sometimes needs to be restarted after significant file changes or dependency updates.

## Remove Azure resources

When you're done with the examples, you can remove the Azure resources created by `azd up`:

```bash
azd down
```

## Examples Overview

This repository contains examples demonstrating various Azure SDK for Rust features and patterns.

### Authentication Examples

| Example | Purpose |
|---------|---------|
| `authenticate_azure_cli.rs` | Authenticate using Azure CLI credentials for local development |
| `authenticate_azure_dev_cli.rs` | Authenticate using Azure Developer CLI credentials for local development |
| `authenticate_server.rs` | Authenticate using Managed Identity for server/production scenarios |

### Client Configuration and Usage

| Example | Purpose |
|---------|---------|
| `client_objects.rs` | Working with Azure service client objects and understanding client lifecycle |
| `global_client_options.rs` | Configure `ClientOptions` globally and share across multiple Azure service clients for consistent behavior |

### HTTP Client Customization

| Example | Purpose |
|---------|---------|
| `custom_reqwest_behavior.rs` | Customize the default reqwest HTTP client with specific timeouts, compression, and connection pooling settings |
| `custom_http_client.rs` | Implement a custom HTTP client using the `HttpClient` trait (demonstrates ureq as an alternative to reqwest) |

### Async Runtime

| Example | Purpose |
|---------|---------|
| `custom_async_runtime.rs` | Demonstrates the concept of replacing the default tokio runtime with custom async runtime implementations (async-std, smol, etc.) |

### Type Conventions and Response Handling

| Example | Purpose |
|---------|---------|
| `custom_header_types.rs` | Access HTTP headers from Azure service responses - demonstrates both non-consuming access and deconstruction approaches |
| `custom_request_types.rs` | Shows request type structures (types ending with "Request") that bundle operation parameters |
| `type_conventions_request.rs` | Working with strongly-typed request and response models from Azure services |
| `response_handling.rs` | Use `Response<T>` to access both deserialized response data and HTTP details (status, headers) |

### Pagination

| Example | Purpose |
|---------|---------|
| `custom_paging_byom.rs` | Define custom types that work with Azure Core paging abstractions (Bring Your Own Model pattern) |
| `page_results.rs` | Process paginated results using `into_pages()` to work with full pages at a time |
| `paging_all_items.rs` | Process paginated results using `try_next()` to iterate through items across all pages |

### Error Handling

| Example | Purpose |
|---------|---------|
| `error_handling.rs` | Handle different error types and status codes from Azure services using `ErrorKind` |
