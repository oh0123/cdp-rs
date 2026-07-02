#![allow(dead_code)]

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Result, bail};
use cdp_core::{
    Browser, Page, WaitForSelectorOptions, protocol::page::events::LoadEventFiredEvent,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::oneshot,
    task::JoinHandle,
};

pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "cdp_core=info,warn".to_string()),
        )
        .with_target(false)
        .try_init();
}

pub async fn launch_browser() -> Result<Arc<Browser>> {
    Browser::launcher()
        .launch()
        .await
        .context("failed to launch browser")
}

pub async fn new_page() -> Result<(Arc<Browser>, Arc<Page>)> {
    let browser = launch_browser().await?;
    let page = browser.new_page().await?;
    Ok((browser, page))
}

pub fn test_asset_url(name: &str) -> Result<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test-assets")
        .join(name);
    if !path.exists() {
        bail!("missing test asset: {}", path.display());
    }
    Ok(format!("file://{}", path.display()))
}

pub fn output_path(name: &str) -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .ancestors()
        .nth(2)
        .context("failed to resolve workspace root from CARGO_MANIFEST_DIR")?;
    let dir = workspace_root.join("target").join("manual-examples");
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(name))
}

pub fn ensure_non_empty_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path)
        .with_context(|| format!("expected output file at {}", path.display()))?;
    if metadata.len() == 0 {
        bail!("output file is empty: {}", path.display());
    }
    Ok(())
}

pub async fn wait_visible(page: &Arc<Page>, selector: &str) -> Result<()> {
    page.wait_for_selector(
        selector,
        Some(
            WaitForSelectorOptions::default()
                .with_timeout_ms(10_000)
                .with_visible(true),
        ),
    )
    .await?;
    Ok(())
}

pub async fn navigate_and_wait(page: &Arc<Page>, url: &str, ready_selector: &str) -> Result<()> {
    page.navigate(url).await?;
    wait_visible(page, ready_selector).await
}

pub async fn wait_load(page: &Arc<Page>) -> Result<LoadEventFiredEvent> {
    Ok(page.wait_for_loaded().await?)
}

pub fn require_str(value: serde_json::Value, expected: &str, label: &str) -> Result<()> {
    if value.as_str() != Some(expected) {
        bail!("{label}: expected {expected:?}, got {value:?}");
    }
    Ok(())
}

pub fn require_bool(value: serde_json::Value, expected: bool, label: &str) -> Result<()> {
    if value.as_bool() != Some(expected) {
        bail!("{label}: expected {expected}, got {value:?}");
    }
    Ok(())
}

