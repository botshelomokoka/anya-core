# Anya System Architecture Map

## Repository Structure
```
anya (anya-core)
├── anya-bitcoin/               # Bitcoin integration
├── anya-enterprise/           # Enterprise core features
├── anya-extensions/          # Extension system
├── dash33/                  # Web dashboard
├── dependencies/           # Shared dependencies
├── enterprise/           # Business implementation
└── mobile/             # Cross-platform mobile app
```

## Core Components

### 1. Bitcoin Integration ([anya-bitcoin](./anya-bitcoin/))
- **Purpose**: Core Bitcoin functionality
- **Documentation**: [Bitcoin Integration Guide](./anya-bitcoin/docs/README.md)
- **Key Files**:
  - [Cargo.toml](./anya-bitcoin/Cargo.toml)
  - [Source](./anya-bitcoin/src/)

### 2. Enterprise Core ([anya-enterprise](./anya-enterprise/))
- **Documentation**: [Enterprise README](./anya-enterprise/README.md)
- **Changelog**: [CHANGELOG.md](./anya-enterprise/CHANGELOG.md)
- **Architecture**: [Source Directory](./anya-enterprise/src/)

### 3. Dashboard (dash33)
- **Web Interface**: [main.py](./dash33/web/main.py)
- **Components**:
  - [Wallet Manager](./dash33/wallet/wallet_manager.py)
  - [Security](./dash33/core/security.py)
  - [AI Analysis](./dash33/ai/analyzer.py)

### 4. Mobile Application
- **Platforms**: Android, iOS, Desktop
- **Documentation**: 
  - [Development Guide](./mobile/DEVELOPMENT.md)
  - [Technical TODO](./mobile/TECHNICAL_TODO.md)
  - [Roadmap](./mobile/ROADMAP.md)
- **Source**: [lib directory](./mobile/lib/)

## Documentation Index

### Architecture Documents
- [Agent Architecture](./AGENT_ARCHITECTURE.md)
- [DAO Structure](./DAO.md)
- [Governance](./GOVERNANCE.md)

### Development Guides
- [Contributing Guide](./CONTRIBUTING.md)
- [Security Policy](./SECURITY.md)
- [Testing Guide](./TESTING.md)

### Planning & Roadmap
- [New Features](./NEW_FEATURES.md)
- [Roadmap](./ROADMAP.md)
- [Changelog](./CHANGELOG.md)

## Configuration Files

### Core Configuration
- [Cargo.toml](./Cargo.toml) - Rust dependencies
- [.env.template](./.env.template) - Environment template
- [docker-compose.yml](./docker-compose.yml) - Container orchestration

### Build & CI
- [build.rs](./build.rs) - Rust build script
- [rust_combined.yml](./rust_combined.yml) - CI pipeline

## Scripts
- [commit_push.ps1](./commit_push.ps1) - Git automation
- [install_dependencies.sh](./install_dependencies.sh) - Setup script
- [reorganize-code.ps1](./reorganize-code.ps1) - Code organization

## Symbolic Links
The following components are symlinked:
- `/anya/dash33` → `[Repository Root]/dash33`
- `/anya/enterprise` → `[Repository Root]/enterprise`
- `/anya/mobile` → `[Repository Root]/mobile`

## System Requirements
- Rust toolchain
- Python 3.8+
- Flutter SDK
- Docker & Docker Compose

## Quick Links
- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [License](./LICENSE.md)
- [Security Policy](./SECURITY.md)

---
*This map is automatically updated through CI/CD pipelines. Last updated: 2024-12-07*


