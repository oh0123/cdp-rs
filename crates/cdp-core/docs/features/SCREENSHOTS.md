# Screenshots

CDP-Core can capture full-page, viewport, and element screenshots. Screenshot APIs save the image to disk and return the saved file path.

## Features

- **Element screenshots** - Capture a specific element
- **Full-page screenshots** - Capture the entire page including the scroll area
- **Viewport screenshots** - Capture only the visible viewport
- **DPR adaptation** - Automatically adjust for device pixel ratio
- **Chainable options** - Configure screenshots without boolean-heavy call sites

## Element Screenshots

### Basic Usage

```rust
let element = page.query_selector("#logo").await?
    .ok_or(anyhow!("Logo not found"))?;

let path = element
    .screenshot(cdp_core::ElementScreenshotOptions::default())
    .await?;

println!("Saved element screenshot: {}", path);
```

### Custom Path

```rust
let path = element
    .screenshot(
        cdp_core::ElementScreenshotOptions::default()
            .save_to("screenshots/logo.png"),
    )
    .await?;
```

### Bounding Box

```rust
// Default: border box, usually best for rounded elements.
let button = page.wait_for_selector("button.rounded", None).await?;

button
    .screenshot(
        cdp_core::ElementScreenshotOptions::default()
            .border_box()
            .save_to("button.png"),
    )
    .await?;

// Capture only the content area.
button
    .screenshot(
        cdp_core::ElementScreenshotOptions::default()
            .content_box()
            .save_to("button-content.png"),
    )
    .await?;

// Include surrounding margin.
button
    .screenshot(
        cdp_core::ElementScreenshotOptions::default()
            .margin_box()
            .save_to("button-margin.png"),
    )
    .await?;
```

### Batch Screenshots

```rust
let products = page.query_selector_all(".product").await?;

for (i, product) in products.iter().enumerate() {
    let path = product
        .screenshot(
            cdp_core::ElementScreenshotOptions::default()
                .save_to(format!("product_{}.png", i)),
        )
        .await?;

    println!("Saved product screenshot: {}", path);
}
```

## Page Screenshots

### Basic Usage

```rust
// Capture entire page including scroll area.
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("fullpage.png"),
    )
    .await?;

// Capture viewport only.
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .viewport()
            .save_to("viewport.png"),
    )
    .await?;
```

### DPR Control

```rust
// Auto-adapt to device pixel ratio. This is the default and recommended behavior.
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("fullpage.png"),
    )
    .await?;

// Fixed scale = 1.0.
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("fullpage-fixed-dpr.png")
            .auto_resolve_dpr(false),
    )
    .await?;
```

## Advanced Usage

### Wait Before Screenshot

```rust
page.navigate("https://example.com").await?;
page.wait_for_selector(".main-content", None).await?;

tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("page.png"),
    )
    .await?;
```

### Screenshot Comparison

```rust
let before = page
    .screenshot(cdp_core::PageScreenshotOptions::default().full_page())
    .await?;

page.evaluate("document.body.style.backgroundColor = 'red'").await?;

let after = page
    .screenshot(cdp_core::PageScreenshotOptions::default().full_page())
    .await?;

if before != after {
    println!("Page changed!");
}
```

### Responsive Screenshots

```rust
use cdp_core::EmulationConfig;

let mobile_config = EmulationConfig {
    viewport_width: Some(375),
    viewport_height: Some(667),
    device_scale_factor: Some(2.0),
    mobile: Some(true),
    ..Default::default()
};

page.emulate_device(mobile_config).await?;
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("mobile.png"),
    )
    .await?;

let desktop_config = EmulationConfig {
    viewport_width: Some(1920),
    viewport_height: Some(1080),
    device_scale_factor: Some(1.0),
    mobile: Some(false),
    ..Default::default()
};

page.emulate_device(desktop_config).await?;
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("desktop.png"),
    )
    .await?;
```

## Complete Example

```rust
use cdp_core::{Browser, ElementScreenshotOptions, PageScreenshotOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    page.navigate("https://example.com").await?;
    page.wait_for_selector("h1", None).await?;

    if let Some(header) = page.query_selector("header").await? {
        header
            .screenshot(
                ElementScreenshotOptions::default()
                    .save_to("header.png"),
            )
            .await?;
    }

    let products = page.query_selector_all(".product").await?;
    for (i, product) in products.iter().enumerate() {
        product
            .screenshot(
                ElementScreenshotOptions::default()
                    .save_to(format!("product_{}.png", i)),
            )
            .await?;
    }

    page
        .screenshot(
            PageScreenshotOptions::default()
                .full_page()
                .save_to("fullpage.png"),
        )
        .await?;

    println!("Screenshots saved");

    Ok(())
}
```

## Tips

### Wait for Content

```rust
page.navigate(url).await?;
page.wait_for_selector(".content", None).await?;
tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
```

### Handle Large Pages

```rust
let path = page
    .screenshot(cdp_core::PageScreenshotOptions::default().full_page())
    .await?;

let size = std::fs::metadata(&path)?.len();
println!("Screenshot size: {} KB", size / 1024);
```

### Element Visibility

```rust
if let Some(element) = page.query_selector("#logo").await? {
    if element.is_visible().await? {
        element
            .screenshot(cdp_core::ElementScreenshotOptions::default())
            .await?;
    }
}
```

### Use DPR Adaptation

```rust
page
    .screenshot(
        cdp_core::PageScreenshotOptions::default()
            .full_page()
            .save_to("page.png"),
    )
    .await?;
```

## Troubleshooting

### Screenshot is blank

- Wait longer after navigation
- Check if the element is visible
- Verify that the page has loaded

### Screenshot is too large or small

- Keep `auto_resolve_dpr` enabled unless fixed DPR is required
- Check viewport and emulation settings

### Element not found

- Use `wait_for_selector` instead of `query_selector`
- Check if the element is in an iframe
- Verify that the CSS selector is correct

## See Also

- [API Reference](../API_REFERENCE.md#screenshots)
- [Element Interaction](ELEMENT_INTERACTION.md)
- [Output capture example](../../examples/api_output_capture.rs)
