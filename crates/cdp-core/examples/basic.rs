use cdp_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("debug"))
        .with_target(false)
        .init();
    let browser = Browser::launcher().launch().await?;
    let context = browser
        .new_context(BrowserContextOptions::default())
        .await?;
    let page = context.new_page().await?;

    println!("Navigating to a page with a prominent search box...");
    page.navigate("https://www.amazon.com").await?;

    print!("   Waiting for search box... ");
    let screenshot_target = page
        .wait_for_selector(
            "#nav-search",
            Some(
                WaitForSelectorOptions::default()
                    .with_timeout_ms(15_000)
                    .with_visible(true),
            ),
        )
        .await?;
    println!("✓ Element found");

    print!("   Testing element screenshot (auto-save)... ");
    let element_path = screenshot_target
        .screenshot(ElementScreenshotOptions::default())
        .await?;
    println!("✓ {}", element_path);

    Ok(())
}
