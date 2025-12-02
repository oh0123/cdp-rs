# Wait Functions

CDP-Core provides comprehensive waiting utilities for synchronizing with page states.

## Overview

Wait functions help you deal with:
- Dynamic content loading
- AJAX requests
- Animations
- Navigation events
- Custom conditions

## Wait for Selector

Wait for an element to appear in the DOM.

### Basic Usage

```rust
// Wait with defaults (30s timeout)
let element = page.wait_for_selector("#button", None).await?;
```

### With Options

```rust
use cdp_core::WaitForSelectorOptions;

let element = page.wait_for_selector(
    ".dynamic-content",
    Some(WaitForSelectorOptions {
        timeout_ms: Some(5000),    // 5 second timeout
        visible: Some(true),       // Element must be visible
        hidden: Some(false),       // Must not be hidden
    })
).await?;
```

### Wait for Hidden

```rust
// Wait for loading spinner to disappear
page.wait_for_selector_hidden(".loading-spinner", None).await?;

// With custom timeout
page.wait_for_selector_hidden(".modal", Some(3000)).await?;
```

## Wait for Navigation

Wait for page navigation to complete.

### Basic Usage

```rust
// Click link and wait for navigation
if let Some(link) = page.query_selector("a[href='/next']").await? {
    link.click().await?;
}
page.wait_for_navigation(None).await?;
```

### With Wait Conditions

```rust
use cdp_core::{WaitForNavigationOptions, WaitUntil};

// Wait for load event
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
    wait_until: Some(WaitUntil::NetworkIdle2),  // <= 2 requests for 500ms
})).await?;

// Wait for fully idle network
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(30000),
    wait_until: Some(WaitUntil::NetworkIdle0),  // 0 requests for 500ms
})).await?;
```

## Wait for Function

Wait for a custom JavaScript condition.

### Basic Usage

```rust
// Wait for page title to change
page.wait_for_function(
    "() => document.title === 'Loaded'",
    Some(5000),
    None
).await?;
```

### Advanced Conditions

```rust
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

// Wait for custom condition
page.wait_for_function(
    r#"
    () => {
        const el = document.querySelector('#status');
        return el && el.textContent === 'Complete';
    }
    "#,
    Some(5000),
    None
).await?;
```

## Wait for Timeout

Simple delay.

```rust
// Wait 2 seconds
page.wait_for_timeout(2000).await;

// Use tokio::time for more control
tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
```

## Wait for Network Idle

Wait for network requests to finish.

```rust
// Internal method, used by wait_for_navigation
// Can also be used directly:

// Wait for fully idle (0 requests for 500ms)
page.wait_for_network_idle(30000, 0).await?;

// Wait for almost idle (<= 2 requests for 500ms)
page.wait_for_network_idle(30000, 2).await?;
```

## Common Patterns

### Pattern 1: Wait After Navigation

```rust
page.navigate("https://example.com").await?;

// Option 1: Wait for specific element
page.wait_for_selector(".main-content", None).await?;

// Option 2: Wait for network idle
page.wait_for_navigation(Some(WaitForNavigationOptions {
    wait_until: Some(WaitUntil::NetworkIdle2),
    ..Default::default()
})).await?;

// Option 3: Fixed delay (least reliable)
page.wait_for_timeout(2000).await;
```

### Pattern 2: Wait for Dynamic Content

```rust
// Click button that loads content
if let Some(btn) = page.query_selector("#load-more").await? {
    btn.click().await?;
}

// Wait for new content
page.wait_for_selector(".new-item", None).await?;

// Or wait for item count
page.wait_for_function(
    "() => document.querySelectorAll('.item').length > 10",
    Some(5000),
    None
).await?;
```

### Pattern 3: Retry with Timeout

```rust
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(10),
    page.wait_for_selector("#button", None)
).await;

match result {
    Ok(Ok(element)) => {
        // Element found
        element.click().await?;
    }
    Ok(Err(e)) => {
        eprintln!("Error: {}", e);
    }
    Err(_) => {
        eprintln!("Timeout after 10 seconds");
    }
}
```

### Pattern 4: Sequential Waits

```rust
// Wait for multiple conditions in sequence
page.navigate("https://example.com").await?;

page.wait_for_selector(".header", None).await?;
page.wait_for_selector(".content", None).await?;
page.wait_for_selector(".footer", None).await?;

println!("Page fully loaded!");
```

## Complete Example

```rust
use cdp_core::{Browser, WaitForSelectorOptions, WaitForNavigationOptions, WaitUntil};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // 1. Navigate and wait for load
    page.navigate("https://example.com").await?;
    page.wait_for_navigation(Some(WaitForNavigationOptions {
        wait_until: Some(WaitUntil::NetworkIdle2),
        ..Default::default()
    })).await?;
    
    // 2. Wait for specific element (with visibility check)
    let button = page.wait_for_selector(
        "#dynamic-button",
        Some(WaitForSelectorOptions {
            visible: Some(true),
            timeout_ms: Some(5000),
            ..Default::default()
        })
    ).await?;
    
    // 3. Click and wait for modal
    button.click().await?;
    page.wait_for_selector(".modal", None).await?;
    
    // 4. Wait for custom condition
    page.wait_for_function(
        "() => document.querySelector('.modal').classList.contains('open')",
        Some(3000),
        None
    ).await?;
    
    // 5. Wait for modal to close
    page.wait_for_selector_hidden(".modal", Some(5000)).await?;
    
    println!("✓ All waits completed successfully!");
    
    Ok(())
}
```

## Tips & Best Practices

### 1. Choose the Right Wait

- **Fast pages**: `WaitUntil::DOMContentLoaded`
- **SPAs**: `WaitUntil::NetworkIdle2`
- **API-heavy**: `WaitUntil::NetworkIdle0`
- **Specific element**: `wait_for_selector`

### 2. Set Appropriate Timeouts

```rust
// Short timeout for fast operations
page.wait_for_selector(".button", Some(WaitForSelectorOptions {
    timeout_ms: Some(2000),
    ..Default::default()
})).await?;

// Longer timeout for slow API calls
page.wait_for_navigation(Some(WaitForNavigationOptions {
    timeout_ms: Some(60000),
    ..Default::default()
})).await?;
```

### 3. Avoid Fixed Delays

```rust
// ❌ Bad: Fixed delay
page.wait_for_timeout(5000).await;

// ✅ Good: Wait for condition
page.wait_for_selector(".content", None).await?;
```

### 4. Use Visibility Checks

```rust
// Wait for element to be both present AND visible
page.wait_for_selector(
    ".modal",
    Some(WaitForSelectorOptions {
        visible: Some(true),
        ..Default::default()
    })
).await?;
```

## Troubleshooting

### Timeout Errors

```rust
// Increase timeout
page.wait_for_selector(".slow-element", Some(WaitForSelectorOptions {
    timeout_ms: Some(30000),  // 30 seconds
    ..Default::default()
})).await?;

// Or check if element exists
match page.query_selector(".element").await? {
    Some(el) => println!("Found immediately"),
    None => page.wait_for_selector(".element", None).await?,
}
```

### Element Never Appears

- Check CSS selector is correct
- Verify element isn't in iframe
- Check if element is dynamically added
- Use browser DevTools to inspect

### NetworkIdle Never Resolves

- Some sites have constant polling
- Use `NetworkIdle2` instead of `NetworkIdle0`
- Or use selector-based waiting

## See Also

- [API Reference](../API_REFERENCE.md#wait-functions)
- [Element Interaction](ELEMENT_INTERACTION.md)
- [Navigation](BROWSER_NAVIGATION.md)
