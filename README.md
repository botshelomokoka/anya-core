# Anya Core

Anya Core is an open-source project that provides essential functionality for blockchain and AI integration.

## Features

- Modular, plugin-based architecture
- Federated learning with basic differential privacy
- Bitcoin, Lightning Network, and Stacks support
- Decentralized identity (DID) integration
- Smart contracts with Clarity and WebAssembly support
- Tiered usage system for free users (10-20% advanced feature access)
- Web5 principles adherence

## Tiered Usage System

Anya Core implements a tiered usage system that rewards positive user behavior with increased access to advanced features. This system adheres to web5 principles, ensuring user data ownership and privacy.

The tiered usage system in Anya Core is designed to incentivize user participation and reward positive behavior. Here's an overview of the system:

1. Free Tier: All users start at this level, with access to 10-20% of advanced features.
2. Bronze Tier: Users who contribute to the network (e.g., by running nodes or participating in federated learning) gain access to 30-40% of advanced features.
3. Silver Tier: Users who actively participate in the community (e.g., by contributing to documentation or helping other users) unlock 50-60% of advanced features.
4. Gold Tier: Users who make significant contributions to the project (e.g., code contributions or major bug reports) gain access to 70-80% of advanced features.
5. Platinum Tier: Reserved for top contributors and long-term active users, providing access to 90-100% of advanced features.

The system uses a points-based mechanism to track user contributions and automatically adjusts tier levels. All user data and tier information are stored locally, adhering to Web5 principles of data ownership and privacy.

For more details on implementation, refer to the `TieredUsage` module in the project's source code.

## Getting Started

To get started with Anya Core:

1. Clone the repository:
   ```
   git clone https://github.com/your-repo/anya-core.git
   cd anya-core
   ```

2. Install Rust and required dependencies:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the project:
   ```
   cargo run --release
   ```

For more detailed instructions, refer to the project documentation.

## Contributing

We welcome contributions to Anya Core! Here's how you can contribute:

1. Fork the repository and create your branch from `development`.
2. Make your changes, ensuring you follow our coding standards and guidelines.
3. Write or update tests as necessary.
4. Update the `CHANGELOG.md` file with details of your changes.
5. Submit a pull request to the `development` branch.

Please refer to the `CONTRIBUTING.md` file for more detailed information on our development process, coding standards, and release procedures.

Key points:
- Use `rustfmt` to format your code
- Run `clippy` and address any warnings before submitting
- Write unit tests for all new functionality
- Aim for at least 80% code coverage
- Follow the Rust style guide

For more information on our project structure and ongoing development, check the `ROADMAP.md` file.

## License

This project is open-source and licensed under the MIT License. Anya Core serves as the foundation for our enterprise offering, Anya Enterprise, which includes additional closed-source features. All improvements and bug fixes made to the core functionality in Anya Enterprise are continuously integrated back into Anya Core, ensuring that the open-source project benefits from ongoing development.
