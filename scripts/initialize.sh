#!/bin/bash

# GrantFlow Contract Initialization Script
# Usage: ./scripts/initialize.sh <ADMIN_ADDRESS>

set -e

if [ -z "$1" ]; then
    echo "❌ Error: Admin address required"
    echo "Usage: ./scripts/initialize.sh <ADMIN_ADDRESS>"
    exit 1
fi

ADMIN_ADDRESS=$1
NETWORK="testnet"

echo "🔧 Initializing GrantFlow Contracts"
echo "===================================="
echo "Admin: $ADMIN_ADDRESS"
echo "Network: $NETWORK"
echo ""

# Check if contracts are deployed
if [ ! -f "deployed-contracts.json" ]; then
    echo "❌ deployed-contracts.json not found. Please deploy contracts first."
    exit 1
fi

# Read contract IDs
GRANT_POOL_ID=$(jq -r '.contracts.grantPool' deployed-contracts.json)
GOVERNANCE_ID=$(jq -r '.contracts.governance' deployed-contracts.json)

echo "📝 Initializing Grant Pool Contract..."
soroban contract invoke \
    --id $GRANT_POOL_ID \
    --network $NETWORK \
    -- initialize \
    --admin $ADMIN_ADDRESS

echo "✅ Grant Pool initialized"

echo ""
echo "📝 Initializing Governance Contract..."
soroban contract invoke \
    --id $GOVERNANCE_ID \
    --network $NETWORK \
    -- initialize \
    --admin $ADMIN_ADDRESS \
    --quorum_threshold 1000 \
    --voting_duration 86400

echo "✅ Governance initialized"

echo ""
echo "🎉 Initialization Complete!"
echo "===================================="
echo ""
echo "Default Parameters:"
echo "  Quorum Threshold: 1000"
echo "  Voting Duration: 86400 seconds (24 hours)"
echo ""
echo "You can now:"
echo "  1. Create grant pools"
echo "  2. Set voting power for addresses"
echo "  3. Accept proposal submissions"
echo ""
