#!/bin/bash

# GrantFlow Deployment Script for Stellar Testnet
# Usage: ./scripts/deploy.sh

set -e

echo "🚀 Deploying GrantFlow Contracts to Stellar Testnet"
echo "=================================================="

# Check if Soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI not found. Please install it first:"
    echo "   cargo install --locked soroban-cli"
    exit 1
fi

# Check if contracts are built
if [ ! -d "target/wasm32-unknown-unknown/release" ]; then
    echo "📦 Building contracts first..."
    cd contracts
    soroban contract build
    cd ..
fi

# Set network
NETWORK="testnet"
echo "🌐 Network: $NETWORK"

# Deploy contracts
echo ""
echo "📝 Deploying Grant Pool Contract..."
GRANT_POOL_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/grant_pool.wasm \
    --network $NETWORK)
echo "✅ Grant Pool Contract: $GRANT_POOL_ID"

echo ""
echo "📝 Deploying Proposal Contract..."
PROPOSAL_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/proposal.wasm \
    --network $NETWORK)
echo "✅ Proposal Contract: $PROPOSAL_ID"

echo ""
echo "📝 Deploying Milestone Contract..."
MILESTONE_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/milestone.wasm \
    --network $NETWORK)
echo "✅ Milestone Contract: $MILESTONE_ID"

echo ""
echo "📝 Deploying Governance Contract..."
GOVERNANCE_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/governance.wasm \
    --network $NETWORK)
echo "✅ Governance Contract: $GOVERNANCE_ID"

# Save contract IDs
echo ""
echo "💾 Saving contract IDs..."
cat > deployed-contracts.json <<EOF
{
  "network": "$NETWORK",
  "contracts": {
    "grantPool": "$GRANT_POOL_ID",
    "proposal": "$PROPOSAL_ID",
    "milestone": "$MILESTONE_ID",
    "governance": "$GOVERNANCE_ID"
  },
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "✅ Contract IDs saved to deployed-contracts.json"

echo ""
echo "🎉 Deployment Complete!"
echo "=================================================="
echo ""
echo "Contract Addresses:"
echo "  Grant Pool:  $GRANT_POOL_ID"
echo "  Proposal:    $PROPOSAL_ID"
echo "  Milestone:   $MILESTONE_ID"
echo "  Governance:  $GOVERNANCE_ID"
echo ""
echo "Next steps:"
echo "  1. Initialize contracts with admin address"
echo "  2. Set governance parameters"
echo "  3. Create your first grant pool"
echo ""
