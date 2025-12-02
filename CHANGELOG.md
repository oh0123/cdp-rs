# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-12-02

### Added

#### cdp-core
- Initial release of cdp-core library
- **Browser Control**: Launch, connect, and manage Chrome/Chromium browser instances
- **Element Interaction**: Find, click, type, and manipulate DOM elements
  - Seamless cross-iframe element querying
  - Shadow Root support (both open and closed)
- **Screenshot Capabilities**: Capture element and full-page screenshots
- **Network Control**: Intercept, monitor, and modify network requests
- **Input Simulation**: Realistic keyboard and mouse input
- **Storage Management**: Control cookies, localStorage, and sessionStorage
- **Smart Waiting**: Wait for selectors, navigation, network idle, and custom conditions
- **Emulation Features**: Device emulation, geolocation, and user agent configuration
- **Accessibility Support**: Inspect accessibility tree
- **Performance Tracing**: Capture performance traces

#### cdp-protocol
- Initial release of cdp-protocol library
- Chrome DevTools Protocol implementation for Rust
- Serialization/deserialization support via serde

#### Documentation
- Comprehensive README with quick start guide
- Getting Started Guide
- API Reference documentation
- Feature-specific guides
- How-to guides and practical examples
- Contributing guidelines

#### Examples
- Basic browser automation examples
- Comprehensive test suite
- Web scraping examples
- Network monitoring demonstrations

#### Infrastructure
- GitHub Actions workflows for CI/CD
- Issue and PR templates
- Code of conduct
- License (MIT)

### Changed

### Deprecated

### Removed

### Fixed

### Security

[Unreleased]: https://github.com/oh0123/cdp-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/oh0123/cdp-rs/releases/tag/v0.1.0
