#!/bin/bash

# GrantFlow End-to-End Test Script
# Demonstrates complete grant lifecycle

set -e

echo "🧪 GrantFlow End-to-End Test"
echo "============================"
echo ""

# Check prerequisites
if [ ! -f "deployed-contracts.json" ]; then
    echo "❌ Contracts not deployed. Run ./scripts/deploy.sh first"
    exit 1
fi

NETWORK="testnet"
GRANT_POOL_ID=$(jq -r '.contracts.grantPool' deployed-contracts.json)
PROPOSAL_ID=$(jq -r '.contracts.proposal' deployed-contracts.json)
MILESTONE_ID=$(jq -r '.contracts.milestone' deployed-contracts.json)
GOVERNANCE_ID=$(jq -r '.contracts.governance' deployed-contracts.json)

echo "📋 Using Contracts:"
echo "  Grant Pool: $GRANT_POOL_ID"
echo "  Proposal: $PROPOSAL_ID"
echo "  Milestone: $MILESTONE_ID"
echo "  Governance: $GOVERNANCE_ID"
echo ""

# Generate test addresses
ADMIN=$(soroban keys generate admin --network $NETWORK 2>&1 | grep "Public key:" | awk '{print $3}')
PROPOSER=$(soroban keys generate proposer --network $NETWORK 2>&1 | grep "Public key:" | awk '{print $3}')
VOTER1=$(soroban keys generate voter1 --network $NETWORK 2>&1 | grep "Public key:" | awk '{print $3}')

echo "👥 Test Addresses Generated:"
echo "  Admin: $ADMIN"
echo "  Proposer: $PROPOSER"
echo "  Voter1: $VOTER1"
echo ""

echo "1️⃣  Creating Grant Pool..."
POOL_ID=$(soroban contract invoke \
    --id $GRANT_POOL_ID \
    --network $NETWORK \
    --source admin \
    -- create_pool \
    --admin $ADMIN \
    --name "Community Grants" \
    --description "Grants for community projects" \
    --total_funds 1000000)

echo "✅ Pool created with ID: $POOL_ID"
echo ""

echo "2️⃣  Setting Voting Power..."
soroban contract invoke \
    --id $GOVERNANCE_ID \
    --network $NETWORK \
    --source admin \
    -- set_voting_power \
    --admin $ADMIN \
    --voter $VOTER1 \
    --power 100

echo "✅ Voting power set for voter1"
echo ""

echo "3️⃣  Submitting Proposal..."
PROPOSAL_NUM=$(soroban contract invoke \
    --id $PROPOSAL_ID \
    --network $NETWORK \
    --source proposer \
    -- submit_proposal \
    --pool_id $POOL_ID \
    --proposer $PROPOSER \
    --title "Build Community Dashboard" \
    --description "A dashboard for tracking metrics" \
    --requested_amount 50000 \
    --voting_duration 86400 \
    --milestones '["Phase 1: Design", "Phase 2: Development"]')

echo "✅ Proposal submitted with ID: $PROPOSAL_NUM"
echo ""

echo "4️⃣  Casting Vote..."
soroban contract invoke \
    --id $PROPOSAL_ID \
    --network $NETWORK \
    --source voter1 \
    -- vote \
    --proposal_id $PROPOSAL_NUM \
    --voter $VOTER1 \
    --vote_for true \
    --weight 100

echo "✅ Vote cast successfully"
echo ""

echo "🎉 Test Flow Complete!"
echo "============================"
echo ""
echo "Summary:"
echo "  ✓ Grant pool created"
echo "  ✓ Voting power assigned"
echo "  ✓ Proposal submitted"
echo "  ✓ Vote cast"
echo ""
echo "Next steps would be:"
echo "  5. Wait for voting period to end"
echo "  6. Finalize voting"
echo "  7. Create milestones"
echo "  8. Submit milestone evidence"
echo "  9. Approve milestones and release funds"
echo ""
