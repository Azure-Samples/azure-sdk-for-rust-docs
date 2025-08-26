# Azure SDK for Rust Examples

Examples used in Microsoft Learn content.

## Getting started

### Prerequisites

* [Rust](https://www.rust-lang.org)
* [Azure Developer CLI](https://aka.ms/azure-dev)

### Test

Before you can run (test) the examples, you need to log into both `az` and `azd` to test different authentication examples,
and to provision resources with `azd`:

```bash
az login
azd auth login
azd up
```

Running the examples will build them automatically. For whichever example file name you want to run:

```bash
cargo build --example authenticate_azure_dev_cli
```

You can also build all examples if you just want to make sure they are compilable:

```bash
cargo build --examples
```
