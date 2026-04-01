// Auto-generated from Chrome at version 146.0.7680.165 domain: Runtime
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type ScriptId = String;
pub type RemoteObjectId = String;
pub type UnserializableValue = String;
pub type ExecutionContextId = JsUInt;
pub type Timestamp = JsFloat;
pub type TimeDelta = JsFloat;
pub type UniqueDebuggerId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SerializationOptionsSerialization {
    #[serde(rename = "deep")]
    Deep,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "idOnly")]
    IdOnly,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DeepSerializedValueType {
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "bigint")]
    Bigint,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "generator")]
    Generator,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RemoteObjectType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "bigint")]
    Bigint,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RemoteObjectSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ObjectPreviewType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "bigint")]
    Bigint,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ObjectPreviewSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PropertyPreviewType {
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "undefined")]
    Undefined,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "symbol")]
    Symbol,
    #[serde(rename = "accessor")]
    Accessor,
    #[serde(rename = "bigint")]
    Bigint,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PropertyPreviewSubtype {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "regexp")]
    Regexp,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "weakmap")]
    Weakmap,
    #[serde(rename = "weakset")]
    Weakset,
    #[serde(rename = "iterator")]
    Iterator,
    #[serde(rename = "generator")]
    Generator,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "promise")]
    Promise,
    #[serde(rename = "typedarray")]
    Typedarray,
    #[serde(rename = "arraybuffer")]
    Arraybuffer,
    #[serde(rename = "dataview")]
    Dataview,
    #[serde(rename = "webassemblymemory")]
    Webassemblymemory,
    #[serde(rename = "wasmvalue")]
    Wasmvalue,
    #[serde(rename = "trustedtype")]
    Trustedtype,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConsoleApiCalledTypeOption {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "dir")]
    Dir,
    #[serde(rename = "dirxml")]
    Dirxml,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "startGroup")]
    StartGroup,
    #[serde(rename = "startGroupCollapsed")]
    StartGroupCollapsed,
    #[serde(rename = "endGroup")]
    EndGroup,
    #[serde(rename = "assert")]
    Assert,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "profileEnd")]
    ProfileEnd,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "timeEnd")]
    TimeEnd,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents options for serialization. Overrides `generatePreview` and `returnByValue`."]
