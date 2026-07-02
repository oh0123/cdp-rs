#[path = "support/mod.rs"]
mod support;

use std::sync::Arc;

use anyhow::{Result, bail, ensure};
use cdp_core::{Browser, BrowserContextOptions, ElementScreenshotOptions, PageScreenshotOptions};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    require_live()?;
    println!("== Amazon live manual API example ==");

    let browser = support::launch_browser().await?;

    amazon_home_selector_and_screenshot(&browser).await?;
    concurrent_product_contexts(&browser).await?;

    browser.disconnect().await?;
    println!("== Amazon live manual API example complete ==");
    Ok(())
}

fn require_live() -> Result<()> {
    if std::env::var("CDP_RS_LIVE").as_deref() != Ok("1") {
        bail!("set CDP_RS_LIVE=1 to run this external-network live example");
    }
    Ok(())
}

async fn amazon_home_selector_and_screenshot(browser: &Arc<Browser>) -> Result<()> {
    println!("-- Amazon home selector and element screenshot");

    let context = browser
        .new_context(BrowserContextOptions::default())
        .await?;
    let page = context.new_page().await?;

    let result = async {
        page.navigate("https://www.amazon.com").await?;
        let element = page
            .wait_for_selector(
                "#nav-search",
                Some(
                    cdp_core::WaitForSelectorOptions::default()
                        .with_timeout_ms(15_000)
                        .with_visible(true),
                ),
            )
            .await?;
        let screenshot = support::output_path("live-amazon-nav-search.png")?;
        element
            .screenshot(ElementScreenshotOptions::default().save_to(&screenshot))
            .await?;
        support::ensure_non_empty_file(&screenshot)?;
        Ok::<_, anyhow::Error>(())
    }
    .await;

    let page_cleanup = page.cleanup().await;
    let context_cleanup = context.close().await;
    result?;
    page_cleanup?;
    context_cleanup?;
    Ok(())
}

async fn concurrent_product_contexts(browser: &Arc<Browser>) -> Result<()> {
    println!("-- concurrent Amazon product contexts");

    let product_urls = [
        "https://www.amazon.com/dp/B08CBVGXZC",
        "https://www.amazon.com/dp/B0BDF8CVBN",
    ];

    let mut tasks = JoinSet::new();
    for (index, url) in product_urls.iter().enumerate() {
        let browser = Arc::clone(browser);
        let url = url.to_string();
        tasks.spawn(async move { visit_product_page(browser, index + 1, url).await });
    }

    let mut completed = 0_usize;
    while let Some(result) = tasks.join_next().await {
        let info = result??;
        ensure!(
            !info.title.is_empty(),
            "product page title should not be empty"
        );
        support::ensure_non_empty_file(&info.screenshot_path)?;
        println!("context {}: {}", info.context_id, info.title);
        completed += 1;
    }

    ensure!(
        completed == product_urls.len(),
        "all live product contexts should complete"
    );
    Ok(())
}

#[derive(Debug)]
struct PageInfo {
    context_id: usize,
    title: String,
    screenshot_path: std::path::PathBuf,
}

async fn visit_product_page(
    browser: Arc<Browser>,
    context_id: usize,
    url: String,
) -> Result<PageInfo> {
    let context = browser
        .new_context(BrowserContextOptions::default())
        .await?;
    let page = context.new_page().await?;

    let result = async {
        page.navigate(&url).await?;
        page.wait_for_loaded().await?;
        let title = page
            .get_title()
            .await
            .unwrap_or_else(|_| "Unknown".to_string());
        let screenshot_path =
            support::output_path(&format!("live-amazon-context-{context_id}.png"))?;
        page.screenshot(PageScreenshotOptions::default().save_to(&screenshot_path))
            .await?;
        Ok::<PageInfo, anyhow::Error>(PageInfo {
            context_id,
            title,
            screenshot_path,
        })
    }
    .await;

    let page_cleanup = page.cleanup().await;
    let context_cleanup = context.close().await;
    let info = result?;
    page_cleanup?;
    context_cleanup?;
    Ok(info)
}
