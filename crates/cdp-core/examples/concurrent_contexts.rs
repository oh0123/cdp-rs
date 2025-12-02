use std::sync::Arc;

use cdp_core::*;
use tokio::task::JoinSet;

/// Example demonstrating concurrent browsing across multiple browser contexts.
/// Each context visits a different Amazon product page independently.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();

    // List of Amazon product URLs to visit concurrently
    let product_urls = vec![
        "https://www.amazon.com/dp/B08CBVGXZC",
        "https://www.amazon.com/dp/B0BDF8CVBN",
        "https://www.amazon.com/dp/B0D1XD1ZV3",
        "https://www.amazon.com/dp/B0CQXM2J9H",
    ];

    println!("Launching browser...");
    let browser = Browser::launcher().launch().await?;
    println!("Browser launched successfully.\n");

    // Create a JoinSet to manage concurrent tasks
    let mut tasks = JoinSet::new();

    // Spawn a task for each product URL
    for (index, url) in product_urls.iter().enumerate() {
        let browser_clone = browser.clone();
        let url_clone = url.to_string();

        tasks.spawn(async move { visit_product_page(browser_clone, index + 1, &url_clone).await });
    }

    // Wait for all tasks to complete and collect results
    println!("Starting concurrent page visits...\n");
    let mut results = Vec::new();

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(page_info)) => {
                println!("✓ Context {} completed successfully", page_info.context_id);
                results.push(page_info);
            }
            Ok(Err(e)) => {
                eprintln!("✗ Task failed: {}", e);
            }
            Err(e) => {
                eprintln!("✗ Task panicked: {}", e);
            }
        }
    }

    // Print summary
    println!("\n{}", "=".repeat(60));
    println!(
        "Summary: {}/{} pages visited successfully",
        results.len(),
        product_urls.len()
    );
    println!("{}", "=".repeat(60));

    for info in results {
        println!("Context {}: {}", info.context_id, info.title);
        println!("  URL: {}", info.url);
        println!("  Screenshot: {}", info.screenshot_path);
        println!();
    }

    Ok(())
}

/// Information about a visited page
#[derive(Debug)]
struct PageInfo {
    context_id: usize,
    url: String,
    title: String,
    screenshot_path: String,
}

/// Visit a product page in its own browser context
async fn visit_product_page(
    browser: Arc<Browser>,
    context_id: usize,
    url: &str,
) -> Result<PageInfo> {
    println!("[Context {}] Creating new context...", context_id);

    // Each context is isolated (separate cookies, storage, etc.)
    let context = browser.new_context().await?;

    println!("[Context {}] Creating new page...", context_id);
    let page = context.new_page().await?;

    println!("[Context {}] Navigating to: {}", context_id, url);
    page.navigate(url).await?;

    println!("[Context {}] Waiting for page to load...", context_id);
    page.wait_for_loaded().await?;

    // Get page title
    let title = page
        .get_title()
        .await
        .unwrap_or_else(|_| "Unknown".to_string());

    println!("[Context {}] Page loaded: {}", context_id, title);

    // Take a screenshot
    println!("[Context {}] Taking screenshot...", context_id);
    let screenshot_path = page.screenshot(false, None).await?;

    println!(
        "[Context {}] Screenshot saved: {}",
        context_id, screenshot_path
    );

    Ok(PageInfo {
        context_id,
        url: url.to_string(),
        title,
        screenshot_path,
    })
}
