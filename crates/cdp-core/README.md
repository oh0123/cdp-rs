# cdp-rs

A robust Rust library for controlling Chrome/Chromium browsers via the Chrome DevTools Protocol (CDP).

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
use cdp_core::{Browser, Page};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch browser
    let browser = Browser::launcher()
        .launch()
        .await?;
    
    // Create a new page
    let page = browser.new_page().await?;
    
    // Navigate to a website
    page.navigate("https://example.com").await?;
    
    // Find and interact with elements
    if let Some(button) = page.query_selector("#button").await? {
        button.click().await?;
    }
    
    // Take a screenshot
    page.screenshot(true, Some("screenshot.png".into())).await?;
    
    Ok(())
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cdp-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Documentation

- **[Getting Started Guide](docs/GETTING_STARTED.md)** - Installation and first steps
- **[API Reference](docs/API_REFERENCE.md)** - Quick reference for all APIs
- **[Feature Guides](docs/features/)** - Detailed documentation for each feature
- **[How-To Guides](docs/howto/)** - Practical recipes and patterns

## Examples

Check out the [examples](examples/) directory for runnable code:

```bash
# Run basic example
cargo run --example basic

# Run comprehensive test
cargo run --example comprehensive_test

# Run web scraping example
cargo run --example web_scraping
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
- Rust 1.70 or later
- Tokio async runtime

## License

This project is licensed under the [MIT license](LICENSE).

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.
