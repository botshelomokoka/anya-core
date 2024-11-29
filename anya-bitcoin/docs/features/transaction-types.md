# Transaction Types

This document describes the different types of transactions supported by the Anya Bitcoin integration.

## Supported Transaction Types

### 1. Standard Transactions
- P2PKH (Pay to Public Key Hash)
- P2SH (Pay to Script Hash)
- P2WPKH (Pay to Witness Public Key Hash)
- P2WSH (Pay to Witness Script Hash)

### 2. Advanced Transactions
- Multi-signature transactions
- Time-locked transactions
- Replace-by-fee transactions
- Child-pays-for-parent transactions

### 3. Smart Contract Transactions
- DLC (Discreet Log Contracts)
- Hash Time Locked Contracts (HTLC)
- RGB Protocol transactions
- Lightning Network transactions

### 4. Special Transactions
- Coinbase transactions
- OP_RETURN data transactions
- Batch transactions
- Fee bump transactions

## Implementation Details
- [Transaction Operations](transaction-operations.md)
- [Transaction Security](../security/transaction-security.md)
- [Script Types](../smart-contracts/script-types.md)