pub fn require_u64(value: serde_json::Value, expected: u64, label: &str) -> Result<()> {
    if value.as_u64() != Some(expected) {
        bail!("{label}: expected {expected}, got {value:?}");
    }
    Ok(())
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub struct LocalServer {
    base_url: String,
    shutdown: Option<oneshot::Sender<()>>,
    handle: JoinHandle<()>,
}

impl LocalServer {
    pub async fn start() -> Result<Self> {
        let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
        let addr = listener.local_addr()?;
        let base_url = format!("http://{}", addr);
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    _ = &mut shutdown_rx => break,
                    accepted = listener.accept() => {
                        match accepted {
                            Ok((stream, _)) => {
                                tokio::spawn(async move {
                                    if let Err(err) = handle_connection(stream).await {
                                        tracing::warn!("local example server connection failed: {err:?}");
                                    }
                                });
                            }
                            Err(err) => {
                                tracing::warn!("local example server accept failed: {err:?}");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            base_url,
            shutdown: Some(shutdown_tx),
            handle,
        })
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl Drop for LocalServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        self.handle.abort();
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request = read_request(&mut stream).await?;
    let response = route_request(&request);
    stream.write_all(&response).await?;
    stream.shutdown().await?;
    Ok(())
}

async fn read_request(stream: &mut TcpStream) -> Result<HttpRequest> {
    let mut buffer = Vec::new();
    let mut temp = [0_u8; 1024];
    let header_end = loop {
        let read = stream.read(&mut temp).await?;
        if read == 0 {
            bail!("connection closed before headers");
        }
        buffer.extend_from_slice(&temp[..read]);
        if let Some(pos) = find_header_end(&buffer) {
            break pos;
        }
        if buffer.len() > 64 * 1024 {
            bail!("request headers too large");
        }
    };

    let header_bytes = &buffer[..header_end];
    let header_text = String::from_utf8_lossy(header_bytes);
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().context("missing request line")?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().unwrap_or_default().to_string();
    let raw_path = request_parts.next().unwrap_or("/").to_string();
    let path = raw_path.split('?').next().unwrap_or("/").to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, value)) = line.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    let content_length = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = buffer[header_end + 4..].to_vec();
    while body.len() < content_length {
        let read = stream.read(&mut temp).await?;
        if read == 0 {
            break;
        }
        body.extend_from_slice(&temp[..read]);
    }
    body.truncate(content_length);

    Ok(HttpRequest {
        method,
        path,
        headers,
        body,
    })
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer.windows(4).position(|window| window == b"\r\n\r\n")
}

fn route_request(request: &HttpRequest) -> Vec<u8> {
    match request.path.as_str() {
        "/" | "/index.html" => html_response(main_page()),
        "/navigation-a" => html_response(navigation_page("A", "/navigation-b")),
        "/navigation-b" => html_response(navigation_page("B", "/navigation-c")),
        "/navigation-c" => html_response(navigation_page("C", "/navigation-a")),
        "/frame-parent" => html_response(frame_parent_page()),
        "/frame-child" => html_response(frame_child_page()),
        "/frame-grandchild" => html_response(frame_grandchild_page()),
        "/download.txt" => response(
            200,
            "text/plain",
            b"manual example download\n",
            &[(
                "Content-Disposition",
                "attachment; filename=\"download.txt\"",
            )],
        ),
        "/style.css" => response(
            200,
            "text/css",
            b"body { --cdp-style-loaded: yes; } #styled { color: rgb(12, 34, 56); }",
            &[],
        ),
        "/app.js" => response(
            200,
            "application/javascript",
            b"window.__localServerScriptLoaded = true;",
            &[],
        ),
        "/pixel.png" => response(200, "image/png", png_1x1(), &[]),
        "/api/data" => response(
            200,
            "application/json",
            br#"{"ok":true,"source":"local","items":[1,2,3]}"#,
            &[("X-Example-Response", "api-data")],
        ),
        "/api/echo" => {
            let header = request
                .headers
                .get("x-cdp-core")
                .cloned()
                .unwrap_or_default();
            let body = String::from_utf8_lossy(&request.body);
            let payload = serde_json::json!({
                "method": request.method,
                "header": header,
                "body": body,
            });
            response(
                200,
                "application/json",
                payload.to_string().as_bytes(),
                &[("X-Echoed-Header", "x-cdp-core")],
            )
        }
        _ => response(404, "text/plain", b"not found", &[]),
    }
}

fn response(status: u16, content_type: &str, body: &[u8], headers: &[(&str, &str)]) -> Vec<u8> {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "OK",
    };
    let mut response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    for (name, value) in headers {
        response.push_str(name);
        response.push_str(": ");
        response.push_str(value);
        response.push_str("\r\n");
    }
    response.push_str("\r\n");

    let mut bytes = response.into_bytes();
    bytes.extend_from_slice(body);
    bytes
}

fn html_response(html: String) -> Vec<u8> {
    response(200, "text/html; charset=utf-8", html.as_bytes(), &[])
}

