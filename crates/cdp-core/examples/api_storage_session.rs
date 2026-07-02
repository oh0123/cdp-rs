#[path = "support/mod.rs"]
mod support;

use anyhow::{Context, Result, ensure};
use cdp_core::{
    CookieManager, CookieSameSite, DeleteCookieOptions, LocalStorageExt, PageSession,
    PageSessionManager, PageSessionSnapshot, SessionStorageExt, SetCookieParams, StorageManager,
    StorageType,
};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Storage, Cookies, and Session manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;
    support::navigate_and_wait(&page, &server.url("/"), "#ready").await?;

    generic_storage(&page).await?;
    convenience_storage(&page).await?;
    cookies(&page, &server).await?;
    page_session_roundtrip(&browser, &page).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Storage, Cookies, and Session manual API example complete ==");
    Ok(())
}

async fn generic_storage(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- generic StorageManager");

    page.set_storage_item(StorageType::Local, "generic-local", "local-value")
        .await?;
    page.set_storage_item(StorageType::Session, "generic-session", "session-value")
        .await?;

    ensure!(
        page.get_storage_item(StorageType::Local, "generic-local")
            .await?
            .as_deref()
            == Some("local-value"),
        "generic localStorage get failed"
    );
    ensure!(
        page.get_storage_item(StorageType::Session, "generic-session")
            .await?
            .as_deref()
            == Some("session-value"),
        "generic sessionStorage get failed"
    );
    ensure!(
        !page.get_storage_items(StorageType::Local).await?.is_empty(),
        "expected localStorage items"
    );
    ensure!(
        page.get_storage_length(StorageType::Session).await? >= 1,
        "expected sessionStorage length"
    );

    page.remove_storage_item(StorageType::Local, "generic-local")
        .await?;
    ensure!(
        page.get_storage_item(StorageType::Local, "generic-local")
            .await?
            .is_none(),
        "generic localStorage remove failed"
    );
    Ok(())
}

async fn convenience_storage(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- localStorage/sessionStorage convenience traits");

    page.set_local_storage_item("theme", "dark").await?;
    page.set_local_storage_item("language", "en-US").await?;
    page.set_session_storage_item("wizard-step", "2").await?;

    ensure!(
        page.get_local_storage_item("theme").await?.as_deref() == Some("dark"),
        "localStorage item mismatch"
    );
    ensure!(
        page.get_session_storage_item("wizard-step")
            .await?
            .as_deref()
            == Some("2"),
        "sessionStorage item mismatch"
    );
    ensure!(
        page.get_local_storage().await?.contains_key("language"),
        "get_local_storage should include language"
    );
    ensure!(
        page.get_session_storage()
            .await?
            .contains_key("wizard-step"),
        "get_session_storage should include wizard-step"
    );

    page.remove_session_storage_item("wizard-step").await?;
    ensure!(
        page.get_session_storage_item("wizard-step")
            .await?
            .is_none(),
        "sessionStorage remove failed"
    );
    Ok(())
}

async fn cookies(
    page: &std::sync::Arc<cdp_core::Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- cookies");

    page.set_cookie(
        SetCookieParams::new("manual_session", "token-123")
            .with_url(server.url("/"))
            .with_path("/")
            .with_same_site(CookieSameSite::Lax),
    )
    .await?;
    page.set_cookie(
        SetCookieParams::new("manual_scoped", "scoped")
            .with_url(server.url("/"))
            .with_path("/")
            .with_http_only(false)
            .with_secure(false),
    )
    .await?;

    let cookies = page.get_cookies(Some(vec![server.url("/")])).await?;
    ensure!(
        cookies.iter().any(|cookie| cookie.name == "manual_session"),
        "manual_session cookie missing"
    );

    page.delete_cookies(
        DeleteCookieOptions::new("manual_session")
            .with_url(server.url("/"))
            .with_path("/"),
    )
    .await?;
    let cookies = page.get_cookies(Some(vec![server.url("/")])).await?;
    ensure!(
        cookies.iter().all(|cookie| cookie.name != "manual_session"),
        "manual_session cookie should be deleted"
    );
    Ok(())
}

async fn page_session_roundtrip(
    browser: &std::sync::Arc<cdp_core::Browser>,
    page: &std::sync::Arc<cdp_core::Page>,
) -> Result<()> {
    println!("-- PageSession round trip");

    page.set_local_storage_item("session-local", "local")
        .await?;
    page.set_session_storage_item("session-temp", "temp")
        .await?;

    let empty = PageSession::new("about:blank".to_string());
    let json = empty.to_json()?;
    let parsed = PageSession::from_json(&json)?;
    ensure!(
        parsed.url == "about:blank",
        "PageSession JSON round trip failed"
    );

    let file_path = support::output_path("page-session.json")?;
    let exported = page.export_session_to_file(&file_path).await?;
    ensure!(
        exported.local_storage.contains_key("session-local"),
        "exported session should include localStorage"
    );
    let loaded = PageSession::load_from_file(&file_path)?;
    ensure!(
        loaded.session_storage.contains_key("session-temp"),
        "loaded session should include sessionStorage"
    );

    let page2 = browser.new_page().await?;
    page2.import_session_from_file(&file_path).await?;
    ensure!(
        page2
            .get_local_storage_item("session-local")
            .await?
            .as_deref()
            == Some("local"),
        "import_session_from_file should restore localStorage"
    );

    let snapshot = page.snapshot().await?;
    let page3 = browser.new_page().await?;
    page3.restore(&snapshot).await?;
    ensure!(
        page3
            .get_session_storage_item("session-temp")
            .await?
            .as_deref()
            == Some("temp"),
        "restore should restore sessionStorage"
    );

    let page4 = browser.new_page().await?;
    page.clone_session_to(&page4).await?;
    ensure!(
        page4
            .get_local_storage_item("session-local")
            .await?
            .as_deref()
            == Some("local"),
        "clone_session_to should restore localStorage"
    );

    page2.close().await.context("close imported page")?;
    page3.close().await.context("close restored page")?;
    page4.close().await.context("close cloned page")?;
    Ok(())
}
