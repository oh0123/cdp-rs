# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### cdp-core
- Version bumped to 0.3.0 (in progress)

#### cdp-protocol
- Version bumped to 0.3.0 (in progress)

## [0.2.0] - 2025-12-13

### Added

#### cdp-protocol
- Updated Chrome DevTools Protocol to Chrome **143.0.7499.110** (from 140.0.7339.186)
- **Emulation domain**:
  - New type `ScreenId` (alias for `String`)
  - New struct `WorkAreaInsets` (top, left, bottom, right optional insets)
  - New struct `ScreenInfo` (full screen descriptor including position, dimensions, DPI, orientation, color depth, label, etc.)
  - New command `GetScreenInfos` — retrieve all screen info objects
  - New command `AddScreen` — add a virtual screen for display emulation
  - New command `RemoveScreen` — remove a previously added virtual screen
- **Network domain**:
  - New enum `IpProxyStatus` (Available, FeatureNotEnabled, MaskedDomainListNotEnabled, MaskedDomainListNotPopulated, AuthTokensUnavailable, Unavailable, BypassedByDevTools)
  - New variant `FedCm` in `InitiatorType` enum
  - New field `isAdRelated: Option<bool>` on `Request`
  - New struct `NetworkConditions` (url-pattern based per-rule network condition)
  - New struct `BlockPattern` (url-pattern based block rule)
  - New command `GetIPProtectionProxyStatus`
  - New command `SetIPProtectionProxyBypassEnabled`
  - New command `EmulateNetworkConditionsByRule` — per-URL-pattern network emulation
  - New command `OverrideNetworkState` — override low-level network state
- **DOM domain**:
  - New pseudo-type variant `InterestHint`
  - New field `affectedByStartingStyles: Option<bool>` on `Node`
  - New event `AffectedByStartingStylesFlagUpdated`
- **Page domain**:
  - New `PermissionsPolicyFeature` variants: `DigitalCredentialsCreate`, `DirectSocketsMulticast`
  - New `BackForwardCacheNotRestoredReason` variants: `SharedWorkerWithNoActiveClient`, `WebBluetooth`
  - Renamed BFCache reason variants: `WebRTCSticky` → `WebRTCUsedWithCCNS`, `WebTransportSticky` → `WebTransportUsedWithCCNS`, `WebSocketSticky` → `WebSocketUsedWithCCNS`
- **Runtime domain**:
  - New subtype variant `Trustedtype` in `RemoteObjectSubtype`, `ObjectPreviewSubtype`, and `PropertyPreviewSubtype`
- **Storage domain**:
  - New command `GetStorageKey` (accepts optional `frameId`, returns `SerializedStorageKey`)
- **Event routing** (`types.rs`):
  - Added missing event variants: `BluetoothEmulation` GATT/characteristic/descriptor operation events
  - Added `DOM.affectedByStartingStylesFlagUpdated` event
  - Added `DeviceAccess.deviceRequestPrompted`, `FedCm.dialogShown/dialogClosed`, `Fetch.requestPaused/authRequired` (relocated to correct position)
  - Added `Inspector.workerScriptLoaded`, `Media` player events (5 variants)
  - Added `Preload` domain events (6 variants: ruleSetUpdated/Removed, preloadEnabledStateUpdated, prefetchStatusUpdated, prerenderStatusUpdated, preloadingAttemptSourcesUpdated)
  - Removed deprecated `Network.subresourceWebBundle*` events (4 variants)

### Changed

#### cdp-protocol
- `ResourceTiming.bytes`: changed type from `Option<String>` to `Option<Vec<u8>>`
- `Page`: screenshot `data` field changed from `String` to `Vec<u8>` for binary correctness

#### Infrastructure
- GitHub Actions `actions/checkout` upgraded from v4 to v6 across all workflows
- `softprops/action-gh-release` upgraded from v1 to v2 in release workflow

### Fixed

#### Dependencies
- Pinned `reqwest` to `0.12.25` (was `0.12`)
- Pinned `tracing-subscriber` to `0.3.22` (was `0.3`)

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

[Unreleased]: https://github.com/oh0123/cdp-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/oh0123/cdp-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/oh0123/cdp-rs/releases/tag/v0.1.0
