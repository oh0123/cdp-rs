#[path = "support/mod.rs"]
mod support;

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use anyhow::{Context, Result, ensure};
use cdp_core::{
    InterceptRequestAction, NetworkControl, NetworkEvent, NetworkInterceptor,
    RequestInterceptorExt, RequestModification, ResponseMock,
};

#[tokio::main]
async fn main() -> Result<()> {
    support::init_tracing();
    println!("== Network local manual API example ==");

    let server = support::LocalServer::start().await?;
    let (browser, page) = support::new_page().await?;

    network_events_and_control(&page, &server).await?;
    request_interception(&page, &server).await?;
    response_monitoring(&page, &server).await?;
    request_helpers(&page, &server).await?;

    page.close().await?;
    browser.disconnect().await?;

    println!("== Network local manual API example complete ==");
    Ok(())
}

async fn network_events_and_control(
    page: &Arc<cdp_core::Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- network events and NetworkControl");

    page.enable_network_monitoring().await?;
    page.clear_browser_cache().await?;
    page.set_cache_disabled(true).await?;
    page.set_bypass_service_worker(true).await?;

    let mut headers = HashMap::new();
    headers.insert("X-CDP-Core".to_string(), "network-control".to_string());
    page.set_extra_http_headers(headers).await?;

    let request_count = Arc::new(AtomicUsize::new(0));
    let response_count = Arc::new(AtomicUsize::new(0));
    let post_request_id = Arc::new(Mutex::new(None::<String>));
    let response_request_id = Arc::new(Mutex::new(None::<String>));

    let requests = Arc::clone(&request_count);
    let responses = Arc::clone(&response_count);
    let post_id = Arc::clone(&post_request_id);
    let response_id = Arc::clone(&response_request_id);
    page.on_network(Arc::new(move |event| match event {
        NetworkEvent::RequestWillBeSent {
            request_id,
            url,
            method,
            ..
        } => {
            requests.fetch_add(1, Ordering::SeqCst);
            if url.contains("/api/echo") && method == "POST" {
                *post_id.lock().expect("post id lock poisoned") = Some(request_id);
            }
        }
        NetworkEvent::ResponseReceived {
            request_id, status, ..
        } => {
            responses.fetch_add(1, Ordering::SeqCst);
            if status == 200 {
                *response_id.lock().expect("response id lock poisoned") = Some(request_id);
            }
        }
        _ => {}
    }))
    .await;

    support::navigate_and_wait(page, &server.url("/"), "#ready").await?;
    let fetch_result = page
        .evaluate(
            r#"
            fetch('/api/echo', {
              method: 'POST',
              headers: { 'content-type': 'text/plain' },
              body: 'posted-body'
            }).then(response => response.json())
            "#,
        )
        .await?;
    ensure!(
        fetch_result["header"] == "network-control",
        "extra HTTP header should be echoed"
    );

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    ensure!(
        request_count.load(Ordering::SeqCst) > 0,
        "expected network request events"
    );
    ensure!(
        response_count.load(Ordering::SeqCst) > 0,
        "expected network response events"
    );
    ensure!(
        page.get_inflight_requests_count() == 0,
        "inflight requests should settle"
    );

    let post_id = post_request_id
        .lock()
        .expect("post id lock poisoned")
        .clone();
    if let Some(id) = post_id {
        let post_data = page.get_request_post_data(id).await?;
        ensure!(
            post_data.post_data.contains("posted-body"),
            "POST data should be retrievable"
        );
    }
    let response_id = response_request_id
        .lock()
        .expect("response id lock poisoned")
        .clone();
    if let Some(id) = response_id {
        let body = page.get_response_body(id).await?;
        ensure!(!body.body.is_empty(), "response body should be retrievable");
    }

    page.set_cache_disabled(false).await?;
    page.set_bypass_service_worker(false).await?;
    Ok(())
}

async fn request_interception(
    page: &Arc<cdp_core::Page>,
    server: &support::LocalServer,
) -> Result<()> {
    println!("-- request interception");

    page.intercept_requests(|request| {
        if request.url.contains("/api/mock") {
            InterceptRequestAction::Fulfill(
                ResponseMock::new(r#"{"mocked":true}"#)
                    .with_status_code(201)
                    .with_header("content-type", "application/json"),
            )
        } else if request.url.contains("/api/echo") {
            InterceptRequestAction::Modify(
                RequestModification::default().with_header("X-CDP-Core", "intercepted"),
            )
        } else if request.url.ends_with("/pixel.png") {
            InterceptRequestAction::Abort("BlockedByClient".to_string())
        } else {
            InterceptRequestAction::Continue
        }
    })
    .await?;

    support::navigate_and_wait(page, &server.url("/"), "#ready").await?;
    let mocked = page
        .evaluate("fetch('/api/mock').then(response => response.json())")
        .await?;
    ensure!(
        mocked["mocked"] == true,
        "fulfilled request should return mocked JSON"
    );

    let modified = page
        .evaluate(
            "fetch('/api/echo', { method: 'POST', body: 'x' }).then(response => response.json())",
        )
        .await?;
    ensure!(
        modified["header"] == "intercepted",
        "modified request header should be echoed"
    );

    page.disable_request_interception().await?;
    Ok(())
}

async fn response_monitoring(
    page: &Arc<cdp_core::Page>,
    _server: &support::LocalServer,
) -> Result<()> {
    println!("-- response monitoring");

    let monitored_count = Arc::new(AtomicUsize::new(0));
    let last_status = Arc::new(Mutex::new(None::<i64>));
    let count = Arc::clone(&monitored_count);
    let status = Arc::clone(&last_status);
    page.monitor_responses(
        |url| url.contains("/api/data"),
        move |response| {
            count.fetch_add(1, Ordering::SeqCst);
            *status.lock().expect("status lock poisoned") = Some(response.status_code);
        },
    )
    .await?;

    page.evaluate("fetch('/api/data').then(response => response.json())")
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    ensure!(
        monitored_count.load(Ordering::SeqCst) >= 1,
        "response monitor should observe /api/data"
    );
    ensure!(
        *last_status.lock().expect("status lock poisoned") == Some(200),
        "response monitor should capture 200 status"
    );

    page.stop_response_monitoring().await?;
    let before = monitored_count.load(Ordering::SeqCst);
    page.evaluate("fetch('/api/data').then(response => response.json())")
        .await?;
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    ensure!(
        monitored_count.load(Ordering::SeqCst) == before,
        "stop_response_monitoring should remove callbacks"
    );

    page.monitor_responses_matching("/api/data", |_| {}).await?;
    page.stop_response_monitoring().await?;
    Ok(())
}

async fn request_helpers(page: &Arc<cdp_core::Page>, server: &support::LocalServer) -> Result<()> {
    println!("-- request helper shortcuts");

    page.intercept_all_requests().await?;
    page.disable_request_interception().await?;

    page.intercept_requests_matching("*://127.0.0.1/*").await?;
    page.disable_request_interception().await?;

    page.block_images().await?;
    support::navigate_and_wait(page, &server.url("/"), "#ready").await?;
    page.disable_request_interception().await?;

    page.block_stylesheets().await?;
    support::navigate_and_wait(page, &server.url("/"), "#ready").await?;
    page.disable_request_interception().await?;

    page.block_urls(["*.png", "*.jpg"]).await?;
    page.set_blocked_url_patterns(Vec::new()).send().await?;
    page.clear_browser_cache()
        .await
        .context("clear cache after helper smoke")?;
    Ok(())
}
