use cdp_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();
    let browser = Browser::launcher().launch().await?;
    let context = browser.new_context().await?;
    let page = context.new_page().await?;

    println!("Navigating to a page with a prominent button...");
    page.navigate("https://www.amazon.com").await?; // Puppeteer's website has a nice big button

    // Wait for load to ensure the page is ready
    page.wait_for_loaded().await?;
    println!("Page loaded.");

    print!("   Testing element screenshot (auto-save)... ");
    let screenshot_target = page
        .query_selector("#nav-search")
        .await?
        .ok_or_else(|| CdpError::element("#nav-search not found".to_string()))?;

    let element_path = screenshot_target.screenshot(None).await?;
    println!("✓ {}", element_path);

    Ok(())
}
