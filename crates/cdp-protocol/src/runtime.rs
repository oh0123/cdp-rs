// Auto-generated from Chrome at version 143.0.7499.110 domain: Runtime
#[allow(unused_imports)]
use super::types::*;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SerializationOptions {
    #[serde(rename = "serialization")]
    pub serialization: SerializationOptionsSerialization,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxDepth")]
    pub max_depth: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeepSerializedValue {
    #[serde(rename = "type")]
    pub r#type: DeepSerializedValueType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectId")]
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "weakLocalObjectReference")]
    pub weak_local_object_reference: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoteObject {
    #[serde(rename = "type")]
    pub r#type: RemoteObjectType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subtype")]
    pub subtype: Option<RemoteObjectSubtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unserializableValue")]
    pub unserializable_value: Option<UnserializableValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deepSerializedValue")]
    pub deep_serialized_value: Option<DeepSerializedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preview")]
    pub preview: Option<ObjectPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customPreview")]
    pub custom_preview: Option<CustomPreview>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CustomPreview {
    #[serde(default)]
    #[serde(rename = "header")]
    pub header: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bodyGetterId")]
    pub body_getter_id: Option<RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ObjectPreview {
    #[serde(rename = "type")]
    pub r#type: ObjectPreviewType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subtype")]
    pub subtype: Option<ObjectPreviewSubtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "overflow")]
    pub overflow: bool,
    #[serde(rename = "properties")]
    pub properties: Vec<PropertyPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entries")]
    pub entries: Option<Vec<EntryPreview>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PropertyPreview {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: PropertyPreviewType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valuePreview")]
    pub value_preview: Option<ObjectPreview>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subtype")]
    pub subtype: Option<PropertyPreviewSubtype>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EntryPreview {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "key")]
    pub key: Option<ObjectPreview>,
    #[serde(rename = "value")]
    pub value: ObjectPreview,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PropertyDescriptor {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "writable")]
    pub writable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "get")]
    pub get: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "set")]
    pub set: Option<RemoteObject>,
    #[serde(default)]
    #[serde(rename = "configurable")]
    pub configurable: bool,
    #[serde(default)]
    #[serde(rename = "enumerable")]
    pub enumerable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "wasThrown")]
    pub was_thrown: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isOwn")]
    pub is_own: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "symbol")]
    pub symbol: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InternalPropertyDescriptor {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrivatePropertyDescriptor {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "get")]
    pub get: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "set")]
    pub set: Option<RemoteObject>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CallArgument {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<Json>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "unserializableValue")]
    pub unserializable_value: Option<UnserializableValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<RemoteObjectId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExecutionContextDescription {
    #[serde(rename = "id")]
    pub id: ExecutionContextId,
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "auxData")]
    pub aux_data: Option<Json>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExceptionDetails {
    #[serde(default)]
    #[serde(rename = "exceptionId")]
    pub exception_id: JsUInt,
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptId")]
    pub script_id: Option<ScriptId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<StackTrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exception")]
    pub exception: Option<RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CallFrame {
    #[serde(default)]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "scriptId")]
    pub script_id: ScriptId,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StackTrace {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<CallFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parent")]
    pub parent: Option<Box<StackTrace>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parentId")]
    pub parent_id: Option<StackTraceId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StackTraceId {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "debuggerId")]
    pub debugger_id: Option<UniqueDebuggerId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AwaitPromise {
    #[serde(rename = "promiseObjectId")]
    pub promise_object_id: RemoteObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CallFunctionOn {
    #[serde(default)]
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "objectId")]
    pub object_id: Option<RemoteObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<Vec<CallArgument>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "silent")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "userGesture")]
    pub user_gesture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "awaitPromise")]
    pub await_promise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "uniqueContextId")]
    pub unique_context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompileScript {
    #[serde(default)]
    #[serde(rename = "expression")]
    pub expression: String,
    #[serde(default)]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[serde(default)]
    #[serde(rename = "persistScript")]
    pub persist_script: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiscardConsoleEntries(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Evaluate {
    #[serde(default)]
    #[serde(rename = "expression")]
    pub expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "silent")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextId")]
    pub context_id: Option<ExecutionContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "userGesture")]
    pub user_gesture: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "awaitPromise")]
    pub await_promise: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeout")]
    pub timeout: Option<TimeDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "disableBreaks")]
    pub disable_breaks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "replMode")]
    pub repl_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "uniqueContextId")]
    pub unique_context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetIsolateId(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapUsage(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetProperties {
    #[serde(rename = "objectId")]
    pub object_id: RemoteObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ownProperties")]
    pub own_properties: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "accessorPropertiesOnly")]
    pub accessor_properties_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "nonIndexedPropertiesOnly")]
    pub non_indexed_properties_only: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GlobalLexicalScopeNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QueryObjects {
    #[serde(rename = "prototypeObjectId")]
    pub prototype_object_id: RemoteObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReleaseObject {
    #[serde(rename = "objectId")]
    pub object_id: RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ReleaseObjectGroup {
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunIfWaitingForDebugger(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RunScript {
    #[serde(rename = "scriptId")]
    pub script_id: ScriptId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "objectGroup")]
    pub object_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "silent")]
    pub silent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "awaitPromise")]
    pub await_promise: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAsyncCallStackDepth {
    #[serde(default)]
    #[serde(rename = "maxDepth")]
    pub max_depth: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCustomObjectFormatterEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetMaxCallStackSizeToCapture {
    #[serde(default)]
    #[serde(rename = "size")]
    pub size: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TerminateExecution(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddBinding {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "executionContextName")]
    pub execution_context_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveBinding {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetExceptionDetails {
    #[serde(rename = "errorObjectId")]
    pub error_object_id: RemoteObjectId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AwaitPromiseReturnObject {
    #[serde(rename = "result")]
    pub result: RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CallFunctionOnReturnObject {
    #[serde(rename = "result")]
    pub result: RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompileScriptReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scriptId")]
    pub script_id: Option<ScriptId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiscardConsoleEntriesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EvaluateReturnObject {
    #[serde(rename = "result")]
    pub result: RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetIsolateIdReturnObject {
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetHeapUsageReturnObject {
    #[serde(default)]
    #[serde(rename = "usedSize")]
    pub used_size: JsFloat,
    #[serde(default)]
    #[serde(rename = "totalSize")]
    pub total_size: JsFloat,
    #[serde(default)]
    #[serde(rename = "embedderHeapUsedSize")]
    pub embedder_heap_used_size: JsFloat,
    #[serde(default)]
    #[serde(rename = "backingStorageSize")]
    pub backing_storage_size: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPropertiesReturnObject {
    #[serde(rename = "result")]
    pub result: Vec<PropertyDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "internalProperties")]
    pub internal_properties: Option<Vec<InternalPropertyDescriptor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privateProperties")]
    pub private_properties: Option<Vec<PrivatePropertyDescriptor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GlobalLexicalScopeNamesReturnObject {
    #[serde(rename = "names")]
    pub names: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QueryObjectsReturnObject {
    #[serde(rename = "objects")]
    pub objects: RemoteObject,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectGroupReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunIfWaitingForDebuggerReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RunScriptReturnObject {
    #[serde(rename = "result")]
    pub result: RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCustomObjectFormatterEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetMaxCallStackSizeToCaptureReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TerminateExecutionReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddBindingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBindingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetExceptionDetailsReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BindingCalledEvent {
        pub params: BindingCalledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BindingCalledEventParams {
        #[serde(default)]
        #[serde(rename = "name")]
        pub name: String,
        #[serde(default)]
        #[serde(rename = "payload")]
        pub payload: String,
        #[serde(rename = "executionContextId")]
        pub execution_context_id: super::ExecutionContextId,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleAPICalledEvent {
        pub params: ConsoleAPICalledEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConsoleAPICalledEventParams {
        #[serde(rename = "type")]
        pub r#type: super::ConsoleApiCalledTypeOption,
        #[serde(rename = "args")]
        pub args: Vec<super::RemoteObject>,
        #[serde(rename = "executionContextId")]
        pub execution_context_id: super::ExecutionContextId,
        #[serde(rename = "timestamp")]
        pub timestamp: super::Timestamp,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stackTrace")]
        pub stack_trace: Option<super::StackTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "context")]
        pub context: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionRevokedEvent {
        pub params: ExceptionRevokedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionRevokedEventParams {
        #[serde(default)]
        #[serde(rename = "reason")]
        pub reason: String,
        #[serde(default)]
        #[serde(rename = "exceptionId")]
        pub exception_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionThrownEvent {
        pub params: ExceptionThrownEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExceptionThrownEventParams {
        #[serde(rename = "timestamp")]
        pub timestamp: super::Timestamp,
        #[serde(rename = "exceptionDetails")]
        pub exception_details: super::ExceptionDetails,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextCreatedEvent {
        pub params: ExecutionContextCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextCreatedEventParams {
        #[serde(rename = "context")]
        pub context: super::ExecutionContextDescription,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextDestroyedEvent {
        pub params: ExecutionContextDestroyedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ExecutionContextDestroyedEventParams {
        #[serde(rename = "executionContextId")]
        pub execution_context_id: super::ExecutionContextId,
        #[serde(default)]
        #[serde(rename = "executionContextUniqueId")]
        pub execution_context_unique_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ExecutionContextsClearedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectRequestedEvent {
        pub params: InspectRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InspectRequestedEventParams {
        #[serde(rename = "object")]
        pub object: super::RemoteObject,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "executionContextId")]
        pub execution_context_id: Option<super::ExecutionContextId>,
    }
}
