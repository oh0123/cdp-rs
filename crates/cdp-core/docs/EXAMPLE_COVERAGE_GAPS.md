# Manual Example Coverage Gap List

This document tracks public `cdp-core` APIs that need runnable manual examples.
Browser-backed examples should live under `crates/cdp-core/examples/` and be run
manually with `cargo run -p cdp-core --example ...`. They should not be placed
under `tests/`, because connecting to a local browser is environment-sensitive
and should not become part of the default CI gate.

Use `API_REFERENCE.md` as the public API baseline. Prefer deterministic local
fixtures (`file://`, `data:`, or a local `127.0.0.1` HTTP server). Keep external
site examples behind explicit live examples or an environment flag.

See [MANUAL_EXAMPLES.md](MANUAL_EXAMPLES.md) for the runnable manual suite and
the commands used to execute it locally.

## Implemented Manual Suite

The first local manual suite now covers the major API groups:

- `api_browser_page`
- `api_element_frame_input`
- `api_network_local`
- `api_storage_session`
- `api_emulation_accessibility_tracing`
- `api_output_capture`
- `api_events_local`
- `api_live_amazon` (explicit live)
- `api_live_fedex` (explicit live)

Known follow-up gaps from the first smoke run:

- Browser window APIs are best-effort in headless mode. The example skips them
  when Chrome reports that no window is available.
- `Frame::evaluate` works for local iframes, but `Frame::query_selector_all`
  and `Page::query_frames` currently return `0` for the nested-frame fixture.
  Keep this as an API behavior investigation before marking frame selector
  helpers fully covered.

## Coverage Status Legend

- `covered`: API is already represented in `API_REFERENCE.md` and has a clear
  candidate scenario in existing examples.
- `partial`: API is documented or shown, but the example is shallow, print-only,
  depends on external sites, or lacks a concrete verification scenario.
- `missing`: API is public but absent from `API_REFERENCE.md` and needs an
  example scenario.
- `live`: the useful scenario intentionally depends on external network behavior.

## Browser And Context Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Launcher options | `Launcher::port`, `browser`, `launch_options`, `configure_options`, `disable_images`, `mute_audio`, `incognito`, `user_data_dir`, `profile_directory`, extension/switch/feature helpers | Launch with a temporary profile, custom port, disabled images, muted audio, and one custom switch; print `browser.command_line()` and assert the expected flags are present | local |
| missing | Browser window control | `window_for_target`, `window_bounds`, `set_window_bounds`, `set_contents_size` | Open a page, fetch its window id, resize bounds and content area, then read bounds again | local |
| missing | Browser permissions | `Browser::set_permission`, `reset_permissions`, `reset_permissions_for_context` | Grant or deny geolocation on a local origin, call `navigator.geolocation.getCurrentPosition`, then reset permissions | local HTTP |
| covered | Browser CDP escape hatch | `Browser::cdp` | Covered by `api_browser_page` with `Browser.getVersion` and version comparison | local |
| partial | Context options | `BrowserContextOptions` builder methods | Create contexts with download, permission, and emulation options; verify new pages inherit the configured behavior | local HTTP |
| missing | Context permissions | `BrowserContext::grant_permissions`, `apply_permission_grant`, `set_permission_override`, `reset_permissions` | Configure geolocation permission for only one context; verify another context remains isolated | local HTTP |
| missing | Context downloads | `DownloadOptions`, `DownloadBehavior`, `set_download_behavior`, `clear_download_behavior` | Serve a downloadable file from localhost, save it to a temp directory, then reset behavior | local HTTP |
| missing | Context emulation lifecycle | `set_emulation_config`, `clear_emulation_config`, `close`, `id` | Apply locale/timezone/user-agent to a context, open two pages, verify both pages observe it, then clear and close | local |

