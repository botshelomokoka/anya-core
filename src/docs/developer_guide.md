# Anya Core Developer Guide

Welcome to the Anya Core developer guide! This document will help you get started with contributing to the development of Anya Wallet and its core components.

## Project Structure

The Anya Core project is organized into several modules:

* **`src/`:** Contains the main source code for the project, including:
  * **`wallet/`:**    Core wallet functionality (key management, transactions, balances, addresses)
  * **`lightning/`:** Lightning Network integration
  * **`taproot/`:**   Taproot assets implementation
  * **`coinjoin/`:**  CoinJoin privacy features
  * **`dao/`:**       DAO functionality (governance, proposals, voting, treasury)
  * **`network/`:**   Bitcoin and RSK network communication
  * **`security/`:**  Encryption and authentication features
* **`tests/`:**       Unit and integration tests
* **`docs/`:**        Project documentation
* **`scripts/`:**     Utility scripts for development and deployment

## Getting Started

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/botshelomokoka/anya-core-main.git
   cd anya-core-main
   ```

2. **Set Up the Development Environment:**
   * Install Rust: https://www.rust-lang.org/tools/install
   * Install required dependencies (see `README.md` for specifics)

3. **Build the Project:**
   ```bash
   cargo build
   ```

4. **Run Tests:**
   ```bash
   cargo test
   ```

## Contributing

We welcome contributions! Please follow these guidelines:

1. Fork the repository and create a new branch for your changes.
2. Write clean, well-documented code following our coding standards.
3. Add tests for new functionality or bug fixes.
4. Submit a pull request with a clear description of your changes.

## Coding Standards

* Follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/)
* Use meaningful variable and function names
* Write clear comments and documentation
* Keep functions small and focused
* Use proper error handling

## Additional Resources

* [Rust Documentation](https://doc.rust-lang.org/book/)
* [Bitcoin Developer Documentation](https://developer.bitcoin.org/)
* [RSK Developer Portal](https://developers.rsk.co/)
* [Lightning Network Resources](https://lightning.network/resources/)

## Getting Help

If you have questions or need assistance:

* Open an issue on our [GitHub repository](https://github.com/botshelomokoka/anya-core-main/issues)
* Join our developer chat on [Discord/Matrix] (link to be added)

Thank you for contributing to Anya Core! Together, we're building a more accessible and private Bitcoin ecosystem.
