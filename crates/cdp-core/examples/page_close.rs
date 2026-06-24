use cdp_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Start logging to see detail
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;
    page.navigate("https://www.google.com").await?;
    page.wait_for_navigation(Some(WaitForNavigationOptions {
        timeout_ms: Some(30000),
        wait_until: Some(WaitUntil::NetworkIdle2),
    }))
    .await?;
    // Verify target_id is populated
    assert!(
        !page.target_id.is_empty(),
        "Page target_id should not be empty"
    );
    println!("Created page with target_id: {}", page.target_id);

    page.cleanup().await?;
    // Close page
    page.close().await?;
    println!("Successfully closed page");

    Ok(())
}
