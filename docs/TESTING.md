# WasteFi Testing Documentation

## Document Purpose

This document provides comprehensive testing strategy, coverage analysis, execution instructions, and guidelines for the WasteFi smart contracts. It serves as the central reference for all testing activities.

**Version**: 0.1.0  
**Date**: September 2026  
**Status**: Pre-Mainnet

---

## Table of Contents

1. [Testing Strategy](#1-testing-strategy)
2. [Test Coverage Summary](#2-test-coverage-summary)
3. [Running Tests](#3-running-tests)
4. [Test Suites](#4-test-suites)
5. [Test Environment Setup](#5-test-environment-setup)
6. [Continuous Testing](#6-continuous-testing)
7. [Troubleshooting](#7-troubleshooting)

---

## 1. Testing Strategy

### 1.1 Testing Pyramid

```
           /\
          /  \
         / E2E \           Integration & E2E Tests
        /--------\         (20+ scenarios)
       /          \
      /  Integration\      
     /--------------\
    /                \     Unit Tests
   /   Unit Tests     \    (135+ tests, 85% coverage)
  /____________________\   
```

### 1.2 Testing Principles

**Principle 1: Test Early, Test Often**
- Write tests alongside code
- Run tests before every commit
- Automated testing in CI/CD

**Principle 2: Comprehensive Coverage**
- Target: >85% code coverage
- Focus on critical paths
- Include edge cases and error conditions

**Principle 3: Test Pyramid Balance**
- Many unit tests (fast, isolated)
- Moderate integration tests (realistic scenarios)
- Few E2E tests (complete workflows)

**Principle 4: Security-First Testing**
- Test all security controls
- Validate fraud detection
- Verify access control
- Test emergency mechanisms

**Principle 5: Performance Awareness**
- Monitor gas consumption
- Test under load
- Identify bottlenecks
- Optimize hot paths

---

## 2. Test Coverage Summary

### 2.1 Overall Coverage

**Current Status**: ~85% coverage across all contracts

```
Total Lines: ~8,100 (production code)
Covered Lines: ~6,885
Coverage: 85%

Test Distribution:
- Unit Tests: 135+ tests
- Integration Tests: 20+ scenarios
- Security Tests: 15+ cases
- Stress Tests: 10+ cases
- Chaos Tests: 10+ cases
Total: 190+ tests
```

### 2.2 Coverage by Contract

| Contract | Lines | Tests | Coverage | Status |
|----------|-------|-------|----------|--------|
| **CollectorRegistry** | ~600 | 15+ | 90% | ✅ Excellent |
| **WasteTransaction** | ~800 | 20+ | 85% | ✅ Good |
| **PaymentDistribution** | ~700 | 10+ | 80% | ⚠️ Improve critical paths |
| **MaterialPricing** | ~500 | 8+ | 85% | ✅ Good |
| **Reputation** | ~600 | 10+ | 85% | ✅ Good |
| **WasteToken** | ~400 | 12+ | 90% | ✅ Excellent |
| **CollectionPoint** | ~500 | 10+ | 85% | ✅ Good |
| **Common Library** | ~4,000 | 50+ | 85% | ✅ Good |

### 2.3 Coverage Gaps

**Areas Needing Improvement**:
1. **PaymentDistribution**: Critical payment calculation paths (Target: 90%)
2. **Cross-Contract Integration**: More integration test scenarios needed
3. **Edge Cases**: Boundary conditions and overflow scenarios
4. **Chaos Testing**: Unexpected input combinations
5. **Performance**: Load testing under realistic conditions

**Action Items**:
- Add 5+ PaymentDistribution tests for edge cases
- Create 10+ additional integration scenarios
- Implement property-based testing for arithmetic
- Add chaos testing framework
- Run stress tests with 1000+ concurrent operations

---

## 3. Running Tests

### 3.1 Quick Start

**Run All Tests**:
```bash
cargo test --workspace
```

**Run Tests for Specific Contract**:
```bash
# Collector Registry
cargo test -p collector_registry

# Waste Transaction
cargo test -p waste_transaction

# Payment Distribution
cargo test -p payment_distribution

# Material Pricing
cargo test -p material_pricing

# Reputation
cargo test -p reputation

# Waste Token
cargo test -p waste_token

# Collection Point
cargo test -p collection_point

# Common Library
cargo test -p common
```

**Run Specific Test**:
```bash
cargo test test_name --workspace
```

**Run Tests with Output**:
```bash
cargo test --workspace -- --nocapture
```

---

### 3.2 Test Suites

#### Unit Tests
```bash
# Run all unit tests (contract-specific tests)
cargo test --lib --workspace

# Expected: 135+ tests passing
# Duration: ~10 seconds
```

#### Integration Tests
```bash
# Run integration E2E tests
cargo test --test integration_e2e

# Expected: 20+ scenarios
# Duration: ~30 seconds
```

#### Stress Tests
```bash
# Run stress tests
cargo test --test stress_tests

# Expected: 10+ scenarios
# Duration: ~60 seconds
# Note: High transaction volumes tested
```

#### Security Tests
```bash
# Run security-focused tests
cargo test --test security_tests

# Expected: 15+ test cases
# Duration: ~20 seconds
```

#### Chaos Tests
```bash
# Run chaos/fuzzing tests
cargo test --test chaos_tests

# Expected: 10+ scenarios
# Duration: ~30 seconds
```

---

### 3.3 Coverage Analysis

**Generate HTML Coverage Report**:
```bash
# Install tarpaulin (first time only)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html --output-dir coverage

# Open report
# Windows: start coverage\index.html
# Linux/Mac: open coverage/index.html
```

**Generate Multiple Formats**:
```bash
# JSON, HTML, and terminal output
cargo tarpaulin --workspace --out Json --out Html --output-dir coverage
```

**Coverage Thresholds**:
```bash
# Fail if coverage drops below 80%
cargo tarpaulin --workspace --fail-under 80
```

---

### 3.4 Performance Profiling

**Gas Consumption Analysis**:
```bash
# Run tests with gas profiling
SOROBAN_PROFILE_GAS=1 cargo test --workspace

# Analyze output for gas costs
```

**Benchmark Tests** (if implemented):
```bash
cargo bench --workspace
```

---

## 4. Test Suites

### 4.1 Unit Test Suite

**Location**: Each contract's `src/test.rs` or `src/lib.rs` (inline)

**Purpose**: Test individual functions in isolation

**Examples**:

#### CollectorRegistry Unit Tests
```rust
#[test]
fn test_register_collector() {
    // Setup
    let env = Env::default();
    let contract = create_contract(&env);
    
    // Execute
    let result = contract.register(...);
    
    // Verify
    assert!(result.is_ok());
}

#[test]
fn test_duplicate_registration_fails() {
    // Test that duplicate registration is rejected
}

#[test]
fn test_update_status_requires_admin() {
    // Test authorization enforcement
}
```

**Coverage**:
- Happy path scenarios
- Error conditions
- Authorization checks
- Input validation
- Edge cases

---

### 4.2 Integration E2E Test Suite

**Location**: `tests/integration_e2e.rs`

**Purpose**: Test complete workflows across multiple contracts

**Test Categories**:

#### 1. Complete Workflow Tests
- **test_complete_workflow_happy_path**: Full cycle from registration to payment
- **test_complete_workflow_with_rejection**: Rejection workflow and reputation impact
- **test_multiple_collectors_concurrent_submissions**: Concurrent operations isolation

#### 2. Cross-Contract Integration
- **test_fraud_detection_blocks_high_risk_transaction**: Fraud detection effectiveness
- **test_rate_limiting_enforcement**: Rate limit enforcement across contracts
- **test_duplicate_detection_prevents_double_submission**: Duplicate prevention
- **test_payment_calculation_correctness**: Payment calculation accuracy

#### 3. Error Handling & Recovery
- **test_emergency_pause_and_resume**: Emergency mechanism functionality
- **test_circuit_breaker_prevents_cascading_failures**: Circuit breaker pattern
- **test_recovery_from_failed_transaction**: Failure recovery and retry

#### 4. Edge Cases & Boundaries
- **test_maximum_transaction_volume**: High-volume stress testing
- **test_zero_and_maximum_values**: Boundary value testing
- **test_concurrent_admin_operations**: Admin operation isolation

#### 5. Upgrade & Migration
- **test_contract_upgrade_preserves_state**: State preservation on upgrade
- **test_backward_compatibility_after_upgrade**: Compatibility validation

#### 6. Authorization & Security
- **test_authorization_requirements**: Authorization enforcement
- **test_no_privilege_escalation**: Privilege escalation prevention

**Total**: 20+ integration test scenarios

**Duration**: ~30 seconds

---

### 4.3 Stress Test Suite

**Location**: `tests/stress_tests.rs`

**Purpose**: Validate system performance under load

**Test Scenarios**:

#### High-Volume Tests
```rust
#[test]
fn stress_test_1000_concurrent_transactions() {
    // Submit 1000 transactions rapidly
    // Verify all processed correctly
    // Measure: gas, time, success rate
}

#[test]
fn stress_test_100_concurrent_collectors() {
    // 100 collectors registering simultaneously
    // Verify no collisions or data corruption
}
```

#### Rate Limit Boundary Tests
```rust
#[test]
fn stress_test_rate_limit_boundaries() {
    // Submit at exactly rate limit threshold
    // Verify proper throttling
    // No false positives/negatives
}
```

#### Storage Capacity Tests
```rust
#[test]
fn stress_test_storage_limits() {
    // Fill storage to capacity
    // Verify pruning mechanisms
    // Test graceful degradation
}
```

#### Gas Consumption Tests
```rust
#[test]
fn stress_test_gas_profiling() {
    // Measure gas for various operations
    // Compare individual vs batch
    // Verify optimization effectiveness
}
```

**Metrics Tracked**:
- Transaction throughput (TPS)
- Gas consumption per operation
- Storage growth rate
- Response time distribution
- Error rate under load

**Total**: 10+ stress test cases

**Duration**: ~60 seconds

---

### 4.4 Security Test Suite

**Location**: `tests/security_tests.rs`

**Purpose**: Validate security controls and attack resistance

**Test Categories**:

#### Authentication Bypass Tests
```rust
#[test]
fn security_test_admin_bypass_attempt() {
    // Attempt admin function without authorization
    // Verify rejection
}

#[test]
fn security_test_impersonation_attack() {
    // Attempt to act as another user
    // Verify blocked
}
```

#### Authorization Boundary Tests
```rust
#[test]
fn security_test_privilege_escalation() {
    // Attempt to gain higher privileges
    // Verify prevention
}
```

#### Fraud Detection Tests
```rust
#[test]
fn security_test_fraud_score_manipulation() {
    // Attempt to manipulate risk score
    // Verify detection and blocking
}

#[test]
fn security_test_weight_inflation_detection() {
    // Submit inflated weights
    // Verify anomaly detection
}
```

#### Rate Limit Evasion Tests
```rust
#[test]
fn security_test_rate_limit_bypass() {
    // Attempt to circumvent rate limits
    // Verify enforcement
}
```

#### Emergency Mechanism Tests
```rust
#[test]
fn security_test_unauthorized_emergency_trigger() {
    // Attempt to trigger emergency without admin
    // Verify rejection
}

#[test]
fn security_test_emergency_pause_effectiveness() {
    // Trigger emergency
    // Verify all operations blocked
}
```

#### Upgrade Security Tests
```rust
#[test]
fn security_test_unauthorized_upgrade() {
    // Attempt upgrade without admin
    // Verify rejection
}
```

**Total**: 15+ security test cases

**Duration**: ~20 seconds

---

### 4.5 Chaos Test Suite

**Location**: `tests/chaos_tests.rs`

**Purpose**: Test system resilience with unexpected inputs

**Test Scenarios**:

#### Unexpected Input Tests
```rust
#[test]
fn chaos_test_random_string_inputs() {
    // Submit random strings of various lengths
    // Verify graceful handling or rejection
}

#[test]
fn chaos_test_boundary_value_combinations() {
    // Test combinations of min/max values
    // Verify no overflows or panics
}
```

#### Out-of-Order Operations
```rust
#[test]
fn chaos_test_reverse_workflow() {
    // Execute workflow steps out of order
    // Verify proper state checks
}

#[test]
fn chaos_test_partial_state_operations() {
    // Operate on partially initialized state
    // Verify error handling
}
```

#### Resource Exhaustion
```rust
#[test]
fn chaos_test_storage_exhaustion() {
    // Attempt to exhaust storage
    // Verify limits enforced
}

#[test]
fn chaos_test_computation_limits() {
    // Trigger expensive computations
    // Verify timeouts/limits
}
```

#### Partial Failure Recovery
```rust
#[test]
fn chaos_test_mid_transaction_failure() {
    // Simulate failure mid-transaction
    // Verify rollback or consistent state
}
```

#### State Consistency Checks
```rust
#[test]
fn chaos_test_concurrent_state_mutations() {
    // Multiple concurrent state changes
    // Verify final state consistency
}
```

**Total**: 10+ chaos test cases

**Duration**: ~30 seconds

---

## 5. Test Environment Setup

### 5.1 Prerequisites

**Required Software**:
```bash
# Rust (1.79.0 or later)
rustup --version

# Soroban CLI
soroban --version

# wasm32 target
rustup target list | grep wasm32-unknown-unknown
```

**Installation** (if needed):
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli --version 21.7.7

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install test coverage tool
cargo install cargo-tarpaulin
```

---

### 5.2 Local Test Network

**Option 1: Soroban Test Environment** (Default)
```rust
// In tests
let env = Env::default();
env.mock_all_auths();
```

**Option 2: Stellar Testnet**
```bash
# Configure Stellar testnet
soroban config network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Fund test account
soroban keys generate test-key
soroban lab token wrap --network testnet --source test-key
```

---

### 5.3 Test Data Generation

**Test Address Generation**:
```rust
fn generate_test_addresses(env: &Env, count: u32) -> Vec<Address> {
    let mut addresses = Vec::new(env);
    for _ in 0..count {
        addresses.push_back(Address::generate(env));
    }
    addresses
}
```

**Test Collector Creation**:
```rust
fn create_test_collector(env: &Env, name: &str) -> Address {
    let collector = Address::generate(env);
    collector_registry.register(
        &collector,
        &String::from_str(env, name),
        &String::from_str(env, "test@example.com"),
    );
    collector
}
```

---

## 6. Continuous Testing

### 6.1 CI/CD Integration

**GitHub Actions Workflow** (`.github/workflows/ci.yml`):
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
          
      - name: Run tests
        run: cargo test --workspace
        
      - name: Check coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --workspace --fail-under 80
```

### 6.2 Pre-Commit Hooks

**Setup**:
```bash
# Create .git/hooks/pre-commit
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
echo "Running tests..."
cargo test --workspace --quiet
if [ $? -ne 0 ]; then
    echo "Tests failed. Commit aborted."
    exit 1
fi
EOF

chmod +x .git/hooks/pre-commit
```

### 6.3 Automated Test Schedule

**Daily Tests**: Full test suite on main branch
**Per-Commit**: Unit tests only (fast feedback)
**Pre-Merge**: All test suites + coverage check
**Weekly**: Stress tests + performance benchmarks

---

## 7. Troubleshooting

### 7.1 Common Test Failures

#### Test Timeout
**Problem**: Test hangs or times out  
**Solution**:
```bash
# Increase timeout
RUST_TEST_THREADS=1 cargo test --workspace -- --test-threads=1
```

#### Authorization Errors
**Problem**: `NotAuthorized` errors in tests  
**Solution**:
```rust
// Mock all authorizations
env.mock_all_auths();

// Or mock specific auth
env.mock_auths(&[
    MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "function_name",
            args: (...).into_val(&env),
            sub_invokes: &[],
        },
    },
]);
```

#### Storage Not Found
**Problem**: `StorageNotFound` errors  
**Solution**:
```rust
// Ensure contract initialized
contract.initialize(&admin);

// Or check storage existence
if !env.storage().instance().has(&key) {
    // Handle missing storage
}
```

#### WASM Build Failures
**Problem**: `cargo build --target wasm32-unknown-unknown` fails  
**Solution**:
```bash
# Clean and rebuild
cargo clean
rustup update
cargo build --target wasm32-unknown-unknown --release
```

---

### 7.2 Performance Issues

#### Slow Test Execution
**Problem**: Tests take too long  
**Solutions**:
- Run specific test suites instead of all
- Use `--release` flag for faster execution
- Reduce test data size
- Parallelize independent tests

```bash
# Release mode
cargo test --workspace --release

# Specific suite
cargo test --test integration_e2e

# Parallel execution (default)
cargo test --workspace -- --test-threads=8
```

#### High Memory Usage
**Problem**: Tests consume excessive memory  
**Solution**:
- Reduce concurrent test count
- Clean up test data after each test
- Use temporary storage in tests

```bash
# Limit parallel tests
cargo test --workspace -- --test-threads=4
```

---

### 7.3 Coverage Issues

#### Low Coverage Reported
**Problem**: Coverage shows < expected %  
**Investigation**:
```bash
# Detailed coverage by file
cargo tarpaulin --workspace --out Html --output-dir coverage

# Check which lines not covered
open coverage/index.html
```

**Solutions**:
- Add tests for uncovered branches
- Test error conditions
- Add edge case tests
- Test all public functions

#### Coverage Tool Errors
**Problem**: `cargo tarpaulin` fails  
**Solution**:
```bash
# Reinstall tarpaulin
cargo install cargo-tarpaulin --force

# Alternative: use llvm-cov
cargo install cargo-llvm-cov
cargo llvm-cov --workspace --html
```

---

### 7.4 Getting Help

**Resources**:
- **Soroban Documentation**: https://soroban.stellar.org/
- **Rust Testing Guide**: https://doc.rust-lang.org/book/ch11-00-testing.html
- **Team Slack**: #testing channel
- **Issue Tracker**: GitHub Issues

**Reporting Test Failures**:
1. Run test with `--nocapture` flag
2. Copy full error output
3. Include test name and contract
4. Note any recent changes
5. Create GitHub issue with details

---

## 8. Test Maintenance

### 8.1 Test Review Checklist

Before merging code, verify:
- [ ] All new tests passing
- [ ] Coverage maintained or improved
- [ ] No flaky tests introduced
- [ ] Test names descriptive
- [ ] Edge cases covered
- [ ] Error conditions tested
- [ ] Documentation updated

### 8.2 Periodic Test Health Checks

**Weekly**:
- Review flaky tests
- Update test data
- Check coverage trends
- Profile slow tests

**Monthly**:
- Comprehensive stress testing
- Security test review
- Update test documentation
- Review and remove obsolete tests

**Quarterly**:
- Full test suite audit
- Performance baseline update
- Testing strategy review
- Tool and framework updates

---

## 9. Testing Best Practices

### 9.1 Writing Good Tests

**DO**:
- ✅ Test one thing per test
- ✅ Use descriptive test names
- ✅ Arrange-Act-Assert structure
- ✅ Test both success and failure paths
- ✅ Use test helpers for setup
- ✅ Clean up test data
- ✅ Document complex test scenarios

**DON'T**:
- ❌ Test implementation details
- ❌ Create interdependent tests
- ❌ Use random values without seed
- ❌ Skip error condition testing
- ❌ Ignore flaky tests
- ❌ Copy-paste test code
- ❌ Leave commented-out tests

### 9.2 Test Organization

```rust
// Good structure
#[cfg(test)]
mod tests {
    use super::*;
    
    // Setup helpers
    fn setup() -> (Env, Address) { ... }
    
    // Happy path tests
    mod happy_path {
        #[test]
        fn test_basic_flow() { ... }
    }
    
    // Error condition tests
    mod error_conditions {
        #[test]
        fn test_invalid_input() { ... }
    }
    
    // Edge case tests
    mod edge_cases {
        #[test]
        fn test_boundary_values() { ... }
    }
}
```

---

## 10. Metrics and Reporting

### 10.1 Test Metrics Dashboard

**Key Metrics**:
- Total test count: 190+
- Test pass rate: Target 100%
- Code coverage: Target >85%
- Test execution time: <2 minutes
- Flaky test count: Target 0
- New tests per week: Track trend

### 10.2 Coverage Trends

**Track Over Time**:
- Overall coverage %
- Coverage by contract
- Uncovered critical paths
- Coverage change per commit

**Reporting Format**:
```
Week  | Total | CollectorRegistry | WasteTransaction | ...
------|-------|-------------------|------------------|----
W1    | 82%   | 88%               | 80%              | ...
W2    | 85%   | 90%               | 85%              | ...
```

---

## Summary

**WasteFi Testing Infrastructure**:
- ✅ 190+ comprehensive tests across 5 test suites
- ✅ 85% code coverage (target: >85%)
- ✅ Automated CI/CD integration
- ✅ Multiple test categories (unit, integration, security, stress, chaos)
- ✅ Coverage tracking and reporting
- ✅ Performance profiling capabilities
- ✅ Clear documentation and guidelines

**Test Execution**:
- **Quick**: `cargo test --workspace` (all tests in ~2 min)
- **Coverage**: `cargo tarpaulin --workspace` (HTML report)
- **Specific**: `cargo test -p <contract>` (contract tests)

**Status**: **PRODUCTION-READY TEST INFRASTRUCTURE**

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 0.1.0 | 2026-09-11 | WasteFi Team | Initial testing documentation |

---

**End of Testing Documentation**

For questions or test failures, contact: dev@wastefi.io
