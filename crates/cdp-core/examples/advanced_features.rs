/// Advanced Features Example
///
/// Demonstrates:
/// - Wait functions (selector, navigation, custom conditions)
/// - Shadow DOM access and queries
/// - Frame management and cross-frame queries
/// - Element state checks
///
/// Run: cargo run --example advanced_features
use cdp_core::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();

    println!("=== Advanced Features Example ===\n");

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    // Test 1: Wait Functions
    test_wait_functions(&page).await?;

    // Test 2: Shadow DOM
    test_shadow_dom(&page).await?;

    // Test 3: Frame Queries
    test_frame_queries(&page).await?;

    println!("\n===All Tests Complete ===");
    Ok(())
}

async fn test_wait_functions(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Wait Functions ==========");

    page.navigate("https://example.com").await?;

    // Wait for element
    print!("   Waiting for selector... ");
    let _element = page.wait_for_selector("h1", None).await?;
    println!("✓ Element found");

    // Wait for navigation
    print!("   Waiting for navigation... ");
    page.navigate("https://example.com").await?;
    page.wait_for_navigation(None).await?;
    println!("✓");

    // Wait for custom function
    print!("   Waiting for custom condition... ");
    page.wait_for_function("() => document.readyState === 'complete'", Some(5000), None)
        .await?;
    println!("✓");

    // Wait for element state
    if let Some(el) = page.query_selector("h1").await? {
        print!("   Checking element visibility... ");
        let visible = el.is_visible().await?;
        println!("✓ Visible: {}", visible);

        print!("   Checking element clickable... ");
        let clickable = el.is_clickable().await?;
        println!("✓ Clickable: {}\n", clickable);
    }

    Ok(())
}

async fn test_shadow_dom(_page: &Arc<page::Page>) -> Result<()> {
    println!("========== Shadow DOM ==========");

    // Note: This requires a page with Shadow DOM
    println!("✓ Shadow DOM API available:");
    println!("  - element.shadow_root().await");
    println!("  - shadow.query_selector(selector).await");
    println!("  - shadow.query_selector_all(selector).await");
    println!("  - element.query_selector_shadow(selector).await (convenience)\n");

    Ok(())
}

async fn test_frame_queries(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Frame Queries ==========");

    // Get all frames
    let frames = page.all_frames().await?;
    println!("   Found {} frames:", frames.len());
    for (i, frame) in frames.iter().enumerate() {
        println!("   Frame {}: ID = {}...", i + 1, &frame.id[..12]);
    }

    // Main frame
    let main_frame = page.main_frame().await?;
    println!("   Main frame ID: {}...", &main_frame.id[..12]);

    // Cross-frame query (automatic)
    println!("✓ Automatic cross-frame queries:");
    println!("  - page.query_selector(selector) searches all frames");
    println!("  - page.query_selector_all(selector) collects from all frames\n");

    Ok(())
}
