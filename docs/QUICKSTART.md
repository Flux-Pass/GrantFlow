# GrantFlow Quick Start Guide

Get up and running with GrantFlow in minutes.

## Prerequisites

Before you begin, ensure you have:

1. **Rust and Cargo** installed
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Soroban CLI** installed
   ```bash
   cargo install --locked soroban-cli
   ```

3. **WebAssembly target** added
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

4. **Stellar account** with testnet XLM
   - Create account at [Stellar Laboratory](https://laboratory.stellar.org/#account-creator?network=test)
   - Fund with testnet XLM from the friendbot

## Step 1: Clone and Build

```bash
# Clone the repository
git clone https://github.com/Harbduls/GrantFlow.git
cd GrantFlow

# Build all contracts
cd contracts
soroban contract build
cd ..
```

You should see WASM files in `target/wasm32-unknown-unknown/release/`:
- `grant_pool.wasm`
- `proposal.wasm`
- `milestone.wasm`
- `governance.wasm`

## Step 2: Configure Stellar Network

```bash
# Add testnet network
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Import your account (or generate new one)
soroban keys generate my-account --network testnet

# Get your address
soroban keys address my-account
```

## Step 3: Deploy Contracts

```bash
# Make deploy script executable
chmod +x scripts/deploy.sh

# Deploy all contracts
./scripts/deploy.sh
```

This will:
- Deploy all four contracts to testnet
- Save contract IDs to `deployed-contracts.json`
- Display contract addresses

## Step 4: Initialize Contracts

```bash
# Make initialize script executable
chmod +x scripts/initialize.sh

# Get your admin address
ADMIN=$(soroban keys address my-account)

# Initialize contracts
./scripts/initialize.sh $ADMIN
```

This sets up:
- Grant Pool contract with your admin address
- Governance contract with default parameters
  - Quorum: 1000
  - Voting duration: 24 hours

## Step 5: Create Your First Grant Pool

```bash
# Read contract ID
GRANT_POOL_ID=$(jq -r '.contracts.grantPool' deployed-contracts.json)

# Create a pool
soroban contract invoke \
  --id $GRANT_POOL_ID \
  --source my-account \
  --network testnet \
  -- create_pool \
  --admin $ADMIN \
  --name "Community Development Fund" \
  --description "Grants for community projects" \
  --total_funds 1000000
```

## Step 6: Submit a Proposal

```bash
# Read contract IDs
PROPOSAL_ID=$(jq -r '.contracts.proposal' deployed-contracts.json)

# Submit proposal
soroban contract invoke \
  --id $PROPOSAL_ID \
  --source my-account \
  --network testnet \
  -- submit_proposal \
  --pool_id 1 \
  --proposer $ADMIN \
  --title "Build Community Dashboard" \
  --description "A web dashboard for tracking community metrics and engagement" \
  --requested_amount 50000 \
  --voting_duration 86400 \
  --milestones '["Phase 1: Design and Planning", "Phase 2: Development", "Phase 3: Testing and Launch"]'
```

## Step 7: Set Voting Power

```bash
# Read governance contract ID
GOVERNANCE_ID=$(jq -r '.contracts.governance' deployed-contracts.json)

# Set voting power for yourself
soroban contract invoke \
  --id $GOVERNANCE_ID \
  --source my-account \
  --network testnet \
  -- set_voting_power \
  --admin $ADMIN \
  --voter $ADMIN \
  --power 1000
```

## Step 8: Vote on Proposal

```bash
# Cast your vote
soroban contract invoke \
  --id $PROPOSAL_ID \
  --source my-account \
  --network testnet \
  -- vote \
  --proposal_id 1 \
  --voter $ADMIN \
  --vote_for true \
  --weight 1000
```

## Step 9: Finalize Voting

After the voting period ends (24 hours by default), finalize:

```bash
# Finalize voting
soroban contract invoke \
  --id $PROPOSAL_ID \
  --source my-account \
  --network testnet \
  -- finalize_voting \
  --proposal_id 1
```

## Step 10: Create Milestones

For approved proposals, create milestones:

```bash
# Read milestone contract ID
MILESTONE_ID=$(jq -r '.contracts.milestone' deployed-contracts.json)

# Create milestones
soroban contract invoke \
  --id $MILESTONE_ID \
  --source my-account \
  --network testnet \
  -- create_milestones \
  --proposal_id 1 \
  --descriptions '["Phase 1: Design and Planning", "Phase 2: Development", "Phase 3: Testing and Launch"]' \
  --amounts '[15000, 20000, 15000]'
```

## Step 11: Submit Milestone Evidence

When a milestone is complete:

```bash
# Submit evidence
soroban contract invoke \
  --id $MILESTONE_ID \
  --source my-account \
  --network testnet \
  -- submit_milestone \
  --milestone_id 1 \
  --submitter $ADMIN \
  --evidence_url "https://github.com/myproject/milestone-1"
```

## Step 12: Approve Milestone

Review and approve the milestone:

```bash
# Approve milestone
soroban contract invoke \
  --id $MILESTONE_ID \
  --source my-account \
  --network testnet \
  -- approve_milestone \
  --milestone_id 1 \
  --approver $ADMIN
```

## Querying Data

### Get Pool Details

```bash
soroban contract invoke \
  --id $GRANT_POOL_ID \
  --network testnet \
  -- get_pool \
  --pool_id 1
```

### Get Proposal Details

```bash
soroban contract invoke \
  --id $PROPOSAL_ID \
  --network testnet \
  -- get_proposal \
  --proposal_id 1
```

### Get Milestone Details

```bash
soroban contract invoke \
  --id $MILESTONE_ID \
  --network testnet \
  -- get_milestone \
  --milestone_id 1
```

### Get Voting Power

```bash
soroban contract invoke \
  --id $GOVERNANCE_ID \
  --network testnet \
  -- get_voting_power \
  --voter $ADMIN
```

## Running Tests

```bash
# Run all tests
cd contracts
cargo test

# Run specific contract tests
cd grant-pool
cargo test

# Run with output
cargo test -- --nocapture
```

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
soroban contract build
```

### Network Issues

If deployment fails:

```bash
# Check network configuration
soroban network ls

# Verify account has funds
soroban keys address my-account
# Check balance at https://laboratory.stellar.org
```

### Contract Invocation Errors

If contract calls fail:

```bash
# Check contract is deployed
soroban contract info --id <CONTRACT_ID> --network testnet

# Verify you're using correct source account
soroban keys ls
```

## Next Steps

- Read the [Architecture Documentation](ARCHITECTURE.md)
- Explore the [API Reference](API.md)
- Check [CONTRIBUTING.md](../CONTRIBUTING.md) for development guidelines
- Build a frontend application
- Integrate with your DAO or community

## Common Use Cases

### For DAOs
1. Create pool for treasury funds
2. Members submit proposals
3. Token-weighted voting
4. Milestone-based fund release

### For Foundations
1. Create multiple pools (categories)
2. Set reputation-based voting
3. Multi-reviewer milestone approval
4. Comprehensive audit trail

### For Communities
1. Crowdfund grant pools
2. Democratic proposal selection
3. Transparent fund tracking
4. Community-driven governance

## Support

- GitHub Issues: [Report bugs or request features](https://github.com/Harbduls/GrantFlow/issues)
- Stellar Discord: Join #soroban channel
- Documentation: [Full docs](https://github.com/Harbduls/GrantFlow/tree/main/docs)

## Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Laboratory](https://laboratory.stellar.org/)
- [Stellar Expert](https://stellar.expert/) - Block explorer
