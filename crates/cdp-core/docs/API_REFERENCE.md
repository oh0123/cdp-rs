# CDP-Core API Quick Reference

Quick reference for all major APIs in CDP-Core.

## Table of Contents

- [Browser](#browser)
- [Page](#page)
- [Element](#element)
- [Frame](#frame)
- [Network](#network)
- [Input](#input)
- [Storage](#storage)
- [Emulation](#emulation)
- [Wait Functions](#wait-functions)

## Browser

###Launch & Connect

```rust
// Launch new browser
let browser = Browser::launcher().launch().await?;

// Connect to existing browser
let browser = Browser::connect("http://localhost:9222").await?;

// Create new page
let page = browser.new_page().await?;

// List all pages
let pages = browser.pages().await?;
```

## Page

### Navigation

```rust
// Navigate to URL
page.navigate("https://example.com").await?;

// Go back/forward
page.go_back().await?;
page.go_forward().await?;

// Reload
page.reload().await?;
```

### Element Queries

```rust
// Query single element
let element = page.query_selector("#id").await?;

// Query all elements
let elements = page.query_selector_all(".class").await?;

// XPath
let element = page.query_xpath("//button[@id='submit']").await?;
```

### Screenshots

```rust
// Full page screenshot
page.screenshot(true, Some("page.png".into())).await?;

// Viewport screenshot
page.screenshot(false, Some("viewport.png".into())).await?;

// With options (DPR control)
page.screenshot_with_options(true, Some("page.png".into()), true).await?;
```

### JavaScript Evaluation

```rust
// Evaluate expression
let result = page.evaluate("document.title").await?;

// Call function with arguments
let result = page.call_function(
    "function(a, b) { return a + b; }",
    vec![json!(1), json!(2)]
).await?;
```

### Network

```rust
// Enable network monitoring
page.enable_network_monitoring().await?;

// Register network callback
page.on_network(Arc::new(|event| {
    // Handle network events
})).await;

// Get active request count
let count = page.get_inflight_requests_count();
```

## Element

### Interaction

```rust
// Click
element.click().await?;

// Double click
element.double_click().await?;

// Type text
element.type_text("Hello").await?;
element.type_text_with_delay("Hello", 50, 150).await?;

// Focus
element.focus().await?;
```

### Properties

```rust
// Get text content
let text = element.text_content().await?;

// Get attribute
let value = element.get_attribute("href").await?;

// Get HTML
let html = element.get_html().await?;

// Check visibility
let visible = element.is_visible().await?;
```

### Screenshot

```rust
// Element screenshot
let base64_png = element.screenshot().await?;

// Decode and save
use base64::Engine;
let bytes = base64::engine::general_purpose::STANDARD.decode(&base64_png)?;
std::fs::write("element.png", bytes)?;
```

### Bounding Box

```rust
// Get element position and size
let bbox = element.bounding_box().await?;
println!("x:{} y:{} width:{} height:{}", bbox.x, bbox.y, bbox.width, bbox.height);
```

## Frame

### Frame Access

```rust
// Get main frame
let main_frame = page.main_frame().await?;

// Get all frames
let frames = page.all_frames().await?;

// Get specific frame
let frame = page.get_frame("frame-id").await;
```

### Frame Operations

```rust
// Evaluate in frame
let result = frame.evaluate("document.title").await?;

// Query in frame
let element = frame.query_selector(".button").await?;

// Check if detached
let detached = frame.is_detached().await;
```

## Network

### Interception

```rust
use cdp_core::{NetworkInterceptor, RequestModification};

// Enable interception
page.enable_network_interception().await?;

// Modify requests
page.intercept_requests(Arc::new(|mut request| {
    request.headers.insert("Custom-Header".to_string(), "value".to_string());
    Ok(RequestModification::Continue(request))
})).await;
```

### Cookies

```rust
use cdp_core::{Cookie, SetCookieParams, CookieSameSite};

// Get cookies
let cookies = page.cookies().await?;

// Set cookie
page.set_cookie(SetCookieParams {
    name: "session".to_string(),
    value: "token123".to_string(),
    domain: Some("example.com".to_string()),
    ..Default::default()
}).await?;

// Delete cookie
page.delete_cookie("session", Some("example.com")).await?;
```

### Response Monitoring

```rust
// Monitor specific responses
page.monitor_responses(
    vec!["https://api.example.com/data"],
    Arc::new(|response| {
        println!("Status: {}", response.status);
        println!("Headers: {:?}", response.headers);
    })
).await?;
```

## Input

### Keyboard

```rust
// Type text
page.keyboard().type_text("Hello World").await?;

// Press key
page.keyboard().press_key("Enter").await?;

// Key combination
page.keyboard().down("Control").await?;
page.keyboard().press_key("a").await?;
page.keyboard().up("Control").await?;
```

### Mouse

```rust
// Move mouse
page.mouse().move_to(100.0, 200.0).await?;

// Click at position
page.mouse().click(100.0, 200.0, None).await?;

// Drag and drop
page.mouse().move_to(start_x, start_y).await?;
page.mouse().down(None).await?;
page.mouse().move_to(end_x, end_y).await?;
page.mouse().up(None).await?;
```

## Storage

### LocalStorage

```rust
use cdp_core::LocalStorageExt;

// Set item
page.local_storage_set("key", "value").await?;

// Get item
let value = page.local_storage_get("key").await?;

// Remove item
page.local_storage_remove("key").await?;

// Clear all
page.local_storage_clear().await?;
```

### SessionStorage

```rust
use cdp_core::SessionStorageExt;

// Set item
page.session_storage_set("key", "value").await?;

// Get item
let value = page.session_storage_get("key").await?;
```

## Emulation

### Device Emulation

```rust
use cdp_core::EmulationConfig;

let config = EmulationConfig {
    viewport_width: Some(375),
    viewport_height: Some(667),
    device_scale_factor: Some(2.0),
    mobile: Some(true),
    ..Default::default()
};

page.emulate_device(config).await?;
```

### Geolocation

```rust
use cdp_core::Geolocation;

page.set_geolocation(Geolocation {
    latitude: 37.7749,
    longitude: -122.4194,
    accuracy: Some(100.0),
}).await?;
```

### User Agent

```rust
page.set_user_agent("Mozilla/5.0 ...").await?;
```

## Wait Functions

### Wait for Selector

```rust
use cdp_core::WaitForSelectorOptions;

// Simple wait
let element = page.wait_for_selector("#button", None).await?;

// With options
let element = page.wait_for_selector(
    ".dynamic-content",
    Some(WaitForSelectorOptions {
        timeout_ms: Some(5000),
        visible: Some(true),
        hidden: Some(false),
    })
).await?;
```

### Wait for Navigation

```rust
use cdp_core::{WaitForNavigationOptions, WaitUntil};

// Wait for load
page.wait_for_navigation(None).await?;

// Wait for network idle
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(30000),
    wait_until: Some(WaitUntil::NetworkIdle2),
})).await?;
```

### Wait for Function

```rust
// Wait for custom condition
page.wait_for_function(
    "() => document.readyState === 'complete'",
    Some(5000),
    None
).await?;
```

### Wait for Timeout

```rust
// Simple delay
page.wait_for_timeout(2000).await;
```

## Common Patterns

### Pattern: Safe Element Handling

```rust
if let Some(element) = page.query_selector("#button").await? {
    element.click().await?;
} else {
    println!("Element not found");
}
```

### Pattern: Batch Operations

```rust
let items = page.query_selector_all(".item").await?;
for item in items {
    let text = item.text_content().await?;
    println!("{}", text);
}
```

### Pattern: Error Recovery

```rust
match page.query_selector("#button").await {
    Ok(Some(btn)) => btn.click().await?,
    Ok(None) => println!("Not found"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## See Also

- [Getting Started Guide](GETTING_STARTED.md)
- [Feature Guides](features/)
- [Examples](../examples/)
