// Auto-generated from Chrome at version 143.0.7499.110 domain: Debugger
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type BreakpointId = String;
pub type CallFrameId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScopeType {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "with")]
    With,
    #[serde(rename = "closure")]
    Closure,
    #[serde(rename = "catch")]
    Catch,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "eval")]
    Eval,
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "wasm-expression-stack")]
    WasmExpressionStack,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BreakLocationType {
    #[serde(rename = "debuggerStatement")]
    DebuggerStatement,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "return")]
    Return,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScriptLanguage {
    #[serde(rename = "JavaScript")]
    JavaScript,
    #[serde(rename = "WebAssembly")]
    WebAssembly,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DebugSymbolsType {
    #[serde(rename = "SourceMap")]
    SourceMap,
    #[serde(rename = "EmbeddedDWARF")]
    EmbeddedDwarf,
    #[serde(rename = "ExternalDWARF")]
    ExternalDwarf,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContinueToLocationTargetCallFramesOption {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "current")]
    Current,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RestartFrameModeOption {
    #[serde(rename = "StepInto")]
    StepInto,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetInstrumentationBreakpointInstrumentationOption {
    #[serde(rename = "beforeScriptExecution")]
    BeforeScriptExecution,
    #[serde(rename = "beforeScriptWithSourceMapExecution")]
    BeforeScriptWithSourceMapExecution,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetPauseOnExceptionsStateOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "caught")]
    Caught,
    #[serde(rename = "uncaught")]
    Uncaught,
    #[serde(rename = "all")]
    All,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StatusOption {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "CompileError")]
    CompileError,
    #[serde(rename = "BlockedByActiveGenerator")]
    BlockedByActiveGenerator,
    #[serde(rename = "BlockedByActiveFunction")]
    BlockedByActiveFunction,
    #[serde(rename = "BlockedByTopLevelEsModuleChange")]
    BlockedByTopLevelEsModuleChange,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PausedReasonOption {
    #[serde(rename = "ambiguous")]
    Ambiguous,
    #[serde(rename = "assert")]
    Assert,
    #[serde(rename = "CSPViolation")]
    CspViolation,
    #[serde(rename = "debugCommand")]
    DebugCommand,
    #[serde(rename = "DOM")]
    Dom,
    #[serde(rename = "EventListener")]
    EventListener,
    #[serde(rename = "exception")]
    Exception,
    #[serde(rename = "instrumentation")]
    Instrumentation,
    #[serde(rename = "OOM")]
    Oom,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "promiseRejection")]
    PromiseRejection,
    #[serde(rename = "XHR")]
    Xhr,
    #[serde(rename = "step")]
    Step,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Location {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScriptPosition {
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LocationRange {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(rename = "start")]
    pub start: ScriptPosition,
    #[serde(rename = "end")]
    pub end: ScriptPosition,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CallFrame {
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId,
    #[serde(default)]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functionLocation")]
    pub function_location: Option<Location>,
    #[serde(rename = "location")]
    pub location: Location,
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "scopeChain")]
    pub scope_chain: Vec<Scope>,
    #[serde(rename = "this")]
    pub this: runtime::RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "returnValue")]
    pub return_value: Option<runtime::RemoteObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "canBeRestarted")]
    pub can_be_restarted: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Scope {
    #[serde(rename = "type")]
    pub r#type: ScopeType,
    #[serde(rename = "object")]
    pub object: runtime::RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startLocation")]
    pub start_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "endLocation")]
    pub end_location: Option<Location>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchMatch {
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsFloat,
    #[serde(default)]
    #[serde(rename = "lineContent")]
    pub line_content: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BreakLocation {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub r#type: Option<BreakLocationType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WasmDisassemblyChunk {
    #[serde(default)]
    #[serde(rename = "lines")]
    pub lines: Vec<String>,
    #[serde(default)]
    #[serde(rename = "bytecodeOffsets")]
    pub bytecode_offsets: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DebugSymbols {
    #[serde(rename = "type")]
    pub r#type: DebugSymbolsType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "externalURL")]
    pub external_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResolvedBreakpoint {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
    #[serde(rename = "location")]
    pub location: Location,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContinueToLocation {
    #[serde(rename = "location")]
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetCallFrames")]
    pub target_call_frames: Option<ContinueToLocationTargetCallFramesOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxScriptsCacheSize")]
    pub max_scripts_cache_size: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EvaluateOnCallFrame {
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId,
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
    #[serde(default)]
    #[serde(rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeout")]
    pub timeout: Option<runtime::TimeDelta>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPossibleBreakpoints {
    #[serde(rename = "start")]
    pub start: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end")]
    pub end: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "restrictToFunction")]
    pub restrict_to_function: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetScriptSource {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisassembleWasmModule {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NextWasmDisassemblyChunk {
    #[serde(default)]
    #[serde(rename = "streamId")]
    pub stream_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWasmBytecode {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStackTrace {
    #[serde(rename = "stackTraceId")]
    pub stack_trace_id: runtime::StackTraceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pause(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PauseOnAsyncCall {
    #[serde(rename = "parentStackTraceId")]
    pub parent_stack_trace_id: runtime::StackTraceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveBreakpoint {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RestartFrame {
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mode")]
    pub mode: Option<RestartFrameModeOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Resume {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "terminateOnResume")]
    pub terminate_on_resume: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInContent {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isRegex")]
    pub is_regex: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAsyncCallStackDepth {
    #[serde(default)]
    #[serde(rename = "maxDepth")]
    pub max_depth: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBlackboxExecutionContexts {
    #[serde(default)]
    #[serde(rename = "uniqueIds")]
    pub unique_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBlackboxPatterns {
    #[serde(default)]
    #[serde(rename = "patterns")]
    pub patterns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "skipAnonymous")]
    pub skip_anonymous: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBlackboxedRanges {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(rename = "positions")]
    pub positions: Vec<ScriptPosition>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpoint {
    #[serde(rename = "location")]
    pub location: Location,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "condition")]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInstrumentationBreakpoint {
    #[serde(rename = "instrumentation")]
    pub instrumentation: SetInstrumentationBreakpointInstrumentationOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointByUrl {
    #[serde(default)]
    #[serde(rename = "lineNumber")]
    pub line_number: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urlRegex")]
    pub url_regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scriptHash")]
    pub script_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "columnNumber")]
    pub column_number: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "condition")]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointOnFunctionCall {
    #[serde(rename = "objectId")]
    pub object_id: runtime::RemoteObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "condition")]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointsActive {
    #[serde(default)]
    #[serde(rename = "active")]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPauseOnExceptions {
    #[serde(rename = "state")]
    pub state: SetPauseOnExceptionsStateOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetReturnValue {
    #[serde(rename = "newValue")]
    pub new_value: runtime::CallArgument,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScriptSource {
    #[serde(rename = "scriptId")]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dryRun")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "allowTopFrameEditing")]
    pub allow_top_frame_editing: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSkipAllPauses {
    #[serde(default)]
    #[serde(rename = "skip")]
    pub skip: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetVariableValue {
    #[serde(default)]
    #[serde(rename = "scopeNumber")]
    pub scope_number: JsUInt,
    #[serde(default)]
    #[serde(rename = "variableName")]
    pub variable_name: String,
    #[serde(rename = "newValue")]
    pub new_value: runtime::CallArgument,
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StepInto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "breakOnAsyncCall")]
    pub break_on_async_call: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skipList")]
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepOut(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StepOver {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "skipList")]
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocationReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject {
    #[serde(rename = "debuggerId")]
    pub debugger_id: runtime::UniqueDebuggerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EvaluateOnCallFrameReturnObject {
    #[serde(rename = "result")]
    pub result: runtime::RemoteObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<runtime::ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPossibleBreakpointsReturnObject {
    #[serde(rename = "locations")]
    pub locations: Vec<BreakLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetScriptSourceReturnObject {
    #[serde(default)]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytecode")]
    pub bytecode: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisassembleWasmModuleReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "streamId")]
    pub stream_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "totalNumberOfLines")]
    pub total_number_of_lines: JsUInt,
    #[serde(rename = "functionBodyOffsets")]
    pub function_body_offsets: Vec<JsUInt>,
    #[serde(rename = "chunk")]
    pub chunk: WasmDisassemblyChunk,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NextWasmDisassemblyChunkReturnObject {
    #[serde(rename = "chunk")]
    pub chunk: WasmDisassemblyChunk,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetWasmBytecodeReturnObject {
    #[serde(rename = "bytecode")]
    pub bytecode: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStackTraceReturnObject {
    #[serde(rename = "stackTrace")]
    pub stack_trace: runtime::StackTrace,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PauseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PauseOnAsyncCallReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBreakpointReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RestartFrameReturnObject {
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<CallFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "asyncStackTrace")]
    pub async_stack_trace: Option<runtime::StackTrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "asyncStackTraceId")]
    pub async_stack_trace_id: Option<runtime::StackTraceId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResumeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SearchInContentReturnObject {
    #[serde(rename = "result")]
    pub result: Vec<SearchMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxExecutionContextsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxPatternsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxedRangesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointReturnObject {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
    #[serde(rename = "actualLocation")]
    pub actual_location: Location,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInstrumentationBreakpointReturnObject {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointByUrlReturnObject {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
    #[serde(rename = "locations")]
    pub locations: Vec<Location>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetBreakpointOnFunctionCallReturnObject {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsActiveReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPauseOnExceptionsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetReturnValueReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScriptSourceReturnObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "callFrames")]
    pub call_frames: Option<Vec<CallFrame>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "stackChanged")]
    pub stack_changed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "asyncStackTrace")]
    pub async_stack_trace: Option<runtime::StackTrace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "asyncStackTraceId")]
    pub async_stack_trace_id: Option<runtime::StackTraceId>,
    #[serde(rename = "status")]
    pub status: StatusOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exceptionDetails")]
    pub exception_details: Option<runtime::ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPausesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableValueReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepIntoReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepOutReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepOverReturnObject {}
impl Method for ContinueToLocation {
    const NAME: &'static str = "Debugger.continueToLocation";
    type ReturnObject = ContinueToLocationReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "Debugger.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Debugger.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for EvaluateOnCallFrame {
    const NAME: &'static str = "Debugger.evaluateOnCallFrame";
    type ReturnObject = EvaluateOnCallFrameReturnObject;
}
impl Method for GetPossibleBreakpoints {
    const NAME: &'static str = "Debugger.getPossibleBreakpoints";
    type ReturnObject = GetPossibleBreakpointsReturnObject;
}
impl Method for GetScriptSource {
    const NAME: &'static str = "Debugger.getScriptSource";
    type ReturnObject = GetScriptSourceReturnObject;
}
impl Method for DisassembleWasmModule {
    const NAME: &'static str = "Debugger.disassembleWasmModule";
    type ReturnObject = DisassembleWasmModuleReturnObject;
}
impl Method for NextWasmDisassemblyChunk {
    const NAME: &'static str = "Debugger.nextWasmDisassemblyChunk";
    type ReturnObject = NextWasmDisassemblyChunkReturnObject;
}
impl Method for GetWasmBytecode {
    const NAME: &'static str = "Debugger.getWasmBytecode";
    type ReturnObject = GetWasmBytecodeReturnObject;
}
impl Method for GetStackTrace {
    const NAME: &'static str = "Debugger.getStackTrace";
    type ReturnObject = GetStackTraceReturnObject;
}
impl Method for Pause {
    const NAME: &'static str = "Debugger.pause";
    type ReturnObject = PauseReturnObject;
}
impl Method for PauseOnAsyncCall {
    const NAME: &'static str = "Debugger.pauseOnAsyncCall";
    type ReturnObject = PauseOnAsyncCallReturnObject;
}
impl Method for RemoveBreakpoint {
    const NAME: &'static str = "Debugger.removeBreakpoint";
    type ReturnObject = RemoveBreakpointReturnObject;
}
impl Method for RestartFrame {
    const NAME: &'static str = "Debugger.restartFrame";
    type ReturnObject = RestartFrameReturnObject;
}
impl Method for Resume {
    const NAME: &'static str = "Debugger.resume";
    type ReturnObject = ResumeReturnObject;
}
impl Method for SearchInContent {
    const NAME: &'static str = "Debugger.searchInContent";
    type ReturnObject = SearchInContentReturnObject;
}
impl Method for SetAsyncCallStackDepth {
    const NAME: &'static str = "Debugger.setAsyncCallStackDepth";
    type ReturnObject = SetAsyncCallStackDepthReturnObject;
}
impl Method for SetBlackboxExecutionContexts {
    const NAME: &'static str = "Debugger.setBlackboxExecutionContexts";
    type ReturnObject = SetBlackboxExecutionContextsReturnObject;
}
impl Method for SetBlackboxPatterns {
    const NAME: &'static str = "Debugger.setBlackboxPatterns";
    type ReturnObject = SetBlackboxPatternsReturnObject;
}
impl Method for SetBlackboxedRanges {
    const NAME: &'static str = "Debugger.setBlackboxedRanges";
    type ReturnObject = SetBlackboxedRangesReturnObject;
}
impl Method for SetBreakpoint {
    const NAME: &'static str = "Debugger.setBreakpoint";
    type ReturnObject = SetBreakpointReturnObject;
}
impl Method for SetInstrumentationBreakpoint {
    const NAME: &'static str = "Debugger.setInstrumentationBreakpoint";
    type ReturnObject = SetInstrumentationBreakpointReturnObject;
}
impl Method for SetBreakpointByUrl {
    const NAME: &'static str = "Debugger.setBreakpointByUrl";
    type ReturnObject = SetBreakpointByUrlReturnObject;
}
impl Method for SetBreakpointOnFunctionCall {
    const NAME: &'static str = "Debugger.setBreakpointOnFunctionCall";
    type ReturnObject = SetBreakpointOnFunctionCallReturnObject;
}
impl Method for SetBreakpointsActive {
    const NAME: &'static str = "Debugger.setBreakpointsActive";
    type ReturnObject = SetBreakpointsActiveReturnObject;
}
impl Method for SetPauseOnExceptions {
    const NAME: &'static str = "Debugger.setPauseOnExceptions";
    type ReturnObject = SetPauseOnExceptionsReturnObject;
}
impl Method for SetReturnValue {
    const NAME: &'static str = "Debugger.setReturnValue";
    type ReturnObject = SetReturnValueReturnObject;
}
impl Method for SetScriptSource {
    const NAME: &'static str = "Debugger.setScriptSource";
    type ReturnObject = SetScriptSourceReturnObject;
}
impl Method for SetSkipAllPauses {
    const NAME: &'static str = "Debugger.setSkipAllPauses";
    type ReturnObject = SetSkipAllPausesReturnObject;
}
impl Method for SetVariableValue {
    const NAME: &'static str = "Debugger.setVariableValue";
    type ReturnObject = SetVariableValueReturnObject;
}
impl Method for StepInto {
    const NAME: &'static str = "Debugger.stepInto";
    type ReturnObject = StepIntoReturnObject;
}
impl Method for StepOut {
    const NAME: &'static str = "Debugger.stepOut";
    type ReturnObject = StepOutReturnObject;
}
impl Method for StepOver {
    const NAME: &'static str = "Debugger.stepOver";
    type ReturnObject = StepOverReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BreakpointResolvedEvent {
        pub params: BreakpointResolvedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BreakpointResolvedEventParams {
        #[serde(rename = "breakpointId")]
        pub breakpoint_id: super::BreakpointId,
        #[serde(rename = "location")]
        pub location: super::Location,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PausedEvent {
        pub params: PausedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PausedEventParams {
        #[serde(rename = "callFrames")]
        pub call_frames: Vec<super::CallFrame>,
        #[serde(rename = "reason")]
        pub reason: super::PausedReasonOption,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "hitBreakpoints")]
        pub hit_breakpoints: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "asyncStackTrace")]
        pub async_stack_trace: Option<super::super::runtime::StackTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "asyncStackTraceId")]
        pub async_stack_trace_id: Option<super::super::runtime::StackTraceId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "asyncCallStackTraceId")]
        pub async_call_stack_trace_id: Option<super::super::runtime::StackTraceId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ResumedEvent(pub Option<serde_json::Value>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptFailedToParseEvent {
        pub params: ScriptFailedToParseEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptFailedToParseEventParams {
        #[serde(rename = "scriptId")]
        pub script_id: super::super::runtime::ScriptId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(default)]
        #[serde(rename = "startLine")]
        pub start_line: JsUInt,
        #[serde(default)]
        #[serde(rename = "startColumn")]
        pub start_column: JsUInt,
        #[serde(default)]
        #[serde(rename = "endLine")]
        pub end_line: JsUInt,
        #[serde(default)]
        #[serde(rename = "endColumn")]
        pub end_column: JsUInt,
        #[serde(rename = "executionContextId")]
        pub execution_context_id: super::super::runtime::ExecutionContextId,
        #[serde(default)]
        #[serde(rename = "hash")]
        pub hash: String,
        #[serde(default)]
        #[serde(rename = "buildId")]
        pub build_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceMapURL")]
        pub source_map_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "hasSourceURL")]
        pub has_source_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "isModule")]
        pub is_module: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "length")]
        pub length: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stackTrace")]
        pub stack_trace: Option<super::super::runtime::StackTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "codeOffset")]
        pub code_offset: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "scriptLanguage")]
        pub script_language: Option<super::super::debugger::ScriptLanguage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "embedderName")]
        pub embedder_name: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptParsedEvent {
        pub params: ScriptParsedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptParsedEventParams {
        #[serde(rename = "scriptId")]
        pub script_id: super::super::runtime::ScriptId,
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(default)]
        #[serde(rename = "startLine")]
        pub start_line: JsUInt,
        #[serde(default)]
        #[serde(rename = "startColumn")]
        pub start_column: JsUInt,
        #[serde(default)]
        #[serde(rename = "endLine")]
        pub end_line: JsUInt,
        #[serde(default)]
        #[serde(rename = "endColumn")]
        pub end_column: JsUInt,
        #[serde(rename = "executionContextId")]
        pub execution_context_id: super::super::runtime::ExecutionContextId,
        #[serde(default)]
        #[serde(rename = "hash")]
        pub hash: String,
        #[serde(default)]
        #[serde(rename = "buildId")]
        pub build_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "isLiveEdit")]
        pub is_live_edit: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "sourceMapURL")]
        pub source_map_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "hasSourceURL")]
        pub has_source_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "isModule")]
        pub is_module: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "length")]
        pub length: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "stackTrace")]
        pub stack_trace: Option<super::super::runtime::StackTrace>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "codeOffset")]
        pub code_offset: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "scriptLanguage")]
        pub script_language: Option<super::super::debugger::ScriptLanguage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "debugSymbols")]
        pub debug_symbols: Option<super::super::debugger::DebugSymbols>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "embedderName")]
        pub embedder_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "resolvedBreakpoints")]
        pub resolved_breakpoints: Option<Vec<super::ResolvedBreakpoint>>,
    }
}
