// Auto-generated from Chrome at version 143.0.7499.110 domain: Animation
use super::dom;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AnimationType {
    #[serde(rename = "CSSTransition")]
    CssTransition,
    #[serde(rename = "CSSAnimation")]
    CssAnimation,
    #[serde(rename = "WebAnimation")]
    WebAnimation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Animation {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "pausedState")]
    pub paused_state: bool,
    #[serde(default)]
    #[serde(rename = "playState")]
    pub play_state: String,
    #[serde(default)]
    #[serde(rename = "playbackRate")]
    pub playback_rate: JsFloat,
    #[serde(default)]
    #[serde(rename = "startTime")]
    pub start_time: JsFloat,
    #[serde(default)]
    #[serde(rename = "currentTime")]
    pub current_time: JsFloat,
    #[serde(rename = "type")]
    pub r#type: AnimationType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<AnimationEffect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "cssId")]
    pub css_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewOrScrollTimeline")]
    pub view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ViewOrScrollTimeline {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sourceNodeId")]
    pub source_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "startOffset")]
    pub start_offset: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "endOffset")]
    pub end_offset: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subjectNodeId")]
    pub subject_node_id: Option<dom::BackendNodeId>,
    #[serde(rename = "axis")]
    pub axis: dom::ScrollOrientation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AnimationEffect {
    #[serde(default)]
    #[serde(rename = "delay")]
    pub delay: JsFloat,
    #[serde(default)]
    #[serde(rename = "endDelay")]
    pub end_delay: JsFloat,
    #[serde(default)]
    #[serde(rename = "iterationStart")]
    pub iteration_start: JsFloat,
    #[serde(default)]
    #[serde(rename = "iterations")]
    pub iterations: JsFloat,
    #[serde(default)]
    #[serde(rename = "duration")]
    pub duration: JsFloat,
    #[serde(default)]
    #[serde(rename = "direction")]
    pub direction: String,
    #[serde(default)]
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyframesRule")]
    pub keyframes_rule: Option<KeyframesRule>,
    #[serde(default)]
    #[serde(rename = "easing")]
    pub easing: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframesRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "keyframes")]
    pub keyframes: Vec<KeyframeStyle>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KeyframeStyle {
    #[serde(default)]
    #[serde(rename = "offset")]
    pub offset: String,
    #[serde(default)]
    #[serde(rename = "easing")]
    pub easing: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCurrentTime {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetPlaybackRate(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReleaseAnimations {
    #[serde(default)]
    #[serde(rename = "animations")]
    pub animations: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveAnimation {
    #[serde(default)]
    #[serde(rename = "animationId")]
    pub animation_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SeekAnimations {
    #[serde(default)]
    #[serde(rename = "animations")]
    pub animations: Vec<String>,
    #[serde(default)]
    #[serde(rename = "currentTime")]
    pub current_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPaused {
    #[serde(default)]
    #[serde(rename = "animations")]
    pub animations: Vec<String>,
    #[serde(default)]
    #[serde(rename = "paused")]
    pub paused: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPlaybackRate {
    #[serde(default)]
    #[serde(rename = "playbackRate")]
    pub playback_rate: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetTiming {
    #[serde(default)]
    #[serde(rename = "animationId")]
    pub animation_id: String,
    #[serde(default)]
    #[serde(rename = "duration")]
    pub duration: JsFloat,
    #[serde(default)]
    #[serde(rename = "delay")]
    pub delay: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCurrentTimeReturnObject {
    #[serde(default)]
    #[serde(rename = "currentTime")]
    pub current_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPlaybackRateReturnObject {
    #[serde(default)]
    #[serde(rename = "playbackRate")]
    pub playback_rate: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAnimationsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolveAnimationReturnObject {
    #[serde(rename = "remoteObject")]
    pub remote_object: runtime::RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeekAnimationsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPausedReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPlaybackRateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetTimingReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Animation.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Animation.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for GetCurrentTime {
    const NAME: &'static str = "Animation.getCurrentTime";
    type ReturnObject = GetCurrentTimeReturnObject;
}
impl Method for GetPlaybackRate {
    const NAME: &'static str = "Animation.getPlaybackRate";
    type ReturnObject = GetPlaybackRateReturnObject;
}
impl Method for ReleaseAnimations {
    const NAME: &'static str = "Animation.releaseAnimations";
    type ReturnObject = ReleaseAnimationsReturnObject;
}
impl Method for ResolveAnimation {
    const NAME: &'static str = "Animation.resolveAnimation";
    type ReturnObject = ResolveAnimationReturnObject;
}
impl Method for SeekAnimations {
    const NAME: &'static str = "Animation.seekAnimations";
    type ReturnObject = SeekAnimationsReturnObject;
}
impl Method for SetPaused {
    const NAME: &'static str = "Animation.setPaused";
    type ReturnObject = SetPausedReturnObject;
}
impl Method for SetPlaybackRate {
    const NAME: &'static str = "Animation.setPlaybackRate";
    type ReturnObject = SetPlaybackRateReturnObject;
}
impl Method for SetTiming {
    const NAME: &'static str = "Animation.setTiming";
    type ReturnObject = SetTimingReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationCanceledEvent {
        pub params: AnimationCanceledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationCanceledEventParams {
        #[serde(default)]
        #[serde(rename = "id")]
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationCreatedEvent {
        pub params: AnimationCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationCreatedEventParams {
        #[serde(default)]
        #[serde(rename = "id")]
        pub id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationStartedEvent {
        pub params: AnimationStartedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationStartedEventParams {
        #[serde(rename = "animation")]
        pub animation: super::Animation,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationUpdatedEvent {
        pub params: AnimationUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationUpdatedEventParams {
        #[serde(rename = "animation")]
        pub animation: super::Animation,
    }
}
