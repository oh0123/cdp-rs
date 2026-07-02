# Network Features

CDP-Core provides request interception, response monitoring, cookie management, and network control helpers.

## Request Interception

### Callback Helper

```rust
use cdp_core::{InterceptRequestAction, RequestModification};

page.intercept_requests(|request| {
    if request.url.contains("analytics") {
        return InterceptRequestAction::Abort("BlockedByClient".to_string());
    }

    if request.url.contains("/api/") {
        return InterceptRequestAction::Modify(
            RequestModification::default().with_header("Authorization", "Bearer token123"),
        );
    }

    InterceptRequestAction::Continue
}).await?;
```

### Mock Responses

```rust
use cdp_core::{InterceptRequestAction, ResponseMock};

page.intercept_requests(|request| {
    if request.url.contains("/api/data") {
        return InterceptRequestAction::Fulfill(
            ResponseMock::new(r#"{"mock":"data"}"#)
                .with_status_code(200)
                .with_header("Content-Type", "application/json"),
        );
    }

    InterceptRequestAction::Continue
}).await?;
```

### Lower-Level Control

```rust
use cdp_core::NetworkInterceptor;

page.enable_request_interception(vec!["*".to_string()]).await?;
page.continue_request("request-id").await?;
page.disable_request_interception().await?;
```

## Response Monitoring

```rust
page.monitor_responses(
    |url| url.contains("https://api.example.com/data"),
    |response| {
        println!("Status: {}", response.status_code);
        println!("Headers: {:?}", response.headers);
        println!("Body: {:?}", response.body);
    },
).await?;

page.monitor_responses_matching("api.example.com", |response| {
    println!("API Response: {}", response.status_code);
}).await?;
```

## Network Events

```rust
use cdp_core::NetworkEvent;
use std::sync::Arc;

page.enable_network_monitoring().await?;

page.on_network(Arc::new(|event| {
    match event {
        NetworkEvent::RequestWillBeSent { url, method, .. } => {
            println!("[{}] {}", method, url);
        }
        NetworkEvent::ResponseReceived { request_id, status, .. } => {
            println!("[{}] status {}", request_id, status);
        }
        NetworkEvent::LoadingFailed { request_id, error_text } => {
            eprintln!("Failed [{}]: {}", request_id, error_text);
        }
        _ => {}
    }
})).await;

let count = page.get_inflight_requests_count();
println!("Active requests: {}", count);
```

## Network Control

```rust
use cdp_core::NetworkControl;

page.clear_browser_cache().await?;
page.set_cache_disabled(true).await?;
page.set_bypass_service_worker(true).await?;
page.block_urls(["*.png", "*.jpg"]).await?;

let body = page.get_response_body("request-id").await?;
let post_data = page.get_request_post_data("request-id").await?;
```

## Cookie Management

### Get Cookies

```rust
use cdp_core::CookieManager;

let cookies = page.get_cookies(None).await?;
let scoped = page
    .get_cookies(Some(vec!["https://example.com".to_string()]))
    .await?;
```

### Set Cookies

```rust
use cdp_core::{CookieManager, CookieSameSite, SetCookieParams};

page.set_cookie(
    SetCookieParams::new("session", "abc123")
        .with_domain("example.com")
        .with_path("/")
        .with_secure(true)
        .with_http_only(true)
        .with_same_site(CookieSameSite::Lax),
).await?;
```

### Delete Cookies

```rust
use cdp_core::{CookieManager, DeleteCookieOptions};

page
    .delete_cookies(DeleteCookieOptions::new("session").with_domain("example.com"))
    .await?;

page.clear_browser_cookies().await?;
```

## Complete Example

```rust
use cdp_core::{
    Browser, InterceptRequestAction, NetworkEvent, RequestModification,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    page.enable_network_monitoring().await?;

    page.on_network(Arc::new(|event| {
        if let NetworkEvent::RequestWillBeSent { url, method, .. } = event {
            println!("[{}] {}", method, url);
        }
    })).await;

    page.intercept_requests(|request| {
        if request.url.contains("/api/") {
            InterceptRequestAction::Modify(
                RequestModification::default().with_header("X-Custom-Header", "my-value"),
            )
        } else {
            InterceptRequestAction::Continue
        }
    }).await?;

    page.monitor_responses_matching("api.example.com", |response| {
        println!("API Response: {}", response.status_code);
    }).await?;

    page.navigate("https://example.com").await?;
    page.wait_for_navigation(None).await?;

    Ok(())
}
```

## See Also

- [API Reference](../API_REFERENCE.md#network)
- [Examples](../../examples/)
