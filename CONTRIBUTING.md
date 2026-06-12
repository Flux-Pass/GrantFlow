# Contributing to GrantFlow

Thank you for your interest in contributing to GrantFlow. This project is a Soroban smart contract codebase for transparent grant allocation on Stellar.

## Development Setup

### Prerequisites

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Soroban CLI:
```bash
cargo install --locked soroban-cli --version 20.0.0
```

3. Add WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

4. Optional helper tools:
```bash
# Needed by the shell scripts that read deployed-contracts.json
jq --version
```

## Project Structure

- `contracts/` - Soroban smart contracts written in Rust
- `scripts/` - Deployment and testing scripts
- `docs/` - Technical documentation

## Building and Testing

### Smart Contracts

```bash
cd contracts
soroban contract build
cargo test
```

### Scripts

The scripts are written for a POSIX shell environment:

```bash
./scripts/deploy.sh
./scripts/initialize.sh <ADMIN_ADDRESS>
./scripts/test-flow.sh
```

## Contribution Guidelines

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for new functionality
4. Ensure all tests pass
5. Commit with clear messages (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Code Standards

- Follow Rust best practices for smart contracts
- Keep documentation in `README.md` and `docs/` current when behavior changes
- Add concise inline documentation for public functions
- Include unit tests for new features

## Security

- Never commit private keys or secrets
- Report security vulnerabilities privately
- Follow Soroban security best practices

## Questions?

Open an issue or reach out to the maintainers.
