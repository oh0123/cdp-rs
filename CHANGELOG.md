# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-04-01

### Added

#### cdp-protocol
- Updated Chrome DevTools Protocol to Chrome **146.0.7680.165** (from 143.0.7499.110)
- **New domain: SmartCardEmulation** (`smart_card_emulation.rs`) — full PC/SC smart card emulation support:
  - Types: `ResultCode`, `ShareMode`, `Disposition`, `Protocol`, `ReaderStateIn`, `ReaderStateOut`, `DeviceAttributes`, `SCardHandle`
  - Commands: `EstablishContext`, `ReleaseContext`, `ListReaders`, `GetStatusChange`, `Cancel`, `Connect`, `Disconnect`, `Transmit`, `Control`, `GetAttrib`, `SetAttrib`, `Status`, `BeginTransaction`, `EndTransaction`
  - 13 corresponding events in event routing
- **Network domain**:
  - New type alias `DeviceBoundSessionEventId`
  - New enum `RenderBlockingBehavior` (Blocking, InBodyParserBlocking, NonBlocking, NonBlockingDynamic, PotentiallyBlocking)
  - New enum `DeviceBoundSessionWithUsageUsage` (6 variants for session usage tracking)
  - New enum `DeviceBoundSessionUrlRuleRuleType` (Exclude / Include)
  - New events: `directUDPSocketJoinedMulticastGroup`, `directUDPSocketLeftMulticastGroup`, `deviceBoundSessionsAdded`, `deviceBoundSessionEventOccurred`
- **Audits domain**:
  - New enum `ConnectionAllowlistError` (6 variants)
  - New enum `PermissionElementIssue` (21 variants covering layout, security, and activation errors)
  - New `InspectorIssueCode` variants: `ConnectionAllowlistIssue`, `PermissionElementIssue`
  - New `GenericIssueErrorType` variants: `NavigationEntryMarkedSkippable`, `AutofillAndManualTextPolicyControlledFeaturesInfo`, `AutofillPolicyControlledFeatureInfo`, `ManualTextPolicyControlledFeatureInfo`
- **DOM domain**: new event `adoptedStyleSheetsModified`
- **Overlay domain**: new events `inspectPanelShowRequested`, `inspectedElementWindowRestored`
- **Event routing** (`types.rs`): 13 `SmartCardEmulation.*` events, 4 new `Network.*` events, 2 new `Overlay.*` events, `DOM.adoptedStyleSheetsModified`
- **`derive_builder`** integration: protocol structs now derive `Builder` for ergonomic construction (opt-in per struct)

#### cdp-core
- `grant_permissions` refactored: each `PermissionType` is now sent as an individual `Browser.setPermission` CDP call, fixing incorrect batch usage of `GrantPermissions`
- `BrowserContext::open_page`: `focus: Some(true)` now passed by default when creating a new page target

#### Dependencies
- Added `derive_builder = "0.20.2"` to workspace dependencies

### Changed

#### cdp-protocol
- **Network domain**:
  - Removed `IpProxyStatus` enum (superseded by updated IP Protection API)
  - `CorsError` enum: renamed private-network variants — `InsecurePrivateNetwork` → `InsecureLocalNetwork`, `InvalidPrivateNetworkAccess` → `InvalidLocalNetworkAccess`; removed `PreflightMissingAllowPrivateNetwork`, `PreflightInvalidAllowPrivateNetwork`, `PreflightMissingPrivateNetworkAccessId`, `PreflightMissingPrivateNetworkAccessName`, `PrivateNetworkAccessPermissionUnavailable`, `PrivateNetworkAccessPermissionDenied`
  - `PrivateNetworkRequestPolicy` renamed to `LocalNetworkAccessRequestPolicy`; removed `PreflightBlock` and `PreflightWarn` variants
  - Cookie enums: removed `SamePartyFromCrossPartyContext` and `SamePartyConflictsWithOtherAttributes` variants
- **Audits domain**: `GenericIssueErrorType` — renamed `FormAriaLabelledByToNonExistingId` → `FormAriaLabelledByToNonExistingIdError`, `FormLabelHasNeitherForNorNestedInput` → `FormLabelHasNeitherForNorNestedInputError`; removed `ExcludeInvalidSameParty` and `ExcludeSamePartyCrossPartyContext` from `CookieExclusionReason`

#### cdp-core
- `CookieManager::set_cookie`: removed deprecated `same_party` field; return value always `true` (API aligned with updated protocol)
- `TracingStartOptions`: removed explicit `categories` / `options` override fields from the generated `Start` command builder

#### Dependencies (workspace)
- `tokio-tungstenite`: `0.28` → `0.29`
- `tokio-stream`: `0.1.17` → `0.1.18`
- `futures-util`: `0.3` → `0.3.32`
- `reqwest`: `0.12.25` → `0.13.2`
- `url`: `2.3` → `2.5.8`
- `regex`: `1.11` → `1.12.3`
- `rand`: `0.9` → `0.10`
- `tracing`: `0.1.43` → `0.1.44`
- `tempfile`: `3.23.0` → `3.27.0`
- `thiserror`: `2.0.17` → `2.0.18`
- `tracing-subscriber`: `0.3.22` → `0.3.23`

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

[Unreleased]: https://github.com/oh0123/cdp-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/oh0123/cdp-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/oh0123/cdp-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/oh0123/cdp-rs/releases/tag/v0.1.0
