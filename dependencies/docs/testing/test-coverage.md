# Test Coverage

This document details the test coverage practices in Anya.

## Coverage Types

### 1. Line Coverage
```rust
// Example of ensuring line coverage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_function_coverage() {
        let result = complex_function(true);
        assert!(result.is_ok());

        let result = complex_function(false);
        assert!(result.is_err());
    }
}

// Function to be tested
fn complex_function(flag: bool) -> Result<(), Error> {
    if flag {
        Ok(())    // This line must be covered
    } else {
        Err(Error::InvalidFlag)    // This line must be covered
    }
}
```

### 2. Branch Coverage
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_all_branches() {
        // Test positive numbers
        assert_eq!(classify_number(5), NumberType::Positive);
        
        // Test negative numbers
        assert_eq!(classify_number(-5), NumberType::Negative);
        
        // Test zero
        assert_eq!(classify_number(0), NumberType::Zero);
    }
}

// Function with multiple branches
fn classify_number(n: i32) -> NumberType {
    match n.cmp(&0) {
        Ordering::Greater => NumberType::Positive,
        Ordering::Less => NumberType::Negative,
        Ordering::Equal => NumberType::Zero,
    }
}
```

### 3. Function Coverage
```rust
#[cfg(test)]
mod tests {
    // Test all public functions
    #[test]
    fn test_public_api() {
        let calculator = Calculator::new();
        
        assert_eq!(calculator.add(2, 2), 4);
        assert_eq!(calculator.subtract(4, 2), 2);
        assert_eq!(calculator.multiply(3, 3), 9);
        assert_eq!(calculator.divide(6, 2), 3);
    }

    // Test private functions
    #[test]
    fn test_internal_functions() {
        assert!(Calculator::validate_input(5));
        assert!(!Calculator::validate_input(-1));
    }
}
```

## Coverage Tools

### 1. Tarpaulin Configuration
```toml
# .config/tarpaulin.toml
[coverage]
# Exclude test files
exclude-files = [
    "tests/*",
    "**/*_test.rs"
]

# Include only specific packages
packages = [
    "anya-core",
    "anya-wallet"
]

# Coverage threshold
minimum_coverage = 80
```

### 2. Coverage Reports
```yaml
# .github/workflows/coverage.yml
name: Coverage

on: [push, pull_request]

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/tarpaulin@v0.1
        with:
          version: '0.22.0'
          args: '--out Xml'
      - uses: codecov/codecov-action@v3
```

### 3. Coverage Monitoring
```rust
// Custom coverage tracker
pub struct CoverageTracker {
    covered_lines: HashSet<usize>,
    total_lines: usize,
}

impl CoverageTracker {
    pub fn record_line(&mut self, line: usize) {
        self.covered_lines.insert(line);
    }

    pub fn coverage_percentage(&self) -> f64 {
        (self.covered_lines.len() as f64 / self.total_lines as f64) * 100.0
    }
}
```

## Best Practices

### 1. Coverage Goals
- Set minimum coverage requirements
- Track coverage trends
- Identify coverage gaps
- Prioritize critical paths

### 2. Implementation
- Regular coverage runs
- Automated reporting
- CI/CD integration
- Documentation updates

### 3. Maintenance
- Coverage monitoring
- Gap analysis
- Improvement planning
- Regular updates

## Related Documentation
- [Unit Testing](unit-testing.md)
- [Integration Testing](integration-testing.md)
- [Performance Testing](performance-testing.md)
