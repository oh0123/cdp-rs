/// Page Screencast Example
///
/// Demonstrates:
/// - Starting `Page.startScreencast` with high-level options
/// - Receiving `Page.screencastFrame` events
/// - Acknowledging each frame with `Page.screencastFrameAck`
/// - Stopping the screencast
///
/// Run: cargo run --example screencast
use base64::{Engine as _, engine::general_purpose::STANDARD};
use cdp_core::*;

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

    page.start_screencast(
        ScreencastOptions::default()
            .with_quality(80)
            .with_max_width(1280)
            .with_max_height(720)
            .with_every_nth_frame(1),
    )
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

        page.screencast_frame_ack(frame.params.session_id).await?;
    }

    page.stop_screencast().await?;
    page.cleanup().await?;
    browser.disconnect().await?;

    Ok(())
}
