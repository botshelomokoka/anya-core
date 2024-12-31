# CI/CD Workflows

This directory contains the GitHub Actions workflows for automated testing, benchmarking, and deployment.

## Workflows

### 1. Performance Benchmarks (`benchmark.yml`)
Runs performance and memory usage tests to ensure code changes don't introduce regressions.

#### Key Features:
- AI model performance testing
- Memory usage profiling
- Load testing
- Resource limit enforcement
- Benchmark result tracking

#### Configuration:
- Memory limit: 4GB virtual memory
- Batch size: 1000 items
- Profiling interval: 0.1s
- Test duration: 300s
- Concurrent users: 100

### 2. Commit Cycle (`commit-cycle.yml`)
Manages the automated code review and deployment process.

#### Key Features:
- Submodule synchronization
- Code quality checks
- Automated testing
- Release tagging
- Status notifications

#### Required Secrets:
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions
- Repository permissions: `contents: write`, `pull-requests: write`

### 3. CodeQL Analysis
Security and code quality analysis using GitHub's CodeQL engine.

#### Scans For:
- Security vulnerabilities
- Code quality issues
- Best practice violations
- Potential bugs

## Usage

### Running Locally
To test workflows locally before pushing:

```bash
# Install act for local GitHub Actions testing
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Run specific workflow
act -W .github/workflows/benchmark.yml
act -W .github/workflows/commit-cycle.yml
```

### Monitoring
Monitor workflow status:
```bash
gh run list
gh run watch
gh pr checks
```

## Troubleshooting

### Common Issues:
1. Memory Usage Tests
   - Error: "Memory limit exceeded"
   - Solution: Adjust `MEMORY_PROFILER_BATCH_SIZE` in benchmark.yml

2. Commit Cycle
   - Error: "No url found for submodule"
   - Solution: Verify .gitmodules configuration

3. CodeQL
   - Error: "Analysis failed"
   - Solution: Check language-specific requirements

## Contributing

When modifying workflows:
1. Test locally using `act`
2. Update documentation
3. Verify all checks pass
4. Request review from DevOps team
