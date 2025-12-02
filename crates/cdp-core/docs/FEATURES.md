# CDP-Core Features

Comprehensive guide to all features in CDP-Core, a Rust library for browser automation using the Chrome DevTools Protocol.

## Table of Contents

1. [Browser & Navigation](#browser--navigation)
2. [Element Interaction](#element-interaction)
3. [DOM Manipulation](#dom-manipulation)
4. [Network Control](#network-control)
5. [Screenshots](#screenshots)
6. [Wait Functions](#wait-functions)
7. [Input Simulation](#input-simulation)
8. [Storage Management](#storage-management)
9. [Page Session Management](#page-session-management)
10. [Emulation](#emulation)
11. [Accessibility](#accessibility)
12. [Tracing](#tracing)
13. [Domain Management](#domain-management)
14. [Resource Cleanup](#resource-cleanup)

---

## Browser & Navigation

Launch browsers, manage contexts, and navigate pages.

### Launch & Connect

```rust
use cdp_core::Browser;

// Launch browser
let browser = Browser::launcher().launch().await?;

// Connect to existing browser
let browser = Browser::connect("http://localhost:9222").await?;

// Create page
let page = browser.new_page().await?;
```

### Navigation

```rust
// Navigate to URL
page.navigate("https://example.com").await?;

// Navigation with wait
page.navigate("https://example.com").await?;
page.wait_for_navigation(None).await?;

// Back/Forward/Reload
page.go_back().await?;
page.go_forward().await?;
page.reload().await?;
```

### Wait for Navigation

Control how long to wait for navigation using `WaitUntil` enum:

```rust
use cdp_core::{WaitForNavigationOptions, WaitUntil};

// Wait for load event (all resources loaded)
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(30000),
    wait_until: Some(WaitUntil::Load),
})).await?;

// Wait for DOMContentLoaded (faster)
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(10000),
    wait_until: Some(WaitUntil::DOMContentLoaded),
})).await?;

// Wait for network idle (recommended for SPAs)
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(30000),
    wait_until: Some(WaitUntil::NetworkIdle2),  // ≤2 requests for 500ms
})).await?;
```

---

## Element Interaction

Find and interact with DOM elements.

### Finding Elements

```rust
// CSS selectors
let element = page.query_selector("#id").await?;
let elements = page.query_selector_all(".class").await?;

// XPath (automatically detected)
let element = page.query_selector("//button[@id='submit']").await?;
let element = page.query_selector("xpath://input[@name='username']").await?;

// Advanced XPath
let button = page.query_selector("//button[text()='Submit']").await?;
let items = page.query_selector_all("//div[contains(@class, 'item')]").await?;

// In frames
let frame = page.main_frame().await?;
let element = frame.query_selector(".button").await?;
```

### Interaction

```rust
if let Some(el) = page.query_selector("#button").await? {
    // Click
    el.click().await?;
    el.double_click().await?;
    
    // Type text
    el.type_text("Hello").await?;
    el.type_text_with_delay("Hello", 50, 150).await?;
    
    // Focus and clear
    el.focus().await?;
    el.clear().await?;
    
    // Get properties
    let text = el.text_content().await?;
    let attr = el.get_attribute("href").await?;
    let html = el.get_html().await?;
    let visible = el.is_visible().await?;
}
```

### Text Input

```rust
// Fast text input (insertText)
element.type_text("username@example.com").await?;

// Human-like typing with delays
element.type_text_with_delay("password123", 50, 150).await?;

// Page-level input (after manually focusing)
element.click().await?;
page.keyboard().type_text("Hello World").await?;
```

---

## DOM Manipulation

Advanced DOM operations including Shadow DOM, mutations, and frame management.

### Shadow DOM

Access Shadow DOM content (both open and closed):

```rust
// Access shadow root
if let Some(host) = page.query_selector("#shadow-host").await? {
    if let Some(shadow) = host.shadow_root().await? {
        // Query inside shadow DOM
        let button = shadow.query_selector("button").await?;
        button.click().await?;
        
        // Get shadow DOM HTML
        let html = shadow.get_html().await?;
    }
}

// Convenience method
if let Some(button) = host.query_selector_shadow("button").await? {
    button.click().await?;
}
```

### Cross-Frame Queries

Automatically search across all frames:

```rust
// Page query automatically searches all frames
let element = page.query_selector(".target").await?;
let elements = page.query_selector_all(".item").await?;

// Query specific frame
let main_frame = page.main_frame().await?;
let element = main_frame.query_selector(".target").await?;
```

### Mutation Monitoring

Monitor DOM changes in real-time:

```rust
use cdp_core::DomMutationEvent;
use std::sync::Arc;

// Enable mutation monitoring
page.enable_dom_mutations().await?;

// Register callback
page.on_dom_mutation(Arc::new(|event| {
    match event {
        DomMutationEvent::ChildNodeInserted { parent_node_id, .. } => {
            println!("Node inserted into {}", parent_node_id);
        }
        DomMutationEvent::AttributeModified { node_id, name, value } => {
            println!("Attribute {} = {} on node {}", name, value, node_id);
        }
        _ => {}
    }
})).await;
```

### Frame Management

```rust
// Get frames
let main_frame = page.main_frame().await?;
let all_frames = page.all_frames().await?;

// Frame snapshots
let snapshot = page.create_frame_snapshot(&frame_id, true).await?;
let all_snapshots = page.create_all_frames_snapshot(false).await?;
```

---

## Network Control

Comprehensive network control including request interception, response monitoring, and cookie management.

### Request Interception

```rust
use cdp_core::{NetworkInterceptor, RequestModification, HttpMethod};
use std::sync::Arc;

// Enable interception
page.enable_network_interception().await?;

// Intercept and modify requests
page.intercept_requests(Arc::new(|mut request| {
    // Modify headers
    request.headers.insert("Authorization".to_string(), "Bearer token123".to_string());
    
    // Continue with modifications
    Ok(RequestModification::Continue(request))
})).await;

// Block requests
page.intercept_requests(Arc::new(|request| {
    if request.url.contains("analytics") {
        return Ok(RequestModification::Abort);
    }
    Ok(RequestModification::Continue(request))
})).await;

// Mock responses
use cdp_core::ResponseMock;

page.intercept_requests(Arc::new(|request| {
    if request.url.contains("/api/data") {
        return Ok(RequestModification::Mock(ResponseMock {
            status: 200,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
            body: br#"{"mock": "data"}"#.to_vec(),
        }));
    }
    Ok(RequestModification::Continue(request))
})).await;
```

### Response Monitoring

Non-blocking response monitoring:

```rust
use cdp_core::NetworkEvent;

// Enable network monitoring
page.enable_network_monitoring().await?;

// Monitor all network events
page.on_network(Arc::new(|event| {
    match event {
        NetworkEvent::RequestWillBeSent { url, method, .. } => {
            println!("[{}] {}", method, url);
        }
        NetworkEvent::ResponseReceived { url, status, .. } => {
            println!("Response: {} - {}", status, url);
        }
        _ => {}
    }
})).await;

// Monitor specific responses
page.monitor_responses(
    vec!["https://api.example.com/data"],
    Arc::new(|response| {
        println!("API Response: {}", response.status);
        println!("Body: {:?}", response.body);
    })
).await?;
```

### Cookie Management

```rust
use cdp_core::{SetCookieParams, CookieSameSite};

// Get cookies
let cookies = page.cookies().await?;

// Set cookie
page.set_cookie(SetCookieParams {
    name: "session".to_string(),
    value: "token123".to_string(),
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    secure: Some(true),
    http_only: Some(true),
    same_site: Some(CookieSameSite::Lax),
    expires: None,
    ..Default::default()
}).await?;

// Delete cookie
page.delete_cookie("session", Some("example.com")).await?;

// Clear all cookies
page.clear_cookies().await?;
```

---

## Screenshots

Capture element and full page screenshots with DPR adaptation.

### Element Screenshots

```rust
let element = page.query_selector("#logo").await?
    .ok_or(anyhow!("Logo not found"))?;

let screenshot_base64 = element.screenshot().await?;

// Save to file
use base64::Engine;
let bytes = base64::engine::general_purpose::STANDARD.decode(&screenshot_base64)?;
std::fs::write("logo.png", bytes)?;
```

### Full Page Screenshots

```rust
// Capture entire page (including scroll area)
page.screenshot(true, Some("fullpage.png".into())).await?;

// Viewport only
page.screenshot(false, Some("viewport.png".into())).await?;

// With DPR control (recommended)
page.screenshot_with_options(
    true,                                // full_page
    Some("fullpage.png".into()),        // save_path
    true                                 // auto_resolve_dpr
).await?;
```

### DPR Adaptation

Automatic device pixel ratio handling for consistent screenshots:

```rust
// Auto-adapt to device DPR (prevents scaling issues)
page.screenshot_with_options(true, Some("page.png".into()), true).await?;

// Fixed DPR (use 1.0)
page.screenshot_with_options(true, Some("page.png".into()), false).await?;
```

---

## Wait Functions

Comprehensive waiting utilities for synchronizing with page states.

### Wait for Selector

```rust
use cdp_core::WaitForSelectorOptions;

// Simple wait
let element = page.wait_for_selector("#button", None).await?;

// With visibility requirement
let element = page.wait_for_selector(
    ".dynamic-content",
    Some(WaitForSelectorOptions {
        timeout_ms: Some(5000),
        visible: Some(true),
        hidden: Some(false),
    })
).await?;

// Wait for element to disappear
page.wait_for_selector_hidden(".loading-spinner", None).await?;
```

### Wait for Function

Wait for custom JavaScript conditions:

```rust
// Wait for page title
page.wait_for_function(
    "() => document.title === 'Loaded'",
    Some(5000),
    None
).await?;

// Wait for global variable
page.wait_for_function(
    "() => window.myApp && window.myApp.ready",
    Some(10000),
    None
).await?;

// Wait for element count
page.wait_for_function(
    "() => document.querySelectorAll('.item').length >= 10",
    Some(15000),
    None
).await?;
```

### Element State Checks

```rust
if let Some(element) = page.query_selector("#button").await? {
    // Check visibility
    if element.is_visible().await? {
        // Check if enabled
        if element.is_enabled().await? {
            // Check if clickable
            if element.is_clickable().await? {
                element.click().await?;
            }
        }
    }
}

// Wait for states
element.wait_for_visible(Some(5000)).await?;
element.wait_for_enabled(Some(3000)).await?;
element.wait_for_clickable(Some(5000)).await?;
```

---

## Input Simulation

Realistic keyboard and mouse input.

### Keyboard

```rust
use cdp_core::{KeyInput, KeyboardModifier};

let keyboard = page.keyboard();

// Type text
keyboard.type_text("Hello World").await?;
keyboard.type_text_with_delay("password", 50, 150).await?;

// Press single key
keyboard.press_key("Enter").await?;
keyboard.press_character('a').await?;

// Key combinations
keyboard.press_with_modifiers(
    "p",
    [KeyboardModifier::Control, KeyboardModifier::Shift],
).await?;

// Custom key properties
let keypad_enter = KeyInput::new("Enter")
    .with_code("NumpadEnter")
    .with_key_code(13)
    .from_keypad();
keyboard.press_key(keypad_enter).await?;
```

### Mouse

```rust
use cdp_core::{MouseButton, PressHoldOptions};
use std::time::Duration;

let mouse = page.mouse();

// Move and click
mouse.move_to(100.0, 200.0).await?;
mouse.click(100.0, 200.0, None).await?;
mouse.right_click(120.0, 260.0).await?;
mouse.double_click(100.0, 200.0, None).await?;

// Press and hold
mouse.press_and_hold(
    100.0, 400.0,
    MouseButton::Left,
    Duration::from_millis(800)
).await?;

// Press and hold until condition
let options = PressHoldOptions::default();
element.press_and_hold_until(options, || async {
    page.wait_for_function("() => window.appReady === true", Some(0), None)
        .await
        .map(|_| true)
}).await?;

// Hover over element
let position = element.hover().await?;
println!("Mouse at ({}, {})", position.viewport_x, position.viewport_y);
```

### File Upload

```rust
// Upload files to input element
element.upload_files(vec![
    "/path/to/file1.pdf",
    "/path/to/file2.jpg",
]).await?;
```

---

## Storage Management

Manage cookies, localStorage, sessionStorage, and page sessions.

### LocalStorage

```rust
use cdp_core::LocalStorageExt;

// Set item
page.local_storage_set("key", "value").await?;

// Get item
let value = page.local_storage_get("key").await?;

// Get all items
let items = page.local_storage_get_all().await?;

// Remove item
page.local_storage_remove("key").await?;

// Clear all
page.local_storage_clear().await?;
```

### SessionStorage

```rust
use cdp_core::SessionStorageExt;

// Same API as LocalStorage
page.session_storage_set("key", "value").await?;
let value = page.session_storage_get("key").await?;
page.session_storage_remove("key").await?;
page.session_storage_clear().await?;
```

---

## Page Session Management

Export and import complete page sessions (cookies + localStorage + sessionStorage).

### Session Export/Import

```rust
use cdp_core::PageSessionManager;
use std::path::PathBuf;

// Export session
let session = page.export_session().await?;
session.save_to_file(&PathBuf::from("session.json"))?;

// Import session to new page
let new_page = browser.new_page().await?;
new_page.import_session_from_file(&PathBuf::from("session.json")).await?;
```

### Session Snapshots

```rust
use cdp_core::PageSessionSnapshot;

// Create snapshot
let snapshot = page.snapshot().await?;

// Restore snapshot
page.restore(&snapshot).await?;

// Clone session to another page
page1.clone_session_to(&page2).await?;
```

### Use Cases

- Save login states
- Test environment reuse
- Session migration
- Session backup
- Cross-browser session sharing

---

## Emulation

Emulate devices, geolocation, and user agents.

### Device Emulation

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

// Desktop viewport
let desktop_config = EmulationConfig {
    viewport_width: Some(1920),
    viewport_height: Some(1080),
    device_scale_factor: Some(1.0),
    mobile: Some(false),
    ..Default::default()
};
page.emulate_device(desktop_config).await?;
```

### Geolocation

```rust
use cdp_core::Geolocation;

page.set_geolocation(Geolocation {
    latitude: 37.7749,
    longitude: -122.4194,
    accuracy: Some(100.0),
}).await?;

// Clear geolocation
page.emulation().clear_geolocation().await?;
```

### User Agent

```rust
// Simple user agent
page.set_user_agent("Mozilla/5.0 ...").await?;

// User agent with metadata
use cdp_core::UserAgentOverride;

page.set_user_agent_override(UserAgentOverride {
    user_agent: "Mozilla/5.0 ...".to_string(),
    accept_language: Some("en-US,en;q=0.9".to_string()),
    platform: Some("Win32".to_string()),
    ..Default::default()
}).await?;
```

### Timezone & Locale

```rust
// Set timezone
page.emulation().set_timezone("America/New_York").await?;

// Reset timezone
page.emulation().reset_timezone().await?;

// Set locale
page.emulation().set_locale("en-US").await?;
```

---

## Accessibility

Inspect the accessibility tree.

```rust
use cdp_core::AccessibilitySnapshotOptions;

// Get full accessibility snapshot
let snapshot = page.accessibility().snapshot(
    AccessibilitySnapshotOptions::default()
).await?;

// Get partial snapshot for specific element
let element = page.query_selector("#main").await?.unwrap();
let snapshot = page.accessibility().snapshot(
    AccessibilitySnapshotOptions::default().with_root(element)
).await?;

// Find nodes by role
let buttons = snapshot.find_by_role("button");

// Inspect node properties
for node in buttons {
    println!("Button: {:?}", node.name);
    println!("Role: {:?}", node.role);
    println!("Focusable: {:?}", node.focusable);
}
```

---

## Tracing

Capture performance traces.

```rust
use cdp_core::TracingStartOptions;

// Start tracing
page.start_tracing(TracingStartOptions {
    categories: vec!["devtools.timeline".to_string()],
    ..Default::default()
}).await?;

// Perform actions
page.navigate("https://example.com").await?;

// Stop and get trace
let trace = page.stop_tracing().await?;

// Save trace (can be opened in chrome://tracing)
std::fs::write("trace.json", trace.data)?;
```

---

## Domain Management

Unified management of Chrome DevTools Protocol domains.

### Domain Types

- **Required Domains** (auto-enabled): Page, Runtime, DOM, Network
- **Optional Domains**: Fetch, Performance, Keyboard, Mouse, CSS, Debugger, Profiler

### Domain Control

```rust
use cdp_core::DomainType;

// Check domain status
let is_enabled = page.domain_manager.is_enabled(DomainType::Fetch).await;

// Enable optional domains
page.domain_manager.enable_fetch_domain().await?;
page.domain_manager.enable_performance_domain().await?;

// Disable optional domains
page.domain_manager.disable_fetch_domain().await?;
page.domain_manager.disable_performance_domain().await?;

// Disable all domains (cleanup)
page.domain_manager.disable_all_domains().await?;
```

### Domain Configuration

```rust
use cdp_core::DomainConfig;

let config = DomainConfig {
    page_enable_file_chooser: true,
    fetch_handle_auth_requests: false,
    ..Default::default()
};

// Config is applied when creating DomainManager
// Typically used internally by Page
```

---

## Resource Cleanup

Properly clean up resources when done.

### Explicit Cleanup

```rust
// Clean up Page resources
page.cleanup().await?;

// Cleanup process:
// 1. Clear network monitors and interceptors
// 2. Clear callbacks
// 3. Clear Frame cache
// 4. Disable all CDP Domains
```

### RAII Automatic Cleanup

```rust
// Page implements Drop trait
// Cleanup happens automatically when Page goes out of scope
{
    let page = browser.new_page().await?;
    // ... use page ...
} // Page automatically cleaned up here

// For Arc<Page>, cleanup happens when last reference is dropped
let page = Arc::new(browser.new_page().await?);
// ... share page ...
drop(page); // Cleanup when last Arc is dropped
```

### Best Practices

- **Long-running programs**: Use explicit `cleanup()` for better error handling
- **Short scripts/tests**: Rely on RAII automatic cleanup
- **Resource-sensitive**: Clean up early, don't wait for scope end
- **Arc sharing**: Ensure no other references exist before cleanup

---

## Complete Example

```rust
use cdp_core::{Browser, WaitForSelectorOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch browser
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // Navigate and wait
    page.navigate("https://example.com").await?;
    page.wait_for_selector("h1", None).await?;
    
    // Interact with elements
    if let Some(button) = page.query_selector("#submit").await? {
        button.click().await?;
    }
    
    // Wait for dynamic content
    page.wait_for_function(
        "() => document.readyState === 'complete'",
        Some(5000),
        None
    ).await?;
    
    // Take screenshot
    page.screenshot(true, Some("result.png".into())).await?;
    
    // Cleanup
    page.cleanup().await?;
    
    Ok(())
}
```

---

## See Also

- [Getting Started](GETTING_STARTED.md) - Installation and first steps
- [API Reference](API_REFERENCE.md) - Quick reference for all APIs
- [Feature Guides](features/) - Detailed guides for specific features
- [How-To Guides](howto/) - Practical recipes and patterns
- [Examples](../examples/) - Runnable code examples
