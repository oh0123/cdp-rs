#[path = "support/mod.rs"]
mod support;

use std::sync::Arc;

use anyhow::{Context, Result, ensure};
use cdp_core::{Page, events};
use futures_util::StreamExt;
use tokio::time::{Duration, timeout};

const EVENT_TIMEOUT_MS: u64 = 5_000;
const STREAM_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Events local manual API example ==");

    let (browser, page) = support::new_page().await?;

    target_events(&browser, &page).await?;
    page_lifecycle_and_runtime_events(&page).await?;
    console_stream_events(&page).await?;
    log_domain_diagnostics(&page).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Events local manual API example complete ==");
    Ok(())
}

async fn target_events(browser: &Arc<cdp_core::Browser>, page: &Arc<Page>) -> Result<()> {
    println!("-- target discovery events");

    page.set_discover_targets(true, None).await?;
    let target_created =
        page.wait_for_event::<events::target::TargetCreatedEvent>(Some(EVENT_TIMEOUT_MS));
    let popup = browser.new_page().await?;
    let event = target_created.await?;

    ensure!(
        !event.params.target_info.target_id.is_empty(),
        "target id should be populated"
    );
    ensure!(
        !event.params.target_info.r#type.is_empty(),
        "target type should be populated"
    );

    let targets = page.get_tabs().await?;
    ensure!(
        !targets.is_empty(),
        "Target.getTargets should return targets"
    );

    popup.close().await?;
    page.set_discover_targets(false, None).await?;
    Ok(())
}

async fn page_lifecycle_and_runtime_events(page: &Arc<Page>) -> Result<()> {
    println!("-- page lifecycle and runtime events");

    page.set_lifecycle_events_enabled(true).await?;
    let lifecycle =
        page.wait_for_event::<events::page::LifecycleEventEvent>(Some(EVENT_TIMEOUT_MS));
    page.navigate(
        "data:text/html,%3C!doctype%20html%3E%3Ctitle%3Ecdp-core%20events%3C%2Ftitle%3E%3Ch1%20id%3D%22ready%22%3Eevents%3C%2Fh1%3E",
    )
    .await?;
    let lifecycle = lifecycle.await?;
    ensure!(
        !lifecycle.params.name.is_empty(),
        "lifecycle event should include a name"
    );

    page.wait_for_function("() => document.readyState === 'complete'", Some(5000), None)
        .await?;

    let console_event =
        page.wait_for_event::<events::runtime::ConsoleAPICalledEvent>(Some(EVENT_TIMEOUT_MS));
    let exception_event =
        page.wait_for_event::<events::runtime::ExceptionThrownEvent>(Some(EVENT_TIMEOUT_MS));
    let main_frame = page.main_frame().await?;
    main_frame
        .evaluate(
            r#"
            (() => {
                console.log("cdp-core console event", 42);
                setTimeout(() => {
                    throw new Error("cdp-core async exception event");
                }, 0);
                return true;
            })()
            "#,
        )
        .await?;

    let console = console_event.await?;
    ensure!(
        format_console_args(&console).contains("cdp-core console event"),
        "console event should include emitted argument"
    );

    let exception = exception_event.await?;
    ensure!(
        exception.params.exception_details.text.contains("Uncaught"),
        "exception event should include uncaught exception text"
    );

    Ok(())
}

async fn console_stream_events(page: &Arc<Page>) -> Result<()> {
    println!("-- runtime console event stream");

    let mut console_events = page.on::<events::runtime::ConsoleAPICalledEvent>();
    let main_frame = page.main_frame().await?;
    main_frame
        .evaluate(
            r#"
            (() => {
                console.log("stream log", 1);
                console.info("stream info", { ok: true });
                console.warn("stream warning");
                console.error("stream error", new Error("boom"));
                return true;
            })()
            "#,
        )
        .await?;

    let mut observed = Vec::new();
    for _ in 0..4 {
        let event = timeout(STREAM_TIMEOUT, console_events.next())
            .await
            .context("timed out waiting for Runtime.consoleAPICalled stream")?
            .context("Runtime.consoleAPICalled stream closed")?;
        observed.push(format!(
            "{:?}: {}",
            event.params.r#type,
            format_console_args(&event)
        ));
    }

    ensure!(
        observed.iter().any(|entry| entry.contains("stream log")),
        "console stream should include log event"
    );
    ensure!(
        observed
            .iter()
            .any(|entry| entry.contains("stream warning")),
        "console stream should include warning event"
    );

    Ok(())
}

async fn log_domain_diagnostics(page: &Arc<Page>) -> Result<()> {
    println!("-- optional log domain diagnostics");

    page.domain_manager.enable_log_domain().await?;
    let main_frame = page.main_frame().await?;
    main_frame
        .evaluate(
            r#"
            (() => {
                setTimeout(() => {
                    const start = performance.now();
                    while (performance.now() - start < 250) {}
                }, 100);
                return true;
            })()
            "#,
        )
        .await?;

    match page
        .wait_for_event::<events::log::EntryAddedEvent>(Some(EVENT_TIMEOUT_MS))
        .await
    {
        Ok(entry) => {
            println!(
                "observed Log.entryAdded: {:?}/{:?} {}",
                entry.params.entry.source, entry.params.entry.level, entry.params.entry.text
            );
        }
        Err(err) => {
            println!("Log.entryAdded not observed in this browser run: {err}");
        }
    }

    page.domain_manager.disable_log_domain().await?;
    Ok(())
}

fn format_console_args(event: &events::runtime::ConsoleAPICalledEvent) -> String {
    event
        .params
        .args
        .iter()
        .map(|arg| {
            arg.value
                .as_ref()
                .map(ToString::to_string)
                .or_else(|| arg.description.clone())
                .unwrap_or_else(|| format!("{:?}", arg.r#type))
        })
        .collect::<Vec<_>>()
        .join(", ")
}
