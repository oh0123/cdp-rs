/// Network Control Example
///
/// Demonstrates:
/// - Network monitoring and event callbacks
/// - Request interception and modification
/// - Response monitoring and body capture
/// - Cookie management
/// - Network idle detection
///
/// Run: cargo run --example network
use cdp_core::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("debug"))
        .with_target(false)
        .init();

    println!("=== Network Control Example ===\n");

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    println!("✓ Browser launched\n");

    // Test 1: Network Monitoring
    test_network_monitoring(&page).await?;

    // Test 2: Request Interception
    test_request_interception(&page).await?;

    // Test 3: Response Monitoring with Body
    test_response_monitoring(&page).await?;

    // Test 4: Network Idle
    test_network_idle(&page).await?;

    println!("\n=== All Tests Complete ===");
    Ok(())
}

async fn test_network_monitoring(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Network Monitoring ==========");

    let request_count = Arc::new(AtomicUsize::new(0));
    let response_count = Arc::new(AtomicUsize::new(0));

    let req_count = Arc::clone(&request_count);
    let res_count = Arc::clone(&response_count);

    // Register network callback
    page.on_network(Arc::new(move |event| match event {
        NetworkEvent::RequestWillBeSent { url, method, .. } => {
            req_count.fetch_add(1, Ordering::SeqCst);
            println!("  📤 [{}] {}", method, url);
        }
        NetworkEvent::ResponseReceived {
            request_id, status, ..
        } => {
            res_count.fetch_add(1, Ordering::SeqCst);
            println!("  📥 [{}] Status: {}", request_id, status);
        }
        NetworkEvent::LoadingFailed {
            request_id,
            error_text,
        } => {
            eprintln!("  ❌ [{}] Failed: {}", request_id, error_text);
        }
        _ => {}
    }))
    .await;

    println!("✓ Network monitoring enabled");

    // Navigate to trigger events
    page.navigate("https://example.com").await?;
    sleep(Duration::from_secs(2)).await;

    println!("\nStatistics:");
    println!("  Total requests: {}", request_count.load(Ordering::SeqCst));
    println!(
        "  Total responses: {}",
        response_count.load(Ordering::SeqCst)
    );
    println!(
        "  Active requests: {}\n",
        page.get_inflight_requests_count()
    );

    Ok(())
}

async fn test_request_interception(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Request Interception ==========");

    // Enable interception
    NetworkInterceptor::enable_request_interception(page, vec!["*".to_string()]).await?;
    println!("✓ Request interception enabled");

    // Example: Modify headers
    let mut headers = HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "test-value".to_string());
    headers.insert("User-Agent".to_string(), "CDP-Test-Agent/1.0".to_string());

    println!("✓ Custom headers ready: {:?}", headers);
    println!("  Note: Headers can be modified in request callback");

    // Example: Block patterns
    println!("✓ Blocking patterns available:");
    println!("  - Block analytics: url.contains(\"analytics\")");
    println!("  - Block images: url.ends_with(\".png\")");
    println!("  - Block ads: url.contains(\"ads\")");

    // Disable interception
    NetworkInterceptor::disable_request_interception(page).await?;
    println!("✓ Request interception disabled\n");

    Ok(())
}

async fn test_response_monitoring(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Response Monitoring ==========");

    // Monitor responses with body capture
    // Note: For POST requests, browsers send OPTIONS (preflight) first, then POST
    page.monitor_responses(
        |url| url.contains("/track/v2/shipments"),
        |response: &InterceptedResponse| {
            // Skip OPTIONS preflight requests (they don't have response bodies we care about)
            if response.status_code == 204 || response.status_code == 200 && response.body.is_none()
            {
                println!(
                    "  [API] OPTIONS preflight - Request ID: {}, Status: {}",
                    response.request_id, response.status_code
                );
                return;
            }

            println!("  [API] POST Request ID: {}", response.request_id);
            println!("  [API] Status: {}", response.status_code);

            if let Some(content_type) = response.headers.get("content-type") {
                println!("  [API] Content-Type: {}", content_type);
            }

            if let Some(body) = &response.body {
                let preview = if body.len() > 100 {
                    format!("{}...", &body[..100])
                } else {
                    body.clone()
                };
                println!("  [API] Body ({} bytes): {}", body.len(), preview);
            } else {
                println!("  [API] No body captured");
            }
            println!();
        },
    )
    .await?;

    println!("✓ Response monitoring enabled for API endpoints");

    page.navigate(
        "https://www.fedex.com/fedextrack/?trknbr=885215382626&trkqual=12029~885215382626~FDEG",
    )
    .await?;
    page.wait_for_navigation(Some(WaitForNavigationOptions {
        wait_until: Some(cdp_core::WaitUntil::NetworkIdle0),
        timeout_ms: Some(30000),
    }))
    .await?;
    sleep(Duration::from_secs(2)).await;

    println!("✓ Response monitoring complete\n");

    Ok(())
}

async fn test_network_idle(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Network Idle Detection ==========");

    page.navigate("https://example.com").await?;

    let start = std::time::Instant::now();
    page.wait_for_navigation(Some(WaitForNavigationOptions {
        timeout_ms: Some(15000),
        wait_until: Some(WaitUntil::NetworkIdle2),
    }))
    .await?;

    println!(
        "✓ NetworkIdle2 triggered, time: {}ms",
        start.elapsed().as_millis()
    );
    println!(
        "  Active requests: {}\n",
        page.get_inflight_requests_count()
    );

    Ok(())
}
