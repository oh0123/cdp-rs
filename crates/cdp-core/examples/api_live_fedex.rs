#[path = "support/mod.rs"]
mod support;

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

use anyhow::{Result, bail, ensure};
use cdp_core::{WaitForNavigationOptions, WaitUntil};

const FEDEX_TRACKING_URL: &str =
    "https://www.fedex.com/fedextrack/?trknbr=885215382626&trkqual=12029~885215382626~FDEG";

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    require_live()?;
    println!("== FedEx live manual API example ==");

    let (browser, page) = support::new_page().await?;

    let captures = Arc::new(AtomicUsize::new(0));
    let last_body_len = Arc::new(Mutex::new(0_usize));
    let capture_count = Arc::clone(&captures);
    let body_len = Arc::clone(&last_body_len);
    page.monitor_responses(
        |url| url.contains("/track/v2/shipments"),
        move |response| {
            captures.fetch_add(1, Ordering::SeqCst);
            if let Some(body) = &response.body {
                *body_len.lock().expect("body length lock poisoned") = body.len();
            }
            println!(
                "FedEx response: status={} body_bytes={}",
                response.status_code,
                response.body.as_ref().map_or(0, String::len)
            );
        },
    )
    .await?;

    page.navigate(FEDEX_TRACKING_URL).await?;
    page.wait_for_navigation(Some(
        WaitForNavigationOptions::default()
            .with_wait_until(WaitUntil::NetworkIdle0)
            .with_timeout_ms(30_000),
    ))
    .await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    ensure!(
        capture_count.load(Ordering::SeqCst) > 0,
        "FedEx live request monitor should capture /track/v2/shipments"
    );
    ensure!(
        *last_body_len.lock().expect("body length lock poisoned") > 0,
        "FedEx live response monitor should capture a response body"
    );

    page.stop_response_monitoring().await?;
    page.close().await?;
    browser.disconnect().await?;

    println!("== FedEx live manual API example complete ==");
    Ok(())
}

fn require_live() -> Result<()> {
    if std::env::var("CDP_RS_LIVE").as_deref() != Ok("1") {
        bail!("set CDP_RS_LIVE=1 to run this external-network live example");
    }
    Ok(())
}
