# Manual Browser Example Suite

The examples in `crates/cdp-core/examples/api_*.rs` are browser-backed manual
smoke cases. They are intentionally not integration tests and should not be
added to the default CI test job. They launch or connect to Chrome, exercise real
CDP behavior, and use deterministic local fixtures whenever possible.

Run them from the workspace root:

```bash
cargo check -p cdp-core --examples
cargo run -p cdp-core --example api_browser_page
cargo run -p cdp-core --example api_element_frame_input
cargo run -p cdp-core --example api_network_local
cargo run -p cdp-core --example api_storage_session
cargo run -p cdp-core --example api_emulation_accessibility_tracing
cargo run -p cdp-core --example api_output_capture
cargo run -p cdp-core --example api_events_local
```

Generated files are written under:

```text
target/manual-examples/
```

## Local Fixture Strategy

- `examples/support/mod.rs` starts a small local HTTP server on `127.0.0.1`.
- Most examples use local HTML, local HTTP routes, `data:` pages, or generated
  files in `target/manual-examples`.
- External network examples should remain separate and explicit. Use live
  examples only when the remote site's real behavior is the scenario being
  validated.

## Coverage Matrix

| Example | Primary API coverage | Fixture |
| --- | --- | --- |
| `api_browser_page` | `Browser::launcher`, `Browser::cdp`, contexts, pages, version, command line, window controls best-effort, navigation/history, targets, `Page::cdp`, `Page::root_cdp`, script injection, JavaScript dialogs, resource tree/content, MHTML snapshot, domain manager | local HTTP, data URL |
| `api_element_frame_input` | element query/property APIs, clicks, double click, hover/mouse movement, right click, press-and-hold, clear/type/upload, element screenshots, Shadow DOM, keyboard modifiers, mouse drag, frame metadata/evaluate, frame snapshots | local HTTP |
| `api_network_local` | network monitoring, inflight counters, extra headers, cache/service-worker controls, request post data, response body, request interception continue/modify/abort/fulfill, response monitoring lifecycle, request helper shortcuts, URL blocking | local HTTP |
| `api_storage_session` | generic `StorageManager`, local/session storage convenience traits, cookie get/set/delete, `PageSession` JSON/file round trip, export/import/snapshot/restore/clone | local HTTP |
| `api_emulation_accessibility_tracing` | emulation config, geolocation setters/clear, timezone/locale/media/user-agent overrides, UA metadata builder, accessibility full/subtree snapshots, tracing categories/start/stop/save | local HTTP, data URL |
| `api_output_capture` | page screenshots, MHTML snapshot, `print_to_pdf`, default PDF options constructor, screencast start/frame/ack/stop | local HTTP |
| `api_events_local` | target discovery events, page lifecycle events, `Runtime.consoleAPICalled` one-shot waits and streams, runtime exception events, optional `Log.entryAdded` diagnostics | data URL, local browser |

## Unique Example Ownership

Each public manual example owns one API area. Historical broad examples such as
`basic`, `comprehensive`, `network`, `events`, `runtime_console_events`,
`screencast`, `storage_and_sessions`, `advanced_features`, `page_close`, and
`concurrent_contexts` were consolidated into the canonical `api_*` suite to
avoid duplicate or shallow coverage.

| Ownership | Canonical example | Type |
| --- | --- | --- |
| Browser, contexts, page lifecycle, CDP escape hatches | `api_browser_page` | local |
| Elements, frames, keyboard, mouse, upload, Shadow DOM | `api_element_frame_input` | local |
| Network monitoring/interception/response helpers | `api_network_local` | local |
| Cookies, local/session storage, page sessions | `api_storage_session` | local |
| Emulation, accessibility, tracing | `api_emulation_accessibility_tracing` | local |
| Screenshots, PDF, MHTML, screencast | `api_output_capture` | local |
| Typed event subscriptions and waits | `api_events_local` | local |
| Amazon complex DOM readiness and concurrent contexts | `api_live_amazon` | live |
| FedEx live API response body capture | `api_live_fedex` | live |

## Current Observations

- `Browser::window_for_target` and related window APIs are best-effort in
  headless/local automation. The manual example tries them and skips the section
  when Chrome reports that no window is available.
- `Frame::evaluate` works against local iframes in the manual suite.
  `Frame::query_selector_all` and `Page::query_frames` currently return `0` for
  the nested-frame fixture, so the example prints those counts instead of using
  them as pass/fail gates. This is a follow-up API behavior gap to investigate.
- `Fetch.fulfillRequest` requires a base64-encoded body on the wire. The manual
  network example exercises `InterceptRequestAction::Fulfill` and validates the
  mocked JSON response.

## Live Examples

Keep live-site checks outside the deterministic local suite. They require
external network access and may fail when the public site changes behavior:

```bash
CDP_RS_LIVE=1 cargo run -p cdp-core --example api_live_amazon
CDP_RS_LIVE=1 cargo run -p cdp-core --example api_live_fedex
```

Those examples should be added only for behavior that genuinely requires a
public site, such as real-world complex DOM readiness or live API body capture.
