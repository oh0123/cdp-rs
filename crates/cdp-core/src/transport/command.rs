use crate::error::Result;
use crate::session::Session;
use crate::transport::websocket_connection::ConnectionInternals;
use cdp_protocol::types::Method;
use std::sync::Arc;
use std::time::Duration;

/// Builder for low-level CDP commands.
///
/// High-level domain APIs should prefer typed options and `Result<T>` returns. This builder is
/// reserved for native CDP command wrappers and escape hatches where callers may need generated
/// protocol parameters and per-command timeout control.
pub struct CdpCommandBuilder<'a, M>
where
    M: Method + serde::Serialize,
{
    target: CdpCommandTarget<'a>,
    method: M,
    timeout: Option<Duration>,
}

enum CdpCommandTarget<'a> {
    Session(&'a Session),
    Root(&'a Session),
    Browser(&'a Arc<ConnectionInternals>),
}

impl<'a, M> CdpCommandBuilder<'a, M>
where
    M: Method + serde::Serialize,
{
    pub(crate) fn session(session: &'a Session, method: M) -> Self {
        Self {
            target: CdpCommandTarget::Session(session),
            method,
            timeout: None,
        }
    }

    pub(crate) fn root(session: &'a Session, method: M) -> Self {
        Self {
            target: CdpCommandTarget::Root(session),
            method,
            timeout: None,
        }
    }

    pub(crate) fn browser(internals: &'a Arc<ConnectionInternals>, method: M) -> Self {
        Self {
            target: CdpCommandTarget::Browser(internals),
            method,
            timeout: None,
        }
    }

    /// Sets a timeout for this command only.
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Clears a previously configured per-command timeout.
    pub fn clear_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Mutates the generated CDP parameter object before sending.
    pub fn set_params<F>(mut self, configure: F) -> Self
    where
        F: FnOnce(&mut M),
    {
        configure(&mut self.method);
        self
    }

    /// Replaces the generated CDP parameter object.
    pub fn replace_params(mut self, method: M) -> Self {
        self.method = method;
        self
    }

    /// Returns the generated CDP parameter object.
    pub fn params(&self) -> &M {
        &self.method
    }

    /// Returns the generated CDP parameter object for in-place mutation.
    pub fn params_mut(&mut self) -> &mut M {
        &mut self.method
    }

    /// Sends the command and deserializes the generated return object.
    pub async fn send(self) -> Result<M::ReturnObject> {
        match self.target {
            CdpCommandTarget::Session(session) => {
                session.send_command(self.method, self.timeout).await
            }
            CdpCommandTarget::Root(session) => {
                session.send_root_command(self.method, self.timeout).await
            }
            CdpCommandTarget::Browser(internals) => {
                internals.send(self.method, None, self.timeout).await
            }
        }
    }
}
