#![allow(dead_code)]

mod session;
mod transport;

pub mod accessibility;
pub mod browser;
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

pub use accessibility::{
    AccessibilityController, AccessibilityNode, AccessibilityProperty, AccessibilitySnapshot,
    AccessibilitySnapshotOptions,
};
pub use browser::manager::{
    Browser, BrowserContext, BrowserContextOptions, DownloadBehavior, DownloadOptions,
    PermissionGrant, PermissionOverride,
};
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
        FrameSnapshot, Page, WaitForNavigationOptions, WaitForSelectorOptions, WaitUntil,
    },
    session::{PageSession, PageSessionManager, PageSessionSnapshot},
};
pub use storage::manager::{
    LocalStorageExt, SessionStorageExt, StorageItem, StorageManager, StorageType,
};
pub use tracing::{TracingController, TracingStartOptions, TracingStopResult};
