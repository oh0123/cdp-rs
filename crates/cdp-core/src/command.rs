use crate::error::Result;
use crate::session::Session;
use crate::transport::websocket_connection::ConnectionInternals;
use cdp_protocol::types::Method;
use std::sync::Arc;
use std::time::Duration;

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

    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn clear_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    pub fn set_params<F>(mut self, configure: F) -> Self
    where
        F: FnOnce(&mut M),
    {
        configure(&mut self.method);
        self
    }

    pub fn replace_params(mut self, method: M) -> Self {
        self.method = method;
        self
    }

    pub fn params(&self) -> &M {
        &self.method
    }

    pub fn params_mut(&mut self) -> &mut M {
        &mut self.method
    }

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
