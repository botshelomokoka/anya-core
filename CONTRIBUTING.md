# Contributing to Anya Core

We love your input! We want to make contributing to this project as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## We Develop with Github

We use github to host code, to track issues and feature requests, as well as accept pull requests.

## We Use [Github Flow](https://guides.github.com/introduction/flow/index.html), So All Code Changes Happen Through Pull Requests

Pull requests are the best way to propose changes to the codebase. We actively welcome your pull requests:

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using Github's [issues](https://github.com/botshelomokoka/anya/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/botshelomokoka/anya/issues/new); it's that easy!

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Use a Consistent Coding Style

- 4 spaces for indentation rather than tabs
- You can try running `cargo fmt` for style unification

## Bitcoin Ethical Principles and Development Standards

### Core Ethical Commitments

1. **Financial Sovereignty**
   - Respect individual economic freedom
   - Prioritize user privacy and financial autonomy
   - Reject censorship and centralized control

2. **Technical Integrity**
   - Maintain the highest standards of cryptographic security
   - Prioritize open-source transparency
   - Ensure robust, auditable code

3. **Decentralization Principles**
   - Design systems that minimize single points of failure
   - Promote network resilience and distributed trust
   - Resist rent-seeking and extractive economic models

### Development Standards

#### Code of Conduct

- **Transparency**: All code must be open, reviewable, and auditable
- **Security First**: Prioritize security over convenience
- **Privacy Protection**: Implement zero-knowledge and minimal data exposure techniques
- **Consent and Opt-in**: Never implement invasive tracking or monitoring

#### Technical Guidelines

1. **Cryptographic Practices**
   - Use latest cryptographic standards
   - Implement constant-time algorithms
   - Avoid proprietary or closed-source cryptographic methods
   - Regular security audits and vulnerability assessments

2. **Performance and Efficiency**
   - Optimize for low resource consumption
   - Minimize blockchain and network overhead
   - Support low-bandwidth and resource-constrained environments

3. **Compatibility and Interoperability**
   - Adhere to Bitcoin Improvement Proposals (BIPs)
   - Ensure cross-platform and cross-implementation compatibility
   - Support emerging standards like Lightning Network, RGB, and Taproot

#### Contribution Process

1. **Proposal Submission**
   - Detailed RFC (Request for Comments) for significant changes
   - Clear problem statement and proposed solution
   - Potential economic and technical impact analysis

2. **Code Review Standards**
   - Minimum two independent code reviews
   - Comprehensive test coverage (>90%)
   - Static and dynamic security analysis
   - Performance benchmarking

3. **Security Vulnerability Handling**
   - Responsible disclosure process
   - Bug bounty program
   - Immediate mitigation and transparent reporting

### Compliance Checklist

- [ ] Adheres to Bitcoin Core coding standards
- [ ] Passes comprehensive test suite
- [ ] Security audit completed
- [ ] Performance benchmarks documented
- [ ] Ethical impact assessment

### Recommended Tools

- Rust Analyzer
- Clippy for linting
- Cargo Audit
- Valgrind
- Coverity Scan
- Formal verification tools

### Recommended Reading

- [Bitcoin Developer Guide](https://bitcoin.org/en/developer-guide)
- [Mastering Bitcoin by Andreas Antonopoulos](https://github.com/bitcoinbook/bitcoinbook)
- [Cryptography Papers by Satoshi Nakamoto](https://bitcoin.org/bitcoin.pdf)

## License

By contributing, you agree that your contributions will be licensed under its MIT License.

## References

This document was adapted from the open-source contribution guidelines for [Facebook's Draft](https://github.com/facebook/draft-js/blob/a9316a723f9e918afde44dea68b5f9f39b7d9b00/CONTRIBUTING.md)

## Git Worktree Workflow

We use Git worktrees to manage different features and versions of the project. Here's how to use them:

1. Create a new worktree for a feature:

   ```bash
   git worktree add -b feature-branch ../anya-core-feature-branch main
   ```

2. Navigate to the new worktree:

   ```bash
   cd ../anya-core-feature-branch
   ```

3. Make your changes, commit them, and push to the remote branch:

   ```bash
   git add .
   git commit -m "Implement new feature"
   git push -u origin feature-branch
   ```

4. When you're done with the feature, you can remove the worktree:

   ```bash
   cd ..
   git worktree remove anya-core-feature-branch
   ```

Remember to keep your worktrees in sync with the main repository by regularly pulling changes from the main branch.

*Last updated: 2024-12-07*
