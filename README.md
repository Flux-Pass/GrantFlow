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
- **Frontend**: React + TypeScript
- **Wallet Integration**: Freighter, Albedo

## Project Structure

```
grantflow/
├── contracts/          # Soroban smart contracts
├── frontend/          # React web application
├── scripts/           # Deployment and utility scripts
└── docs/             # Documentation
```

## Getting Started

### Prerequisites

- Rust and Cargo
- Soroban CLI
- Node.js and npm
- Stellar account with testnet XLM

### Installation

```bash
# Install Soroban CLI
cargo install --locked soroban-cli

# Clone and setup
git clone <repository-url>
cd grantflow
```

### Build Contracts

```bash
cd contracts
soroban contract build
```

### Run Frontend

```bash
cd frontend
npm install
npm run dev
```

## Smart Contract Architecture

### Main Contracts

1. **GrantPool Contract**: Manages grant pools and fund allocation
2. **Proposal Contract**: Handles proposal submission and voting
3. **Milestone Contract**: Tracks milestone completion and fund release
4. **Governance Contract**: Manages voting rights and parameters

## Contributing

This project is designed for Stellar ecosystem contributions. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details

## Maintainer Value

GrantFlow combines three critical components:
- **Governance**: Democratic decision-making for fund allocation
- **Payments**: Secure, automated fund distribution via Stellar
- **Tracking**: Complete audit trail and milestone verification

This combination creates a powerful tool for DAOs, foundations, and community organizations managing grants transparently.
