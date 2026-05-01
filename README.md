# 📊 GrantFlow – Transparent Grant Allocation Engine

A decentralized system for managing and distributing community grants on the Stellar network.

## Problem Statement

Traditional grant systems lack transparency and proper tracking mechanisms, making it difficult for communities to:
- Track fund allocation and usage
- Ensure accountability in grant distribution
- Verify milestone completion
- Maintain an auditable history of decisions

## Solution

GrantFlow provides a transparent, blockchain-based grant management system with:

### Core Features

- **Grant Pool Creation**: Organizations can create dedicated grant pools with specific budgets
- **Proposal Submission**: Community members submit detailed grant proposals
- **Voting Mechanism**: Token-weighted or reputation-based voting for proposal approval
- **Milestone-Based Release**: Funds released incrementally upon milestone completion
- **Audit Trail**: Complete event history stored on-chain for transparency

## Technical Stack

- **Blockchain**: Stellar Network
- **Smart Contracts**: Soroban (Rust)
- **Language**: Rust with WebAssembly compilation

## Project Structure

```
grantflow/
├── contracts/
│   ├── grant-pool/      # Pool management and fund allocation
│   ├── proposal/        # Proposal submission and voting
│   ├── milestone/       # Milestone tracking and fund release
│   └── governance/      # Voting rights and parameters
├── CONTRIBUTING.md      # Contribution guidelines
├── LICENSE             # MIT License
└── README.md           # This file
```

## Smart Contract Architecture

### 1. Grant Pool Contract
Manages grant pools and fund allocation:
- Create pools with specific budgets
- Track available funds
- Allocate and return funds
- Pool activation/deactivation

### 2. Proposal Contract
Handles proposal lifecycle:
- Submit proposals with milestones
- Token-weighted voting system
- Automatic vote tallying
- Status management (Pending → Approved/Rejected → Active → Completed)

### 3. Milestone Contract
Tracks milestone completion:
- Create milestones for approved proposals
- Submit completion evidence
- Approve/reject submissions
- Track payment status

### 4. Governance Contract
Manages voting parameters:
- Set voting power (token-weighted or reputation-based)
- Configure quorum thresholds
- Adjust voting durations
- Check quorum requirements

## Getting Started

### Prerequisites

- Rust and Cargo
- Soroban CLI (v20.0.0+)
- Stellar account with testnet XLM

### Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Clone repository
git clone https://github.com/Harbduls/GrantFlow.git
cd GrantFlow
```

### Build Contracts

```bash
cd contracts

# Build all contracts
soroban contract build

# Run tests
soroban contract test
```

### Deploy to Testnet

```bash
# Deploy grant-pool contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/grant_pool.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet

# Repeat for other contracts
```

## Usage Example

### 1. Create a Grant Pool

```rust
// Initialize and create pool
let pool_id = grant_pool_client.create_pool(
    &admin,
    &"Community Development Fund",
    &"Grants for community projects",
    &1_000_000  // 1M stroops
);
```

### 2. Submit a Proposal

```rust
let proposal_id = proposal_client.submit_proposal(
    &pool_id,
    &proposer,
    &"Build Community Dashboard",
    &"A dashboard for tracking community metrics",
    &50_000,  // Requested amount
    &86400,   // 24h voting period
    &milestones
);
```

### 3. Vote on Proposal

```rust
proposal_client.vote(
    &proposal_id,
    &voter,
    &true,    // Vote for
    &100      // Voting weight
);
```

### 4. Submit Milestone Evidence

```rust
milestone_client.submit_milestone(
    &milestone_id,
    &proposer,
    &"https://github.com/project/milestone-1"
);
```

### 5. Approve and Release Funds

```rust
milestone_client.approve_milestone(
    &milestone_id,
    &admin
);
```

## Event Tracking

All contracts emit events for complete audit trail:

- `pool_created` - New grant pool created
- `proposal_submitted` - New proposal submitted
- `vote_cast` - Vote recorded
- `voting_finalized` - Voting period ended
- `milestone_submitted` - Milestone evidence submitted
- `milestone_approved` - Milestone approved, funds released
- `funds_allocated` - Funds allocated from pool
- `voting_power_set` - Voting power updated

## Testing

```bash
# Run all tests
cd contracts
cargo test

# Run specific contract tests
cd grant-pool
cargo test
```

## Security Considerations

- All state-changing functions require authentication
- Voting periods are enforced on-chain
- Double-voting prevention
- Fund allocation checks before approval
- Admin-only governance parameter updates

## Maintainer Value

GrantFlow combines three critical components:

1. **Governance**: Democratic decision-making for fund allocation
2. **Payments**: Secure, automated fund distribution via Stellar
3. **Tracking**: Complete audit trail and milestone verification

This combination creates a powerful tool for DAOs, foundations, and community organizations managing grants transparently.

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Roadmap

- [x] Core smart contracts (Grant Pool, Proposal, Milestone, Governance)
- [x] Comprehensive test coverage
- [ ] Frontend web application
- [ ] Multi-signature approval support
- [ ] Reputation-based voting system
- [ ] Integration with Stellar DEX for token swaps
- [ ] Mobile application
- [ ] Analytics dashboard

## License

MIT License - see [LICENSE](LICENSE) for details

## Contact

For questions or support, please open an issue on GitHub.

---

**Built for the Stellar ecosystem** 🌟

Leveraging Soroban smart contracts for transparent, efficient grant management.
