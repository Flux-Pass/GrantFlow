# GrantFlow - Transparent Grant Allocation Engine

GrantFlow is a Soroban smart contract project for managing community grants on the Stellar network. It models grant pools, proposal voting, milestone review, and governance parameters so fund allocation decisions can be tracked transparently on-chain.

## What It Solves

Traditional grant programs often rely on off-chain spreadsheets, manual approvals, and fragmented reporting. GrantFlow provides a contract-level foundation for:

- Creating named grant pools with dedicated budgets
- Accepting grant proposals tied to a pool
- Recording weighted votes and proposal outcomes
- Tracking milestone submissions, approvals, rejections, and payment state
- Emitting events that create an auditable history of grant activity

## Current Scope

This repository contains the smart contracts, tests, deployment scripts, and technical documentation. It does not currently include a frontend application or token payment integration.

Important implementation notes:

- The contracts are deployed independently.
- The proposal contract accepts vote weights supplied at call time; it does not read the governance contract directly.
- The governance contract stores voting power and quorum settings, but quorum is not enforced by the proposal contract yet.
- Milestone approval updates milestone state; actual asset transfers are not implemented in the current contracts.

## Contracts

| Contract | Purpose |
| --- | --- |
| `grant-pool` | Creates pools, tracks total and available funds, allocates and returns budget amounts. |
| `proposal` | Stores proposals, accepts weighted votes, finalizes proposals by simple majority. |
| `milestone` | Creates milestones, records evidence URLs, tracks review and payment status. |
| `governance` | Stores voting power, quorum threshold, and default voting duration. |

## Repository Layout

```text
GrantFlow/
├── contracts/
│   ├── grant-pool/
│   ├── proposal/
│   ├── milestone/
│   └── governance/
├── docs/
│   ├── API.md
│   ├── ARCHITECTURE.md
│   ├── DEVELOPMENT.md
│   └── QUICKSTART.md
├── scripts/
│   ├── deploy.sh
│   ├── initialize.sh
│   └── test-flow.sh
├── CONTRIBUTING.md
├── LICENSE
└── README.md
```

## Prerequisites

- Rust and Cargo
- `wasm32-unknown-unknown` Rust target
- Soroban CLI compatible with Soroban SDK `20.0.0`
- A funded Stellar testnet account for deployment
- `jq` when using the helper scripts

## Quick Start

```bash
# Clone the repository
git clone https://github.com/Harbduls/GrantFlow.git
cd GrantFlow

# Install the WASM target
rustup target add wasm32-unknown-unknown

# Build contracts
cd contracts
soroban contract build

# Run tests
cargo test
```

For a full deployment walkthrough, see [docs/QUICKSTART.md](docs/QUICKSTART.md).

## Documentation

- [Quick Start](docs/QUICKSTART.md): Build, deploy, initialize, and invoke the contracts.
- [Architecture](docs/ARCHITECTURE.md): Contract responsibilities, lifecycle, storage, events, and security model.
- [API Reference](docs/API.md): Public functions, parameters, return values, events, and data structures.
- [Development Guide](docs/DEVELOPMENT.md): Local workflow, scripts, testing strategy, and current limitations.
- [Contributing](CONTRIBUTING.md): Contributor setup, standards, and pull request process.

## Common Workflow

1. Initialize the grant pool and governance contracts.
2. Create a grant pool with an available budget.
3. Submit a proposal against that pool.
4. Set voting power in the governance contract.
5. Vote on the proposal with an explicit weight.
6. Finalize voting after the voting deadline.
7. Create milestones for approved proposals.
8. Submit milestone evidence and approve or reject it.
9. Mark approved milestones as paid after payment is handled externally.

## Events

The contracts emit events for key lifecycle actions, including:

- `pool_created`
- `funds_allocated`
- `funds_returned`
- `proposal_submitted`
- `vote_cast`
- `voting_finalized`
- `milestones_created`
- `milestone_submitted`
- `milestone_approved`
- `milestone_rejected`
- `milestone_paid`
- `voting_power_set`

## Testing

```bash
cd contracts
cargo test
```

You can also run an individual contract test suite:

```bash
cd contracts/grant-pool
cargo test
```

## Roadmap

- [x] Core Soroban contracts
- [x] Contract unit tests
- [x] Deployment and initialization scripts
- [ ] Cross-contract enforcement for governance and pool allocation
- [ ] Token-based payment release
- [ ] Frontend application
- [ ] Multi-signature milestone approvals
- [ ] Reputation-based voting integration
- [ ] Analytics dashboard

## License

GrantFlow is licensed under the MIT License. See [LICENSE](LICENSE) for details.
