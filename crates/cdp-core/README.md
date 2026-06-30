# cdp-core

A high-level async Rust crate for controlling Chrome/Chromium browsers via the Chrome DevTools Protocol (CDP).

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
use cdp_core::{Browser, WaitForNavigationOptions, WaitUntil};

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
    page.screenshot(true, Some("screenshot.png".into()), true).await?;

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
cdp-core = "0.4.0"
tokio = { version = "1", features = ["full"] }
```

## Current API Style

High-level APIs that manage cdp-core state or return ergonomic results execute directly:

```rust
page.navigate("https://example.com").await?;
page.reload(ReloadOptions::default()).await?;
let path = page.screenshot(true, Some("page.png".into()), true).await?;
```

Native CDP wrappers return command builders so advanced parameters and per-command timeouts stay chain-configured:

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

Page-scoped extension traits provide network, storage, and session helpers:

```rust
use cdp_core::NetworkControl;

page.clear_browser_cache().await?;
page.block_urls(["*.png", "*.jpg"]).await?;
```

## Lifecycle Management

For short-lived or exclusive browser connections, explicitly clean up the page and disconnect the browser when work is complete.

When reusing a shared `Browser`, keep `page.cleanup()` and `context.close()` at request boundaries, and call `browser.disconnect()` only during process shutdown or after the shared connection is known to be unhealthy.

## Documentation

- **[Getting Started Guide](docs/GETTING_STARTED.md)** - Installation and first steps
- **[API Reference](docs/API_REFERENCE.md)** - Quick reference for the high-level API
- **[Feature Guides](docs/features/)** - Detailed documentation for each feature area
- **[How-To Guides](docs/howto/)** - Practical recipes and usage patterns

## Examples

Check out the [examples](examples/) directory for runnable code:

```bash
# Run basic example
cargo run --example basic

# Run comprehensive example
cargo run --example comprehensive

# Run network example
cargo run --example network

# Run event handling example
cargo run --example events

# Run Runtime.consoleAPICalled example
cargo run --example runtime_console_events

# Run screencast example
cargo run --example screencast
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

Contributions welcome! See [../../CONTRIBUTING.md](../../CONTRIBUTING.md) for repository-wide guidelines.
