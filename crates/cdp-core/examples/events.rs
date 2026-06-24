/// Event Handling Example
///
/// Demonstrates:
/// - Generic strongly typed event streams with `page.on::<Event>()`
/// - Browser-level Target discovery events
/// - Page lifecycle events
/// - Runtime console and exception events
/// - Optional Log domain diagnostic event subscription
///
/// Run: cargo run -p cdp-core --example events
use cdp_core::{Browser, Result, events};

const EVENT_TIMEOUT_MS: u64 = 5000;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_target(false)
        .try_init();

    println!("=== Event Handling Example ===\n");

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    test_target_events(&browser, &page).await?;
    test_page_and_runtime_events(&page).await?;
    test_log_event_subscription(&page).await?;

    page.close().await?;

    println!("\n=== Event Tests Complete ===");
    Ok(())
}

async fn test_target_events(
    browser: &std::sync::Arc<Browser>,
    page: &std::sync::Arc<cdp_core::Page>,
) -> Result<()> {
    println!("========== Target Events ==========");

    page.set_discover_targets(true, None).await?;
    println!("✓ Target discovery enabled");

    let target_created_event =
        page.wait_for_event::<events::target::TargetCreatedEvent>(Some(EVENT_TIMEOUT_MS));
    let popup = browser.new_page().await?;
    let target_created = target_created_event.await?;

    println!(
        "✓ Target created: type={}, id={}",
        target_created.params.target_info.r#type, target_created.params.target_info.target_id
    );

    let tabs = page.get_tabs().await?;
    println!("✓ Target.getTargets returned {} targets", tabs.len());

    popup.close().await?;
    page.set_discover_targets(false, None).await?;

    Ok(())
}

async fn test_page_and_runtime_events(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("\n========== Page and Runtime Events ==========");

    page.set_lifecycle_events_enabled(true).await?;
    println!("✓ Page lifecycle events enabled");

    let lifecycle_event =
        page.wait_for_event::<events::page::LifecycleEventEvent>(Some(EVENT_TIMEOUT_MS));
    page.navigate(
        "data:text/html,%3C!doctype%20html%3E%3Ctitle%3Ecdp-core%20events%3C%2Ftitle%3E%3Ch1%3Eevents%3C%2Fh1%3E",
    )
    .await?;

    let lifecycle = lifecycle_event.await?;
    println!(
        "✓ Lifecycle event: {} for frame {}",
        lifecycle.params.name, lifecycle.params.frame_id
    );

    page.wait_for_function("() => document.readyState === 'complete'", Some(5000), None)
        .await?;

    let main_frame = page.main_frame().await?;
    let console_event =
        page.wait_for_event::<events::runtime::ConsoleAPICalledEvent>(Some(EVENT_TIMEOUT_MS));
    let exception_event =
        page.wait_for_event::<events::runtime::ExceptionThrownEvent>(Some(EVENT_TIMEOUT_MS));
    main_frame
        .evaluate(
            r#"
            (() => {
                console.log("cdp-core console event", 42);
                setTimeout(() => {
                    throw new Error("cdp-core async exception event");
                }, 0);
                return "runtime events scheduled";
            })()
            "#,
        )
        .await?;

    let console = console_event.await?;
    let exception = exception_event.await?;
    let console_args = console
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
        .join(", ");
    println!(
        "✓ Console event: {:?} [{}]",
        console.params.r#type, console_args
    );

    println!(
        "✓ Exception event: {}",
        exception.params.exception_details.text
    );

    Ok(())
}

async fn test_log_event_subscription(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("\n========== Log Events ==========");

    page.domain_manager.enable_log_domain().await?;
    println!("✓ Log domain enabled");

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
                "✓ Log event: {:?}/{:?} {}",
                entry.params.entry.source, entry.params.entry.level, entry.params.entry.text
            );
        }
        Err(err) => {
            println!("! No Log.entryAdded event observed: {}", err);
            println!("  Console output is exposed through Runtime.consoleAPICalled.");
            println!("  Log.entryAdded is reserved for browser diagnostic entries.");
        }
    }

    page.domain_manager.disable_log_domain().await?;

    Ok(())
}
