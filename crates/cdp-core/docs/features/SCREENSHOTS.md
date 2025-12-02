# Screenshots

CDP-Core provides powerful screenshot capabilities for both elements and full pages.

## Features

- **Element Screenshots** - Capture specific elements
- **Full Page Screenshots** - Capture entire page including scroll area
- **DPR Adaptation** - Automatic device pixel ratio handling
- **Custom Viewports** - Control what gets captured

## Element Screenshots

### Basic Usage

```rust
let element = page.query_selector("#logo").await?
    .ok_or(anyhow!("Logo not found"))?;

let screenshot_base64 = element.screenshot().await?;

// Decode and save
use base64::Engine;
let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot_base64)?;
std::fs::write("logo.png", bytes)?;
```

### Batch Screenshots

```rust
let products = page.query_selector_all(".product").await?;

for (i, product) in products.iter().enumerate() {
    let screenshot = product.screenshot().await?;
    use base64::Engine;
    let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot)?;
    std::fs::write(format!("product_{}.png", i), bytes)?;
}
```

## Full Page Screenshots

### Basic Usage

```rust
// Capture entire page (including scroll area)
page.screenshot(true, Some("fullpage.png".into())).await?;

// Capture viewport only
page.screenshot(false, Some("viewport.png".into())).await?;
```

### With DPR Control

```rust
// Auto-adapt to device pixel ratio (recommended)
page.screenshot_with_options(
    true,                                // full_page
    Some("fullpage.png".into()),        // save_path
    true                                 // auto_resolve_dpr
).await?;

// Fixed DPR (use 1.0)
page.screenshot_with_options(
    true,
    Some("fullpage.png".into()),
    false  // No DPR adaptation
).await?;
```

## Advanced Usage

### Wait Before Screenshot

```rust
// Navigate and wait for content
page.navigate("https://example.com").await?;
page.wait_for_selector(".main-content", None).await?;

// Small delay for animations
tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

// Take screenshot
page.screenshot(true, Some("page.png".into())).await?;
```

### Screenshot Comparison

```rust
// Baseline screenshot
let before = page.screenshot(true, None).await?;

// Make changes
page.evaluate("document.body.style.backgroundColor = 'red'").await?;

// Updated screenshot
let after = page.screenshot(true, None).await?;

// Compare (you'll need an image diff library)
if before != after {
    println!("Page changed!");
}
```

### Responsive Screenshots

```rust
use cdp_core::EmulationConfig;

// Mobile viewport
let mobile_config = EmulationConfig {
    viewport_width: Some(375),
    viewport_height: Some(667),
    device_scale_factor: Some(2.0),
    mobile: Some(true),
    ..Default::default()
};

page.emulate_device(mobile_config).await?;
page.screenshot(true, Some("mobile.png".into())).await?;

// Desktop viewport
let desktop_config = EmulationConfig {
    viewport_width: Some(1920),
    viewport_height: Some(1080),
    device_scale_factor: Some(1.0),
    mobile: Some(false),
    ..Default::default()
};

page.emulate_device(desktop_config).await?;
page.screenshot(true, Some("desktop.png".into())).await?;
```

## Helper Function

```rust
use base64::Engine;

async fn save_screenshot(
    screenshot_base64: String,
    path: impl AsRef<std::path::Path>
) -> anyhow::Result<()> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&screenshot_base64)?;
    std::fs::write(path, bytes)?;
    Ok(())
}

// Usage
let screenshot = element.screenshot().await?;
save_screenshot(screenshot, "output.png").await?;
```

## Complete Example

```rust
use cdp_core::Browser;
use base64::Engine;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Navigate
    page.navigate("https://example.com").await?;
    page.wait_for_selector("h1", None).await?;
    
    // Element screenshots
    if let Some(header) = page.query_selector("header").await? {
        let screenshot = header.screenshot().await?;
        let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot)?;
        std::fs::write("header.png", bytes)?;
    }
    
    let products = page.query_selector_all(".product").await?;
    for (i, product) in products.iter().enumerate() {
        let screenshot = product.screenshot().await?;
        let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot)?;
        std::fs::write(format!("product_{}.png", i), bytes)?;
    }
    
    // Full page screenshot
    page.screenshot(true, Some("fullpage.png".into())).await?;
    
    println!("✓ Screenshots saved!");
    
    Ok(())
}
```

## Tips & Best Practices

### 1. Wait for Content

Always wait for dynamic content to load:

```rust
page.navigate(url).await?;
page.wait_for_selector(".content", None).await?;
tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
```

### 2. Handle Large Pages

For very long pages, be aware of memory usage:

```rust
// Full page might be large
let screenshot = page.screenshot(true, None).await?;
let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot)?;
println!("Screenshot size: {} KB", bytes.len() / 1024);
```

### 3. Element Visibility

Ensure elements are visible before screenshot:

```rust
if let Some(element) = page.query_selector("#logo").await? {
    if element.is_visible().await? {
        element.screenshot().await?;
    } else {
        println!("Element not visible");
    }
}
```

### 4. Use DPR Adaptation

Always use DPR adaptation for consistent results:

```rust
page.screenshot_with_options(true, Some("page.png".into()), true).await?;
```

## Troubleshooting

### Screenshot is blank

- Wait longer after navigation
- Check if element is actually visible
- Verify page has loaded

### Screenshot is too large/small

- Use `screenshot_with_options` with `auto_resolve_dpr = true`
- Check viewport settings

### Element not found

- Use `wait_for_selector` instead of `query_selector`
- Check if element is in iframe
- Verify CSS selector is correct

## See Also

- [API Reference](../API_REFERENCE.md#screenshots)
- [Element Interaction](ELEMENT_INTERACTION.md)
- [Examples](../../examples/comprehensive_test.rs)