## Page Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Navigation helpers | `bring_to_front`, `wait_for_loaded`, `get_title`, `reset_navigation_history` | Navigate through three local pages, wait for load/title, reset history, then verify back navigation is no longer available | local HTTP |
| covered | Page snapshots and PDF | `capture_snapshot`, `print_to_pdf`, `default_print_to_pdf_options` | Covered by `api_output_capture`; MHTML and PDF outputs are written under `target/manual-examples` and checked for bytes | local HTML |
| covered | Script injection | `add_script_to_evaluate_on_new_document`, `remove_script_to_evaluate_on_new_document` | Covered by `api_browser_page` with a before-navigation script and removal check | local HTML |
| covered | JavaScript dialogs | `handle_javascript_dialog` plus dialog events | Covered by `api_browser_page` with local `alert` and dialog handling | data URL |
| covered | Resource tree/content | `get_resource_tree`, `get_resource_content` | Covered by `api_browser_page` against local CSS/JS resources | local HTTP |
| partial | Target filters | `get_targets_with_filter`, `FilterEntry`, `TargetFilter` | Open multiple targets and fetch only page-like targets through a filter | local |
| covered | CDP escape hatches | `Page::cdp`, `Page::root_cdp`, `CdpCommandBuilder` | Covered by `api_browser_page` with page and root CDP commands | local |
| missing | Frame cache and hierarchy | `get_parent_frame`, `get_child_frames`, `get_ancestor_frames`, `get_descendant_frames`, `query_frame`, `query_frames`, `clear_frame_cache` | Load nested iframes and verify the frame tree relationships and selector-based frame lookup | local HTML |
| missing | Execution context bookkeeping | `register_execution_context`, `remove_execution_context`, `clear_execution_contexts` | Treat as advanced/internal-facing public surface; document whether manual examples are appropriate or whether these should be de-emphasized | local |
| missing | Frame lifecycle callbacks | `on_frame_lifecycle`, `FrameLifecycleEvent` | Register lifecycle callback, navigate an iframe, and count attach/navigate/detach events | local HTML |
| missing | DOM mutation callbacks | `enable_dom_mutations`, `on_dom_mutation`, `DomMutationEvent` | Add/remove nodes and modify attributes from script; verify callback counters | local HTML |
| missing | Frame snapshots | `FrameSnapshot`, `create_frame_snapshot`, `create_all_frames_snapshot`, `compare_snapshots` | Snapshot all frames, mutate one iframe DOM, take another snapshot, and print diffs | local HTML |
| partial | Page-level typing | `Page::type_text`, `Page::type_text_with_delay` | Focus an input, type through page-level helpers, and verify input events and value | local HTML |
| covered | Screencast | `ScreencastOptions`, `start_screencast`, `wait_for_screencast_frame`, `screencast_frame_ack`, `stop_screencast` | Covered by `api_output_capture`; three frames are saved and acknowledged | local HTML |
| partial | Page cleanup | `cleanup`, `close` | Open a page, verify target id, call cleanup/close, then verify it disappears from `get_tabs()` | local |

## Element Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Mouse variants | `left_click`, `right_click`, `move_mouse_to`, `press_and_hold`, `press_and_hold_until` | Use buttons and a long-press target; verify click type, hover position, and hold duration through DOM counters | local HTML |
| partial | Text input variants | `clear`, `upload_files`, `type_text_with_delay` | Clear an input, type delayed text, upload a temp file through `<input type=file>`, verify file name and input value | local HTML |
| partial | Element screenshot options | `ScreenshotBoxType`, `box_type`, `padding_box`, `border_box`, `margin_box`, `auto_resolve_dpr` | Capture the same styled element with content/padding/border/margin boxes and compare file sizes or dimensions | local HTML |
| missing | Element wait helpers | `wait_for_visible`, `wait_for_hidden`, `wait_for_clickable`, `wait_for_enabled` | Toggle hidden/disabled/overlay states from JavaScript and wait for each state transition | local HTML |
| partial | Shadow DOM variants | `query_selector_all_shadow`, `ShadowRoot::shadow_root_type`, `ShadowRoot::get_html`, `ShadowRoot::query_selector_all` | Query multiple elements from an open shadow root, inspect shadow HTML, and document closed-root behavior | local HTML |

## Frame Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Frame metadata | `Frame::id`, `execution_context_id`, `name`, `url`, `is_detached` | Load named iframes, print ids/names/URLs, remove an iframe, verify detached state | local HTML |
| missing | Frame function calls | `call_function_on`, `call_function_on_with_retry`, `evaluate_with_retry`, `RetryConfig` | Evaluate in an iframe during a controlled reload; compare no-retry/conservative/aggressive behavior | local HTML |
| partial | Frame queries | `query_selector_all` | Query repeated elements inside a specific iframe and verify count and text | local HTML |

## Network Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| covered | Network control | `set_extra_http_headers`, `set_blocked_url_patterns`, `get_response_body`, `get_request_post_data` | Covered by `api_network_local` with local echo routes and captured request ids | local HTTP |
| covered | Request interception actions | `InterceptRequestAction::Abort`, `Fulfill`, `Modify`, `Continue`; `RequestModification` URL/method/header/post-data helpers; `ResponseMock` | Covered by `api_network_local` with local modify/fulfill/abort/continue cases | local HTTP |
| partial | Low-level interception | `continue_request`, `continue_request_with_modification`, `fail_request`, `fulfill_request`, `continue_response`, `continue_response_with_modification` | Use `page.on::<fetch::RequestPausedEvent>()` to exercise explicit low-level methods | local HTTP |
| covered | Request helpers | `intercept_all_requests`, `intercept_requests_matching`, `block_images`, `block_stylesheets` | Covered by `api_network_local` helper smoke cases | local HTTP |
| covered | Response monitoring lifecycle | `monitor_responses`, `monitor_responses_matching`, `stop_response_monitoring` | Covered by `api_network_local`; live response body behavior is additionally covered by `api_live_fedex` | local HTTP, live |
| missing | Cookie options | `SetCookieParams` secure/httpOnly/sameSite/expires/priority builders; `DeleteCookieOptions` URL/domain/path builders | Set multiple scoped cookies on localhost, query them, delete by path/domain, verify remaining cookies | local HTTP |
| live | Real response body capture | FedEx-like API response monitoring | Keep as explicit live smoke because real response timing/body behavior is the value | external |

