# CDP-Core API Quick Reference

Quick reference for the current `cdp-core` public API.

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

### Launch & Connect

```rust
// Launch a new browser.
let browser = Browser::launcher().launch().await?;

// Connect to an existing browser endpoint.
let browser = Browser::launcher()
    .connect_to_existing("http://localhost:9222")
    .launch()
    .await?;

// Create a new page in the default context.
let page = browser.new_page().await?;

// List pages tracked by this browser connection.
let pages = browser.pages().await;

// Create an isolated browser context.
let context = browser
    .new_context(BrowserContextOptions::default())
    .await?;
let page = context.new_page().await?;

// List tracked contexts.
let contexts = browser.contexts().await;
```

### Browser Info

```rust
let version = browser.version().await?;
let args = browser.command_line().await?;
```

## Page

### Navigation

```rust
// Navigate to URL.
page.navigate("https://example.com").await?;

// Go back / forward through navigation history.
page.go_back().await?;
page.go_forward().await?;

// Reload with default options.
page.reload(cdp_core::ReloadOptions::default()).await?;

// Reload while ignoring browser cache.
page
    .reload(cdp_core::ReloadOptions::default().with_ignore_cache(true))
    .await?;

// Use browser navigation history directly.
let history = page.get_navigation_history().await?;
if let Some(entry) = history.entries.last() {
    page.navigate_to_history_entry(entry.id).await?;
}
```

### Targets

```rust
// Native Target.getTargets() wrapper.
let targets = page.get_targets().await?;

// Page-level alias for page-like targets.
let tabs = page.get_tabs().await?;

// Enable target lifecycle events.
page.set_discover_targets(true, None).await?;
```

### Events

```rust
use cdp_core::events;
use futures_util::StreamExt;

// Continuous JavaScript exception stream.
let mut exceptions = page.on::<events::runtime::ExceptionThrownEvent>();
if let Some(event) = exceptions.next().await {
    println!("Exception: {}", event.params.exception_details.text);
}

// One-shot event wait. The subscription is created immediately.
let console_event = page.wait_for_event::<events::runtime::ConsoleAPICalledEvent>(Some(5000));
page.main_frame()
    .await?
    .evaluate("console.log('ready')")
    .await?;
let event = console_event.await?;

// Page lifecycle events require explicit enablement.
page.set_lifecycle_events_enabled(true).await?;
let mut lifecycle = page.on::<events::page::LifecycleEventEvent>();

// Browser diagnostic log entries require the Log domain.
// For console.log/warn/error, prefer Runtime.consoleAPICalled.
page.domain_manager.enable_log_domain().await?;
let mut logs = page.on::<events::log::EntryAddedEvent>();
```

### Element Queries

```rust
// Query single element.
let element = page.query_selector("#id").await?;

// Query all elements.
let elements = page.query_selector_all(".class").await?;

// XPath.
let element = page.query_xpath("//button[@id='submit']").await?;
let elements = page.query_xpath_all("//a").await?;
```

### Screenshots

```rust
use cdp_core::PageScreenshotOptions;

// Full page screenshot.
let path = page
    .screenshot(PageScreenshotOptions::default().full_page().save_to("page.png"))
    .await?;

// Viewport screenshot.
let path = page
    .screenshot(PageScreenshotOptions::default().viewport().save_to("viewport.png"))
    .await?;

// With DPR control.
let path = page
    .screenshot(
        PageScreenshotOptions::default()
            .full_page()
            .save_to("page.png")
            .auto_resolve_dpr(false),
    )
    .await?;
```

### JavaScript Evaluation

```rust
// Evaluate in the main frame.
let result = page.evaluate("document.title").await?;

// Call a function in the main frame.
let result = page
    .call_function(
        "function(a, b) { return a + b; }",
        vec![serde_json::json!(1), serde_json::json!(2)],
    )
    .await?;

// Evaluate in a specific frame.
let frame = page.main_frame().await?;
let result = frame.evaluate("document.body.innerText").await?;
```

### Network Events

