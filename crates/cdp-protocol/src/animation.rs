// Auto-generated from Chrome at version 146.0.7680.165 domain: Animation
#![allow(dead_code)]
use super::dom;
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AnimationType {
    #[serde(rename = "CSSTransition")]
    CssTransition,
    #[serde(rename = "CSSAnimation")]
    CssAnimation,
    #[serde(rename = "WebAnimation")]
    WebAnimation,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Animation instance."]
pub struct Animation {
    #[serde(default)]
    #[doc = "`Animation`'s id."]
    pub id: String,
    #[serde(default)]
    #[doc = "`Animation`'s name."]
    pub name: String,
    #[serde(default)]
    #[doc = "`Animation`'s internal paused state."]
    pub paused_state: bool,
    #[serde(default)]
    #[doc = "`Animation`'s play state."]
    pub play_state: String,
    #[serde(default)]
    #[doc = "`Animation`'s playback rate."]
    pub playback_rate: JsFloat,
    #[serde(default)]
    #[doc = "`Animation`'s start time.\n Milliseconds for time based animations and\n percentage \\[0 - 100\\] for scroll driven animations\n (i.e. when viewOrScrollTimeline exists)."]
    pub start_time: JsFloat,
    #[serde(default)]
    #[doc = "`Animation`'s current time."]
    pub current_time: JsFloat,
    #[doc = "Animation type of `Animation`."]
    pub r#type: AnimationType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`Animation`'s source animation node."]
    pub source: Option<AnimationEffect>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A unique ID for `Animation` representing the sources that triggered this CSS\n animation/transition."]
    pub css_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "View or scroll timeline"]
    pub view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Timeline instance"]
pub struct ViewOrScrollTimeline {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Scroll container node"]
    pub source_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Represents the starting scroll position of the timeline\n as a length offset in pixels from scroll origin."]
    pub start_offset: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Represents the ending scroll position of the timeline\n as a length offset in pixels from scroll origin."]
    pub end_offset: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The element whose principal box's visibility in the\n scrollport defined the progress of the timeline.\n Does not exist for animations with ScrollTimeline"]
    pub subject_node_id: Option<dom::BackendNodeId>,
    #[doc = "Orientation of the scroll"]
    pub axis: dom::ScrollOrientation,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "AnimationEffect instance"]
pub struct AnimationEffect {
    #[serde(default)]
    #[doc = "`AnimationEffect`'s delay."]
    pub delay: JsFloat,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s end delay."]
    pub end_delay: JsFloat,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s iteration start."]
    pub iteration_start: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "`AnimationEffect`'s iterations. Omitted if the value is infinite."]
    pub iterations: Option<JsFloat>,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s iteration duration.\n Milliseconds for time based animations and\n percentage \\[0 - 100\\] for scroll driven animations\n (i.e. when viewOrScrollTimeline exists)."]
    pub duration: JsFloat,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s playback direction."]
    pub direction: String,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s fill mode."]
    pub fill: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`AnimationEffect`'s target node."]
    pub backend_node_id: Option<dom::BackendNodeId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "`AnimationEffect`'s keyframes."]
    pub keyframes_rule: Option<KeyframesRule>,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s timing function."]
    pub easing: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Keyframes Rule"]
pub struct KeyframesRule {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "CSS keyframed animation's name."]
    pub name: Option<String>,
    #[doc = "List of animation keyframes."]
    pub keyframes: Vec<KeyframeStyle>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Keyframe Style"]
