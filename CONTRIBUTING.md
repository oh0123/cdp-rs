# Contributing to cdp-rs

Thank you for your interest in contributing to cdp-rs! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Provide specific examples and code samples
- Describe the behavior you observed and what you expected
- Include your environment details (OS, Rust version, browser version)

### Suggesting Enhancements

Enhancement suggestions are welcome! When suggesting an enhancement:

- Use a clear and descriptive title
- Provide a detailed description of the suggested enhancement
- Explain why this enhancement would be useful
- Include code examples of how the feature might be used

### Pull Requests

1. Fork the repository and create your branch from `main`
2. Make your changes following our coding standards
3. Add tests for your changes
4. Ensure all tests pass
5. Update documentation as needed
6. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.85.0 or later
- Chrome or Chromium browser
- Git

### Setup Steps

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/cdp-rs.git
cd cdp-rs

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

## Coding Standards

### Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Write meaningful commit messages

### Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting a PR
- Include integration tests for complex features
- Test on multiple platforms when possible

### Documentation

- Document all public APIs with doc comments
- Include code examples in documentation
- Update README.md and relevant documentation files
- Keep CHANGELOG.md updated

## Project Structure

```
cdp-rs/
├── crates/
│   ├── cdp-core/          # Main library with high-level APIs
│   └── cdp-protocol/      # Low-level CDP protocol bindings
├── docs/                  # Documentation
├── examples/              # Example code
└── tests/                 # Integration tests
```

## Commit Messages

Follow conventional commits format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(browser): add support for custom browser arguments
fix(page): resolve race condition in navigation
docs(readme): update installation instructions
```

## Release Process

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.x.x`
4. Push the tag: `git push origin v0.x.x`
5. GitHub Actions will automatically publish to crates.io

## Questions?

If you have questions, feel free to:
- Open a discussion on GitHub Discussions
- Ask in an issue
- Reach out to the maintainers

## License

By contributing to cdp-rs, you agree that your contributions will be licensed under the MIT License.
