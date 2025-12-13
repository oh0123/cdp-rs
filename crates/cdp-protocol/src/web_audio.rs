// Auto-generated from Chrome at version 143.0.7499.110 domain: WebAudio
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type GraphObjectId = String;
pub type NodeType = String;
pub type ParamType = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContextType {
    #[serde(rename = "realtime")]
    Realtime,
    #[serde(rename = "offline")]
    Offline,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ChannelCountMode {
    #[serde(rename = "clamped-max")]
    ClampedMax,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "max")]
    Max,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ChannelInterpretation {
    #[serde(rename = "discrete")]
    Discrete,
    #[serde(rename = "speakers")]
    Speakers,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AutomationRate {
    #[serde(rename = "a-rate")]
    ARate,
    #[serde(rename = "k-rate")]
    KRate,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContextRealtimeData {
    #[serde(default)]
    #[serde(rename = "currentTime")]
    pub current_time: JsFloat,
    #[serde(default)]
    #[serde(rename = "renderCapacity")]
    pub render_capacity: JsFloat,
    #[serde(default)]
    #[serde(rename = "callbackIntervalMean")]
    pub callback_interval_mean: JsFloat,
    #[serde(default)]
    #[serde(rename = "callbackIntervalVariance")]
    pub callback_interval_variance: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BaseAudioContext {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "contextType")]
    pub context_type: ContextType,
    #[serde(rename = "contextState")]
    pub context_state: ContextState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "realtimeData")]
    pub realtime_data: Option<ContextRealtimeData>,
    #[serde(default)]
    #[serde(rename = "callbackBufferSize")]
    pub callback_buffer_size: JsFloat,
    #[serde(default)]
    #[serde(rename = "maxOutputChannelCount")]
    pub max_output_channel_count: JsFloat,
    #[serde(default)]
    #[serde(rename = "sampleRate")]
    pub sample_rate: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AudioListener {
    #[serde(rename = "listenerId")]
    pub listener_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AudioNode {
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,
    #[serde(default)]
    #[serde(rename = "numberOfInputs")]
    pub number_of_inputs: JsFloat,
    #[serde(default)]
    #[serde(rename = "numberOfOutputs")]
    pub number_of_outputs: JsFloat,
    #[serde(default)]
    #[serde(rename = "channelCount")]
    pub channel_count: JsFloat,
    #[serde(rename = "channelCountMode")]
    pub channel_count_mode: ChannelCountMode,
    #[serde(rename = "channelInterpretation")]
    pub channel_interpretation: ChannelInterpretation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AudioParam {
    #[serde(rename = "paramId")]
    pub param_id: GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "paramType")]
    pub param_type: ParamType,
    #[serde(rename = "rate")]
    pub rate: AutomationRate,
    #[serde(default)]
    #[serde(rename = "defaultValue")]
    pub default_value: JsFloat,
    #[serde(default)]
    #[serde(rename = "minValue")]
    pub min_value: JsFloat,
    #[serde(default)]
    #[serde(rename = "maxValue")]
    pub max_value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRealtimeData {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRealtimeDataReturnObject {
    #[serde(rename = "realtimeData")]
    pub realtime_data: ContextRealtimeData,
}
impl Method for Enable {
    const NAME: &'static str = "WebAudio.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "WebAudio.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for GetRealtimeData {
    const NAME: &'static str = "WebAudio.getRealtimeData";
    type ReturnObject = GetRealtimeDataReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextCreatedEvent {
        pub params: ContextCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextCreatedEventParams {
        #[serde(rename = "context")]
        pub context: super::BaseAudioContext,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextWillBeDestroyedEvent {
        pub params: ContextWillBeDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextWillBeDestroyedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextChangedEvent {
        pub params: ContextChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ContextChangedEventParams {
        #[serde(rename = "context")]
        pub context: super::BaseAudioContext,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerCreatedEvent {
        pub params: AudioListenerCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerCreatedEventParams {
        #[serde(rename = "listener")]
        pub listener: super::AudioListener,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerWillBeDestroyedEvent {
        pub params: AudioListenerWillBeDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioListenerWillBeDestroyedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "listenerId")]
        pub listener_id: super::GraphObjectId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeCreatedEvent {
        pub params: AudioNodeCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeCreatedEventParams {
        #[serde(rename = "node")]
        pub node: super::AudioNode,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeWillBeDestroyedEvent {
        pub params: AudioNodeWillBeDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioNodeWillBeDestroyedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "nodeId")]
        pub node_id: super::GraphObjectId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamCreatedEvent {
        pub params: AudioParamCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamCreatedEventParams {
        #[serde(rename = "param")]
        pub param: super::AudioParam,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamWillBeDestroyedEvent {
        pub params: AudioParamWillBeDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AudioParamWillBeDestroyedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "nodeId")]
        pub node_id: super::GraphObjectId,
        #[serde(rename = "paramId")]
        pub param_id: super::GraphObjectId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesConnectedEvent {
        pub params: NodesConnectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesConnectedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "sourceId")]
        pub source_id: super::GraphObjectId,
        #[serde(rename = "destinationId")]
        pub destination_id: super::GraphObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceOutputIndex")]
        pub source_output_index: Option<JsFloat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "destinationInputIndex")]
        pub destination_input_index: Option<JsFloat>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesDisconnectedEvent {
        pub params: NodesDisconnectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodesDisconnectedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "sourceId")]
        pub source_id: super::GraphObjectId,
        #[serde(rename = "destinationId")]
        pub destination_id: super::GraphObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceOutputIndex")]
        pub source_output_index: Option<JsFloat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "destinationInputIndex")]
        pub destination_input_index: Option<JsFloat>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamConnectedEvent {
        pub params: NodeParamConnectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamConnectedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "sourceId")]
        pub source_id: super::GraphObjectId,
        #[serde(rename = "destinationId")]
        pub destination_id: super::GraphObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceOutputIndex")]
        pub source_output_index: Option<JsFloat>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamDisconnectedEvent {
        pub params: NodeParamDisconnectedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct NodeParamDisconnectedEventParams {
        #[serde(rename = "contextId")]
        pub context_id: super::GraphObjectId,
        #[serde(rename = "sourceId")]
        pub source_id: super::GraphObjectId,
        #[serde(rename = "destinationId")]
        pub destination_id: super::GraphObjectId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceOutputIndex")]
        pub source_output_index: Option<JsFloat>,
    }
}