```rust
use std::sync::Arc;

// Explicitly enable the Network domain for monitoring.
page.enable_network_monitoring().await?;

// Register network callbacks for cdp-core's network monitor.
page.on_network(Arc::new(|event| {
    println!("Network event: {:?}", event);
})).await;

// Get active request count tracked by the network monitor.
let count = page.get_inflight_requests_count();
```

## Element

### Interaction

```rust
// Click.
element.click().await?;

// Focus.
element.focus().await?;

// Double click.
element.double_click().await?;

// Type text into the focused element.
element.type_text("Hello").await?;
element.type_text_with_delay("Hello", 50, 150).await?;

// Hover / move mouse to element center.
let position = element.hover().await?;
```

### Properties

```rust
// Get text content.
let text = element.text_content().await?;

// Get attribute.
let value = element.get_attribute("href").await?;

// Get HTML.
let html = element.get_html().await?;

// Check state.
let visible = element.is_visible().await?;
let enabled = element.is_enabled().await?;
let clickable = element.is_clickable().await?;

// Get viewport-relative bounds.
let bbox = element.bounding_box().await?;
println!("x:{} y:{} width:{} height:{}", bbox.x, bbox.y, bbox.width, bbox.height);
```

### Screenshot

```rust
use cdp_core::ElementScreenshotOptions;

// Element screenshot. The API saves the image and returns the saved path.
let path = element
    .screenshot(ElementScreenshotOptions::default().save_to("element.png"))
    .await?;

// Capture a content-only box.
let path = element
    .screenshot(
        ElementScreenshotOptions::default()
            .content_box()
            .save_to("element-content.png"),
    )
    .await?;
```

### Shadow DOM

```rust
if let Some(shadow_root) = element.shadow_root().await? {
    let inner = shadow_root.query_selector(".inner").await?;
}

let inner = element.query_selector_shadow(".inner").await?;
```

## Frame

### Frame Access

```rust
// Get main frame.
let main_frame = page.main_frame().await?;

// Get all frames.
let frames = page.all_frames().await?;

// Get specific frame.
let frame = page.get_frame("frame-id").await;
```

### Frame Operations

```rust
// Evaluate in frame.
let result = frame.evaluate("document.title").await?;

// Query in frame.
let element = frame.query_selector(".button").await?;

// Check if detached.
let detached = frame.is_detached().await;
```

## Network

### Request Interception

```rust
use cdp_core::{InterceptRequestAction, NetworkInterceptor, RequestModification};

// Callback-driven interception helper.
page.intercept_requests(|request| {
    if request.url.contains("/api/") {
        InterceptRequestAction::Modify(
            RequestModification::default().with_header("x-cdp-core", "true"),
        )
    } else {
        InterceptRequestAction::Continue
    }
}).await?;

// Lower-level request interception is still available.
page.enable_request_interception(vec!["*".to_string()]).await?;
page.disable_request_interception().await?;
```

### Request Helpers

```rust
use cdp_core::RequestInterceptorExt;

page.intercept_all_requests().await?;
page.intercept_requests_matching("*://example.com/*").await?;
page.block_images().await?;
page.block_stylesheets().await?;
```

### Network Control

```rust
use cdp_core::NetworkControl;

page.clear_browser_cache().await?;
page.set_cache_disabled(true).await?;
page.set_bypass_service_worker(true).await?;
page.block_urls(["*.png", "*.jpg"]).await?;
```

### Cookies

```rust
use cdp_core::{CookieManager, CookieSameSite, DeleteCookieOptions, SetCookieParams};

// Get cookies.
let cookies = page.get_cookies(None).await?;

// Set cookie.
page.set_cookie(SetCookieParams {
    name: "session".to_string(),
    value: "token123".to_string(),
    domain: Some("example.com".to_string()),
    same_site: Some(CookieSameSite::Lax),
    ..Default::default()
}).await?;

// Delete cookie.
page
    .delete_cookies(DeleteCookieOptions::new("session").with_domain("example.com"))
    .await?;
```

### Response Monitoring

```rust
// Monitor responses using a URL filter.
page.monitor_responses(
    |url| url.contains("https://api.example.com/data"),
    |response| {
        println!("Status: {}", response.status_code);
        println!("Headers: {:?}", response.headers);
    },
).await?;

// Or use a substring helper.
page.monitor_responses_matching("api.example.com", |response| {
    println!("Status: {}", response.status_code);
}).await?;
```

