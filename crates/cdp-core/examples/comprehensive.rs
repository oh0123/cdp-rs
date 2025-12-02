/// Comprehensive Features Example
///
/// Demonstrates:
/// - Element interaction (click, double-click, type)
/// - Content extraction (text, attributes, HTML)
/// - Screenshots (element and full page)
/// - DOM queries (selector, querySelectorAll)
/// - DOM mutations monitoring
/// - Frame snapshots
///
/// Run: cargo run --example comprehensive
use cdp_core::*;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();

    println!("🚀 CDP-Core Comprehensive Test\n");

    // Connect to browser
    let page = Page::connect_to_active_page(None, None).await?;
    println!("✅ Connected to Chrome\n");

    // Load test page
    let html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-assets")
        .join("comprehensive-test.html");

    let url = format!("file://{}", html_path.display());
    println!("📄 Loading test page: {}", url);
    page.navigate(&url).await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("✅ Page loaded\n");

    // Test 1: Element Interaction
    test_element_interaction(&page).await?;

    // Test 2: Content Extraction
    test_content_extraction(&page).await?;

    // Test 3: Screenshots
    test_screenshots(&page).await?;

    // Test 4: DOM Mutations
    test_dom_mutations(&page).await?;

    // Test 5: Frame Management
    test_frames(&page).await?;

    println!("\n✅ All Tests Passed!");
    Ok(())
}

async fn test_element_interaction(page: &Arc<Page>) -> Result<()> {
    println!("========== Element Interaction ==========");

    // Click button
    let btn = page
        .query_selector("#btn-click")
        .await?
        .ok_or_else(|| CdpError::element("Button not found".to_string()))?;

    print!("   Testing click... ");
    for i in 1..=3 {
        btn.click().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        print!("{} ", i);
    }
    println!("✓");

    // Double click
    let dbl_btn = page
        .query_selector("#btn-dblclick")
        .await?
        .ok_or_else(|| CdpError::element("Double-click button not found".to_string()))?;

    print!("   Testing double-click... ");
    dbl_btn.double_click().await?;
    println!("✓");

    // Type text
    let input = page
        .query_selector("#input-fast")
        .await?
        .ok_or_else(|| CdpError::element("Input not found".to_string()))?;

    print!("   Testing text input... ");
    input.type_text("Hello, CDP-Core!").await?;
    println!("✓\n");

    Ok(())
}

async fn test_content_extraction(page: &Arc<Page>) -> Result<()> {
    println!("========== Content Extraction ==========");

    // Query all elements
    let items = page.query_selector_all(".list-item").await?;
    println!("   Found {} list items:", items.len());
    for (i, item) in items.iter().enumerate() {
        let text = item.text_content().await?;
        println!("   • Item {}: {}", i + 1, text.trim());
    }

    // Get attributes
    let attr_demo = page
        .query_selector(".attribute-demo")
        .await?
        .ok_or_else(|| CdpError::element("Attribute demo not found".to_string()))?;

    if let Some(id) = attr_demo.get_attribute("id").await? {
        println!("   ID attribute: \"{}\"", id);
    }
    if let Some(title) = attr_demo.get_attribute("title").await? {
        println!("   Title attribute: \"{}\"", title);
    }

    // Get HTML
    let content = page
        .query_selector("#content-html")
        .await?
        .ok_or_else(|| CdpError::element("Content not found".to_string()))?;

    let html = content.get_html().await?;
    println!("   Element HTML (first 100 chars):");
    println!("   {}\n", &html.chars().take(100).collect::<String>());

    Ok(())
}

async fn test_screenshots(page: &Arc<Page>) -> Result<()> {
    println!("========== Screenshots ==========");

    // Element screenshot
    print!("   Element screenshot (auto-save)... ");
    let element = page
        .query_selector("#content-html")
        .await?
        .ok_or_else(|| CdpError::element("Element not found".to_string()))?;

    let path = element.screenshot(None).await?;
    println!("✓ {}", path);

    // Full page screenshot
    print!("   Full page screenshot... ");
    let fullpage_path = page.screenshot(true, None).await?;
    println!("✓ {}\n", fullpage_path);

    Ok(())
}

async fn test_dom_mutations(page: &Arc<Page>) -> Result<()> {
    println!("========== DOM Mutations ==========");

    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let mutation_count = Arc::new(AtomicUsize::new(0));
    let count_clone = Arc::clone(&mutation_count);

    // Enable DOM mutation monitoring
    page.enable_dom_mutations().await?;

    // Register callback
    page.on_dom_mutation(Arc::new(move |event| {
        count_clone.fetch_add(1, Ordering::SeqCst);
        match event {
            DomMutationEvent::ChildNodeInserted { parent_node_id, .. } => {
                println!("   Node inserted into parent: {}", parent_node_id);
            }
            DomMutationEvent::AttributeModified {
                node_id,
                name,
                value,
            } => {
                println!(
                    "   Attribute modified: {} = {} on node {}",
                    name, value, node_id
                );
            }
            _ => {}
        }
    }))
    .await;

    println!("✓ DOM mutation monitoring enabled");

    // Trigger some mutations (requires JavaScript execution)
    // page.main_frame().await?.evaluate(...) would work but skipping for simplicity

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    println!(
        "   Total mutations detected: {}\n",
        mutation_count.load(Ordering::SeqCst)
    );

    Ok(())
}

async fn test_frames(page: &Arc<Page>) -> Result<()> {
    println!("========== Frame Management ==========");

    // Get all frames
    let frames = page.all_frames().await?;
    println!("   Found {} frames:", frames.len());
    for (i, frame) in frames.iter().enumerate() {
        let url: serde_json::Value = frame.evaluate("window.location.href").await?;
        let url_str = url.as_str().unwrap_or("unknown");
        let display_url = if url_str.starts_with("file://") {
            "file://...comprehensive-test.html"
        } else {
            url_str
        };
        println!(
            "   Frame {}: {} (ID: {}...)",
            i + 1,
            display_url,
            &frame.id[..12]
        );
    }

    // Create frame snapshot
    let main_frame = page.main_frame().await?;
    let snapshot = page.create_frame_snapshot(&main_frame.id, false).await?;
    println!("✓ Created frame snapshot (URL: {})\n", snapshot.url);

    Ok(())
}
