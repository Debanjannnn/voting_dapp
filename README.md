# 🗳️ Soroban Voting dApp

## 📌 Project Description
<img width="1920" height="963" alt="image" src="https://github.com/user-attachments/assets/b62f9cac-5f08-4a20-b544-f85b4c07a017" />

A decentralized voting application built on Stellar Soroban that allows users to create proposals and vote securely on-chain. The contract ensures transparency, immutability, and prevents double voting.

## ⚙️ What it does
This smart contract enables:
- Creation of proposals
- Voting on proposals
- Preventing duplicate votes per user
- Fetching proposal details and vote counts

All logic is handled fully on-chain using Soroban smart contracts.

## 🚀 Features
- ✅ Decentralized governance model
- 🔐 One vote per user per proposal (anti double voting)
- 📊 Transparent vote counting
- ⚡ Lightweight and efficient storage using Soroban SDK
- 🧱 Easily extendable for DAO or governance systems

## 📦 Contract Functions

### `init()`
Initializes the contract state.

### `create_proposal(name: Symbol) -> u32`
Creates a new proposal and returns its ID.

### `vote(voter: Address, proposal_id: u32)`
Allows a user to vote for a proposal.

### `get_proposal(proposal_id: u32) -> Proposal`
Fetch proposal details including vote count.

### `get_proposal_count() -> u32`
Returns total number of proposals.

## 🔗 Deployed Smart Contract Link
(https://stellar.expert/explorer/testnet/contract/CBF3AVXHVHD2QA6IJX6X46PJOMTPUDNCVJESWV3BNH6HM53HBODWRHU7)

## 🛠️ Tech Stack
- Rust
- Soroban SDK
- Stellar Blockchain

## 📈 Future Improvements
- Token-weighted voting
- Proposal deadlines
- DAO treasury integration
- Frontend UI (React + Stellar Wallet Kit)

## 🤝 Contributing
Feel free to fork the repo and submit PRs to improve the contract.

## 📜 License
MIT
