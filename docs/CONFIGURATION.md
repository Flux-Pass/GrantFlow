# GrantFlow Configuration

This document describes the configuration values used by the GrantFlow contracts, scripts, and local development workflow.

## Network Configuration

The helper scripts target Stellar testnet by default:

```text
Network name: testnet
RPC URL: https://soroban-testnet.stellar.org:443
Passphrase: Test SDF Network ; September 2015
```

Configure the network with:

```bash
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Account Configuration

The scripts assume you have a Soroban key available for signing transactions.

Create a testnet key:

```bash
soroban keys generate my-account --network testnet
```

Read the public address:

```bash
soroban keys address my-account
```

Use this address as the admin when initializing contracts:

```bash
./scripts/initialize.sh <ADMIN_ADDRESS>
```

## Deployment Output

`scripts/deploy.sh` writes deployed contract IDs to:

```text
deployed-contracts.json
```

Expected shape:

```json
{
  "network": "testnet",
  "contracts": {
    "grantPool": "<GRANT_POOL_CONTRACT_ID>",
    "proposal": "<PROPOSAL_CONTRACT_ID>",
    "milestone": "<MILESTONE_CONTRACT_ID>",
    "governance": "<GOVERNANCE_CONTRACT_ID>"
  },
  "deployedAt": "<UTC_TIMESTAMP>"
}
```

The initialization and test-flow scripts read this file with `jq`.

## Governance Defaults

`scripts/initialize.sh` initializes the governance contract with:

| Setting | Default |
| --- | --- |
| Quorum threshold | `1000` |
| Voting duration | `86400` seconds |

The governance contract also returns a hardcoded `min_proposal_amount` of `1000` from `get_params()`.

These values can be changed on-chain after initialization:

```bash
soroban contract invoke \
  --id <GOVERNANCE_CONTRACT_ID> \
  --source my-account \
  --network testnet \
  -- update_quorum \
  --admin <ADMIN_ADDRESS> \
  --new_threshold 2000
```

```bash
soroban contract invoke \
  --id <GOVERNANCE_CONTRACT_ID> \
  --source my-account \
  --network testnet \
  -- update_voting_duration \
  --admin <ADMIN_ADDRESS> \
  --new_duration 172800
```

## Contract Build Configuration

The contract workspace is configured in `contracts/Cargo.toml`.

Important settings:

| Setting | Value |
| --- | --- |
| Workspace resolver | `2` |
| Soroban SDK | `20.0.0` |
| Release optimization | `z` |
| Overflow checks | `true` |
| Panic strategy | `abort` |
| LTO | `true` |

Build all contracts from the workspace:

```bash
cd contracts
soroban contract build
```

## Script Configuration

The scripts currently use fixed values:

| Script | Configured value |
| --- | --- |
| `scripts/deploy.sh` | `NETWORK="testnet"` |
| `scripts/initialize.sh` | `NETWORK="testnet"` |
| `scripts/test-flow.sh` | `NETWORK="testnet"` |

To use another network, update the `NETWORK` variable in the relevant script and make sure the matching Soroban network is configured locally.

## Environment Files

There is no required `.env` file in the current repository. Avoid committing private keys, seed phrases, RPC credentials, or production contract IDs.

For local notes, prefer an ignored file such as:

```text
.env.local
```

Recommended local variables:

```bash
SOROBAN_NETWORK=testnet
SOROBAN_ACCOUNT=my-account
ADMIN_ADDRESS=<PUBLIC_ADDRESS>
```

These variables are for developer convenience only. The current scripts do not automatically read them.

## Configuration Checklist

Before deploying or invoking contracts, confirm:

- Soroban CLI is installed.
- The `testnet` network is configured.
- Your signing account exists and is funded.
- Contracts have been built.
- `deployed-contracts.json` exists after deployment.
- The grant pool and governance contracts have been initialized.
- `jq` is available when using helper scripts.
