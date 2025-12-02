use thiserror::Error;

/// Convenient result type alias used across the crate.
pub type Result<T> = std::result::Result<T, CdpError>;

/// Unified error type for the `cdp-core` crate.
#[derive(Debug, Error)]
pub enum CdpError {
    #[error("Browser error: {message}")]
    Browser { message: String },
    #[error("Connection error: {message}")]
    Connection { message: String },
    #[error("Page error: {message}")]
    Page { message: String },
    #[error("Frame error: {message}")]
    Frame { message: String },
    #[error("Element error: {message}")]
    Element { message: String },
    #[error("Network error: {message}")]
    Network { message: String },
    #[error("Session error: {message}")]
    Session { message: String },
    #[error("Storage error: {message}")]
    Storage { message: String },
    #[error("Cookie error: {message}")]
    Cookie { message: String },
    #[error("Tooling error: {message}")]
    Tool { message: String },
    #[error("Protocol error: {message}")]
    Protocol { message: String },
    #[error("WebSocket resolution error: {message}")]
    Ws { message: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("JSON processing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Channel closed unexpectedly: {0}")]
    ChannelClosed(#[from] tokio::sync::oneshot::error::RecvError),
    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

impl CdpError {
    pub fn browser(message: impl Into<String>) -> Self {
        Self::Browser {
            message: message.into(),
        }
    }

    pub fn connection(message: impl Into<String>) -> Self {
        Self::Connection {
            message: message.into(),
        }
    }

    pub fn page(message: impl Into<String>) -> Self {
        Self::Page {
            message: message.into(),
        }
    }

    pub fn frame(message: impl Into<String>) -> Self {
        Self::Frame {
            message: message.into(),
        }
    }

    pub fn element(message: impl Into<String>) -> Self {
        Self::Element {
            message: message.into(),
        }
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    pub fn session(message: impl Into<String>) -> Self {
        Self::Session {
            message: message.into(),
        }
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
        }
    }

    pub fn cookie(message: impl Into<String>) -> Self {
        Self::Cookie {
            message: message.into(),
        }
    }

    pub fn tool(message: impl Into<String>) -> Self {
        Self::Tool {
            message: message.into(),
        }
    }

    pub fn protocol(message: impl Into<String>) -> Self {
        Self::Protocol {
            message: message.into(),
        }
    }

    pub fn ws(message: impl Into<String>) -> Self {
        Self::Ws {
            message: message.into(),
        }
    }
}
