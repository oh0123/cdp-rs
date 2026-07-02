#[path = "support/mod.rs"]
mod support;

use std::time::Duration;

use anyhow::{Context, Result, ensure};
use cdp_core::{
    ElementScreenshotOptions, ScreenshotBoxType,
    input::{
        keyboard::{KeyModifiers, KeyboardModifier},
        mouse::DragOptions,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Element, Frame, and Input manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;

    support::navigate_and_wait(&page, &server.url("/"), "#ready").await?;

    element_queries_and_properties(&page).await?;
    element_interactions_and_waits(&page).await?;
    element_screenshots(&page).await?;
    shadow_dom(&page).await?;
    keyboard_and_mouse(&page).await?;
    file_upload_and_clear(&page).await?;
    frames(&page, &server).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Element, Frame, and Input manual API example complete ==");
    Ok(())
}

async fn element_queries_and_properties(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- element queries and properties");

    let box_el = page.query_selector("#box").await?.context("missing #box")?;
    let text = box_el.text_content().await?;
    ensure!(text.contains("Box text"), "unexpected #box text: {text}");
    ensure!(
        box_el.get_attribute("data-kind").await?.as_deref() == Some("box"),
        "missing data-kind attribute"
    );
    ensure!(
        box_el.get_html().await?.contains("inner span"),
        "expected inner span in HTML"
    );
    ensure!(box_el.is_visible().await?, "#box should be visible");
    ensure!(box_el.is_enabled().await?, "#box should be enabled");
    ensure!(box_el.is_clickable().await?, "#box should be clickable");

    let bbox = box_el.bounding_box().await?;
    ensure!(
        bbox.width > 0.0 && bbox.height > 0.0,
        "invalid bounding box"
    );

    let events = page.query_selector_all("#event-log .event").await?;
    ensure!(events.is_empty(), "event log should start empty");

    let by_xpath = page.query_xpath("//button[@id='click-button']").await?;
    ensure!(by_xpath.is_some(), "XPath button lookup failed");
    Ok(())
}

async fn element_interactions_and_waits(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- element interactions and waits");

    let click = page
        .query_selector("#click-button")
        .await?
        .context("missing click button")?;
    click.click().await?;
    let count = page
        .evaluate("document.getElementById('click-count').textContent")
        .await?;
    support::require_str(count, "1", "click count")?;

    let double = page
        .query_selector("#double-button")
        .await?
        .context("missing double button")?;
    double.double_click().await?;
    let count = page
        .evaluate("document.getElementById('double-count').textContent")
        .await?;
    support::require_str(count, "1", "double click count")?;

    let hidden = page
        .query_selector("#hidden-target")
        .await?
        .context("missing hidden target")?;
    hidden.wait_for_hidden(Some(5_000)).await?;
    let box_for_move = page
        .query_selector("#box")
        .await?
        .context("missing #box for mouse move")?;
    box_for_move.move_mouse_to().await?;

    let toggle = page
        .query_selector("#toggle-visible")
        .await?
        .context("missing toggle button")?;
    toggle.click().await?;
    let hidden = page
        .query_selector("#hidden-target")
        .await?
        .context("missing hidden target after toggle")?;
    hidden.wait_for_visible(Some(5_000)).await?;

    let disabled = page
        .query_selector("#disabled-button")
        .await?
        .context("missing disabled button")?;
    ensure!(
        !disabled.is_enabled().await?,
        "disabled button should be disabled"
    );
    page.evaluate("document.getElementById('disabled-button').disabled = false")
        .await?;
    let disabled = page
        .query_selector("#disabled-button")
        .await?
        .context("missing enabled button")?;
    disabled.wait_for_enabled(Some(5_000)).await?;
    disabled.wait_for_clickable(Some(5_000)).await?;

    let box_el = page.query_selector("#box").await?.context("missing #box")?;
    box_el.press_and_hold(Duration::from_millis(350)).await?;
    let hold_count = page
        .evaluate("document.getElementById('hold-count').textContent")
        .await?;
    support::require_str(hold_count, "1", "hold count")?;
    Ok(())
}

async fn element_screenshots(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- element screenshots");

    let box_el = page.query_selector("#box").await?.context("missing #box")?;
    let border_path = support::output_path("element-border.png")?;
    let content_path = support::output_path("element-content.png")?;
    let margin_path = support::output_path("element-margin.png")?;

    box_el
        .screenshot(
            ElementScreenshotOptions::default()
                .box_type(ScreenshotBoxType::Border)
                .save_to(&border_path),
        )
        .await?;
    box_el
        .screenshot(
            ElementScreenshotOptions::default()
                .content_box()
                .auto_resolve_dpr(false)
                .save_to(&content_path),
        )
        .await?;
    box_el
        .screenshot(
            ElementScreenshotOptions::default()
                .margin_box()
                .save_to(&margin_path),
        )
        .await?;

    support::ensure_non_empty_file(border_path)?;
    support::ensure_non_empty_file(content_path)?;
    support::ensure_non_empty_file(margin_path)?;
    Ok(())
}

async fn shadow_dom(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- shadow DOM");

    let host = page
        .query_selector("#shadow-host")
        .await?
        .context("missing shadow host")?;
    let shadow = host
        .shadow_root()
        .await?
        .context("missing open shadow root")?;
    let inner = shadow
        .query_selector("#shadow-input")
        .await?
        .context("missing shadow input")?;
    ensure!(
        inner.get_attribute("value").await?.as_deref() == Some("shadow"),
        "unexpected shadow input value"
    );
    let all_inner = shadow.query_selector_all(".inner").await?;
    ensure!(all_inner.len() == 2, "expected two shadow .inner elements");
    ensure!(
        shadow.get_html().await?.contains("shadow-card"),
        "shadow HTML should include shadow-card"
    );

    let direct = host
        .query_selector_shadow("#shadow-input")
        .await?
        .context("missing direct shadow input")?;
    direct.type_text(" updated").await?;
    Ok(())
}

async fn keyboard_and_mouse(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- keyboard and mouse");

    let input = page
        .query_selector("#text-input")
        .await?
        .context("missing text input")?;
    input.focus().await?;
    input.clear().await?;
    page.keyboard().insert_text("Hello").await?;
    page.keyboard().press_key("Enter").await?;
    page.keyboard()
        .press_key_with_modifiers("a", KeyModifiers::from_iter([KeyboardModifier::Control]))
        .await?;
    let input = page
        .query_selector("#text-input")
        .await?
        .context("missing text input before delayed typing")?;
    input.clear().await?;
    page.keyboard().type_text_with_delay("Typed", 5, 10).await?;

    let value = page
        .evaluate("document.getElementById('text-input').value")
        .await?;
    support::require_str(value, "Typed", "keyboard final value")?;

    page.mouse().move_to(25.0, 25.0).await?;
    page.mouse()
        .click(25.0, 25.0, cdp_core::MouseClickOptions::default())
        .await?;

    let box_el = page.query_selector("#box").await?.context("missing #box")?;
    box_el.right_click().await?;
    let logged = page
        .evaluate("document.getElementById('event-log').innerText")
        .await?;
    ensure!(
        logged.as_str().unwrap_or_default().contains("contextmenu"),
        "right click should trigger contextmenu"
    );

    page.mouse()
        .drag_to(
            45.0,
            385.0,
            160.0,
            385.0,
            DragOptions::default().with_steps(4),
        )
        .await?;
    Ok(())
}

async fn file_upload_and_clear(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- file upload");

    let upload_path = support::output_path("upload-fixture.txt")?;
    std::fs::write(&upload_path, "upload fixture")?;

    let file = page
        .query_selector("#file-input")
        .await?
        .context("missing file input")?;
    file.upload_files([&upload_path]).await?;
    page.wait_for_function(
        "() => document.getElementById('event-log').innerText.includes('upload-fixture.txt')",
        Some(5_000),
        None,
    )
    .await?;
    Ok(())
}

async fn frames(
    page: &std::sync::Arc<cdp_core::Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- frames");

    page.navigate(&server.url("/frame-parent")).await?;
    support::wait_visible(page, "#ready").await?;
    wait_for_frame_count(page, 3).await?;

    let frames = page.all_frames().await?;
    ensure!(
        frames.len() >= 3,
        "expected parent, child, and grandchild frames"
    );

    let mut child_frame = None;
    for frame in &frames {
        if frame.url().await?.ends_with("/frame-child") {
            child_frame = Some(frame.clone());
            break;
        }
    }
    let child_frame = child_frame.context("missing child frame by URL")?;
    ensure!(
        child_frame.url().await?.ends_with("/frame-child"),
        "unexpected child frame URL"
    );
    let child_text = child_frame
        .evaluate("document.querySelector('.frame-button')?.textContent")
        .await?;
    support::require_str(child_text, "Frame button", "child frame button text")?;
    let frame_query_count = child_frame.query_selector_all(".frame-button").await?.len();
    println!("Frame::query_selector_all('.frame-button') found {frame_query_count}");

    let nested_frames = page.query_frames(".nested-input").await?;
    println!(
        "query_frames('.nested-input') found {}",
        nested_frames.len()
    );

    let main = page.main_frame().await?;
    let snapshot_before = page.create_frame_snapshot(&main.id, true).await?;
    page.evaluate(
        "document.body.appendChild(Object.assign(document.createElement('div'), { id: 'snapshot-added', textContent: 'added' }))",
    )
    .await?;
    let snapshot_after = page.create_frame_snapshot(&main.id, true).await?;
    let diff = cdp_core::Page::compare_snapshots(&snapshot_before, &snapshot_after);
    ensure!(
        !diff.is_empty(),
        "snapshot comparison should detect changes"
    );

    let all_snapshots = page.create_all_frames_snapshot(false).await?;
    ensure!(
        all_snapshots.len() >= 3,
        "expected snapshots for all loaded frames"
    );

    page.evaluate("document.getElementById('child-frame').remove()")
        .await?;
    tokio::time::sleep(Duration::from_millis(250)).await;
    Ok(())
}

async fn wait_for_frame_count(
    page: &std::sync::Arc<cdp_core::Page>,
    expected: usize,
) -> Result<()> {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let frames = page.all_frames().await?;
        if frames.len() >= expected {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            ensure!(
                frames.len() >= expected,
                "expected at least {expected} frames, got {}",
                frames.len()
            );
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
