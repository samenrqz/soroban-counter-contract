# ⛓️ Soroban Counter dApp

A decentralized counter smart contract built on **Stellar Testnet** using the **Soroban SDK**.

## Features
- ✅ Initialize counter
- ✅ Increment / Decrement
- ✅ On-chain event emission
- ✅ Stunning animated frontend UI

## Contract Details
- **Network:** Stellar Testnet
- **Contract ID:** `YOUR_CONTRACT_ID_HERE`
- **Language:** Rust + Soroban SDK v21

## Project Structure
```
soroban-counter-contract/
├── src/lib.rs       # Smart contract
├── frontend/        # Web UI
│   └── index.html
├── Cargo.toml
└── README.md
```

## Run Tests
```bash
cargo test
```

## Build
```bash
cargo build --release --target wasm32-unknown-unknown
```

## Deploy
```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_counter_contract.wasm \
  --source alice \
  --network testnet
```

## Built With
- [Soroban SDK](https://soroban.stellar.org)
- [Stellar CLI](https://github.com/stellar/stellar-cli)