pub struct KeyframeStyle {
    #[serde(default)]
    #[doc = "Keyframe's time offset."]
    pub offset: String,
    #[serde(default)]
    #[doc = "`AnimationEffect`'s timing function."]
    pub easing: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the current time of the an animation."]
pub struct GetCurrentTime {
    #[serde(default)]
    #[doc = "Id of animation."]
    pub id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPlaybackRate(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Releases a set of animations to no longer be manipulated."]
pub struct ReleaseAnimations {
    #[serde(default)]
    #[doc = "List of animation ids to seek."]
    pub animations: Vec<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the remote object of the Animation."]
pub struct ResolveAnimation {
    #[serde(default)]
    #[doc = "Animation id."]
    pub animation_id: String,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Seek a set of animations to a particular time within each animation."]
pub struct SeekAnimations {
    #[serde(default)]
    #[doc = "List of animation ids to seek."]
    pub animations: Vec<String>,
    #[serde(default)]
    #[doc = "Set the current time of each animation."]
    pub current_time: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the paused state of a set of animations."]
pub struct SetPaused {
    #[serde(default)]
    #[doc = "Animations to set the pause state of."]
    pub animations: Vec<String>,
    #[serde(default)]
    #[doc = "Paused state to set to."]
    pub paused: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the playback rate of the document timeline."]
pub struct SetPlaybackRate {
    #[serde(default)]
    #[doc = "Playback rate for animations on page"]
    pub playback_rate: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets the timing of an animation node."]
pub struct SetTiming {
    #[serde(default)]
    #[doc = "Animation id."]
    pub animation_id: String,
    #[serde(default)]
    #[doc = "Duration of the animation."]
    pub duration: JsFloat,
    #[serde(default)]
    #[doc = "Delay of the animation."]
    pub delay: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables animation domain notifications."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables animation domain notifications."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the current time of the an animation."]
pub struct GetCurrentTimeReturnObject {
    #[serde(default)]
    #[doc = "Current time of the page."]
    pub current_time: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the playback rate of the document timeline."]
pub struct GetPlaybackRateReturnObject {
    #[serde(default)]
    #[doc = "Playback rate for animations on page."]
    pub playback_rate: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Releases a set of animations to no longer be manipulated."]
pub struct ReleaseAnimationsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the remote object of the Animation."]
pub struct ResolveAnimationReturnObject {
    #[doc = "Corresponding remote object."]
    pub remote_object: runtime::RemoteObject,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Seek a set of animations to a particular time within each animation."]
pub struct SeekAnimationsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets the paused state of a set of animations."]
pub struct SetPausedReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets the playback rate of the document timeline."]
pub struct SetPlaybackRateReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets the timing of an animation node."]
pub struct SetTimingReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "Animation.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "Animation.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for GetCurrentTime {
    const NAME: &'static str = "Animation.getCurrentTime";
    type ReturnObject = GetCurrentTimeReturnObject;
}
#[allow(deprecated)]
impl Method for GetPlaybackRate {
    const NAME: &'static str = "Animation.getPlaybackRate";
    type ReturnObject = GetPlaybackRateReturnObject;
}
#[allow(deprecated)]
impl Method for ReleaseAnimations {
    const NAME: &'static str = "Animation.releaseAnimations";
    type ReturnObject = ReleaseAnimationsReturnObject;
}
#[allow(deprecated)]
impl Method for ResolveAnimation {
    const NAME: &'static str = "Animation.resolveAnimation";
    type ReturnObject = ResolveAnimationReturnObject;
}
#[allow(deprecated)]
impl Method for SeekAnimations {
    const NAME: &'static str = "Animation.seekAnimations";
    type ReturnObject = SeekAnimationsReturnObject;
}
#[allow(deprecated)]
impl Method for SetPaused {
    const NAME: &'static str = "Animation.setPaused";
    type ReturnObject = SetPausedReturnObject;
}
#[allow(deprecated)]
impl Method for SetPlaybackRate {
    const NAME: &'static str = "Animation.setPlaybackRate";
    type ReturnObject = SetPlaybackRateReturnObject;
}
#[allow(deprecated)]
impl Method for SetTiming {
    const NAME: &'static str = "Animation.setTiming";
    type ReturnObject = SetTimingReturnObject;
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
    pub struct AnimationCanceledEvent {
        pub params: AnimationCanceledEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AnimationCanceledEventParams {
        #[serde(default)]
        #[doc = "Id of the animation that was cancelled."]
        pub id: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationCreatedEvent {
        pub params: AnimationCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AnimationCreatedEventParams {
        #[serde(default)]
        #[doc = "Id of the animation that was created."]
        pub id: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationStartedEvent {
        pub params: AnimationStartedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AnimationStartedEventParams {
        #[doc = "Animation that was started."]
        pub animation: super::Animation,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AnimationUpdatedEvent {
        pub params: AnimationUpdatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AnimationUpdatedEventParams {
        #[doc = "Animation that was updated."]
        pub animation: super::Animation,
    }
}