fn main_page() -> String {
    r#"<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>CDP Core Manual Fixture</title>
  <link rel="stylesheet" href="/style.css">
  <script src="/app.js"></script>
  <style>
    body { font-family: sans-serif; }
    #drop-zone { width: 180px; height: 80px; border: 2px solid #444; }
    #box { width: 120px; height: 60px; padding: 10px; border: 5px solid #333; margin: 8px; background: #d9f0ff; }
    #hidden-target { display: none; }
    #overlay { position: fixed; inset: 0; display: none; background: rgba(0,0,0,.1); }
    #drag-target { position: absolute; left: 20px; top: 360px; width: 50px; height: 50px; background: #f7c948; }
  </style>
</head>
<body>
  <h1 id="ready">Manual fixture ready</h1>
  <button id="click-button">Click me</button>
  <button id="double-button">Double click me</button>
  <button id="disabled-button" disabled>Disabled</button>
  <button id="toggle-visible">Toggle visible</button>
  <input id="text-input" value="initial">
  <textarea id="textarea"></textarea>
  <input id="file-input" type="file">
  <div id="click-count">0</div>
  <div id="double-count">0</div>
  <div id="hold-count">0</div>
  <div id="hidden-target">Now visible</div>
  <div id="box" data-kind="box">Box text <span>inner span</span></div>
  <div id="styled">Styled text</div>
  <div id="event-log"></div>
  <div id="shadow-host"></div>
  <div id="drag-target" draggable="true">drag</div>
  <img id="pixel" src="/pixel.png">
  <div id="overlay"></div>
  <script>
    const log = (message) => {
      const entry = document.createElement('div');
      entry.className = 'event';
      entry.textContent = message;
      document.getElementById('event-log').appendChild(entry);
    };
    let clicks = 0;
    let doubles = 0;
    let holds = 0;
    document.getElementById('click-button').addEventListener('click', () => {
      clicks++;
      document.getElementById('click-count').textContent = String(clicks);
      log('click');
    });
    document.getElementById('double-button').addEventListener('dblclick', () => {
      doubles++;
      document.getElementById('double-count').textContent = String(doubles);
      log('double');
    });
    document.getElementById('toggle-visible').addEventListener('click', () => {
      document.getElementById('hidden-target').style.display = 'block';
    });
    document.getElementById('box').addEventListener('contextmenu', (event) => {
      event.preventDefault();
      log('contextmenu');
    });
    document.getElementById('box').addEventListener('mousedown', () => {
      setTimeout(() => {
        if (document.activeElement !== document.getElementById('text-input')) {
          holds++;
          document.getElementById('hold-count').textContent = String(holds);
        }
      }, 250);
    });
    const shadow = document.getElementById('shadow-host').attachShadow({mode: 'open'});
    shadow.innerHTML = '<section class="shadow-card"><input id="shadow-input" class="inner" value="shadow"><span class="inner">shadow text</span></section>';
    document.addEventListener('keydown', (event) => log('key:' + event.key + ':' + event.ctrlKey));
    document.getElementById('file-input').addEventListener('change', (event) => {
      log('file:' + Array.from(event.target.files).map(file => file.name).join(','));
    });
  </script>
</body>
</html>"#
        .to_string()
}

fn navigation_page(name: &str, next: &str) -> String {
    format!(
        r#"<!doctype html>
<html><head><title>Navigation {name}</title></head>
<body>
  <h1 id="ready">Navigation {name}</h1>
  <a id="next-link" href="{next}">Next</a>
</body></html>"#
    )
}

fn frame_parent_page() -> String {
    r#"<!doctype html>
<html><head><title>Frame Parent</title></head>
<body>
  <h1 id="ready">Frame Parent</h1>
  <iframe id="child-frame" name="child-frame" src="/frame-child"></iframe>
</body></html>"#
        .to_string()
}

fn frame_child_page() -> String {
    r#"<!doctype html>
<html><head><title>Frame Child</title></head>
<body>
  <h2 id="child-ready">Frame Child</h2>
  <button class="frame-button">Frame button</button>
  <iframe id="grandchild-frame" name="grandchild-frame" src="/frame-grandchild"></iframe>
</body></html>"#
        .to_string()
}

fn frame_grandchild_page() -> String {
    r#"<!doctype html>
<html><head><title>Frame Grandchild</title></head>
<body>
  <h3 id="grandchild-ready">Frame Grandchild</h3>
  <input class="nested-input" value="nested">
</body></html>"#
        .to_string()
}

fn png_1x1() -> &'static [u8] {
    &[
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f,
        0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
    ]
}
