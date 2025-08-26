// Copyright Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

using './main.bicep'

// cspell:ignore westus
param environmentName = readEnvironmentVariable('AZURE_ENV_NAME', 'rust-dev')
param location = readEnvironmentVariable('AZURE_LOCATION', 'westus')
param principalId = readEnvironmentVariable('AZURE_PRINCIPAL_ID', '')
param vaultName = readEnvironmentVariable('AZURE_KEYVAULT_NAME', '')
param vaultSku = readEnvironmentVariable('AZURE_KEYVAULT_SKU', 'standard')
