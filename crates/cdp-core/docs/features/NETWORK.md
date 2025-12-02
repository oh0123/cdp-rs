# Network Features

CDP-Core provides comprehensive network control including request interception, response monitoring, and cookie management.

## Features

- **Request Interception** - Modify or block requests
- **Response Monitoring** - Capture response bodies
- **Cookie Management** - Get, set, delete cookies
- **Network Events** - Monitor all network activity

## Request Interception

### Enable Interception

```rust
use cdp_core::{NetworkInterceptor, RequestModification, HttpMethod};
use std::sync::Arc;

// Enable interception
page.enable_network_interception().await?;

// Intercept and modify requests
page.intercept_requests(Arc::new(|mut request| {
    // Modify headers
    request.headers.insert(
        "Authorization".to_string(),
        "Bearer token123".to_string()
    );
    
    // Continue with modifications
    Ok(RequestModification::Continue(request))
})).await;
```

### Block Requests

```rust
page.intercept_requests(Arc::new(|request| {
    // Block analytics
    if request.url.contains("analytics") {
        return Ok(RequestModification::Abort);
    }
    
    // Block specific methods
    if request.method == HttpMethod::Post {
        return Ok(RequestModification::Abort);
    }
    
    Ok(RequestModification::Continue(request))
})).await;
```

### Mock Responses

```rust
use cdp_core::ResponseMock;

page.intercept_requests(Arc::new(|request| {
    if request.url.contains("/api/data") {
        return Ok(RequestModification::Mock(ResponseMock {
            status: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string())
            ],
            body: br#"{"mock": "data"}"#.to_vec(),
        }));
    }
    
    Ok(RequestModification::Continue(request))
})).await;
```

## Response Monitoring

### Monitor Specific URLs

```rust
// Monitor API responses
page.monitor_responses(
    vec!["https://api.example.com/data"],
    Arc::new(|response| {
        println!("Status: {}", response.status);
        println!("Body: {}", String::from_utf8_lossy(&response.body));
    })
).await?;
```

### Monitor All Responses

```rust
page.enable_network_monitoring().await?;

page.on_network(Arc::new(|event| {
    match event {
        NetworkEvent::ResponseReceived { request_id, url, status, .. } => {
            println!("[{}] {} - {}", status, request_id, url);
        }
        NetworkEvent::LoadingFailed { request_id, error_text, .. } => {
            eprintln!("Failed [{}]: {}", request_id, error_text);
        }
        _ => {}
    }
})).await;
```

## Cookie Management

### Get Cookies

```rust
// Get all cookies
let cookies = page.cookies().await?;
for cookie in cookies {
    println!("{} = {}", cookie.name, cookie.value);
}

// Get cookies for specific URL
let cookies = page.cookies_for_url("https://example.com").await?;
```

### Set Cookies

```rust
use cdp_core::{SetCookieParams, CookieSameSite};

page.set_cookie(SetCookieParams {
    name: "session".to_string(),
    value: "abc123".to_string(),
    domain: Some("example.com".to_string()),
    path: Some("/".to_string()),
    secure: Some(true),
    http_only: Some(true),
    same_site: Some(CookieSameSite::Lax),
    expires: None,  // Session cookie
    ..Default::default()
}).await?;
```

### Delete Cookies

```rust
// Delete specific cookie
page.delete_cookie("session", Some("example.com")).await?;

// Clear all cookies
page.clear_cookies().await?;
```

## Network Monitoring

### Track Active Requests

```rust
page.enable_network_monitoring().await?;

// Get count of active requests
let count = page.get_inflight_requests_count();
println!("Active requests: {}", count);

// Wait for network idle
page.wait_for_network_idle(30000, 0).await?;  // 0 = fully idle
page.wait_for_network_idle(30000, 2).await?;  // <= 2 requests
```

## Complete Example

```rust
use cdp_core::{Browser, NetworkEvent, RequestModification};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    
    // 1. Enable monitoring
    page.enable_network_monitoring().await?;
    
    // 2. Log all requests
    page.on_network(Arc::new(|event| {
        if let NetworkEvent::RequestWillBeSent { url, method, .. } = event {
            println!("[{}] {}", method, url);
        }
    })).await;
    
    // 3. Intercept and modify
    page.enable_network_interception().await?;
    page.intercept_requests(Arc::new(|mut request| {
        // Add custom header to all requests
        request.headers.insert(
            "X-Custom-Header".to_string(),
            "my-value".to_string()
        );
        Ok(RequestModification::Continue(request))
    })).await;
    
    // 4. Monitor specific responses
    page.monitor_responses(
        vec!["https://api.example.com/"],
        Arc::new(|response| {
            println!("API Response: {}", response.status);
        })
    ).await?;
    
    // 5. Navigate
    page.navigate("https://example.com").await?;
    
    // 6. Wait for network idle
    page.wait_for_network_idle(10000, 2).await?;
    
    Ok(())
}
```

## Tips & Best Practices

### Performance

- Only enable monitoring when needed
- Use specific URL patterns for response monitoring
- Clear monitors when done

### Error Handling

```rust
match page.intercept_requests(Arc::new(|req| {
    // Your logic
    Ok(RequestModification::Continue(req))
})).await {
    Ok(_) => println!("Interception enabled"),
    Err(e) => eprintln!("Failed to enable interception: {}", e),
}
```

### Debugging

```rust
// Log all network activity
page.on_network(Arc::new(|event| {
    println!("{:?}", event);
})).await;
```

## See Also

- [API Reference](../API_REFERENCE.md#network)
- [Examples](../../examples/network_monitoring_test.rs)
