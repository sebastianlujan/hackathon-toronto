# Dutch Auction on Stellar Soroban with Privacy

## 🔒 Privacy-Preserving Dutch Auction with Homomorphic Encryption

This project implements a fully decentralized and privacy-preserving Dutch auction system on Stellar's Soroban smart contract platform, featuring fully homomorphic encryption for confidential bidding.

## 🚀 Technologies Used

### Stellar Soroban Platform
- **Smart Contract Framework**: Built using Soroban's `#[contractimpl]` attribute for robust auction logic.
- **On-chain Storage**: Leverages `env.storage().instance().set/get` for persistent encrypted state management.
- **Blockchain Time**: Uses Soroban's environment for precise auction timing and state transitions.

### Privacy Layer: Zama's tFHE
- **Fully Homomorphic Encryption**: Enables computations on encrypted data without decryption.
- **Private Auction Parameters**:
  - Encrypted starting price.
  - Encrypted minimum price.
  - Encrypted price decrement.

## ✨ Key Features

- **Privacy-Preserving Bidding**: All price calculations happen on encrypted values.
- **Automatic Price Degradation**: Time-based homomorphic calculations adjust price over time.
- **Secure Settlement**: Winner determination without revealing other bids.
- **Transparent Execution**: Leverages Stellar's reliable consensus while maintaining data privacy.

## 📋 Installation & Setup

Follow these steps to set up the project: