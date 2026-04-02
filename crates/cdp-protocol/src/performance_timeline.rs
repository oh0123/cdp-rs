// Auto-generated from Chrome at version 146.0.7680.165 domain: PerformanceTimeline
#![allow(dead_code)]
use super::dom;
use super::network;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "See <https://github.com/WICG/LargestContentfulPaint> and largest_contentful_paint.idl"]
pub struct LargestContentfulPaint {
    pub render_time: network::TimeSinceEpoch,
    pub load_time: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "The number of pixels being painted."]
    pub size: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The id attribute of the element, if available."]
    pub element_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The URL of the image (may be trimmed)."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<dom::BackendNodeId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct LayoutShiftAttribution {
    pub previous_rect: dom::Rect,
    pub current_rect: dom::Rect,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<dom::BackendNodeId>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "See <https://wicg.github.io/layout-instability/#sec-layout-shift> and layout_shift.idl"]
pub struct LayoutShift {
    #[serde(default)]
    #[doc = "Score increment produced by this event."]
    pub value: JsFloat,
    #[serde(default)]
    pub had_recent_input: bool,
    pub last_input_time: network::TimeSinceEpoch,
    pub sources: Vec<LayoutShiftAttribution>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    #[doc = "Identifies the frame that this event is related to. Empty for non-frame targets."]
    pub frame_id: page::FrameId,
    #[serde(default)]
    #[doc = "The event type, as specified in <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>\n This determines which of the optional \"details\" fields is present."]
    pub r#type: String,
    #[serde(default)]
    #[doc = "Name may be empty depending on the type."]
    pub name: String,
    #[doc = "Time in seconds since Epoch, monotonically increasing within document lifetime."]
    pub time: network::TimeSinceEpoch,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Event duration, if applicable."]
    pub duration: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcp_details: Option<LargestContentfulPaint>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_shift_details: Option<LayoutShift>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Previously buffered events would be reported before method returns.\n See also: timelineEventAdded"]
pub struct Enable {
    #[serde(default)]
    #[doc = "The types of event to report, as specified in\n <https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype>\n The specified filter overrides any previous filters, passing empty\n filter disables recording.\n Note that not all types exposed to the web platform are currently supported."]
    pub event_types: Vec<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Previously buffered events would be reported before method returns.\n See also: timelineEventAdded"]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "PerformanceTimeline.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(dead_code)]
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TimelineEventAddedEvent {
        pub params: TimelineEventAddedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct TimelineEventAddedEventParams {
        pub event: super::TimelineEvent,
    }
}