## Storage And Session Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Generic storage | `StorageManager`, `StorageType`, `get_storage_items`, `get_storage_item`, `set_storage_item`, `remove_storage_item`, `clear_storage`, `get_storage_length` | Exercise the generic API for local and session storage before using convenience helpers | local HTML |
| partial | Convenience storage | `get_local_storage`, `get_session_storage` | API_REFERENCE shows item-level operations but should also show full map retrieval | local HTML |
| covered | Page session object | `PageSession::new`, `to_json`, `from_json`, `save_to_file`, `load_from_file` | Covered by `api_storage_session` | local |
| covered | Page session lifecycle | `export_session`, `import_session`, `export_session_to_file`, `import_session_from_file`, `snapshot`, `restore`, `clone_session_to` | Covered by `api_storage_session` | local HTTP |

## Emulation Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Full config builder | `with_timezone`, `with_locale`, `with_media`, full `Geolocation` fields, `UserAgentOverride` accept-language/platform/metadata | Apply a full emulation config and verify `Intl.DateTimeFormat`, `navigator.language`, `matchMedia`, geolocation, and UA fields | local HTML |
| missing | Individual clears/resets | `clear_geolocation`, `set_timezone`, `reset_timezone`, `set_locale(None)`, `set_media`, `clear_media` | Apply each override, verify page-visible behavior, clear/reset, and verify fallback | local HTML |
| missing | UA client hints metadata | `UserAgentMetadataOverride`, `UserAgentBrand` | Set user-agent metadata and inspect `navigator.userAgentData` when Chrome exposes it | local HTML |

## Accessibility And Tracing Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| covered | Accessibility snapshot | `AccessibilityController::enable`, `disable`, `snapshot`; `AccessibilitySnapshotOptions`; `AccessibilitySnapshot::is_empty` | Covered by `api_emulation_accessibility_tracing` | local HTML |
| covered | Tracing lifecycle | `TracingController::categories`, `is_recording`, `start`, `stop`; `TracingStartOptions`; `TracingStopResult` | Covered by `api_emulation_accessibility_tracing`; trace output is saved under `target/manual-examples` | local HTML |

## Domain Manager Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Optional domains | `enable_fetch_domain`, `disable_fetch_domain`, `enable_log_domain`, `disable_log_domain`, `enable_performance_domain`, `disable_performance_domain`, domain state checks | Show explicit enable/disable around request interception, log events, and performance metrics | local |
| missing | Domain config builders | `DomainConfig` builder methods | Document whether users can currently pass custom config into a launched page; if not, keep this as API-reference-only or future design work | local |

## Input Gaps

| Status | API area | APIs | Recommended manual example | Fixture |
| --- | --- | --- | --- | --- |
| partial | Keyboard lower-level API | `key_down_with_modifiers`, `key_up_with_modifiers`, `press_key_with_modifiers`, `press_character`, `press_with_modifiers`, `KeyInput`, `KeyModifiers`, `KeyboardModifier` | Use shortcuts, keypad/location metadata, and character entry; verify DOM keyboard event logs | local HTML |
| partial | Mouse advanced API | `right_click`, `press_and_hold`, `press_and_hold_until`, `DragOptions`, `DragEasing`, jitter/steps/timing options | Use a drag target and context-menu target; verify event order and final position | local HTML |

## Existing Live Examples To Keep Separate

| Status | Historical example | Canonical owner |
| --- | --- | --- |
| live | `basic.rs` Amazon selector/screenshot | `api_live_amazon` |
| live | `concurrent_contexts.rs` Amazon product pages | `api_live_amazon` |
| live | `network.rs` FedEx response monitoring | `api_live_fedex`; local response monitoring lives in `api_network_local` |
| local | `events.rs` and `runtime_console_events.rs` | `api_events_local` |
| local | `storage_and_sessions.rs` GitHub storage/session flow | `api_storage_session` |
| local | `page_close.rs` Google target closing flow | `api_browser_page` |
| local | `screencast.rs` example.com capture flow | `api_output_capture` |
| local | `advanced_features.rs` example.com shadow/frame/wait flow | `api_element_frame_input` and `api_browser_page` |
