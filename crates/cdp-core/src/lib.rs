#![allow(dead_code)]

mod session;
mod transport;

pub mod accessibility;
pub mod browser;
pub mod command;
pub mod domain_manager;
pub mod emulation;
pub mod error;
pub mod input;
pub mod network;
pub mod page;
pub mod storage;
pub mod tracing;

/// Default capacity for event channels.
pub const DEFAULT_CHANNEL_CAPACITY: usize = 128;

/// Strongly typed CDP events that can be used with [`Page::on`].
pub mod events {
    /// DOM domain events.
    pub mod dom {
        pub use cdp_protocol::dom::events::{
            AttributeModifiedEvent, AttributeRemovedEvent, CharacterDataModifiedEvent,
            ChildNodeCountUpdatedEvent, ChildNodeInsertedEvent, ChildNodeRemovedEvent,
            DocumentUpdatedEvent, SetChildNodesEvent, ShadowRootPoppedEvent, ShadowRootPushedEvent,
        };
    }

    /// Fetch domain events.
    pub mod fetch {
        pub use cdp_protocol::fetch::events::{AuthRequiredEvent, RequestPausedEvent};
    }

    /// Inspector domain events.
    pub mod inspector {
        pub use cdp_protocol::inspector::events::{
            DetachedEvent, TargetCrashedEvent, TargetReloadedAfterCrashEvent,
        };
    }

    /// Log domain events.
    ///
    /// `Log.entryAdded` is intended for browser diagnostic entries such as network, security,
    /// deprecation, intervention, and violation messages. It is not the primary stream for
    /// JavaScript console output; use [`crate::events::runtime::ConsoleAPICalledEvent`] for `console.log`,
    /// `console.warn`, `console.error`, and related console API calls.
    pub mod log {
        pub use cdp_protocol::log::events::EntryAddedEvent;
    }

    /// Network domain events.
    pub mod network {
        pub use cdp_protocol::network::events::{
            DataReceivedEvent, EventSourceMessageReceivedEvent, LoadingFailedEvent,
            LoadingFinishedEvent, RequestServedFromCacheEvent, RequestWillBeSentEvent,
            RequestWillBeSentExtraInfoEvent, ResponseReceivedEarlyHintsEvent,
            ResponseReceivedEvent, ResponseReceivedExtraInfoEvent, WebSocketClosedEvent,
            WebSocketCreatedEvent, WebSocketFrameErrorEvent, WebSocketFrameReceivedEvent,
            WebSocketFrameSentEvent, WebSocketHandshakeResponseReceivedEvent,
            WebSocketWillSendHandshakeRequestEvent,
        };
    }

    /// Page domain events.
    pub mod page {
        pub use cdp_protocol::page::events::{
            BackForwardCacheNotUsedEvent, DomContentEventFiredEvent, DownloadProgressEvent,
            DownloadWillBeginEvent, FileChooserOpenedEvent, FrameAttachedEvent,
            FrameClearedScheduledNavigationEvent, FrameDetachedEvent, FrameNavigatedEvent,
            FrameRequestedNavigationEvent, FrameScheduledNavigationEvent, FrameStartedLoadingEvent,
            FrameStartedNavigatingEvent, FrameStoppedLoadingEvent, FrameSubtreeWillBeDetachedEvent,
            JavascriptDialogClosedEvent, JavascriptDialogOpeningEvent, LifecycleEventEvent,
            LoadEventFiredEvent, NavigatedWithinDocumentEvent, ScreencastFrameEvent,
            ScreencastVisibilityChangedEvent, WindowOpenEvent,
        };
    }

    /// Runtime domain events.
    pub mod runtime {
        pub use cdp_protocol::runtime::events::{
            BindingCalledEvent, ConsoleAPICalledEvent, ExceptionRevokedEvent, ExceptionThrownEvent,
            ExecutionContextCreatedEvent, ExecutionContextDestroyedEvent,
            ExecutionContextsClearedEvent, InspectRequestedEvent,
        };
    }

    /// Target domain events.
    pub mod target {
        pub use cdp_protocol::target::events::{
            AttachedToTargetEvent, DetachedFromTargetEvent, ReceivedMessageFromTargetEvent,
            TargetCrashedEvent, TargetCreatedEvent, TargetDestroyedEvent, TargetInfoChangedEvent,
        };
    }

    /// Tracing domain events.
    pub mod tracing {
        pub use cdp_protocol::tracing::events::{
            BufferUsageEvent, DataCollectedEvent, TracingCompleteEvent,
        };
    }
}

pub use accessibility::{
    AccessibilityController, AccessibilityNode, AccessibilityProperty, AccessibilitySnapshot,
    AccessibilitySnapshotOptions,
};
pub use browser::manager::{
    Browser, BrowserContext, BrowserContextOptions, DownloadBehavior, DownloadOptions,
    PermissionGrant, PermissionOverride,
};
pub use cdp_protocol as protocol;
pub use cdp_protocol::{
    browser::{
        Bounds as BrowserWindowBounds, GetVersionReturnObject as BrowserVersion,
        GetWindowForTargetReturnObject, PermissionDescriptor, PermissionSetting, PermissionType,
        WindowId as BrowserWindowId, WindowState as BrowserWindowState,
    },
    inspector::events::TargetCrashedEvent as InspectorTargetCrashedEvent,
    network::{
        BlockPattern, GetRequestPostDataReturnObject, GetResponseBodyReturnObject, RequestId,
    },
    target::{FilterEntry, TargetFilter, TargetId, TargetInfo, events::TargetCrashedEvent},
};
pub use command::CdpCommandBuilder;
pub use domain_manager::{DomainConfig, DomainManager, DomainState, DomainType};
pub use emulation::{
    EmulationConfig, EmulationController, Geolocation, MediaEmulation, MediaFeatureOverride,
    UserAgentBrand, UserAgentMetadataOverride, UserAgentOverride,
};
pub use error::{CdpError, Result};
pub use input::{
    keyboard::{KeyInput, KeyModifiers, Keyboard, KeyboardModifier},
    mouse::{
        DoubleClickOptions, Mouse, MouseButton, MouseClickOptions, MousePosition, PressHoldOptions,
    },
};
pub use network::{
    cookies::{Cookie, CookieManager, CookiePriority, CookieSameSite, SetCookieParams},
    network_intercept::{
        HttpMethod, InterceptedRequest, InterceptedResponse, NetworkEvent, NetworkEventCallback,
        NetworkInterceptor, RequestInterceptorExt, RequestModification, ResponseFilterCallback,
        ResponseHandlerCallback, ResponseMock,
    },
};
pub use page::{
    element::{ElementHandle, ScreenshotBoxType, ShadowRoot},
    frame::{Frame, RetryConfig},
    page_core::{
        DomMutationCallback, DomMutationEvent, FrameLifecycleCallback, FrameLifecycleEvent,
        FrameSnapshot, Page, ScreencastOptions, WaitForNavigationOptions, WaitForSelectorOptions,
        WaitUntil,
    },
    session::{PageSession, PageSessionManager, PageSessionSnapshot},
};
pub use storage::manager::{
    LocalStorageExt, SessionStorageExt, StorageItem, StorageManager, StorageType,
};
pub use tracing::{TracingController, TracingStartOptions, TracingStopResult};
