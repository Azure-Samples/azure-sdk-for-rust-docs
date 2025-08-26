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

## Remove Azure resources

When you're done with the examples, you can remove the Azure resources created by `azd up`:

```bash
azd down
```