# cdp-rs

A Rust workspace for Chrome DevTools Protocol automation.

The repository currently contains:

- cdp-core: a high-level async API for launching browsers, driving pages, simulating input, intercepting network traffic, and managing storage.
- cdp-protocol: generated Chrome DevTools Protocol types and command/event definitions.

## Features

- 🌐 **Browser Control** - Launch, connect, and manage browser instances
- 🔍 **Element Interaction** - Find, click, type, and manipulate DOM elements. Features seamless **Cross-iframe** querying and **Shadow Root** (open/closed) support
- 📸 **Screenshots** - Capture element and full-page screenshots
- 🌐 **Network Control** - Intercept, monitor, and modify network requests
- ⌨️ **Input Simulation** - Realistic keyboard and mouse input
- 💾 **Storage Management** - Control cookies, localStorage, and sessionStorage
- ⏳ **Smart Waiting** - Wait for selectors, navigation, network idle, custom conditions
- 🎭 **Emulation** - Device emulation, geolocation, user agents
- ♿ **Accessibility** - Inspect accessibility tree
- 📊 **Performance Tracing** - Capture performance traces

## Quick Start

```rust
use cdp_core::{Browser, PageScreenshotOptions, WaitForNavigationOptions, WaitUntil};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch browser
    let browser = Browser::launcher().launch().await?;

    // Create a new page
    let page = browser.new_page().await?;

    // Navigate to a website
    page.navigate("https://example.com").await?;
    page.wait_for_navigation(Some(
        WaitForNavigationOptions::default()
            .with_timeout_ms(10_000)
            .with_wait_until(WaitUntil::NetworkIdle2),
    ))
    .await?;

    // Find and interact with elements
    if let Some(button) = page.query_selector("#button").await? {
        button.click().await?;
    }

    // Take a screenshot
    page
        .screenshot(
            PageScreenshotOptions::default()
                .full_page()
                .save_to("screenshot.png"),
        )
        .await?;

    // Explicitly release page and connection resources when done
    page.cleanup().await?;
    browser.disconnect().await?;

    Ok(())
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cdp-core = "0.5.0"
tokio = { version = "1", features = ["full"] }
```

Use `cdp-protocol = "0.3.1"` only if you need low-level generated protocol types directly.

## Current API Style

High-level APIs that manage `cdp-core` state or return ergonomic results execute directly:

```rust
use cdp_core::{NetworkControl, PageScreenshotOptions, ReloadOptions};

page.navigate("https://example.com").await?;
page.reload(ReloadOptions::default()).await?;
page.clear_browser_cache().await?;
page.block_urls(["*.png", "*.jpg"]).await?;

let path = page
    .screenshot(
        PageScreenshotOptions::default()
            .full_page()
            .save_to("page.png"),
    )
    .await?;
```

Native CDP wrappers use chain-configured command builders when advanced protocol parameters or per-command timeouts are useful:

```rust
use std::time::Duration;

let pdf = page
    .print_to_pdf()
    .set_params(|params| {
        params.print_background = Some(true);
        params.prefer_css_page_size = Some(true);
    })
    .set_timeout(Duration::from_secs(30))
    .send()
    .await?;
```

For lower-level commands, use the CDP escape hatches directly:

```rust
use cdp_protocol::page;

let history = page
    .cdp(page::GetNavigationHistory(None))
    .send()
    .await?;
```

## Lifecycle Management

For short-lived or exclusive browser connections, explicitly clean up the page and disconnect the browser when work is complete.

When reusing a shared `Browser`, keep `page.cleanup()` and `context.close()` at request boundaries, and call `browser.disconnect()` only during process shutdown or after the shared connection is known to be unhealthy.

## Documentation

- **[Getting Started Guide](crates/cdp-core/docs/GETTING_STARTED.md)** - Installation and first steps
- **[API Reference](crates/cdp-core/docs/API_REFERENCE.md)** - Quick reference for the high-level API
- **[Feature Guides](crates/cdp-core/docs/features/)** - Detailed documentation for each feature area
- **[How-To Guides](crates/cdp-core/docs/howto/)** - Practical recipes and usage patterns
- **[cdp-core examples](crates/cdp-core/examples/)** - Manual browser examples split into deterministic local examples and explicit live-site examples
- **[Manual Example Suite](crates/cdp-core/docs/MANUAL_EXAMPLES.md)** - Canonical local/live example ownership and run commands

## Examples

From the workspace root, run examples from [crates/cdp-core/examples/](crates/cdp-core/examples/):

```bash
# Compile every manual example, including live examples
cargo check -p cdp-core --examples

# Deterministic local examples
cargo run -p cdp-core --example api_browser_page
cargo run -p cdp-core --example api_element_frame_input
cargo run -p cdp-core --example api_network_local
cargo run -p cdp-core --example api_storage_session
cargo run -p cdp-core --example api_emulation_accessibility_tracing
cargo run -p cdp-core --example api_output_capture
cargo run -p cdp-core --example api_events_local

# Explicit external-network examples
CDP_RS_LIVE=1 cargo run -p cdp-core --example api_live_amazon
CDP_RS_LIVE=1 cargo run -p cdp-core --example api_live_fedex
```

## Architecture

cdp-core provides a high-level, async Rust API over the Chrome DevTools Protocol:

```
┌─────────────────────────────────────┐
│         Your Application            │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│          cdp-core API               │
│  (Browser, Page, Element, Network)  │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│    Chrome DevTools Protocol (CDP)   │
└─────────────────┬───────────────────┘
                  │
┌─────────────────▼───────────────────┐
│      Chrome/Chromium Browser        │
└─────────────────────────────────────┘
```

## Requirements

- Chrome or Chromium browser
- Rust 1.85 or later
- Tokio async runtime

## License

This project is licensed under the [MIT license](LICENSE).

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.