## Input

### Keyboard

```rust
// Insert text quickly.
page.keyboard().insert_text("Hello World").await?;

// Type text with randomized delay.
page.keyboard().type_text_with_delay("Hello", 50, 150).await?;

// Press key.
page.keyboard().press_key("Enter").await?;

// Key combination.
page.keyboard().key_down("Control").await?;
page.keyboard().press_key("a").await?;
page.keyboard().key_up("Control").await?;
```

### Mouse

```rust
use cdp_core::MouseClickOptions;
use cdp_core::input::mouse::DragOptions;

// Move mouse.
page.mouse().move_to(100.0, 200.0).await?;

// Click at position.
page.mouse().click(100.0, 200.0, MouseClickOptions::default()).await?;

// Drag and drop.
page
    .mouse()
    .drag_to(start_x, start_y, end_x, end_y, DragOptions::default())
    .await?;
```

## Storage

### LocalStorage

```rust
use cdp_core::LocalStorageExt;

// Set item.
page.set_local_storage_item("key", "value").await?;

// Get item.
let value = page.get_local_storage_item("key").await?;

// Remove item.
page.remove_local_storage_item("key").await?;

// Clear all.
page.clear_local_storage().await?;
```

### SessionStorage

```rust
use cdp_core::SessionStorageExt;

// Set item.
page.set_session_storage_item("key", "value").await?;

// Get item.
let value = page.get_session_storage_item("key").await?;

// Remove item.
page.remove_session_storage_item("key").await?;

// Clear all.
page.clear_session_storage().await?;
```

## Emulation

### Apply Emulation Config

```rust
use cdp_core::{EmulationConfig, Geolocation, UserAgentOverride};

let config = EmulationConfig::default()
    .with_geolocation(Geolocation::new(37.7749, -122.4194).with_accuracy(100.0))
    .with_user_agent(UserAgentOverride::new("Mozilla/5.0 ..."));

page.emulation().apply_config(&config).await?;

// Or use the page-level shortcut.
page.emulate_device(config).await?;
```

### Individual Overrides

```rust
use cdp_core::{Geolocation, UserAgentOverride};

page
    .set_geolocation(Geolocation::new(37.7749, -122.4194).with_accuracy(100.0))
    .await?;

page
    .set_user_agent(UserAgentOverride::new("Mozilla/5.0 ..."))
    .await?;

page.emulation().clear_geolocation().await?;
```

## Wait Functions

### Wait for Selector

```rust
use cdp_core::WaitForSelectorOptions;

// Simple wait.
let element = page.wait_for_selector("#button", None).await?;

// With options.
let element = page.wait_for_selector(
    ".dynamic-content",
    Some(
        WaitForSelectorOptions::default()
            .with_timeout_ms(5000)
            .with_visible(true),
    ),
).await?;
```

### Wait for Navigation

```rust
use cdp_core::{WaitForNavigationOptions, WaitUntil};

// Wait for default network-idle condition.
page.wait_for_navigation(None).await?;

// Wait for network idle.
page
    .wait_for_navigation(Some(
        WaitForNavigationOptions::default()
            .with_timeout_ms(30000)
            .with_wait_until(WaitUntil::NetworkIdle2),
    ))
    .await?;
```

### Wait for Function

```rust
// Wait for custom condition.
page.wait_for_function(
    "() => document.readyState === 'complete'",
    Some(5000),
    None,
).await?;
```

### Wait for Timeout

```rust
// Simple delay.
page.wait_for_timeout(2000).await;
```

## Common Patterns

### Safe Element Handling

```rust
if let Some(element) = page.query_selector("#button").await? {
    element.click().await?;
} else {
    println!("Element not found");
}
```

### Batch Operations

```rust
let items = page.query_selector_all(".item").await?;
for item in items {
    let text = item.text_content().await?;
    println!("{}", text);
}
```

### Error Recovery

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
- [Manual Browser Example Suite](MANUAL_EXAMPLES.md)
- [Manual Example Coverage Gap List](EXAMPLE_COVERAGE_GAPS.md)
- [Examples](../examples/)
