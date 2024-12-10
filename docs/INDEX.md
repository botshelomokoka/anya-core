---
layout: default
title: Anya Core
description: AI-Powered Bitcoin Protocol
show_support: true
---

# Welcome to Anya Core

Anya Core is an AI-powered Bitcoin protocol that enables advanced blockchain capabilities through machine learning and Web5 integration. This documentation will help you understand and implement Anya's powerful features.

## Quick Navigation

### Core Features
- [Getting Started](/anya-core/getting-started) - Quick setup guide
- [Architecture](/anya-core/architecture) - System design and components
- [Bitcoin Integration](/anya-core/bitcoin) - Bitcoin protocol features
- [Web5 Integration](/anya-core/web5) - Web5 implementation details

### Development
- [API Documentation](/anya-core/api) - Complete API reference
- [Security](/anya-core/security) - Security features and best practices
- [Contributing](/anya-core/contributing) - How to contribute
- [Testing](/anya-core/testing) - Testing procedures

## Latest Updates

### Version {{ site.version }} ({{ site.release_date }})
- AI-powered Bitcoin analytics
- Web5 protocol integration
- Enhanced security features
- Cross-platform support
- Community-driven development

[View Full Roadmap](/anya-core/roadmap)

## Support

### Community Support (anya-core)
The core protocol is community-supported through:
- [GitHub Issues]({{ site.github.repository_url }}/issues)
- [Discussions]({{ site.github.repository_url }}/discussions)
- [Contributing Guide]({{ site.github.repository_url }}/blob/main/CONTRIBUTING.md)

### Support Hours
Community support is available during Johannesburg business hours:
- Time Zone: {{ site.support.timezone }}
- Hours: {{ site.support.hours }}
- Location: {{ site.support.location }}

### Enterprise Support
For enterprise solutions and dedicated support:
- Email: {{ site.support.enterprise_email }}
- [Enterprise Features](/anya-core/enterprise)
- [Custom Solutions](/anya-core/enterprise/solutions)

## Security

For security-related matters:
- Email: {{ site.support.security_email }}
- [Security Policy]({{ site.github.repository_url }}/security/policy)
- [Responsible Disclosure]({{ site.github.repository_url }}/security/advisories)

## Quick Start

```rust
use anya_core::Anya;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize Anya Core
    let anya = Anya::new()
        .with_bitcoin()
        .with_web5()
        .build()
        .await?;

    // Start the service
    anya.start().await?;
    
    Ok(())
}
```

[Get Started →](/anya-core/getting-started)

*Last updated: 2024-12-07*
