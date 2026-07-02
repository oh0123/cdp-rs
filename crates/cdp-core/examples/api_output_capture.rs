#[path = "support/mod.rs"]
mod support;

use anyhow::{Result, ensure};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use cdp_core::{PageScreenshotOptions, ScreencastOptions, protocol};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Output capture manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;
    support::navigate_and_wait(&page, &server.url("/"), "#ready").await?;

    screenshots(&page).await?;
    pdf_and_mhtml(&page).await?;
    screencast(&page).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Output capture manual API example complete ==");
    Ok(())
}

async fn screenshots(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- screenshots");

    let full = support::output_path("page-full.png")?;
    let viewport = support::output_path("page-viewport.png")?;
    let no_dpr = support::output_path("page-no-dpr.png")?;

    page.screenshot(PageScreenshotOptions::default().full_page().save_to(&full))
        .await?;
    page.screenshot(
        PageScreenshotOptions::default()
            .viewport()
            .save_to(&viewport),
    )
    .await?;
    page.screenshot(
        PageScreenshotOptions::default()
            .full_page()
            .auto_resolve_dpr(false)
            .save_to(&no_dpr),
    )
    .await?;

    support::ensure_non_empty_file(full)?;
    support::ensure_non_empty_file(viewport)?;
    support::ensure_non_empty_file(no_dpr)?;
    Ok(())
}

async fn pdf_and_mhtml(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- PDF and MHTML");

    let snapshot = page.capture_snapshot().await?;
    ensure!(
        snapshot.contains("Manual fixture ready"),
        "capture_snapshot should include fixture text"
    );
    let mhtml = support::output_path("page.mhtml")?;
    std::fs::write(&mhtml, snapshot)?;
    support::ensure_non_empty_file(&mhtml)?;

    let pdf_path = support::output_path("page.pdf")?;
    let pdf = page
        .print_to_pdf()
        .set_params(|params| {
            params.print_background = Some(true);
            params.display_header_footer = Some(false);
            params.prefer_css_page_size = Some(true);
        })
        .send()
        .await?;
    let bytes = STANDARD.decode(pdf.data.as_bytes())?;
    std::fs::write(&pdf_path, bytes)?;
    support::ensure_non_empty_file(pdf_path)?;

    let _defaults = cdp_core::Page::default_print_to_pdf_options();
    Ok(())
}

async fn screencast(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- screencast");

    page.evaluate(
        r#"
        (() => {
          let box = document.getElementById('box');
          let x = 0;
          setInterval(() => {
            x = (x + 5) % 120;
            box.style.transform = `translateX(${x}px)`;
          }, 50);
          return true;
        })()
        "#,
    )
    .await?;

    page.start_screencast(
        ScreencastOptions::default()
            .with_format(protocol::page::StartScreencastFormatOption::Jpeg)
            .with_quality(80)
            .with_max_width(800)
            .with_max_height(600)
            .with_every_nth_frame(1),
    )
    .await?;

    for index in 0..3 {
        let frame = page.wait_for_screencast_frame().await?;
        let bytes = STANDARD.decode(frame.params.data.as_bytes())?;
        let path = support::output_path(&format!("screencast-frame-{index}.jpg"))?;
        std::fs::write(&path, bytes)?;
        support::ensure_non_empty_file(path)?;
        page.screencast_frame_ack(frame.params.session_id).await?;
    }

    page.stop_screencast().await?;
    Ok(())
}
