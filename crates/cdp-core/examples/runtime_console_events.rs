/// Runtime Console Events Example
///
/// Demonstrates:
/// - Listening to `Runtime.consoleAPICalled` with `page.on::<Event>()`
/// - Waiting for one console event with `page.wait_for_event::<Event>()`
/// - Reading console event type and arguments
///
/// Run: cargo run -p cdp-core --example runtime_console_events
use cdp_core::{Browser, CdpError, Result, events};
use futures_util::StreamExt;
use tokio::time::{Duration, timeout};

const EVENT_TIMEOUT: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .try_init();

    println!("=== Runtime Console Events Example ===\n");

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    page.navigate(
        "data:text/html,%3C!doctype%20html%3E%3Ctitle%3ERuntime%20Console%20Events%3C%2Ftitle%3E%3Ch1%3Econsole%3C%2Fh1%3E",
    )
    .await?;
    page.wait_for_function("() => document.readyState === 'complete'", Some(5000), None)
        .await?;

    collect_console_stream(&page).await?;
    wait_for_one_console_event(&page).await?;

    page.close().await?;

    println!("\n=== Runtime Console Events Complete ===");
    Ok(())
}

async fn collect_console_stream(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("========== Continuous Console Stream ==========");

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

    for _ in 0..4 {
        let event = timeout(EVENT_TIMEOUT, console_events.next())
            .await
            .map_err(|_| CdpError::page("Timed out waiting for Runtime.consoleAPICalled"))?
            .ok_or_else(|| CdpError::page("Console event stream closed"))?;

        println!(
            "✓ {:?}: {}",
            event.params.r#type,
            format_console_args(&event)
        );
    }

    Ok(())
}

async fn wait_for_one_console_event(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("\n========== One-shot Console Wait ==========");

    let console_event = page.wait_for_event::<events::runtime::ConsoleAPICalledEvent>(Some(5000));
    let main_frame = page.main_frame().await?;
    main_frame
        .evaluate(
            r#"
            (() => {
                console.debug("one-shot debug", 2026);
                return true;
            })()
            "#,
        )
        .await?;

    let event = console_event.await?;
    println!(
        "✓ {:?}: {}",
        event.params.r#type,
        format_console_args(&event)
    );

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
