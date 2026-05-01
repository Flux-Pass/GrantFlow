# Contributing to GrantFlow

Thank you for your interest in contributing to GrantFlow! This project is designed for submission to Stellar ecosystem grant programs.

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

4. Install Node.js dependencies:
```bash
cd frontend && npm install
```

## Project Structure

- `contracts/` - Soroban smart contracts written in Rust
- `frontend/` - React-based web interface
- `scripts/` - Deployment and testing scripts
- `docs/` - Technical documentation

## Building and Testing

### Smart Contracts

```bash
cd contracts
soroban contract build
soroban contract test
```

### Frontend

```bash
cd frontend
npm run build
npm run test
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
- Use TypeScript for frontend code
- Add inline documentation for public functions
- Include unit tests for new features

## Security

- Never commit private keys or secrets
- Report security vulnerabilities privately
- Follow Soroban security best practices

## Questions?

Open an issue or reach out to the maintainers.
