/// Page Screencast Example
///
/// Demonstrates:
/// - Starting `Page.startScreencast` with chain-configured command parameters
/// - Receiving `Page.screencastFrame` events
/// - Acknowledging each frame with `Page.screencastFrameAck`
/// - Stopping the screencast
///
/// Run: cargo run --example screencast
use base64::{Engine as _, engine::general_purpose::STANDARD};
use cdp_core::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .with_target(false)
        .init();

    let browser = Browser::launcher().launch().await?;
    let page = browser.new_page().await?;

    page.navigate("https://example.com").await?;
    page.wait_for_navigation(Some(
        WaitForNavigationOptions::default()
            .with_timeout_ms(10_000)
            .with_wait_until(WaitUntil::NetworkIdle2),
    ))
    .await?;

    page.start_screencast()
        .set_params(|params| {
            params.quality = Some(80);
            params.max_width = Some(1280);
            params.max_height = Some(720);
            params.every_nth_frame = Some(1);
        })
        .set_timeout(Duration::from_secs(10))
        .send()
        .await?;

    tokio::fs::create_dir_all("target/screencast").await?;

    for index in 0..3 {
        let frame = page.wait_for_screencast_frame().await?;
        let bytes = STANDARD
            .decode(frame.params.data.as_bytes())
            .map_err(|err| CdpError::page(format!("failed to decode screencast frame: {err}")))?;
        let path = format!("target/screencast/frame-{index}.jpg");
        tokio::fs::write(&path, bytes).await?;
        println!("Saved {path}");

        page.screencast_frame_ack(frame.params.session_id)
            .send()
            .await?;
    }

    page.stop_screencast().send().await?;
    page.cleanup().await?;
    browser.disconnect().await?;

    Ok(())
}
