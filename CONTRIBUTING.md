# Contributing to Anya Core Project

We welcome contributions to the Anya Core Project! This document outlines our development and release process.

## Development Process

1. Fork the repository and create your branch from `development`.
2. Make your changes, ensuring you follow our coding standards and guidelines.
3. Write or update tests as necessary.
4. Update the `CHANGELOG.md` file with details of your changes.
5. Submit a pull request to the `development` branch.

## Release Process

1. Development occurs in feature branches and is merged into the `development` branch.
2. Once a phase is complete and thoroughly tested, a release candidate branch is created (e.g., `release-1.0.0-rc`).
3. The release candidate undergoes extensive testing and any necessary bug fixes.
4. When deemed production-ready, the release candidate is merged into `main`.
5. A new tag is created for the release, following semantic versioning (e.g., v1.0.0).
6. The `VERSION` file is updated with the new version number.
7. The `CHANGELOG.md` file is updated to reflect the new release.

## Versioning

We use [Semantic Versioning](https://semver.org/). Version numbers are in the format MAJOR.MINOR.PATCH.

- MAJOR version for incompatible API changes
- MINOR version for backwards-compatible functionality additions
- PATCH version for backwards-compatible bug fixes

## Reporting Issues

If you find a bug or have a suggestion for improvement, please open an issue on our GitHub repository.

## Code Style

- Follow the Rust style guide
- Use `rustfmt` to format your code
- Run `clippy` and address any warnings before submitting

## Testing

- Write unit tests for all new functionality
- Use property-based testing for complex logic
- Aim for at least 80% code coverage

## Submitting Changes

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes, including tests and documentation
4. Run all tests and ensure they pass
5. Submit a pull request to the `development` branch

## Review Process

- All changes must be reviewed by at least one core contributor
- Changes to critical components require review by two core contributors
- All CI checks must pass before merging

## Documentation

- Update relevant documentation for any changes
- Provide clear, concise comments in your code
- For significant changes, update the CHANGELOG.md file

Thank you for contributing to the Anya Core Project!