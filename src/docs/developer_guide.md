# Anya Core Developer Guide

Welcome to the Anya Core developer guide! This document will help you get started with contributing to the development of Anya Wallet and its core components.

## Project Structure

The Anya Core project is organized into several modules:

*   **`wallet/`:** Contains the core wallet functionality, including key management, transaction handling, balance tracking, address management, and integrations with Lightning Network, Taproot assets, and CoinJoin.
*   **`dao/`:** Implements the decentralized autonomous organization (DAO) features, including governance, proposals, voting, treasury management, and membership.
*   **`network/`:** Handles communication with the Bitcoin and RSK networks, including transaction broadcasting, UTXO fetching, and Lightning Network interactions.
*   **`security/`:** Implements security features like encryption and authentication.
*   **`utils/`:** Contains helper functions used across multiple modules.
*   **`tests/`:** Houses unit and integration tests for each module.
*   **`docs/`:** Contains documentation files, including this developer guide, the user guide, and the API reference.

## Getting Started

1.  **Clone the Repository:**
    ```bash
    git clone (https://github.com/botshelomokoka/anya-core.git)
    ```

2.  **Install Dependencies:**
    *   Navigate to the project directory: `cd anya-core`
    *   Create a virtual environment (recommended): `python3 -m venv venv`
    *   Activate the virtual environment:
        *   On macOS/Linux: `source venv/bin/activate`
        *   On Windows: `venv\Scripts\activate`
    *   Install project dependencies: `pip install -r requirements.txt`

3.  **Run Tests:**
    *   Ensure all tests are passing: `pytest` (or equivalent for your chosen testing framework)

## Contributing

We welcome contributions from the community! Please follow these guidelines:

*   **Fork the repository** and create a new branch for your changes.
*   **Write clean and well-documented code.** Follow the project's coding style and conventions.
*   **Add tests** for any new functionality or bug fixes.
*   **Submit a pull request** with a clear description of your changes.

## Coding Standards

*   **Python:** Follow the [PEP 8 style guide](https://www.python.org/dev/peps/pep-0008/).
*   **Solidity:** Adhere to the [Solidity Style Guide](https://docs.soliditylang.org/en/v0.8.13/style-guide.html).
*   **Other Languages:** Use appropriate style guides and conventions for any other languages used in the project.

## Additional Resources

*   **Bitcoin Developer Documentation:** [https://developer.bitcoin.org/](https://developer.bitcoin.org/)
*   **RSK Developer Portal:** [https://developers.rsk.co/](https://developers.rsk.co/)
*   **Lightning Network Documentation:** [https://lightning.network/](https://lightning.network/)

## Contact

If you have any questions or need assistance, feel free to reach out to us on our [forum/chat channel].

We appreciate your interest in contributing to Anya Core! Let's build a better Bitcoin future together.
