// docs/INDEX.md

# Anya Core Documentation Index

## Core Systems

### ML System
- [ML System Architecture](ML_SYSTEM_ARCHITECTURE.md)
- [ML Metrics](ML_METRICS.md)
- [Agent Architecture](AGENT_ARCHITECTURE.md)
- Code:
  - [MLCore](../src/ml_core/mod.rs)
  - [ML Agents](../src/ml/agents/mod.rs)
  - [ML Pipeline](../src/ml_core/pipeline.rs)

### Blockchain Integration
- Code:
  - [Bitcoin Core](../src/bitcoin/mod.rs)
  - [Lightning](../src/lightning.rs)
  - [RGB](../src/rgb/mod.rs)
  - [DLC](../src/dlc.rs)
  - [Stacks](../src/stacks/mod.rs)

### Network Layer
- Code:
  - [Network Discovery](../src/network/discovery.rs)
  - [Kademlia](../src/kademlia.rs)
  - [Unified Network](../src/unified_network/mod.rs)

### Security & Privacy
- Code:
  - [Privacy Module](../src/privacy/mod.rs)
  - [Secure Storage](../src/secure_storage/mod.rs)
  - [Identity](../src/identity/mod.rs)

## Enterprise Features

### Analytics
- [Advanced Analytics](../anya-enterprise/src/advanced_analytics/mod.rs)
- [High Volume Trading](../anya-enterprise/src/high_volume_trading/mod.rs)
- [Research](../anya-enterprise/src/ml/research.rs)

### Integration
- [API](../anya-enterprise/src/api.rs)
- [Enterprise Core](../anya-enterprise/src/lib.rs)

## Development

### Project Management
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [New Features](NEW_FEATURES.md)

### CI/CD
- [Workflow](.github/workflows/ci.yml)
- [Build Script](../build.rs)
