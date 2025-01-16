# License Compliance

This section of the documentation covers how to manage license compliance for the dependencies of the Anya project.

## Overview

The Anya project uses a mix of open-source and proprietary dependencies. To ensure that we are compliant with the licensing terms of all of these dependencies, we have implemented a set of processes and tools.

## Tools

The following tools are used to manage license compliance for the Anya project:

- [cargo-deny](https://github.com/cargo-deny/cargo-deny): A cargo plugin that allows you to specify which licenses are allowed or disallowed in your project.
- [cargo-license](https://github.com/mthiruringa/cargo-license): A cargo plugin that allows you to generate a report of all of the licenses in your project.
- [license_finder](https://github.com/pivotal/LicenseFinder): A tool that allows you to generate a report of all of the licenses in your project.

## Process

The following process is used to manage license compliance for the Anya project:

1. When a new dependency is added to the project, the license for that dependency is checked against the list of allowed licenses specified in the `cargo-deny.toml` file.
2. If the license is not allowed, the dependency is not added to the project.
3. If the license is allowed, the dependency is added to the project and the license is tracked in the `license_report.txt` file.
4. On a regular basis (e.g. weekly), the `license_report.txt` file is reviewed to ensure that all dependencies are compliant with the licensing terms.
5. If a dependency is found to be non-compliant, the dependency is removed from the project.
