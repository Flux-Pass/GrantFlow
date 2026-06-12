# GrantFlow Development Guide

This guide covers the day-to-day workflow for developing, testing, and deploying the GrantFlow Soroban contracts.

## Repository Requirements

Install the following before working on the contracts:

- Rust and Cargo
- Soroban CLI compatible with `soroban-sdk = "20.0.0"`
- `wasm32-unknown-unknown` Rust target
- `jq` for deployment helper scripts
- A POSIX shell environment for scripts in `scripts/`

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli --version 20.0.0
```

## Local Build

Run builds from the `contracts/` workspace:

```bash
cd contracts
soroban contract build
```

The build outputs WASM artifacts under:

```text
contracts/target/wasm32-unknown-unknown/release/
```

Expected contract artifacts:

- `grant_pool.wasm`
- `proposal.wasm`
- `milestone.wasm`
- `governance.wasm`

## Tests

Run every contract test:

```bash
cd contracts
cargo test
```

Run one contract test suite:

```bash
cd contracts/proposal
cargo test
```

Use `-- --nocapture` when you need test output:

```bash
cargo test -- --nocapture
```

## Contract Responsibilities

### Grant Pool

The grant pool contract owns pool metadata and available budget accounting. It can create pools, reserve funds, return funds, deactivate pools, and report pool counts.

### Proposal

The proposal contract owns proposal metadata, voting state, and proposal status. Voting uses weights supplied by the caller. Finalization currently uses a simple majority: `votes_for > votes_against`.

### Milestone

The milestone contract stores milestone descriptions, amounts, evidence URLs, timestamps, and review status. It records payment state through `mark_paid`, but it does not transfer Stellar assets.

### Governance

The governance contract stores voting power and governance parameters. It exposes quorum checks and voting duration settings. Current proposal voting does not call governance automatically.

## Deployment Workflow

Configure the Stellar testnet once:

```bash
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

Generate or import a source account:

```bash
soroban keys generate my-account --network testnet
soroban keys address my-account
```

Deploy all contracts:

```bash
./scripts/deploy.sh
```

The deployment script writes contract IDs to:

```text
deployed-contracts.json
```

Initialize contracts:

```bash
ADMIN=$(soroban keys address my-account)
./scripts/initialize.sh "$ADMIN"
```

Only the grant pool and governance contracts require initialization in the current code.

## Helper Scripts

| Script | Purpose |
| --- | --- |
| `scripts/deploy.sh` | Builds if needed, deploys all contracts to testnet, and writes `deployed-contracts.json`. |
| `scripts/initialize.sh` | Initializes the grant pool and governance contracts with an admin address. |
| `scripts/test-flow.sh` | Demonstrates pool creation, voting power assignment, proposal submission, and voting on testnet. |

## Current Limitations

- Cross-contract calls are not wired yet.
- Proposal finalization does not check governance quorum.
- Pool fund allocation is not automatically called when a proposal is approved.
- Milestone approval does not transfer tokens or native assets.
- `update_status`, `allocate_funds`, `return_funds`, and `mark_paid` do not currently restrict caller contracts.
- The shell scripts contain user-facing status text only; they are not a substitute for contract tests.

## Recent Additions

- Grant pools can now be activated after being deactivated.
- Grant pool names and descriptions can be updated.
- Pending proposals can be cancelled by the proposer.

Document these limitations in issues or pull requests when proposing changes that depend on production-grade enforcement.

## Documentation Checklist

When changing contract behavior, update the relevant docs:

- Update [API.md](API.md) for function signatures, authorization, panics, events, or data structures.
- Update [ARCHITECTURE.md](ARCHITECTURE.md) when responsibilities, data flow, or security assumptions change.
- Update [QUICKSTART.md](QUICKSTART.md) if commands, script behavior, or deployment steps change.
- Update [README.md](../README.md) for major scope or roadmap changes.

## Suggested Development Flow

1. Create or update tests for the contract behavior.
2. Implement the contract change.
3. Run `cargo test` from `contracts/`.
4. Build WASM artifacts with `soroban contract build`.
5. Update documentation.
6. Test deployment flow on testnet when deployment behavior changes.
