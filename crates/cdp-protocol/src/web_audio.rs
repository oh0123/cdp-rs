// Auto-generated from Chrome at version 146.0.7680.165 domain: WebAudio
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type GraphObjectId = String;
pub type NodeType = String;
pub type ParamType = String;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContextType {
    #[serde(rename = "realtime")]
    Realtime,
    #[serde(rename = "offline")]
    Offline,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContextState {
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "interrupted")]
    Interrupted,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ChannelCountMode {
    #[serde(rename = "clamped-max")]
    ClampedMax,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "max")]
    Max,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ChannelInterpretation {
    #[serde(rename = "discrete")]
    Discrete,
    #[serde(rename = "speakers")]
    Speakers,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AutomationRate {
    #[serde(rename = "a-rate")]
    ARate,
    #[serde(rename = "k-rate")]
    KRate,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fields in AudioContext that change in real-time."]
pub struct ContextRealtimeData {
    #[serde(default)]
    #[doc = "The current context time in second in BaseAudioContext."]
    pub current_time: JsFloat,
    #[serde(default)]
    #[doc = "The time spent on rendering graph divided by render quantum duration,\n and multiplied by 100. 100 means the audio renderer reached the full\n capacity and glitch may occur."]
    pub render_capacity: JsFloat,
    #[serde(default)]
    #[doc = "A running mean of callback interval."]
    pub callback_interval_mean: JsFloat,
    #[serde(default)]
    #[doc = "A running variance of callback interval."]
    pub callback_interval_variance: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Protocol object for BaseAudioContext"]
pub struct BaseAudioContext {
    pub context_id: GraphObjectId,
    pub context_type: ContextType,
    pub context_state: ContextState,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_data: Option<ContextRealtimeData>,
    #[serde(default)]
    #[doc = "Platform-dependent callback buffer size."]
    pub callback_buffer_size: JsFloat,
    #[serde(default)]
    #[doc = "Number of output channels supported by audio hardware in use."]
    pub max_output_channel_count: JsFloat,
    #[serde(default)]
    #[doc = "Context sample rate."]
    pub sample_rate: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Protocol object for AudioListener"]
pub struct AudioListener {
    pub listener_id: GraphObjectId,
    pub context_id: GraphObjectId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Protocol object for AudioNode"]
pub struct AudioNode {
    pub node_id: GraphObjectId,
    pub context_id: GraphObjectId,
    pub node_type: NodeType,
    #[serde(default)]
    pub number_of_inputs: JsFloat,
    #[serde(default)]
    pub number_of_outputs: JsFloat,
    #[serde(default)]
    pub channel_count: JsFloat,
    pub channel_count_mode: ChannelCountMode,
    pub channel_interpretation: ChannelInterpretation,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Protocol object for AudioParam"]
pub struct AudioParam {
    pub param_id: GraphObjectId,
    pub node_id: GraphObjectId,
    pub context_id: GraphObjectId,
    pub param_type: ParamType,
    pub rate: AutomationRate,
    #[serde(default)]
    pub default_value: JsFloat,
    #[serde(default)]
    pub min_value: JsFloat,
    #[serde(default)]
    pub max_value: JsFloat,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Fetch the realtime data from the registered contexts."]
pub struct GetRealtimeData {
    pub context_id: GraphObjectId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the WebAudio domain and starts sending context lifetime events."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the WebAudio domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Fetch the realtime data from the registered contexts."]
pub struct GetRealtimeDataReturnObject {
    pub realtime_data: ContextRealtimeData,
}
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "WebAudio.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "WebAudio.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for GetRealtimeData {
    const NAME: &'static str = "WebAudio.getRealtimeData";
    type ReturnObject = GetRealtimeDataReturnObject;
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
    pub struct ContextCreatedEvent {
        pub params: ContextCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextCreatedEventParams {
        pub context: super::BaseAudioContext,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextWillBeDestroyedEvent {
        pub params: ContextWillBeDestroyedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextWillBeDestroyedEventParams {
        pub context_id: super::GraphObjectId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextChangedEvent {
        pub params: ContextChangedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ContextChangedEventParams {
        pub context: super::BaseAudioContext,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerCreatedEvent {
        pub params: AudioListenerCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioListenerCreatedEventParams {
        pub listener: super::AudioListener,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerWillBeDestroyedEvent {
        pub params: AudioListenerWillBeDestroyedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioListenerWillBeDestroyedEventParams {
        pub context_id: super::GraphObjectId,
        pub listener_id: super::GraphObjectId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeCreatedEvent {
        pub params: AudioNodeCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioNodeCreatedEventParams {
        pub node: super::AudioNode,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeWillBeDestroyedEvent {
        pub params: AudioNodeWillBeDestroyedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioNodeWillBeDestroyedEventParams {
        pub context_id: super::GraphObjectId,
        pub node_id: super::GraphObjectId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamCreatedEvent {
        pub params: AudioParamCreatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioParamCreatedEventParams {
        pub param: super::AudioParam,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamWillBeDestroyedEvent {
        pub params: AudioParamWillBeDestroyedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AudioParamWillBeDestroyedEventParams {
        pub context_id: super::GraphObjectId,
        pub node_id: super::GraphObjectId,
        pub param_id: super::GraphObjectId,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesConnectedEvent {
        pub params: NodesConnectedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodesConnectedEventParams {
        pub context_id: super::GraphObjectId,
        pub source_id: super::GraphObjectId,
        pub destination_id: super::GraphObjectId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub source_output_index: Option<JsFloat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub destination_input_index: Option<JsFloat>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesDisconnectedEvent {
        pub params: NodesDisconnectedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodesDisconnectedEventParams {
        pub context_id: super::GraphObjectId,
        pub source_id: super::GraphObjectId,
        pub destination_id: super::GraphObjectId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub source_output_index: Option<JsFloat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub destination_input_index: Option<JsFloat>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamConnectedEvent {
        pub params: NodeParamConnectedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeParamConnectedEventParams {
        pub context_id: super::GraphObjectId,
        pub source_id: super::GraphObjectId,
        pub destination_id: super::GraphObjectId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub source_output_index: Option<JsFloat>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamDisconnectedEvent {
        pub params: NodeParamDisconnectedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeParamDisconnectedEventParams {
        pub context_id: super::GraphObjectId,
        pub source_id: super::GraphObjectId,
        pub destination_id: super::GraphObjectId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub source_output_index: Option<JsFloat>,
    }
}
