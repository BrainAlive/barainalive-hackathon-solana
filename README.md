# BAAI Node System

## Project Structure

## Overview
BAAI FOCII (Browser-based extension powered by AI) is a decentralized system that enables browser-based engagement analytics using Edge AI computation with zero-knowledge proofs and token rewards on Solana.

## Architecture

### Components

1. **BAAI Token & Node License System**
   - SPL Token contract for BAAI tokens
   - NFT-based Node License system
   - Reward distribution mechanism

2. **FOCII Verifier**
   - Zero-knowledge proof verification
   - Data integrity validation
   - Reward triggering system

### Flow

1. **Node License Creation**
   - Users obtain a Basic Node License (NFT) through the BAAI FOCII Extension
   - Each license is uniquely identified and tracked on-chain

2. **Data Processing**
   - Edge AI extracts features in the browser
   - Data is encrypted and streamed to Lighthouse storage
   - Metadata is updated on the Node License NFT

3. **Verification & Rewards**
   - ZKML Prover generates proofs for processed data
   - Solana contract verifies proofs
   - Verified computations trigger BAAI token rewards

## Smart Contracts

### BAAI Token Contract
```rust
// Key functions:
initialize()           // Set up token and NFT mints
create_node_license() // Create new node license NFT
distribute_rewards()  // Distribute BAAI tokens to verified nodes
```

### State Management
- `ProgramState`: Tracks global contract state
- `Node`: Individual node metadata and statistics

## Setup & Development

### Prerequisites
- Rust 1.68.0 or later
- Solana Tool Suite 1.16.0 or later
- Anchor Framework 0.28.0
- Node.js and Yarn
- Python 3

### Installation
```bash
# Install Solana
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.28.0
avm use 0.28.0

# Install dependencies
yarn install
```

### Build
```bash
anchor build
```

### Deploy
```bash
anchor deploy
```

### Test
```bash
anchor test
```

## Integration Points

### 1. Browser Extension
- Initiates node license creation
- Handles Edge AI computation
- Manages data encryption and storage

### 2. Lighthouse Storage
- Stores encrypted feature data
- Maintains data availability for verification

### 3. ZKML Prover
- Generates zero-knowledge proofs
- Interfaces with Solana for verification

## Security Considerations

1. **Node License NFTs**
   - Non-transferable licenses
   - One license per node
   - Permanent record of computations

2. **Verification System**
   - Zero-knowledge proof validation
   - Multiple verification checks
   - Tamper-proof reward distribution

3. **Token Economics**
   - Controlled minting through verification
   - Reward distribution based on valid computations
   - Anti-spam mechanisms

## Testing

```bash
# Run all tests
anchor test

# Run specific test
anchor test test_name
```

## License
[MIT License](./LICENSE)