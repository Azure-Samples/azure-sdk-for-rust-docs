#!/bin/bash
# chmod +x create-resources.sh
# az login --use-device-code
# az account list --query "[].{name:name}" -o table
# az account list-locations --query "[].name" -o tsv

# usage: ./create-resources.sh <resource-group-name> <location>
# example: ./create-resources.sh "rust-keyvault-demo" "eastus2"
# This script creates an Azure resource group and an Azure Key Vault resource with appropriate RBAC permissions.

# Exit on error
set -e

# Check if the user is logged into Azure
if ! az account show > /dev/null 2>&1; then
  echo "You are not logged into Azure. Please run 'az login' to log in."
  exit 1
fi
echo "Logged into Azure."

# Check for required arguments
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <resource-group-name> <location>"
  exit 1
fi
# Input parameters
RESOURCE_GROUP_NAME_BASE=$1
LOCATION=$2
echo "Arguments provided: $1 $2"

# Retrieve the subscription ID from the current Azure account
SUBSCRIPTION_ID=$(az account show --query "id" -o tsv)
echo "Subscription ID: $SUBSCRIPTION_ID"

# Generate a random 6-character string
RANDOM_SUFFIX=$(openssl rand -hex 3)
echo "Random suffix: $RANDOM_SUFFIX"

# Append the random string to the resource group name
RESOURCE_GROUP_NAME="${RESOURCE_GROUP_NAME_BASE}-${RANDOM_SUFFIX}"
echo "Resource group name: $RESOURCE_GROUP_NAME"

# Variables
KEY_VAULT_NAME="keyvault-${RANDOM_SUFFIX}"

# Set the subscription
az account set --subscription "$SUBSCRIPTION_ID"

# Create the resource group
echo "Creating resource group: $RESOURCE_GROUP_NAME in $LOCATION..."
az group create --location $LOCATION --resource-group $RESOURCE_GROUP_NAME

# Create the Key Vault resource
echo "Creating Key Vault resource: $KEY_VAULT_NAME..."
az keyvault create \
  --name "$KEY_VAULT_NAME" \
  --resource-group "$RESOURCE_GROUP_NAME" \
  --location "$LOCATION" \
  --enable-rbac-authorization true \
  --retention-days 90 \
  --enable-soft-delete true \
  --enable-purge-protection true

# Get the current user's object ID for RBAC assignments
USER_OBJECT_ID=$(az ad signed-in-user show --query "id" -o tsv)
echo "Current user object ID: $USER_OBJECT_ID"

# Assign RBAC roles for Key Vault resource
echo "Assigning RBAC roles for Key Vault resource..."

# Key Vault Secrets Officer role for full secret management (create, read, update, delete)
echo "Assigning Key Vault Secrets Officer role..."
az role assignment create \
  --assignee "$USER_OBJECT_ID" \
  --role "b86a8fe4-44ce-4948-aee5-eccb2c155cd7" \
  --scope "/subscriptions/$SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP_NAME/providers/Microsoft.KeyVault/vaults/$KEY_VAULT_NAME" \
  --only-show-errors

# Key Vault Administrator role for full Key Vault management
echo "Assigning Key Vault Administrator role..."
az role assignment create \
  --assignee "$USER_OBJECT_ID" \
  --role "00482a5a-887f-4fb3-b363-3b7fe8e74483" \
  --scope "/subscriptions/$SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP_NAME/providers/Microsoft.KeyVault/vaults/$KEY_VAULT_NAME" \
  --only-show-errors

echo "Waiting for role assignments to propagate..."
sleep 30

# Get Key Vault details
KEY_VAULT_URI=$(az keyvault show \
  --name "$KEY_VAULT_NAME" \
  --resource-group "$RESOURCE_GROUP_NAME" \
  --query "properties.vaultUri" -o tsv)

# Create a sample secret to demonstrate functionality
echo "Creating a sample secret in Key Vault..."
az keyvault secret set \
  --vault-name "$KEY_VAULT_NAME" \
  --name "sample-secret" \
  --value "This is a sample secret value for testing" \
  --only-show-errors

# Create the .env file
ENV_FILE=".env"
echo "Creating .env file..."
cat <<EOL > $ENV_FILE
DEBUG=true
USE_PASSWORDLESS="true"

# ========================================
# Azure Key Vault Settings
# ========================================
AZURE_KEY_VAULT_NAME="$KEY_VAULT_NAME"
AZURE_KEY_VAULT_URI="$KEY_VAULT_URI"

# ========================================
# Azure Resource Information
# ========================================
AZURE_SUBSCRIPTION_ID="$SUBSCRIPTION_ID"
AZURE_RESOURCE_GROUP="$RESOURCE_GROUP_NAME"
AZURE_LOCATION="$LOCATION"

# ========================================
# User Information
# ========================================
AZURE_USER_OBJECT_ID="$USER_OBJECT_ID"
EOL

echo ".env file created successfully."

echo ""
echo "🎉 Resource creation completed successfully!"
echo "=============================================="
echo "📋 Resources Created:"
echo "  📁 Resource Group: $RESOURCE_GROUP_NAME"
echo "  🔐 Key Vault: $KEY_VAULT_NAME"
echo "  🌐 Key Vault URI: $KEY_VAULT_URI"
echo ""
echo "🔐 RBAC Permissions Configured:"
echo "  ✅ Key Vault Secrets Officer role assigned to current user"
echo "  ✅ Key Vault Administrator role assigned to current user"
echo ""
echo "🗝️  Sample Secret Created:"
echo "  � Secret Name: sample-secret"
echo "  � Secret Value: This is a sample secret value for testing"
echo ""
echo "📄 Configuration file created: .env"
echo "👤 Current user ($USER_OBJECT_ID) has full access to Key Vault"
echo ""
echo "🔧 Next steps:"
echo "  1. Copy the .env file to your project directory"
echo "  2. Use the Azure Key Vault Rust SDK to:"
echo "     - Create secrets: az keyvault secret set --vault-name $KEY_VAULT_NAME --name <secret-name> --value <secret-value>"
echo "     - Read secrets: az keyvault secret show --vault-name $KEY_VAULT_NAME --name <secret-name>"
echo "     - Update secrets: az keyvault secret set --vault-name $KEY_VAULT_NAME --name <secret-name> --value <new-value>"
echo "     - Delete secrets: az keyvault secret delete --vault-name $KEY_VAULT_NAME --name <secret-name>"
echo "  3. Test your Rust application with managed identity authentication"
echo "=============================================="
