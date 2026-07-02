#[path = "support/mod.rs"]
mod support;

use std::time::Duration;

use anyhow::{Context, Result, ensure};
use cdp_core::{BrowserContextOptions, BrowserWindowBounds, BrowserWindowState, Page, protocol};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Browser and Page manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;

    browser_info_and_cdp(&browser).await?;
    browser_contexts(&browser).await?;
    browser_window_controls(&browser, &page).await?;
    navigation_and_history(&page, &server).await?;
    targets_and_page_cdp(&page).await?;
    script_injection_and_dialogs(&page, &server).await?;
    resource_tree_and_snapshot(&page, &server).await?;
    domain_manager_smoke(&page).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Browser and Page manual API example complete ==");
    Ok(())
}

async fn browser_info_and_cdp(browser: &std::sync::Arc<cdp_core::Browser>) -> Result<()> {
    println!("-- browser info and Browser::cdp");

    let version = browser.version().await?;
    ensure!(
        !version.product.is_empty(),
        "browser product should not be empty"
    );

    let via_cdp = browser
        .cdp(protocol::browser::GetVersion(None))
        .set_timeout(Duration::from_secs(10))
        .send()
        .await?;
    ensure!(
        version.product == via_cdp.product,
        "Browser::cdp version mismatch"
    );

    let command_line = browser.command_line().await.unwrap_or_default();
    println!(
        "browser product={}, command line args={}",
        version.product,
        command_line.len()
    );
    Ok(())
}

async fn browser_contexts(browser: &std::sync::Arc<cdp_core::Browser>) -> Result<()> {
    println!("-- browser contexts");

    let initial_contexts = browser.contexts().await.len();
    let context = browser
        .new_context(BrowserContextOptions::default().with_dispose_on_detach(true))
        .await?;
    ensure!(!context.id().is_empty(), "context id should not be empty");

    let context_page = context.new_page().await?;
    let pages = browser.pages().await;
    ensure!(
        pages
            .iter()
            .any(|page| page.target_id == context_page.target_id),
        "context page should be tracked by browser.pages()"
    );

    let contexts = browser.contexts().await;
    ensure!(
        contexts.len() > initial_contexts,
        "new context should be tracked"
    );

    context.close().await?;
    Ok(())
}

async fn browser_window_controls(
    browser: &std::sync::Arc<cdp_core::Browser>,
    _page: &std::sync::Arc<Page>,
) -> Result<()> {
    println!("-- browser window controls");

    let window = match browser.window_for_target(None).await {
        Ok(window) => window,
        Err(err) => {
            println!("window controls skipped in this browser mode: {err}");
            return Ok(());
        }
    };
    let before = browser.window_bounds(window.window_id).await?;
    println!("window {} before: {:?}", window.window_id, before);

    let new_bounds = BrowserWindowBounds {
        left: Some(20),
        top: Some(20),
        width: Some(900),
        height: Some(700),
        window_state: Some(BrowserWindowState::Normal),
    };
    browser
        .set_window_bounds(window.window_id, new_bounds)
        .await
        .context("set_window_bounds failed")?;
    browser
        .set_contents_size(window.window_id, Some(800), Some(600))
        .await
        .context("set_contents_size failed")?;
    let after = browser.window_bounds(window.window_id).await?;
    println!("window {} after: {:?}", window.window_id, after);
    Ok(())
}

async fn navigation_and_history(
    page: &std::sync::Arc<Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- navigation and history");

    support::navigate_and_wait(page, &server.url("/navigation-a"), "#ready").await?;
    ensure!(page.get_title().await? == "Navigation A", "title A");

    page.navigate(&server.url("/navigation-b")).await?;
    page.wait_for_loaded().await?;
    ensure!(page.get_title().await? == "Navigation B", "title B");

    page.navigate(&server.url("/navigation-c")).await?;
    page.wait_for_loaded().await?;
    ensure!(page.get_title().await? == "Navigation C", "title C");

    page.go_back().await?;
    page.wait_for_function(
        "() => document.title === 'Navigation B'",
        Some(10_000),
        None,
    )
    .await?;
    page.go_forward().await?;
    page.wait_for_function(
        "() => document.title === 'Navigation C'",
        Some(10_000),
        None,
    )
    .await?;

    let history = page.get_navigation_history().await?;
    ensure!(
        !history.entries.is_empty(),
        "navigation history should not be empty"
    );
    if let Some(first) = history.entries.first() {
        page.navigate_to_history_entry(first.id).await?;
        page.wait_for_loaded().await?;
    }

    page.reload(cdp_core::ReloadOptions::default().with_ignore_cache(true))
        .await?;
    page.wait_for_loaded().await?;
    page.bring_to_front().await?;
    page.reset_navigation_history().await?;

    Ok(())
}

