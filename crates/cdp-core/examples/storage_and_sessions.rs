/// Storage and Session Management Example
///
/// Demonstrates:
/// - LocalStorage operations
/// - SessionStorage operations
/// - Cookie management
/// - Page session export/import
/// - Session snapshots and cloning
///
/// Run: cargo run --example storage_and_sessions
use cdp_core::*;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();

    let browser = Arc::new(Browser::launcher().launch().await?);
    let page = browser.new_page().await?;

    // Navigate to test page
    page.navigate("https://www.github.com").await?;
    page.wait_for_navigation(None).await?;
    println!("✓ Navigated to github.com\n");

    // ========== LocalStorage Tests ==========
    test_local_storage(&page).await?;

    // ========== SessionStorage Tests ==========
    test_session_storage(&page).await?;

    // ========== Cookie Management ==========
    test_cookies(&page).await?;

    // ========== Page Session Management ==========
    test_page_sessions(&browser, &page).await?;

    println!("\n========== All Tests Passed! ==========");
    Ok(())
}

async fn test_local_storage(page: &Arc<page::Page>) -> Result<()> {
    println!("========== LocalStorage Tests ==========");

    // Set items
    page.set_local_storage_item("user_name", "Alice").await?;
    page.set_local_storage_item("theme", "dark").await?;
    page.set_local_storage_item("language", "en").await?;
    println!("✓ Set 3 items in localStorage");

    // Get single item
    if let Some(value) = page.get_local_storage_item("user_name").await? {
        println!("✓ Got localStorage item 'user_name': {}", value);
    }

    // Get all items
    let all_items = page.get_local_storage().await?;
    println!("✓ All localStorage items ({} total):", all_items.len());
    for (key, value) in &all_items {
        println!("  - {}: {}", key, value);
    }

    // Get length
    let length = page.get_storage_length(StorageType::Local).await?;
    println!("✓ localStorage length: {}", length);

    // Remove item
    page.remove_local_storage_item("theme").await?;
    println!("✓ Removed 'theme' from localStorage");

    // Clear all
    page.clear_local_storage().await?;
    let length_after_clear = page.get_storage_length(StorageType::Local).await?;
    println!(
        "✓ Cleared localStorage (length now: {})\n",
        length_after_clear
    );

    Ok(())
}

async fn test_session_storage(page: &Arc<page::Page>) -> Result<()> {
    println!("========== SessionStorage Tests ==========");

    // Set items
    page.set_session_storage_item("session_id", "abc123")
        .await?;
    page.set_session_storage_item("temp_data", "temporary")
        .await?;
    println!("✓ Set 2 items in sessionStorage");

    // Get single item
    if let Some(value) = page.get_session_storage_item("session_id").await? {
        println!("✓ Got sessionStorage item 'session_id': {}", value);
    }

    // Get all items
    let all_items = page.get_session_storage().await?;
    println!("✓ All sessionStorage items ({} total):", all_items.len());
    for (key, value) in &all_items {
        println!("  - {}: {}", key, value);
    }

    // Clear
    page.clear_session_storage().await?;
    let length = page.get_storage_length(StorageType::Session).await?;
    println!("✓ Cleared sessionStorage (length now: {})\n", length);

    Ok(())
}

async fn test_cookies(page: &Arc<page::Page>) -> Result<()> {
    println!("========== Cookie Management ==========");

    // Set cookie
    page.set_cookie(SetCookieParams {
        name: "test_cookie".to_string(),
        value: "cookie_value_123".to_string(),
        url: Some("https://github.com".to_string()),
        ..Default::default()
    })
    .await?;
    println!("✓ Set cookie: test_cookie = cookie_value_123");

    // Get cookies
    let cookies = page.get_cookies(None).await?;
    println!("✓ Got {} cookies", cookies.len());
    for cookie in cookies.iter().take(3) {
        println!("  - {} = {}", cookie.name, cookie.value);
    }

    // Delete cookie
    page.delete_cookies(
        "test_cookie",
        Some("https://github.com".to_string()),
        Some("github.com".to_string()),
        None,
    )
    .await?;
    println!("✓ Deleted test_cookie\n");

    Ok(())
}

async fn test_page_sessions(browser: &Arc<Browser>, page1: &Arc<page::Page>) -> Result<()> {
    println!("========== Page Session Management ==========");

    // Set up state
    page1
        .set_local_storage_item("user_preference", "dark_mode")
        .await?;
    page1.set_local_storage_item("language", "en-US").await?;
    page1
        .set_session_storage_item("session_id", "xyz789")
        .await?;
    page1
        .set_cookie(SetCookieParams {
            name: "session_cookie".to_string(),
            value: "session_value".to_string(),
            url: Some("https://github.com".to_string()),
            ..Default::default()
        })
        .await?;
    println!("✓ Set up page state (localStorage, sessionStorage, cookies)");

    // Export session
    let session = page1.export_session().await?;
    println!("✓ Exported session:");
    println!("  - URL: {}", session.url);
    println!("  - Cookies: {} items", session.cookies.len());
    println!("  - localStorage: {} items", session.local_storage.len());
    println!(
        "  - sessionStorage: {} items",
        session.session_storage.len()
    );

    // Save to file
    let file_path = PathBuf::from("/tmp/github_session.json");
    session.save_to_file(&file_path)?;
    println!("✓ Saved session to {:?}", file_path);

    // Import to new page
    let page2 = browser.new_page().await?;
    page2.import_session(&session).await?;
    println!("✓ Imported session to new page");

    // Verify
    let language = page2.get_local_storage_item("language").await?;
    println!("✓ Verified localStorage: language = {:?}", language);

    // Test snapshot
    let snapshot = page1.snapshot().await?;
    let page3 = browser.new_page().await?;
    page3.restore(&snapshot).await?;
    println!("✓ Created and restored snapshot");

    // Test clone
    let page4 = browser.new_page().await?;
    page1.clone_session_to(&page4).await?;
    println!("✓ Cloned session to another page\n");

    Ok(())
}
