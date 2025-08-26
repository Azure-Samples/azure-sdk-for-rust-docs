// Copyright Microsoft Corporation. All rights reserved.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

@minLength(1)
@maxLength(64)
@description('Name of the environment that can be used as part of naming resource convention')
param environmentName string

@minLength(1)
@description('Primary location for all resources')
param location string = resourceGroup().location

@description('User principal ID')
param principalId string

@description('The vault name; default is a unique string based on the resource group ID')
param vaultName string = ''

@description('The vault SKU; default is "standard"')
@allowed(['standard', 'premium'])
param vaultSku string = 'standard'

resource kv 'Microsoft.KeyVault/vaults@2023-07-01' = {
  name: empty(vaultName) ? 't${uniqueString(resourceGroup().id, environmentName)}' : vaultName
  location: location
  properties: {
    tenantId: subscription().tenantId
    sku: {
      name: vaultSku
      family: 'A'
    }
    enableRbacAuthorization: true
    softDeleteRetentionInDays: 7
  }

  // Allocate some secrets to enumerate.
  resource secretNumbers 'secrets' = [
    for i in range(0, 3): {
      name: 'secret-${i}'
      properties: {
        contentType: 'text/plain'
        value: uniqueString('secret', string(i))
      }
    }
  ]
}

var kvSecretsOfficerDef = subscriptionResourceId(
  'Microsoft.Authorization/roleDefinitions',
  'b86a8fe4-44ce-4948-aee5-eccb2c155cd7'
)

resource kvSecretsOfficer 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: guid(resourceGroup().id, environmentName, principalId, kvSecretsOfficerDef)
  scope: kv
  properties: {
    roleDefinitionId: kvSecretsOfficerDef
    principalId: principalId
  }
}

output AZURE_PRINCIPAL_ID string = principalId
output AZURE_KEYVAULT_NAME string = kv.name
output AZURE_KEYVAULT_SKU string = kv.properties.sku.name
output AZURE_KEYVAULT_URL string = kv.properties.vaultUri
