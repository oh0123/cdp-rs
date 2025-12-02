# Getting Started with CDP-Core

This guide will help you get started with CDP-Core, a Rust library for browser automation using the Chrome DevTools Protocol.

## Installation

### Prerequisites

1. **Chrome or Chromium** - Install the latest version
2. **Rust toolchain** - Install from [rustup.rs](https://rustup.rs/)

### Add CDP-Core to Your Project

```toml
[dependencies]
cdp-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

## Your First Script

### 1. Launch a Browser

There are two ways to work with a browser:

#### Option A: Launch a New Browser Instance

```rust
use cdp_core::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch a new browser instance
    let browser = Browser::launcher()
        .launch()
        .await?;
    
    let page = browser.new_page().await?;
    page.navigate("https://example.com").await?;
    
    Ok(())
}
```

#### Option B: Connect to Existing Browser

First, start Chrome with remote debugging:

```bash
# Mac
/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome \
    --remote-debugging-port=9222 \
    --user-data-dir=/tmp/chrome-debug

# Linux
google-chrome --remote-debugging-port=9222 --user-data-dir=/tmp/chrome-debug

# Windows
"C:\Program Files\Google\Chrome\Application\chrome.exe" ^
    --remote-debugging-port=9222 ^
    --user-data-dir=C:\temp\chrome-debug
```

Then connect:

```rust
use cdp_core::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::connect("http://localhost:9222").await?;
    let page = browser.new_page().await?;
    
    Ok(())
}
```

### 2. Navigate and Interact

```rust
use cdp_core::Browser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Navigate to a page
    page.navigate("https://example.com").await?;
    
    // Wait for page to load
    page.wait_for_selector("h1", None).await?;
    
    // Find an element
    if let Some(heading) = page.query_selector("h1").await? {
        // Get text content
        let text = heading.text_content().await?;
        println!("Page heading: {}", text);
        
        // Click the element
        heading.click().await?;
    }
    
    Ok(())
}
```

### 3. Take Screenshots

```rust
// Element screenshot
if let Some(element) = page.query_selector("#logo").await? {
    let screenshot_base64 = element.screenshot().await?;
    
    // Decode and save
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot_base64)?;
    std::fs::write("logo.png", bytes)?;
}

// Full page screenshot
page.screenshot(true, Some("fullpage.png".into())).await?;
```

### 4. Handle Forms

```rust
// Find input field
if let Some(input) = page.query_selector("input[name='search']").await? {
    // Type text (fast)
    input.type_text("Hello World").await?;
    
    // Or type with human-like delays
    input.type_text_with_delay("Hello World", 50, 150).await?;
}

// Submit form
if let Some(button) = page.query_selector("button[type='submit']").await? {
    button.click().await?;
}
```

## Common Patterns

### Pattern 1: Wait for Dynamic Content

```rust
// Wait for element to appear
let element = page.wait_for_selector(".dynamic-content", None).await?;

// Wait for navigation to complete
page.wait_for_navigation(None).await?;

// Wait for custom condition
page.wait_for_function(
    "() => document.readyState === 'complete'",
    Some(5000),
    None
).await?;
```

### Pattern 2: Batch Element Operations

```rust
// Find all matching elements
let items = page.query_selector_all(".product").await?;

for (i, item) in items.iter().enumerate() {
    // Get attributes
    if let Some(name) = item.get_attribute("data-name").await? {
        println!("Product {}: {}", i, name);
    }
    
    // Take screenshot
    let screenshot = item.screenshot().await?;
    // ... save screenshot
}
```

### Pattern 3: Network Monitoring

```rust
use cdp_core::NetworkEvent;
use std::sync::Arc;

// Enable network monitoring
page.enable_network_monitoring().await?;

// Register callback
page.on_network(Arc::new(|event| {
    match event {
        NetworkEvent::RequestWillBeSent { url, method, .. } => {
            println!("[{}] {}", method, url);
        }
        _ => {}
    }
})).await;

// Navigate (will trigger network events)
page.navigate("https://example.com").await?;
```

## Error Handling

CDP-Core uses `Result<T, CdpError>` for all operations:

```rust
use cdp_core::CdpError;

match page.query_selector("#button").await {
    Ok(Some(element)) => {
        // Element found
        element.click().await?;
    }
    Ok(None) => {
        // Element not found
        println!("Button not found!");
    }
    Err(e) => {
        // CDP error occurred
        eprintln!("Error: {}", e);
    }
}
```

## Best Practices

### 1. Use Arc for Shared Access

```rust
use std::sync::Arc;

let page = Arc::new(browser.new_page().await?);

// Can be cloned and shared across tasks
let page_clone = Arc::clone(&page);
tokio::spawn(async move {
    page_clone.navigate("https://example.com").await
});
```

### 2. Add Delays for Dynamic Content

```rust
// After navigation
page.navigate("https://example.com").await?;
tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

// After actions
button.click().await?;
tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
```

### 3. Always Check for None

```rust
// ❌ Don't do this
let element = page.query_selector("#button").await?.unwrap();

// ✅ Do this
let element = page.query_selector("#button").await?
    .ok_or_else(|| anyhow::anyhow!("Button not found"))?;

// Or this
if let Some(element) = page.query_selector("#button").await? {
    element.click().await?;
}
```

### 4. Clean Up Resources

```rust
// Explicit cleanup
page.cleanup().await?;

// Or let Drop handle it
drop(page);
drop(browser);
```

## Next Steps

- **[API Reference](API_REFERENCE.md)** - Quick reference for all methods
- **[Feature Guides](features/)** - Deep dive into specific features
- **[How-To Guides](howto/)** - Practical recipes and patterns
- **[Examples](../examples/)** - Runnable code examples

## Troubleshooting

### "Connection refused"

Make sure Chrome is running with `--remote-debugging-port=9222`

### "Element not found"

- Add delays after navigation
- Use `wait_for_selector()` instead of `query_selector()`
- Check if element is in an iframe

### "Screenshot failed"

- Ensure element is visible on page
- Check that element is not hidden
- Try scrolling element into view first

## Getting Help

- Check the [examples](../examples/) directory
- Review [feature documentation](features/)
- Search existing issues on GitHub