pub struct SerializationOptions {
    pub serialization: SerializationOptionsSerialization,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Deep serialization depth. Default is full depth. Respected only in `deep` serialization mode."]
    pub max_depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM\n serialization via `maxNodeDepth: integer` and `includeShadowTree: \"none\" | \"open\" | \"all\"`.\n Values can be only of type string or integer."]
    pub additional_parameters: Option<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents deep serialized value."]
pub struct DeepSerializedValue {
    pub r#type: DeepSerializedValueType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<Json>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Set if value reference met more then once during serialization. In such\n case, value is provided only to one of the serialized values. Unique\n per value in the scope of one CDP call."]
    pub weak_local_object_reference: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Mirror object referencing original JavaScript object."]
pub struct RemoteObject {
    #[doc = "Object type."]
    pub r#type: RemoteObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Object subtype hint. Specified for `object` type values only.\n NOTE: If you change anything here, make sure to also update\n `subtype` in `ObjectPreview` and `PropertyPreview` below."]
    pub subtype: Option<RemoteObjectSubtype>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Object class (constructor) name. Specified for `object` type values only."]
    pub class_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Remote object value in case of primitive values or JSON values (if it was requested)."]
    pub value: Option<Json>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Primitive value which can not be JSON-stringified does not have `value`, but gets this\n property."]
    pub unserializable_value: Option<UnserializableValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String representation of the object."]
    pub description: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Deep serialized value."]
    pub deep_serialized_value: Option<DeepSerializedValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Unique object identifier (for non-primitive values)."]
    pub object_id: Option<RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Preview containing abbreviated property values. Specified for `object` type values only."]
    pub preview: Option<ObjectPreview>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_preview: Option<CustomPreview>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct CustomPreview {
    #[serde(default)]
    #[doc = "The JSON-stringified result of formatter.header(object, config) call.\n It contains json ML array that represents RemoteObject."]
    pub header: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If formatter returns true as a result of formatter.hasBody call then bodyGetterId will\n contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.\n The result value is json ML array."]
    pub body_getter_id: Option<RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object containing abbreviated remote object value."]
pub struct ObjectPreview {
    #[doc = "Object type."]
    pub r#type: ObjectPreviewType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Object subtype hint. Specified for `object` type values only."]
    pub subtype: Option<ObjectPreviewSubtype>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String representation of the object."]
    pub description: Option<String>,
    #[serde(default)]
    #[doc = "True iff some of the properties or entries of the original object did not fit."]
    pub overflow: bool,
    #[doc = "List of the properties."]
    pub properties: Vec<PropertyPreview>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "List of the entries. Specified for `map` and `set` subtype values only."]
    pub entries: Option<Vec<EntryPreview>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview {
    #[serde(default)]
    #[doc = "Property name."]
    pub name: String,
    #[doc = "Object type. Accessor means that the property itself is an accessor property."]
    pub r#type: PropertyPreviewType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "User-friendly property value string."]
    pub value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Nested value preview."]
    pub value_preview: Option<ObjectPreview>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Object subtype hint. Specified for `object` type values only."]
    pub subtype: Option<PropertyPreviewSubtype>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct EntryPreview {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Preview of the key. Specified for map-like collection entries."]
    pub key: Option<ObjectPreview>,
    #[doc = "Preview of the value."]
    pub value: ObjectPreview,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object property descriptor."]
pub struct PropertyDescriptor {
    #[serde(default)]
    #[doc = "Property name or symbol description."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value associated with the property."]
    pub value: Option<RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the value associated with the property may be changed (data descriptors only)."]
    pub writable: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A function which serves as a getter for the property, or `undefined` if there is no getter\n (accessor descriptors only)."]
    pub get: Option<RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A function which serves as a setter for the property, or `undefined` if there is no setter\n (accessor descriptors only)."]
    pub set: Option<RemoteObject>,
    #[serde(default)]
    #[doc = "True if the type of this property descriptor may be changed and if the property may be\n deleted from the corresponding object."]
    pub configurable: bool,
    #[serde(default)]
    #[doc = "True if this property shows up during enumeration of the properties on the corresponding\n object."]
    pub enumerable: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the result was thrown during the evaluation."]
    pub was_thrown: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "True if the property is owned for the object."]
    pub is_own: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Property symbol object, if the property is of the `symbol` type."]
    pub symbol: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object internal property descriptor. This property isn't normally visible in JavaScript code."]
pub struct InternalPropertyDescriptor {
    #[serde(default)]
    #[doc = "Conventional property name."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value associated with the property."]
    pub value: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Object private field descriptor."]
pub struct PrivatePropertyDescriptor {
    #[serde(default)]
    #[doc = "Private property name."]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value associated with the private property."]
    pub value: Option<RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A function which serves as a getter for the private property,\n or `undefined` if there is no getter (accessor descriptors only)."]
    pub get: Option<RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A function which serves as a setter for the private property,\n or `undefined` if there is no setter (accessor descriptors only)."]
    pub set: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents function call argument. Either remote object id `objectId`, primitive `value`,\n unserializable primitive value or neither of (for undefined) them should be specified."]
pub struct CallArgument {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Primitive value or serializable javascript object."]
    pub value: Option<Json>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Primitive value which can not be JSON-stringified."]
    pub unserializable_value: Option<UnserializableValue>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Remote object handle."]
    pub object_id: Option<RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Description of an isolated world."]
pub struct ExecutionContextDescription {
    #[doc = "Unique id of the execution context. It can be used to specify in which execution context\n script evaluation should be performed."]
    pub id: ExecutionContextId,
    #[serde(default)]
    #[doc = "Execution context origin."]
    pub origin: String,
    #[serde(default)]
    #[doc = "Human readable name describing given context."]
    pub name: String,
    #[serde(default)]
    #[doc = "A system-unique execution context identifier. Unlike the id, this is unique across\n multiple processes, so can be reliably used to identify specific context while backend\n performs a cross-process navigation."]
    pub unique_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
    pub aux_data: Option<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Detailed information about exception (or error) that was thrown during script compilation or\n execution."]
pub struct ExceptionDetails {
    #[serde(default)]
    #[doc = "Exception id."]
    pub exception_id: JsUInt,
    #[serde(default)]
    #[doc = "Exception text, which should be used together with exception object when available."]
    pub text: String,
    #[serde(default)]
    #[doc = "Line number of the exception location (0-based)."]
    pub line_number: JsUInt,
    #[serde(default)]
    #[doc = "Column number of the exception location (0-based)."]
    pub column_number: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Script ID of the exception location."]
    pub script_id: Option<ScriptId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the exception location, to be used when the script was not reported."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "JavaScript stack trace if available."]
    pub stack_trace: Option<StackTrace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception object if available."]
    pub exception: Option<RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the context where exception happened."]
    pub execution_context_id: Option<ExecutionContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Dictionary with entries of meta data that the client associated\n with this exception, such as information about associated network\n requests, etc."]
    pub exception_meta_data: Option<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Stack entry for runtime errors and assertions."]
pub struct CallFrame {
    #[serde(default)]
    #[doc = "JavaScript function name."]
    pub function_name: String,
    #[doc = "JavaScript script id."]
    pub script_id: ScriptId,
    #[serde(default)]
    #[doc = "JavaScript script name or url."]
    pub url: String,
    #[serde(default)]
    #[doc = "JavaScript script line number (0-based)."]
    pub line_number: JsUInt,
    #[serde(default)]
    #[doc = "JavaScript script column number (0-based)."]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Call frames for assertions or error messages."]
pub struct StackTrace {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String label of this stack trace. For async traces this may be a name of the function that\n initiated the async call."]
    pub description: Option<String>,
    #[doc = "JavaScript function name."]
    pub call_frames: Vec<CallFrame>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Asynchronous JavaScript stack trace that preceded this stack, if available."]
    pub parent: Option<Box<StackTrace>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Asynchronous JavaScript stack trace that preceded this stack, if available."]
    pub parent_id: Option<StackTraceId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This\n allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages."]
pub struct StackTraceId {
    #[serde(default)]
    pub id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debugger_id: Option<UniqueDebuggerId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Add handler to promise with given promise object id."]
pub struct AwaitPromise {
    #[doc = "Identifier of the promise."]
    pub promise_object_id: RemoteObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the result is expected to be a JSON object that should be sent by value."]
    pub return_by_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether preview should be generated for the result."]
    pub generate_preview: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Calls function with given declaration on the given object. Object group of the result is\n inherited from the target object."]
pub struct CallFunctionOn {
    #[serde(default)]
    #[doc = "Declaration of the function to call."]
    pub function_declaration: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Identifier of the object to call function on. Either objectId or executionContextId should\n be specified."]
    pub object_id: Option<RemoteObjectId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Call arguments. All call arguments must belong to the same JavaScript world as the target\n object."]
    pub arguments: Option<Vec<CallArgument>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\n execution. Overrides `setPauseOnException` state."]
    pub silent: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the result is expected to be a JSON object which should be sent by value.\n Can be overriden by `serializationOptions`."]
    pub return_by_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether preview should be generated for the result."]
    pub generate_preview: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether execution should be treated as initiated by user in the UI."]
    pub user_gesture: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\n resolved."]
    pub await_promise: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies execution context which global object will be used to call function on. Either\n executionContextId or objectId should be specified."]
    pub execution_context_id: Option<ExecutionContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release multiple objects. If objectGroup is not\n specified and objectId is, objectGroup will be inherited from object."]
    pub object_group: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation."]
    pub throw_on_side_effect: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An alternative way to specify the execution context to call function on.\n Compared to contextId that may be reused across processes, this is guaranteed to be\n system-unique, so it can be used to prevent accidental function call\n in context different than intended (e.g. as a result of navigation across process\n boundaries).\n This is mutually exclusive with `executionContextId`."]
    pub unique_context_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies the result serialization. If provided, overrides\n `generatePreview` and `returnByValue`."]
    pub serialization_options: Option<SerializationOptions>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Compiles expression."]
pub struct CompileScript {
    #[serde(default)]
    #[doc = "Expression to compile."]
    pub expression: String,
    #[serde(default)]
    #[doc = "Source url to be set for the script."]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(default)]
    #[doc = "Specifies whether the compiled script should be persisted."]
    pub persist_script: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies in which execution context to perform script run. If the parameter is omitted the\n evaluation will be performed in the context of the inspected page."]
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DiscardConsoleEntries(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates expression on global object."]
pub struct Evaluate {
    #[serde(default)]
    #[doc = "Expression to evaluate."]
    pub expression: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    pub object_group: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Determines whether Command Line API should be available during the evaluation."]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\n execution. Overrides `setPauseOnException` state."]
    pub silent: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies in which execution context to perform evaluation. If the parameter is omitted the\n evaluation will be performed in the context of the inspected page.\n This is mutually exclusive with `uniqueContextId`, which offers an\n alternative way to identify the execution context that is more reliable\n in a multi-process environment."]
    pub context_id: Option<ExecutionContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the result is expected to be a JSON object that should be sent by value."]
    pub return_by_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether preview should be generated for the result."]
    pub generate_preview: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether execution should be treated as initiated by user in the UI."]
    pub user_gesture: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\n resolved."]
    pub await_promise: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation.\n This implies `disableBreaks` below."]
    pub throw_on_side_effect: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Terminate execution after timing out (number of milliseconds)."]
    pub timeout: Option<TimeDelta>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Disable breakpoints during execution."]
    pub disable_breaks: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Setting this flag to true enables `let` re-declaration and top-level `await`.\n Note that `let` variables can only be re-declared if they originate from\n `replMode` themselves."]
    pub repl_mode: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The Content Security Policy (CSP) for the target might block 'unsafe-eval'\n which includes eval(), Function(), setTimeout() and setInterval()\n when called with non-callable arguments. This flag bypasses CSP for this\n evaluation and allows unsafe-eval. Defaults to true."]
    #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An alternative way to specify the execution context to evaluate in.\n Compared to contextId that may be reused across processes, this is guaranteed to be\n system-unique, so it can be used to prevent accidental evaluation of the expression\n in context different than intended (e.g. as a result of navigation across process\n boundaries).\n This is mutually exclusive with `contextId`."]
    pub unique_context_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies the result serialization. If provided, overrides\n `generatePreview` and `returnByValue`."]
    pub serialization_options: Option<SerializationOptions>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetIsolateId(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHeapUsage(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns properties of a given object. Object group of the result is inherited from the target\n object."]
pub struct GetProperties {
    #[doc = "Identifier of the object to return properties for."]
    pub object_id: RemoteObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, returns properties belonging only to the element itself, not to its prototype\n chain."]
    pub own_properties: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, returns accessor properties (with getter/setter) only; internal properties are not\n returned either."]
    pub accessor_properties_only: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether preview should be generated for the results."]
    pub generate_preview: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, returns non-indexed properties only."]
    pub non_indexed_properties_only: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all let, const and class variables from global scope."]
pub struct GlobalLexicalScopeNames {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies in which execution context to lookup global scope variables."]
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct QueryObjects {
    #[doc = "Identifier of the prototype to return objects for."]
    pub prototype_object_id: RemoteObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release the results."]
    pub object_group: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Releases remote object with given id."]
pub struct ReleaseObject {
    #[doc = "Identifier of the object to release."]
    pub object_id: RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Releases all remote objects that belong to a given group."]
pub struct ReleaseObjectGroup {
    #[serde(default)]
    #[doc = "Symbolic object group name."]
    pub object_group: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RunIfWaitingForDebugger(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Runs script with given id in a given context."]
pub struct RunScript {
    #[doc = "Id of the script to run."]
    pub script_id: ScriptId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies in which execution context to perform script run. If the parameter is omitted the\n evaluation will be performed in the context of the inspected page."]
    pub execution_context_id: Option<ExecutionContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    pub object_group: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\n execution. Overrides `setPauseOnException` state."]
    pub silent: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Determines whether Command Line API should be available during the evaluation."]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the result is expected to be a JSON object which should be sent by value."]
    pub return_by_value: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether preview should be generated for the result."]
    pub generate_preview: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\n resolved."]
    pub await_promise: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables or disables async call stacks tracking."]
pub struct SetAsyncCallStackDepth {
    #[serde(default)]
    #[doc = "Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async\n call stacks (default)."]
    pub max_depth: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetCustomObjectFormatterEnabled {
    #[serde(default)]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetMaxCallStackSizeToCapture {
    #[serde(default)]
    pub size: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TerminateExecution(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "If executionContextId is empty, adds binding with the given name on the\n global objects of all inspected contexts, including those created later,\n bindings survive reloads.\n Binding function takes exactly one argument, this argument should be string,\n in case of any other input, function throws an exception.\n Each binding function call produces Runtime.bindingCalled notification."]
pub struct AddBinding {
    #[serde(default)]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If specified, the binding would only be exposed to the specified\n execution context. If omitted and `executionContextName` is not set,\n the binding is exposed to all execution contexts of the target.\n This parameter is mutually exclusive with `executionContextName`.\n Deprecated in favor of `executionContextName` due to an unclear use case\n and bugs in implementation (crbug.com/1169639). `executionContextId` will be\n removed in the future."]
    #[deprecated]
    pub execution_context_id: Option<ExecutionContextId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If specified, the binding is exposed to the executionContext with\n matching name, even for contexts created after the binding is added.\n See also `ExecutionContext.name` and `worldName` parameter to\n `Page.addScriptToEvaluateOnNewDocument`.\n This parameter is mutually exclusive with `executionContextId`."]
    pub execution_context_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This method does not remove binding function from global object but\n unsubscribes current runtime agent from Runtime.bindingCalled notifications."]
pub struct RemoveBinding {
    #[serde(default)]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This method tries to lookup and populate exception details for a\n JavaScript Error object.\n Note that the stackTrace portion of the resulting exceptionDetails will\n only be populated if the Runtime domain was enabled at the time when the\n Error was thrown."]
pub struct GetExceptionDetails {
    #[doc = "The error object for which to resolve the exception details."]
    pub error_object_id: RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Add handler to promise with given promise object id."]
pub struct AwaitPromiseReturnObject {
    #[doc = "Promise result. Will contain rejected value if promise was rejected."]
    pub result: RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details if stack strace is available."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Calls function with given declaration on the given object. Object group of the result is\n inherited from the target object."]
pub struct CallFunctionOnReturnObject {
    #[doc = "Call result."]
    pub result: RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Compiles expression."]
pub struct CompileScriptReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Id of the script."]
    pub script_id: Option<ScriptId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables reporting of execution contexts creation."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Discards collected exceptions and console API calls."]
pub struct DiscardConsoleEntriesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables reporting of execution contexts creation by means of `executionContextCreated` event.\n When the reporting gets enabled the event will be sent immediately for each existing execution\n context."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates expression on global object."]
pub struct EvaluateReturnObject {
    #[doc = "Evaluation result."]
    pub result: RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the isolate id."]
pub struct GetIsolateIdReturnObject {
    #[serde(default)]
    #[doc = "The isolate id."]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the JavaScript heap usage.\n It is the total usage of the corresponding isolate not scoped to a particular Runtime."]
pub struct GetHeapUsageReturnObject {
    #[serde(default)]
    #[doc = "Used JavaScript heap size in bytes."]
    pub used_size: JsFloat,
    #[serde(default)]
    #[doc = "Allocated JavaScript heap size in bytes."]
    pub total_size: JsFloat,
    #[serde(default)]
    #[doc = "Used size in bytes in the embedder's garbage-collected heap."]
    pub embedder_heap_used_size: JsFloat,
    #[serde(default)]
    #[doc = "Size in bytes of backing storage for array buffers and external strings."]
    pub backing_storage_size: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns properties of a given object. Object group of the result is inherited from the target\n object."]
pub struct GetPropertiesReturnObject {
    #[doc = "Object properties."]
    pub result: Vec<PropertyDescriptor>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Internal object properties (only of the element itself)."]
    pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Object private properties."]
    pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all let, const and class variables from global scope."]
pub struct GlobalLexicalScopeNamesReturnObject {
    pub names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsReturnObject {
    #[doc = "Array with objects."]
    pub objects: RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Releases remote object with given id."]
pub struct ReleaseObjectReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Releases all remote objects that belong to a given group."]
pub struct ReleaseObjectGroupReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Tells inspected instance to run if it was waiting for debugger to attach."]
pub struct RunIfWaitingForDebuggerReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Runs script with given id in a given context."]
pub struct RunScriptReturnObject {
    #[doc = "Run result."]
    pub result: RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables or disables async call stacks tracking."]
pub struct SetAsyncCallStackDepthReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCustomObjectFormatterEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetMaxCallStackSizeToCaptureReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Terminate current or next JavaScript execution.\n Will cancel the termination when the outer-most script execution ends."]
pub struct TerminateExecutionReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "If executionContextId is empty, adds binding with the given name on the\n global objects of all inspected contexts, including those created later,\n bindings survive reloads.\n Binding function takes exactly one argument, this argument should be string,\n in case of any other input, function throws an exception.\n Each binding function call produces Runtime.bindingCalled notification."]
pub struct AddBindingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "This method does not remove binding function from global object but\n unsubscribes current runtime agent from Runtime.bindingCalled notifications."]
pub struct RemoveBindingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "This method tries to lookup and populate exception details for a\n JavaScript Error object.\n Note that the stackTrace portion of the resulting exceptionDetails will\n only be populated if the Runtime domain was enabled at the time when the\n Error was thrown."]
pub struct GetExceptionDetailsReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_details: Option<ExceptionDetails>,
}
impl Method for AwaitPromise {
    const NAME: &'static str = "Runtime.awaitPromise";
    type ReturnObject = AwaitPromiseReturnObject;
}
impl Method for CallFunctionOn {
    const NAME: &'static str = "Runtime.callFunctionOn";
    type ReturnObject = CallFunctionOnReturnObject;
}
impl Method for CompileScript {
    const NAME: &'static str = "Runtime.compileScript";
    type ReturnObject = CompileScriptReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Runtime.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for DiscardConsoleEntries {
    const NAME: &'static str = "Runtime.discardConsoleEntries";
    type ReturnObject = DiscardConsoleEntriesReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Runtime.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Evaluate {
    const NAME: &'static str = "Runtime.evaluate";
    type ReturnObject = EvaluateReturnObject;
}
impl Method for GetIsolateId {
    const NAME: &'static str = "Runtime.getIsolateId";
    type ReturnObject = GetIsolateIdReturnObject;
}
impl Method for GetHeapUsage {
    const NAME: &'static str = "Runtime.getHeapUsage";
    type ReturnObject = GetHeapUsageReturnObject;
}
impl Method for GetProperties {
    const NAME: &'static str = "Runtime.getProperties";
    type ReturnObject = GetPropertiesReturnObject;
}
impl Method for GlobalLexicalScopeNames {
    const NAME: &'static str = "Runtime.globalLexicalScopeNames";
    type ReturnObject = GlobalLexicalScopeNamesReturnObject;
}
impl Method for QueryObjects {
    const NAME: &'static str = "Runtime.queryObjects";
    type ReturnObject = QueryObjectsReturnObject;
}
impl Method for ReleaseObject {
    const NAME: &'static str = "Runtime.releaseObject";
    type ReturnObject = ReleaseObjectReturnObject;
}
impl Method for ReleaseObjectGroup {
    const NAME: &'static str = "Runtime.releaseObjectGroup";
    type ReturnObject = ReleaseObjectGroupReturnObject;
}
impl Method for RunIfWaitingForDebugger {
    const NAME: &'static str = "Runtime.runIfWaitingForDebugger";
    type ReturnObject = RunIfWaitingForDebuggerReturnObject;
}
impl Method for RunScript {
    const NAME: &'static str = "Runtime.runScript";
    type ReturnObject = RunScriptReturnObject;
}
impl Method for SetAsyncCallStackDepth {
    const NAME: &'static str = "Runtime.setAsyncCallStackDepth";
    type ReturnObject = SetAsyncCallStackDepthReturnObject;
}
impl Method for SetCustomObjectFormatterEnabled {
    const NAME: &'static str = "Runtime.setCustomObjectFormatterEnabled";
    type ReturnObject = SetCustomObjectFormatterEnabledReturnObject;
}
impl Method for SetMaxCallStackSizeToCapture {
    const NAME: &'static str = "Runtime.setMaxCallStackSizeToCapture";
    type ReturnObject = SetMaxCallStackSizeToCaptureReturnObject;
}
impl Method for TerminateExecution {
    const NAME: &'static str = "Runtime.terminateExecution";
    type ReturnObject = TerminateExecutionReturnObject;
}
impl Method for AddBinding {
    const NAME: &'static str = "Runtime.addBinding";
    type ReturnObject = AddBindingReturnObject;
}
impl Method for RemoveBinding {
    const NAME: &'static str = "Runtime.removeBinding";
    type ReturnObject = RemoveBindingReturnObject;
}
impl Method for GetExceptionDetails {
    const NAME: &'static str = "Runtime.getExceptionDetails";
    type ReturnObject = GetExceptionDetailsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BindingCalledEvent {
        pub params: BindingCalledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct BindingCalledEventParams {
        #[serde(default)]
        pub name: String,
        #[serde(default)]
        pub payload: String,
        #[doc = "Identifier of the context where the call was made."]
        pub execution_context_id: super::ExecutionContextId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleAPICalledEvent {
        pub params: ConsoleAPICalledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ConsoleAPICalledEventParams {
        #[doc = "Type of the call."]
        pub r#type: super::ConsoleApiCalledTypeOption,
        #[doc = "Call arguments."]
        pub args: Vec<super::RemoteObject>,
        #[doc = "Identifier of the context where the call was made."]
        pub execution_context_id: super::ExecutionContextId,
        #[doc = "Call timestamp."]
        pub timestamp: super::Timestamp,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Stack trace captured when the call was made. The async stack chain is automatically reported for\n the following call types: `assert`, `error`, `trace`, `warning`. For other types the async call\n chain can be retrieved using `Debugger.getStackTrace` and `stackTrace.parentId` field."]
        pub stack_trace: Option<super::StackTrace>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Console context descriptor for calls on non-default console context (not console.*):\n 'anonymous#unique-logger-id' for call on unnamed context, 'name#unique-logger-id' for call\n on named context."]
        pub context: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionRevokedEvent {
        pub params: ExceptionRevokedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ExceptionRevokedEventParams {
        #[serde(default)]
        #[doc = "Reason describing why exception was revoked."]
        pub reason: String,
        #[serde(default)]
        #[doc = "The id of revoked exception, as reported in `exceptionThrown`."]
        pub exception_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionThrownEvent {
        pub params: ExceptionThrownEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ExceptionThrownEventParams {
        #[doc = "Timestamp of the exception."]
        pub timestamp: super::Timestamp,
        pub exception_details: super::ExceptionDetails,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextCreatedEvent {
        pub params: ExecutionContextCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ExecutionContextCreatedEventParams {
        #[doc = "A newly created execution context."]
        pub context: super::ExecutionContextDescription,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextDestroyedEvent {
        pub params: ExecutionContextDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct ExecutionContextDestroyedEventParams {
        #[doc = "Id of the destroyed context"]
        #[deprecated]
        pub execution_context_id: super::ExecutionContextId,
        #[serde(default)]
        #[doc = "Unique Id of the destroyed context"]
        pub execution_context_unique_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextsClearedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectRequestedEvent {
        pub params: InspectRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct InspectRequestedEventParams {
        pub object: super::RemoteObject,
        #[serde(default)]
        pub hints: Json,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Identifier of the context where the call was made."]
        pub execution_context_id: Option<super::ExecutionContextId>,
    }
}