async fn targets_and_page_cdp(page: &std::sync::Arc<Page>) -> Result<()> {
    println!("-- targets and Page::cdp/root_cdp");

    page.set_discover_targets(true, None).await?;
    let targets = page.get_targets().await?;
    let tabs = page.get_tabs().await?;
    ensure!(
        !targets.is_empty(),
        "Target.getTargets should return targets"
    );
    ensure!(!tabs.is_empty(), "get_tabs should return page-like targets");

    let target_via_root = page
        .root_cdp(protocol::target::GetTargets { filter: None })
        .send()
        .await?;
    ensure!(
        !target_via_root.target_infos.is_empty(),
        "Page::root_cdp Target.getTargets should return targets"
    );

    let evaluated = page
        .cdp(protocol::runtime::Evaluate {
            expression: "'page-cdp-ok'".to_string(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        })
        .set_timeout(Duration::from_secs(5))
        .send()
        .await?;
    support::require_str(
        evaluated.result.value.unwrap_or_default(),
        "page-cdp-ok",
        "Runtime.evaluate through Page::cdp",
    )?;

    page.set_discover_targets(false, None).await?;
    Ok(())
}

async fn script_injection_and_dialogs(
    page: &std::sync::Arc<Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- script injection and JavaScript dialogs");

    let identifier = page
        .add_script_to_evaluate_on_new_document("window.__cdpCoreInjected = 'yes';")
        .await?;
    page.navigate(&server.url("/")).await?;
    support::wait_visible(page, "#ready").await?;
    let injected = page.evaluate("window.__cdpCoreInjected").await?;
    support::require_str(injected, "yes", "injected script")?;

    page.remove_script_to_evaluate_on_new_document(identifier)
        .await?;

    let dialog_event =
        page.wait_for_event::<protocol::page::events::JavascriptDialogOpeningEvent>(Some(5000));
    page.evaluate("setTimeout(() => alert('manual dialog'), 0);")
        .await?;
    let dialog = dialog_event.await?;
    ensure!(
        dialog.params.message == "manual dialog",
        "dialog message mismatch"
    );
    page.handle_javascript_dialog(true, None).await?;
    Ok(())
}

async fn resource_tree_and_snapshot(
    page: &std::sync::Arc<Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- resource tree, resource content, and page snapshot");

    page.navigate(&server.url("/")).await?;
    support::wait_visible(page, "#ready").await?;
    page.wait_for_function(
        "() => window.__localServerScriptLoaded === true",
        Some(10_000),
        None,
    )
    .await?;

    let tree = page.get_resource_tree().await?;
    ensure!(
        !tree.frame.url.is_empty(),
        "resource tree should include a frame URL"
    );
    let css = page
        .get_resource_content(&tree.frame.id, &server.url("/style.css"))
        .await?;
    ensure!(
        css.content.contains("--cdp-style-loaded"),
        "expected CSS marker in resource content"
    );

    let snapshot = page.capture_snapshot().await?;
    ensure!(
        snapshot.contains("Manual fixture ready"),
        "MHTML snapshot should include page text"
    );
    Ok(())
}

async fn domain_manager_smoke(page: &std::sync::Arc<Page>) -> Result<()> {
    println!("-- domain manager optional domains");

    page.domain_manager.enable_log_domain().await?;
    ensure!(
        page.domain_manager
            .is_enabled(cdp_core::DomainType::Log)
            .await,
        "Log domain should be enabled"
    );
    page.domain_manager.disable_log_domain().await?;

    page.domain_manager.enable_performance_domain().await?;
    ensure!(
        page.domain_manager
            .is_enabled(cdp_core::DomainType::Performance)
            .await,
        "Performance domain should be enabled"
    );
    page.domain_manager.disable_performance_domain().await?;

    page.domain_manager.enable_fetch_domain().await?;
    page.domain_manager.disable_fetch_domain().await?;
    Ok(())
}
