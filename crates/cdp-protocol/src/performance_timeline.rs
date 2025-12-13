// Auto-generated from Chrome at version 143.0.7499.110 domain: PerformanceTimeline
use super::dom;
use super::network;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LargestContentfulPaint {
    #[serde(rename = "renderTime")]
    pub render_time: network::TimeSinceEpoch,
    #[serde(rename = "loadTime")]
    pub load_time: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "elementId")]
    pub element_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayoutShiftAttribution {
    #[serde(rename = "previousRect")]
    pub previous_rect: dom::Rect,
    #[serde(rename = "currentRect")]
    pub current_rect: dom::Rect,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeId")]
    pub node_id: Option<dom::BackendNodeId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayoutShift {
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
    #[serde(default)]
    #[serde(rename = "hadRecentInput")]
    pub had_recent_input: bool,
    #[serde(rename = "lastInputTime")]
    pub last_input_time: network::TimeSinceEpoch,
    #[serde(rename = "sources")]
    pub sources: Vec<LayoutShiftAttribution>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TimelineEvent {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "time")]
    pub time: network::TimeSinceEpoch,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "duration")]
    pub duration: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lcpDetails")]
    pub lcp_details: Option<LargestContentfulPaint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "layoutShiftDetails")]
    pub layout_shift_details: Option<LayoutShift>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(default)]
    #[serde(rename = "eventTypes")]
    pub event_types: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "PerformanceTimeline.enable";
    type ReturnObject = EnableReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TimelineEventAddedEvent {
        pub params: TimelineEventAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TimelineEventAddedEventParams {
        #[serde(rename = "event")]
        pub event: super::TimelineEvent,
    }
}
