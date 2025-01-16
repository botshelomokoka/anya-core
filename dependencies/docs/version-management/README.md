# Version Management

This documentation provides information about the version management strategy used in the Anya project.

## Concepts

### Semantic Versioning

Semantic Versioning is a versioning scheme used to keep track of changes to the Anya project. It is based on the following rules:

- The version number is divided into three parts: `x.y.z`.
- The first part, `x`, is the major version number. This version number is incremented when there are breaking changes to the API.
- The second part, `y`, is the minor version number. This version number is incremented when there are new features or enhancements, but no breaking changes.
- The third part, `z`, is the patch version number. This version number is incremented when there are only bug fixes.

### Versioning Scheme

The versioning scheme used in the Anya project is based on the following formula: `x.y.z-rc.X` or `x.y.z-rc.X-dev.Y`.

- The first part, `x.y.z`, is the semantic version number.
- The second part, `-rc.X`, is the release candidate number.
- The third part, `-dev.Y`, is the development version number.

### Version Control

The Anya project uses a git repository for version control. The repository is hosted on GitHub.

### Release Process

The release process is as follows:

1. Branch from `main` to create a new branch for the release.
2. Update the version number in the `Cargo.toml` file.
3. Create a pull request to merge the branch into `main`.
4. Once the pull request has been approved, merge the branch into `main`.
5. Tag the release.
6. Push the tag to the remote repository.

## Tools

### cargo

The `cargo` command line tool is used to manage the version of the Anya project.

### cargo-audit

The `cargo-audit` command line tool is used to audit the dependencies used in the Anya project for security vulnerabilities.

### cargo-deny

The `cargo-deny` command line tool is used to deny certain dependencies from being used in the Anya project.

### cargo-watch

The `cargo-watch` command line tool is used to watch the project for changes and rebuild the project automatically.

### rustfmt

The `rustfmt` command line tool is used to format the code in the Anya project.
