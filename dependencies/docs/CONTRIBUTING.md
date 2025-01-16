# Contributing to Anya Core

We welcome contributions to the Anya Core project! This document provides guidelines for contributing to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Getting Started](#getting-started)
3. [Coding Standards](#coding-standards)
4. [Making Changes](#making-changes)
5. [Project Structure](#project-structure)
6. [Code Review](#code-review)
7. [Pull Requests](#pull-requests)
8. [Documentation](#documentation)
9. [Reporting Issues](#reporting-issues)
10. [Key Areas for Contribution](#key-areas-for-contribution)
11. [Community](#community)

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) to foster an open and welcoming environment.

## Getting Started

1. Fork the repository on GitHub: <https://github.com/botshelomokoka/anya-core>
2. Clone your fork locally: `git clone https://github.com/your-username/anya-core.git`
3. Create a new branch for your feature or bug fix: `git checkout -b your-branch-name`

## Making Changes

1. Choose an issue to work on or create a new one
2. Create a new branch for your feature or bug fix
3. Make your changes and commit them with a clear commit message
4. Push your changes to your fork on GitHub
5. Submit a pull request to the main repository

## Coding Standards

- Follow the Rust style guide
- Use meaningful variable and function names
- Write clear comments and documentation
- Keep functions small and focused on a single task
- Use error handling appropriately

## Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

## Pull Requests

- Provide a clear description of the changes in your pull request
- Include any relevant issue numbers
- Update documentation if necessary
- Ensure all tests pass before submitting

## Testing

- Write unit tests for new code
- Update existing tests if necessary
- Ensure all tests pass locally before submitting a pull request

## Documentation

- Update the README.md if necessary
- Document new features or changes in behavior
- Keep API documentation up-to-date

## Community

Join our community channels to discuss the project, ask questions, and get help:

- [Discord](https://discord.gg/anyacore)
- [Telegram](https://t.me/anyacore)
- [Forum](https://forum.anyacore.org)

Thank you for contributing to Anya Core!

## Project Structure

Familiarize yourself with the project structure:

anya-core/
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
├── src/
│   ├── main_system.rs
│   ├── network_discovery.rs
│   ├── user_management.rs
│   ├── stx_support.rs
│   ├── bitcoin_support.rs
│   ├── lightning_support.rs
│   ├── dlc_support.rs
│   ├── kademlia.rs
│   ├── setup_project.rs
│   ├── setup_check.rs
│   └── ml_logic/
│       ├── mod.rs
│       ├── federated_learning.rs
│       └── system_evaluation.rs
├── tests/
│   ├── integration_tests.rs
│   └── unit_tests/
│       ├── user_management_tests.rs
│       ├── blockchain_integration_tests.rs
│       └── ml_logic_tests.rs
├── docs/
│   ├── API.md
│   └── CONTRIBUTING.md
└── scripts/
    ├── setup.sh
    └── run_tests.sh

## Code Review

Your pull request will be reviewed by the maintainers. They may suggest changes or improvements. Please be patient and responsive during this process.

## Reporting Issues

If you find a bug or have a suggestion for improvement:

1. Check if the issue already exists in the GitHub issue tracker.
2. If not, create a new issue with a clear title and description.
3. Include steps to reproduce the issue if it's a bug.
4. If possible, provide a minimal code example that demonstrates the issue.

## Key Areas for Contribution

- Enhancing the ML-driven components in `src/ml_logic/`
- Improving Bitcoin, Stacks, and Lightning Network integrations
- Expanding the capabilities of the Discreet Log Contracts (DLCs) support
- Optimizing the Kademlia DHT implementation for better network discovery
- Enhancing privacy features and security measures
- Improving documentation and test coverage
