# Contributing Guide

Welcome to the CodeCrypt project! We appreciate your interest in contributing to this educational cryptography and error correction coding library. This guide will help you get started with contributing effectively.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Code Style Guidelines](#code-style-guidelines)
- [Contributing Workflow](#contributing-workflow)
- [Types of Contributions](#types-of-contributions)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Standards](#documentation-standards)
- [Pull Request Process](#pull-request-process)
- [Community Guidelines](#community-guidelines)

## Getting Started

### Before You Begin

1. **Read the Documentation**: Familiarize yourself with the project by reading:
   - [README](../../README.md)
   - [Getting Started Tutorial](../tutorials/getting-started.md)
   - [API Documentation](../api/)
   - [Theory Documentation](../theory/)

2. **Understand the Project Goals**:
   - Educational focus on cryptography and error correction
   - Clear, readable implementations over extreme optimization
   - Comprehensive documentation and examples
   - Safe, idiomatic Rust code

3. **Check Existing Issues**: Look at the [issue tracker](https://github.com/yourusername/codecrypt/issues) for:
   - Bug reports
   - Feature requests
   - Good first issues (labeled `good-first-issue`)
   - Help wanted items (labeled `help-wanted`)

### Setting Up Your Environment

#### Prerequisites
- Rust 1.70.0 or later
- Git
- A GitHub account
- Your favorite code editor/IDE

#### Fork and Clone
```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/codecrypt.git
cd codecrypt

# Add the original repository as upstream
git remote add upstream https://github.com/yourusername/codecrypt.git
```

#### Build and Test
```bash
# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linting
cargo clippy -- -D warnings
```

## Development Environment

### Recommended Tools

**Essential:**
- **rustup**: Rust toolchain manager
- **cargo**: Build tool and package manager
- **rustfmt**: Code formatter
- **clippy**: Linter for catching common mistakes

**Helpful:**
- **rust-analyzer**: Language server for IDEs
- **cargo-edit**: Add/remove dependencies easily
- **cargo-watch**: Automatic rebuild on file changes

### Editor Configuration

#### VS Code
Recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML

#### Vim/Neovim
- Install rust.vim or use LSP with rust-analyzer

#### IntelliJ/CLion
- Rust plugin with cargo support

### Environment Setup
```bash
# Install recommended cargo extensions
cargo install cargo-edit cargo-watch

# Set up git hooks (optional)
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
EOF
chmod +x .git/hooks/pre-commit
```

## Code Style Guidelines

### General Principles

1. **Clarity over Cleverness**: Code should be easy to understand
2. **Educational Focus**: Add comments explaining cryptographic concepts
3. **Safety First**: Use Rust's safety features, avoid `unsafe` unless absolutely necessary
4. **Consistency**: Follow established patterns in the codebase

### Rust Style Guidelines

#### Formatting
- Use `cargo fmt` for consistent formatting
- Line length: 100 characters maximum
- Use 4 spaces for indentation (enforced by rustfmt)

#### Naming Conventions
```rust
// Modules: snake_case
mod convolutional_codes;

// Types: PascalCase
struct ConvolutionalEncoder;
enum CipherAlgorithm;

// Functions and variables: snake_case
fn encode_message();
let constraint_length = 3;

// Constants: SCREAMING_SNAKE_CASE
const DEFAULT_CONSTRAINT_LENGTH: u32 = 3;
```

#### Documentation
```rust
/// Brief description of the function.
///
/// More detailed explanation if needed. Explain the algorithm,
/// parameters, and any important implementation details.
///
/// # Arguments
///
/// * `input` - Description of the input parameter
/// * `key` - Description of the key parameter
///
/// # Returns
///
/// Description of what the function returns
///
/// # Examples
///
/// ```
/// let encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
/// let encoded = encoder.encode(&[true, false, true]);
/// ```
///
/// # Panics
///
/// Describe when this function might panic
///
/// # Safety
///
/// If using unsafe code, explain the safety requirements
pub fn encode(&mut self, input: &[bool]) -> Vec<bool> {
    // Implementation
}
```

#### Error Handling
```rust
// Use Result for operations that can fail
pub fn parse_polynomial(input: &str) -> Result<u32, ParseError> {
    // Implementation
}

// Use Option for values that may not exist
pub fn find_best_path(&self, state: usize) -> Option<&Path> {
    // Implementation
}

// Document panic conditions
/// # Panics
/// Panics if the input length is not a multiple of the rate denominator
pub fn decode(&self, input: &[bool]) -> Vec<bool> {
    assert_eq!(input.len() % self.rate_denominator, 0);
    // Implementation
}
```

### Algorithm Implementation Guidelines

#### Mathematical Operations
```rust
// Use clear variable names that match mathematical notation
let constraint_length = 3;  // K in literature
let generator_polynomials = vec![7, 5];  // G1, G2 in octal

// Add comments explaining the mathematical operations
// Calculate parity using XOR of polynomial taps
let mut parity = 0;
let mut temp = state & polynomial;
while temp != 0 {
    parity ^= temp & 1;  // XOR each bit
    temp >>= 1;
}
```

#### State Management
```rust
// Make state transitions explicit
impl ConvolutionalEncoder {
    pub fn step(&mut self, input_bit: bool) -> Vec<bool> {
        // Shift register: move old bits left, add new bit at LSB
        self.state = ((self.state << 1) | (input_bit as u32)) & self.register_mask;

        // Generate output for each polynomial
        self.polynomials.iter()
            .map(|&poly| self.compute_parity(poly))
            .collect()
    }
}
```

### Testing Style

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let input = vec![true, false, true];

        // Act
        let output = encoder.encode(&input);

        // Assert
        assert_eq!(output.len(), input.len() * 2);  // Rate 1/2
        // Add more specific assertions
    }

    #[test]
    fn test_known_vectors() {
        // Test against known good values from literature
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let input = vec![true, false, true, true, false];
        let expected = vec![
            true, true,    // First input bit
            true, false,   // Second input bit
            false, true,   // Third input bit
            true, true,    // Fourth input bit
            true, false,   // Fifth input bit
        ];

        let actual = encoder.encode(&input);
        assert_eq!(actual, expected);
    }
}
```

## Contributing Workflow

### Branching Strategy

1. **main**: Stable release branch
2. **develop**: Integration branch for features
3. **feature/**: Feature development branches
4. **bugfix/**: Bug fix branches
5. **docs/**: Documentation improvements

### Development Workflow

```bash
# 1. Start from main and pull latest changes
git checkout main
git pull upstream main

# 2. Create a feature branch
git checkout -b feature/new-cipher-algorithm

# 3. Make your changes
# ... edit files ...

# 4. Test your changes
cargo test
cargo fmt
cargo clippy

# 5. Commit your changes
git add .
git commit -m "Add XYZ cipher algorithm

- Implement basic XYZ cipher encryption/decryption
- Add comprehensive unit tests
- Update documentation and examples"

# 6. Push to your fork
git push origin feature/new-cipher-algorithm

# 7. Create a pull request on GitHub
```

### Commit Message Format

```
<type>: <short description>

<longer description if needed>

<footer with issue references>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Build system, dependencies, etc.

**Examples:**
```
feat: add Blowfish cipher implementation

- Implement Blowfish encryption and decryption
- Add key schedule generation
- Include comprehensive test vectors
- Update CLI to support new cipher

Closes #123

fix: correct Viterbi decoder path metric calculation

The branch metric was incorrectly calculated for soft-decision
decoding. This fix ensures proper likelihood ratio computation.

Fixes #456

docs: improve error correction theory documentation

- Add mathematical foundations section
- Include trellis diagram examples
- Fix typos in algorithm descriptions
```

## Types of Contributions

### 1. Bug Fixes

**What We Need:**
- Clear bug reports with reproduction steps
- Fixes with test cases demonstrating the issue
- Explanation of the root cause

**Process:**
1. Create an issue describing the bug
2. Create a `bugfix/` branch
3. Fix the issue with appropriate tests
4. Submit a pull request

### 2. New Features

**Cryptographic Algorithms:**
- Additional classical ciphers (Hill, Playfair, etc.)
- Modern symmetric ciphers (for educational comparison)
- Hash functions and MACs
- Key derivation functions

**Error Correction:**
- LDPC codes
- Reed-Solomon codes
- BCH codes
- Polar codes

**Infrastructure:**
- Performance improvements
- Better error handling
- CLI enhancements
- Configuration system

**Process:**
1. Discuss the feature in an issue first
2. Get consensus on the approach
3. Implement with comprehensive tests
4. Update documentation

### 3. Documentation

**Types Needed:**
- API documentation improvements
- Tutorial enhancements
- Theory explanations
- Example applications
- Performance analysis

**Standards:**
- Clear, accessible language
- Working code examples
- Mathematical accuracy
- Cross-references between sections

### 4. Testing

**Areas for Improvement:**
- Test coverage expansion
- Integration tests
- Performance benchmarks
- Property-based testing
- Fuzzing tests

### 5. Infrastructure

**Welcome Contributions:**
- CI/CD improvements
- Build system enhancements
- Development tooling
- Release automation

## Testing Guidelines

### Test Organization

```
tests/
├── unit/           # Unit tests (also in src/ files)
├── integration/    # Integration tests
├── benchmarks/     # Performance tests
└── vectors/        # Test vectors and known answers
```

### Testing Requirements

#### New Features Must Include:
1. **Unit tests** for all public functions
2. **Integration tests** for CLI commands
3. **Known answer tests** using standard test vectors
4. **Error case tests** for invalid inputs

#### Test Categories:

**Correctness Tests:**
```rust
#[test]
fn test_round_trip_encryption() {
    let cipher = Cipher::new("vigenere".to_string(), "key".to_string());
    let message = "Test message";

    let encrypted = cipher.encrypt(message);
    let decrypted = cipher.decrypt(&encrypted);

    assert_eq!(message, decrypted);
}
```

**Edge Case Tests:**
```rust
#[test]
fn test_empty_input() {
    let cipher = Cipher::new("caesar".to_string(), "5".to_string());
    assert_eq!(cipher.encrypt(""), "");
}

#[test]
fn test_non_alphabetic_characters() {
    let cipher = Cipher::new("caesar".to_string(), "3".to_string());
    assert_eq!(cipher.encrypt("123!@#"), "123!@#");
}
```

**Property Tests:**
```rust
#[test]
fn test_vigenere_key_independence() {
    let message = "HELLO WORLD";
    let key1 = "KEY";
    let key2 = "DIFFERENT";

    let cipher1 = Cipher::new("vigenere".to_string(), key1.to_string());
    let cipher2 = Cipher::new("vigenere".to_string(), key2.to_string());

    assert_ne!(cipher1.encrypt(message), cipher2.encrypt(message));
}
```

### Performance Tests

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_convolutional_encoding() {
        let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
        let input = vec![true; 10000];  // 10K bits

        let start = Instant::now();
        let _encoded = encoder.encode(&input);
        let duration = start.elapsed();

        println!("Encoded 10K bits in {:?}", duration);
        assert!(duration.as_millis() < 100);  // Should be fast
    }
}
```

## Documentation Standards

### Code Documentation

#### Module-Level Documentation
```rust
//! Convolutional encoding and decoding implementations.
//!
//! This module provides implementations of convolutional encoders and
//! Viterbi decoders for forward error correction. The algorithms follow
//! standard telecommunications practices and are suitable for educational
//! and research purposes.
//!
//! # Examples
//!
//! Basic convolutional encoding:
//!
//! ```
//! use codecrypt::convolutional::ConvolutionalEncoder;
//!
//! let mut encoder = ConvolutionalEncoder::new(3, vec![7, 5]);
//! let encoded = encoder.encode(&[true, false, true]);
//! ```

pub mod encoder;
pub mod viterbi;
```

#### Function Documentation
- Always document public functions
- Include examples for complex functions
- Explain algorithm choices and trade-offs
- Reference academic sources when appropriate

### External Documentation

#### Markdown Standards
- Use proper heading hierarchy
- Include code examples that actually work
- Cross-reference related sections
- Keep line length reasonable for readability

#### Mathematical Notation
```markdown
<!-- Use LaTeX-style math when needed -->
The encoder output is calculated as:

```
c_i = Σ g_j × u_{i-j} mod 2
```

where g_j are the generator polynomial coefficients.
```

### Documentation Contributions

**High-Value Areas:**
- Algorithm explanations with examples
- Theory-to-implementation bridges
- Common use case tutorials
- Troubleshooting guides
- Performance analysis

## Pull Request Process

### Before Submitting

**Checklist:**
- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test`)
- [ ] Code is properly formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated (if applicable)

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Documentation
- [ ] Code comments updated
- [ ] API documentation updated
- [ ] User documentation updated

## Related Issues
Fixes #(issue number)
```

### Review Process

1. **Automated Checks**: CI must pass
2. **Code Review**: At least one maintainer review
3. **Testing**: Comprehensive test coverage
4. **Documentation**: Must be complete and accurate

### Review Criteria

**Code Quality:**
- Follows project style guidelines
- Has appropriate error handling
- Includes comprehensive tests
- Well-documented with clear comments

**Functionality:**
- Correctly implements the intended feature
- Handles edge cases appropriately
- Performance is reasonable
- API is intuitive and consistent

**Documentation:**
- API documentation is complete
- User-facing documentation updated
- Examples work correctly
- Theory documentation accurate

## Community Guidelines

### Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). In summary:

- Be kind and respectful
- Focus on constructive feedback
- Welcome newcomers and help them learn
- Assume good intentions
- When in doubt, ask questions

### Communication

**Preferred Channels:**
- **GitHub Issues**: Bug reports, feature requests, discussions
- **Pull Request Comments**: Code-specific discussions
- **GitHub Discussions**: General questions and ideas

**Response Expectations:**
- Issues: Response within 1-2 weeks
- Pull Requests: Initial review within 1 week
- Questions: Best effort, community-driven support

### Recognition

Contributors are recognized through:
- GitHub contributor lists
- CONTRIBUTORS.md file
- Release notes acknowledgments
- Special recognition for significant contributions

## Development Best Practices

### Security Considerations

**For Cryptographic Code:**
- Never implement production cryptography
- Clearly label educational implementations
- Include security warnings in documentation
- Follow constant-time practices where applicable

**For All Code:**
- Validate all inputs
- Handle errors gracefully
- Avoid panic in library code
- Use safe Rust practices

### Performance Guidelines

**Optimization Priority:**
1. Correctness first
2. Readability second
3. Performance third

**When to Optimize:**
- Profile first, optimize second
- Focus on algorithmic improvements
- Consider memory allocation patterns
- Use benchmarks to verify improvements

### Maintenance

**Long-term Considerations:**
- APIs should be stable and well-designed
- Dependencies should be minimal and well-maintained
- Code should be easy to understand and modify
- Documentation should be comprehensive and current

## Getting Help

### Resources

- **Documentation**: Start with the docs in this repository
- **Rust Learning**: [The Rust Book](https://doc.rust-lang.org/book/)
- **Cryptography**: Academic papers and textbooks referenced in theory docs
- **GitHub**: Search existing issues and discussions

### Asking Questions

**Good Questions Include:**
- What you're trying to accomplish
- What you've already tried
- Specific error messages or unexpected behavior
- Relevant code snippets or examples

**Where to Ask:**
- **Implementation questions**: GitHub Issues
- **Usage questions**: GitHub Discussions
- **Bug reports**: GitHub Issues with reproduction steps

## Thank You

Thank you for your interest in contributing to CodeCrypt! Your contributions help make cryptography and error correction more accessible for learning and research.

Every contribution matters, whether it's:
- Fixing a typo in documentation
- Adding a test case
- Implementing a new algorithm
- Improving performance
- Helping other users

We look forward to working with you!

---

*This guide is a living document. If you find areas that need clarification or improvement, please contribute your suggestions!*