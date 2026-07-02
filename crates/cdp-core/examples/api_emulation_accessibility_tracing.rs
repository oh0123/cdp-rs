#[path = "support/mod.rs"]
mod support;

use anyhow::{Result, ensure};
use cdp_core::{
    AccessibilitySnapshotOptions, EmulationConfig, Geolocation, MediaEmulation,
    MediaFeatureOverride, TracingStartOptions, UserAgentBrand, UserAgentMetadataOverride,
    UserAgentOverride,
};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Emulation, Accessibility, and Tracing manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;
    support::navigate_and_wait(&page, &server.url("/"), "#ready").await?;

    emulation_config(&page).await?;
    accessibility_snapshot(&page).await?;
    tracing_lifecycle(&page).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Emulation, Accessibility, and Tracing manual API example complete ==");
    Ok(())
}

async fn emulation_config(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- emulation config");

    let metadata = UserAgentMetadataOverride::default()
        .with_brand(UserAgentBrand::new("CDP Core", "1"))
        .with_full_version_entry(UserAgentBrand::new("CDP Core", "1.0.0"))
        .with_platform("Linux")
        .with_platform_version("6.0")
        .with_architecture("x86")
        .with_bitness("64")
        .with_mobile(false)
        .with_form_factor("Desktop");

    let user_agent = UserAgentOverride::new("CDP-Core-Manual/1.0")
        .with_accept_language("fr-FR")
        .with_platform("Linux x86_64")
        .with_metadata(metadata);
    let media = MediaEmulation::default()
        .with_media_type("screen")
        .with_feature(MediaFeatureOverride::new("prefers-color-scheme", "dark"));
    let geolocation = Geolocation::new(37.7749, -122.4194)
        .with_accuracy(5.0)
        .with_altitude(10.0)
        .with_altitude_accuracy(1.0)
        .with_heading(90.0)
        .with_speed(1.5);

    let config = EmulationConfig::default()
        .with_user_agent(user_agent)
        .with_locale("fr-FR")
        .with_timezone("Europe/Paris")
        .with_media(media)
        .with_geolocation(geolocation);
    page.emulation().apply_config(&config).await?;

    page.navigate("data:text/html,<h1 id='ready'>emulation</h1>")
        .await?;
    support::wait_visible(page, "#ready").await?;

    let ua = page.evaluate("navigator.userAgent").await?;
    ensure!(
        ua.as_str().unwrap_or_default().contains("CDP-Core-Manual"),
        "user agent override should be visible"
    );
    let language = page.evaluate("navigator.language").await?;
    ensure!(
        language.as_str().unwrap_or_default().starts_with("fr"),
        "locale override should be visible"
    );
    let timezone = page
        .evaluate("Intl.DateTimeFormat().resolvedOptions().timeZone")
        .await?;
    support::require_str(timezone, "Europe/Paris", "timezone override")?;
    let dark = page
        .evaluate("matchMedia('(prefers-color-scheme: dark)').matches")
        .await?;
    support::require_bool(dark, true, "media feature override")?;

    page.set_user_agent(UserAgentOverride::new("CDP-Core-Shortcut/2.0"))
        .await?;
    let ua = page.evaluate("navigator.userAgent").await?;
    ensure!(
        ua.as_str()
            .unwrap_or_default()
            .contains("CDP-Core-Shortcut"),
        "page-level set_user_agent should be visible"
    );

    page.set_geolocation(Geolocation::new(1.0, 2.0).with_accuracy(1.0))
        .await?;
    page.emulation().clear_geolocation().await?;
    page.emulation().reset_timezone().await?;
    page.emulation().set_locale(None).await?;
    page.emulation().clear_media().await?;
    Ok(())
}

async fn accessibility_snapshot(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- accessibility snapshot");

    page.navigate(
        "data:text/html,<main><h1 id='ready'>AX</h1><button id='button'>Run</button><label>Name <input id='name'></label></main>",
    )
    .await?;
    support::wait_visible(page, "#ready").await?;

    let ax = page.accessibility();
    ax.enable().await?;
    let full = ax
        .snapshot(Some(
            AccessibilitySnapshotOptions::default()
                .with_interesting_only(false)
                .with_max_depth(6),
        ))
        .await?;
    ensure!(
        !full.is_empty(),
        "full accessibility snapshot should not be empty"
    );

    let button = page
        .query_selector("#button")
        .await?
        .expect("button should exist");
    let subtree = ax
        .snapshot(Some(
            AccessibilitySnapshotOptions::default()
                .with_root(button)
                .with_interesting_only(false),
        ))
        .await?;
    ensure!(
        !subtree.is_empty(),
        "button accessibility subtree should not be empty"
    );
    ax.disable().await?;
    Ok(())
}

async fn tracing_lifecycle(page: &std::sync::Arc<cdp_core::Page>) -> Result<()> {
    println!("-- tracing lifecycle");

    let tracing = page.tracing();
    let categories = tracing.categories().await?;
    ensure!(
        !categories.is_empty(),
        "tracing categories should not be empty"
    );
    ensure!(!tracing.is_recording().await, "tracing should start idle");

    let trace_path = support::output_path("trace.json")?;
    tracing
        .start(Some(
            TracingStartOptions::default()
                .with_screenshots(true)
                .with_categories(["devtools.timeline", "v8.execute"])
                .with_buffer_usage_reporting_interval_ms(100.0),
        ))
        .await?;
    ensure!(tracing.is_recording().await, "tracing should be active");

    page.navigate("data:text/html,<h1 id='ready'>trace</h1><script>for(let i=0;i<5000;i++){Math.sqrt(i)}</script>")
        .await?;
    support::wait_visible(page, "#ready").await?;
    let result = tracing.stop(Some(trace_path.clone())).await?;
    ensure!(!result.data.is_empty(), "trace data should not be empty");
    ensure!(
        result.saved_path.as_ref() == Some(&trace_path),
        "trace saved_path mismatch"
    );
    support::ensure_non_empty_file(trace_path)?;
    ensure!(
        !tracing.is_recording().await,
        "tracing should be idle after stop"
    );
    Ok(())
}
