// Auto-generated from Chrome at version 146.0.7680.165 domain: Debugger
use super::runtime;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Location in the source code."]
pub struct Location {
    #[doc = "Script identifier as reported in the `Debugger.scriptParsed`."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "Line number in the script (0-based)."]
    pub line_number: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Column number in the script (0-based)."]
    pub column_number: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Location in the source code."]
pub struct ScriptPosition {
    #[serde(default)]
    pub line_number: JsUInt,
    #[serde(default)]
    pub column_number: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Location range within one script."]
pub struct LocationRange {
    pub script_id: runtime::ScriptId,
    pub start: ScriptPosition,
    pub end: ScriptPosition,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "JavaScript call frame. Array of call frames form the call stack."]
pub struct CallFrame {
    #[doc = "Call frame identifier. This identifier is only valid while the virtual machine is paused."]
    pub call_frame_id: CallFrameId,
    #[serde(default)]
    #[doc = "Name of the JavaScript function called on this call frame."]
    pub function_name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Location in the source code."]
    pub function_location: Option<Location>,
    #[doc = "Location in the source code."]
    pub location: Location,
    #[serde(default)]
    #[doc = "JavaScript script name or url.\n Deprecated in favor of using the `location.scriptId` to resolve the URL via a previously\n sent `Debugger.scriptParsed` event."]
    #[deprecated]
    pub url: String,
    #[doc = "Scope chain for this call frame."]
    pub scope_chain: Vec<Scope>,
    #[doc = "`this` object for this call frame."]
    pub this: runtime::RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The value being returned, if the function is at return point."]
    pub return_value: Option<runtime::RemoteObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Valid only while the VM is paused and indicates whether this frame\n can be restarted or not. Note that a `true` value here does not\n guarantee that Debugger#restartFrame with this CallFrameId will be\n successful, but it is very likely."]
    pub can_be_restarted: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Scope description."]
pub struct Scope {
    #[doc = "Scope type."]
    pub r#type: ScopeType,
    #[doc = "Object representing the scope. For `global` and `with` scopes it represents the actual\n object; for the rest of the scopes, it is artificial transient object enumerating scope\n variables as its properties."]
    pub object: runtime::RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Location in the source code where scope starts"]
    pub start_location: Option<Location>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Location in the source code where scope ends"]
    pub end_location: Option<Location>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Search match for resource."]
pub struct SearchMatch {
    #[serde(default)]
    #[doc = "Line number in resource content."]
    pub line_number: JsFloat,
    #[serde(default)]
    #[doc = "Line with match content."]
    pub line_content: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation {
    #[doc = "Script identifier as reported in the `Debugger.scriptParsed`."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "Line number in the script (0-based)."]
    pub line_number: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Column number in the script (0-based)."]
    pub column_number: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<BreakLocationType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct WasmDisassemblyChunk {
    #[serde(default)]
    #[doc = "The next chunk of disassembled lines."]
    pub lines: Vec<String>,
    #[serde(default)]
    #[doc = "The bytecode offsets describing the start of each line."]
    pub bytecode_offsets: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Debug symbols available for a wasm script."]
pub struct DebugSymbols {
    #[doc = "Type of the debug symbols."]
    pub r#type: DebugSymbolsType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the external symbol source."]
    #[serde(rename = "externalURL")]
    pub external_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ResolvedBreakpoint {
    #[doc = "Breakpoint unique identifier."]
    pub breakpoint_id: BreakpointId,
    #[doc = "Actual breakpoint location."]
    pub location: Location,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Continues execution until specific location is reached."]
pub struct ContinueToLocation {
    #[doc = "Location to continue to."]
    pub location: Location,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_call_frames: Option<ContinueToLocationTargetCallFramesOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables debugger for the given page. Clients should not assume that the debugging has been\n enabled until the result for this command is received."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The maximum size in bytes of collected scripts (not referenced by other heap objects)\n the debugger can hold. Puts no limit if parameter is omitted."]
    pub max_scripts_cache_size: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates expression on a given call frame."]
pub struct EvaluateOnCallFrame {
    #[doc = "Call frame identifier to evaluate on."]
    pub call_frame_id: CallFrameId,
    #[serde(default)]
    #[doc = "Expression to evaluate."]
    pub expression: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String object group name to put result into (allows rapid releasing resulting object handles\n using `releaseObjectGroup`)."]
    pub object_group: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies whether command line API should be available to the evaluated expression, defaults\n to false."]
    #[serde(rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\n execution. Overrides `setPauseOnException` state."]
    pub silent: Option<bool>,
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
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation."]
    pub throw_on_side_effect: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Terminate execution after timing out (number of milliseconds)."]
    pub timeout: Option<runtime::TimeDelta>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns possible locations for breakpoint. scriptId in start and end range locations should be\n the same."]
pub struct GetPossibleBreakpoints {
    #[doc = "Start of range to search possible breakpoint locations in."]
    pub start: Location,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "End of range to search possible breakpoint locations in (excluding). When not specified, end\n of scripts is used as end of range."]
    pub end: Option<Location>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Only consider locations which are in the same (non-nested) function as start."]
    pub restrict_to_function: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns source for the script with given id."]
pub struct GetScriptSource {
    #[doc = "Id of the script to get source for."]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModule {
    #[doc = "Id of the script to disassemble"]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Disassemble the next chunk of lines for the module corresponding to the\n stream. If disassembly is complete, this API will invalidate the streamId\n and return an empty chunk. Any subsequent calls for the now invalid stream\n will return errors."]
pub struct NextWasmDisassemblyChunk {
    #[serde(default)]
    pub stream_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This command is deprecated. Use getScriptSource instead."]
#[deprecated]
pub struct GetWasmBytecode {
    #[doc = "Id of the Wasm script to get source for."]
    pub script_id: runtime::ScriptId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns stack trace with given `stackTraceId`."]
pub struct GetStackTrace {
    pub stack_trace_id: runtime::StackTraceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Pause(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[deprecated]
pub struct PauseOnAsyncCall {
    #[doc = "Debugger will pause when async call with given stack trace is started."]
    pub parent_stack_trace_id: runtime::StackTraceId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes JavaScript breakpoint."]
pub struct RemoveBreakpoint {
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Restarts particular call frame from the beginning. The old, deprecated\n behavior of `restartFrame` is to stay paused and allow further CDP commands\n after a restart was scheduled. This can cause problems with restarting, so\n we now continue execution immediatly after it has been scheduled until we\n reach the beginning of the restarted frame.\n \n To stay back-wards compatible, `restartFrame` now expects a `mode`\n parameter to be present. If the `mode` parameter is missing, `restartFrame`\n errors out.\n \n The various return values are deprecated and `callFrames` is always empty.\n Use the call frames from the `Debugger#paused` events instead, that fires\n once V8 pauses at the beginning of the restarted function."]
pub struct RestartFrame {
    #[doc = "Call frame identifier to evaluate on."]
    pub call_frame_id: CallFrameId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The `mode` parameter must be present and set to 'StepInto', otherwise\n `restartFrame` will error out."]
    pub mode: Option<RestartFrameModeOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resumes JavaScript execution."]
pub struct Resume {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Set to true to terminate execution upon resuming execution. In contrast\n to Runtime.terminateExecution, this will allows to execute further\n JavaScript (i.e. via evaluation) until execution of the paused code\n is actually resumed, at which point termination is triggered.\n If execution is currently not paused, this parameter has no effect."]
    pub terminate_on_resume: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in script content."]
pub struct SearchInContent {
    #[doc = "Id of the script to search in."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "String to search for."]
    pub query: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, search is case sensitive."]
    pub case_sensitive: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, treats string parameter as regex."]
    pub is_regex: Option<bool>,
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
#[doc = "Replace previous blackbox execution contexts with passed ones. Forces backend to skip\n stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by\n performing 'step in' several times, finally resorting to 'step out' if unsuccessful."]
pub struct SetBlackboxExecutionContexts {
    #[serde(default)]
    #[doc = "Array of execution context unique ids for the debugger to ignore."]
    pub unique_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in\n scripts with url matching one of the patterns. VM will try to leave blackboxed script by\n performing 'step in' several times, finally resorting to 'step out' if unsuccessful."]
pub struct SetBlackboxPatterns {
    #[serde(default)]
    #[doc = "Array of regexps that will be used to check script url for blackbox state."]
    pub patterns: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, also ignore scripts with no source url."]
    pub skip_anonymous: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted\n scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.\n Positions array contains positions where blackbox state is changed. First interval isn't\n blackboxed. Array should be sorted."]
pub struct SetBlackboxedRanges {
    #[doc = "Id of the script."]
    pub script_id: runtime::ScriptId,
    pub positions: Vec<ScriptPosition>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint at a given location."]
pub struct SetBreakpoint {
    #[doc = "Location to set breakpoint in."]
    pub location: Location,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will only stop on the\n breakpoint if this expression evaluates to true."]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets instrumentation breakpoint."]
pub struct SetInstrumentationBreakpoint {
    #[doc = "Instrumentation name."]
    pub instrumentation: SetInstrumentationBreakpointInstrumentationOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this\n command is issued, all existing parsed scripts will have breakpoints resolved and returned in\n `locations` property. Further matching script parsing will result in subsequent\n `breakpointResolved` events issued. This logical breakpoint will survive page reloads."]
pub struct SetBreakpointByUrl {
    #[serde(default)]
    #[doc = "Line number to set breakpoint at."]
    pub line_number: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "URL of the resources to set breakpoint on."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Regex pattern for the URLs of the resources to set breakpoints on. Either `url` or\n `urlRegex` must be specified."]
    pub url_regex: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Script hash of the resources to set breakpoint on."]
    pub script_hash: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Offset in the line to set breakpoint at."]
    pub column_number: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will only stop on the\n breakpoint if this expression evaluates to true."]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint before each call to the given function.\n If another function was created from the same source as a given one,\n calling it will also trigger the breakpoint."]
pub struct SetBreakpointOnFunctionCall {
    #[doc = "Function object id."]
    pub object_id: runtime::RemoteObjectId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will\n stop on the breakpoint if this expression evaluates to true."]
    pub condition: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Activates / deactivates all breakpoints on the page."]
pub struct SetBreakpointsActive {
    #[serde(default)]
    #[doc = "New value for breakpoints active state."]
    pub active: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,\n or caught exceptions, no exceptions. Initial pause on exceptions state is `none`."]
pub struct SetPauseOnExceptions {
    #[doc = "Pause on exceptions mode."]
    pub state: SetPauseOnExceptionsStateOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Changes return value in top frame. Available only at return break position."]
pub struct SetReturnValue {
    #[doc = "New return value."]
    pub new_value: runtime::CallArgument,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Edits JavaScript source live.\n \n In general, functions that are currently on the stack can not be edited with\n a single exception: If the edited function is the top-most stack frame and\n that is the only activation of that function on the stack. In this case\n the live edit will be successful and a `Debugger.restartFrame` for the\n top-most function is automatically triggered."]
pub struct SetScriptSource {
    #[doc = "Id of the script to edit."]
    pub script_id: runtime::ScriptId,
    #[serde(default)]
    #[doc = "New content of the script."]
    pub script_source: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true the change will not actually be applied. Dry run may be used to get result\n description without actually modifying the code."]
    pub dry_run: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, then `scriptSource` is allowed to change the function on top of the stack\n as long as the top-most stack frame is the only activation of that function."]
    pub allow_top_frame_editing: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc)."]
pub struct SetSkipAllPauses {
    #[serde(default)]
    #[doc = "New value for skip pauses state."]
    pub skip: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Changes value of variable in a callframe. Object-based scopes are not supported and must be\n mutated manually."]
pub struct SetVariableValue {
    #[serde(default)]
    #[doc = "0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'\n scope types are allowed. Other scopes could be manipulated manually."]
    pub scope_number: JsUInt,
    #[serde(default)]
    #[doc = "Variable name."]
    pub variable_name: String,
    #[doc = "New variable value."]
    pub new_value: runtime::CallArgument,
    #[doc = "Id of callframe that holds variable."]
    pub call_frame_id: CallFrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Steps into the function call."]
pub struct StepInto {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Debugger will pause on the execution of the first async task which was scheduled\n before next pause."]
    pub break_on_async_call: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The skipList specifies location ranges that should be skipped on step into."]
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StepOut(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Steps over the statement."]
pub struct StepOver {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The skipList specifies location ranges that should be skipped on step over."]
    pub skip_list: Option<Vec<LocationRange>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Continues execution until specific location is reached."]
pub struct ContinueToLocationReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables debugger for given page."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Enables debugger for the given page. Clients should not assume that the debugging has been\n enabled until the result for this command is received."]
pub struct EnableReturnObject {
    #[doc = "Unique identifier of the debugger."]
    pub debugger_id: runtime::UniqueDebuggerId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Evaluates expression on a given call frame."]
pub struct EvaluateOnCallFrameReturnObject {
    #[doc = "Object wrapper for the evaluation result."]
    pub result: runtime::RemoteObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details."]
    pub exception_details: Option<runtime::ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns possible locations for breakpoint. scriptId in start and end range locations should be\n the same."]
pub struct GetPossibleBreakpointsReturnObject {
    #[doc = "List of the possible breakpoint locations."]
    pub locations: Vec<BreakLocation>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns source for the script with given id."]
pub struct GetScriptSourceReturnObject {
    #[serde(default)]
    #[doc = "Script source (empty in case of Wasm bytecode)."]
    pub script_source: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Wasm bytecode."]
    pub bytecode: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "For large modules, return a stream from which additional chunks of\n disassembly can be read successively."]
    pub stream_id: Option<String>,
    #[serde(default)]
    #[doc = "The total number of lines in the disassembly text."]
    pub total_number_of_lines: JsUInt,
    #[doc = "The offsets of all function bodies, in the format [start1, end1,\n start2, end2, ...] where all ends are exclusive."]
    pub function_body_offsets: Vec<JsUInt>,
    #[doc = "The first chunk of disassembly."]
    pub chunk: WasmDisassemblyChunk,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Disassemble the next chunk of lines for the module corresponding to the\n stream. If disassembly is complete, this API will invalidate the streamId\n and return an empty chunk. Any subsequent calls for the now invalid stream\n will return errors."]
pub struct NextWasmDisassemblyChunkReturnObject {
    #[doc = "The next chunk of disassembly."]
    pub chunk: WasmDisassemblyChunk,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "This command is deprecated. Use getScriptSource instead."]
#[deprecated]
pub struct GetWasmBytecodeReturnObject {
    #[doc = "Script source."]
    pub bytecode: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns stack trace with given `stackTraceId`."]
pub struct GetStackTraceReturnObject {
    pub stack_trace: runtime::StackTrace,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Stops on the next JavaScript statement."]
pub struct PauseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[deprecated]
pub struct PauseOnAsyncCallReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes JavaScript breakpoint."]
pub struct RemoveBreakpointReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Restarts particular call frame from the beginning. The old, deprecated\n behavior of `restartFrame` is to stay paused and allow further CDP commands\n after a restart was scheduled. This can cause problems with restarting, so\n we now continue execution immediatly after it has been scheduled until we\n reach the beginning of the restarted frame.\n \n To stay back-wards compatible, `restartFrame` now expects a `mode`\n parameter to be present. If the `mode` parameter is missing, `restartFrame`\n errors out.\n \n The various return values are deprecated and `callFrames` is always empty.\n Use the call frames from the `Debugger#paused` events instead, that fires\n once V8 pauses at the beginning of the restarted function."]
pub struct RestartFrameReturnObject {
    #[doc = "New stack trace."]
    #[deprecated]
    pub call_frames: Vec<CallFrame>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Async stack trace, if any."]
    #[deprecated]
    pub async_stack_trace: Option<runtime::StackTrace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Async stack trace, if any."]
    #[deprecated]
    pub async_stack_trace_id: Option<runtime::StackTraceId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resumes JavaScript execution."]
pub struct ResumeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Searches for given string in script content."]
pub struct SearchInContentReturnObject {
    #[doc = "List of search matches."]
    pub result: Vec<SearchMatch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables or disables async call stacks tracking."]
pub struct SetAsyncCallStackDepthReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Replace previous blackbox execution contexts with passed ones. Forces backend to skip\n stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by\n performing 'step in' several times, finally resorting to 'step out' if unsuccessful."]
pub struct SetBlackboxExecutionContextsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in\n scripts with url matching one of the patterns. VM will try to leave blackboxed script by\n performing 'step in' several times, finally resorting to 'step out' if unsuccessful."]
pub struct SetBlackboxPatternsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted\n scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.\n Positions array contains positions where blackbox state is changed. First interval isn't\n blackboxed. Array should be sorted."]
pub struct SetBlackboxedRangesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint at a given location."]
pub struct SetBreakpointReturnObject {
    #[doc = "Id of the created breakpoint for further reference."]
    pub breakpoint_id: BreakpointId,
    #[doc = "Location this breakpoint resolved into."]
    pub actual_location: Location,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets instrumentation breakpoint."]
pub struct SetInstrumentationBreakpointReturnObject {
    #[doc = "Id of the created breakpoint for further reference."]
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this\n command is issued, all existing parsed scripts will have breakpoints resolved and returned in\n `locations` property. Further matching script parsing will result in subsequent\n `breakpointResolved` events issued. This logical breakpoint will survive page reloads."]
pub struct SetBreakpointByUrlReturnObject {
    #[doc = "Id of the created breakpoint for further reference."]
    pub breakpoint_id: BreakpointId,
    #[doc = "List of the locations this breakpoint resolved into upon addition."]
    pub locations: Vec<Location>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sets JavaScript breakpoint before each call to the given function.\n If another function was created from the same source as a given one,\n calling it will also trigger the breakpoint."]
pub struct SetBreakpointOnFunctionCallReturnObject {
    #[doc = "Id of the created breakpoint for further reference."]
    pub breakpoint_id: BreakpointId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Activates / deactivates all breakpoints on the page."]
pub struct SetBreakpointsActiveReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,\n or caught exceptions, no exceptions. Initial pause on exceptions state is `none`."]
pub struct SetPauseOnExceptionsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Changes return value in top frame. Available only at return break position."]
pub struct SetReturnValueReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Edits JavaScript source live.\n \n In general, functions that are currently on the stack can not be edited with\n a single exception: If the edited function is the top-most stack frame and\n that is the only activation of that function on the stack. In this case\n the live edit will be successful and a `Debugger.restartFrame` for the\n top-most function is automatically triggered."]
pub struct SetScriptSourceReturnObject {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "New stack trace in case editing has happened while VM was stopped."]
    #[deprecated]
    pub call_frames: Option<Vec<CallFrame>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether current call stack  was modified after applying the changes."]
    #[deprecated]
    pub stack_changed: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Async stack trace, if any."]
    #[deprecated]
    pub async_stack_trace: Option<runtime::StackTrace>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Async stack trace, if any."]
    #[deprecated]
    pub async_stack_trace_id: Option<runtime::StackTraceId>,
    #[doc = "Whether the operation was successful or not. Only `Ok` denotes a\n successful live edit while the other enum variants denote why\n the live edit failed."]
    pub status: StatusOption,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Exception details if any. Only present when `status` is `CompileError`."]
    pub exception_details: Option<runtime::ExceptionDetails>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc)."]
pub struct SetSkipAllPausesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Changes value of variable in a callframe. Object-based scopes are not supported and must be\n mutated manually."]
pub struct SetVariableValueReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Steps into the function call."]
pub struct StepIntoReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Steps out of the function call."]
pub struct StepOutReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Steps over the statement."]
pub struct StepOverReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BreakpointResolvedEvent {
        pub params: BreakpointResolvedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct BreakpointResolvedEventParams {
        #[doc = "Breakpoint unique identifier."]
        pub breakpoint_id: super::BreakpointId,
        #[doc = "Actual breakpoint location."]
        pub location: super::Location,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct PausedEvent {
        pub params: PausedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct PausedEventParams {
        #[doc = "Call stack the virtual machine stopped on."]
        pub call_frames: Vec<super::CallFrame>,
        #[doc = "Pause reason."]
        pub reason: super::PausedReasonOption,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Object containing break-specific auxiliary properties."]
        pub data: Option<Json>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Hit breakpoints IDs"]
        pub hit_breakpoints: Option<Vec<String>>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Async stack trace, if any."]
        pub async_stack_trace: Option<super::super::runtime::StackTrace>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Async stack trace, if any."]
        pub async_stack_trace_id: Option<super::super::runtime::StackTraceId>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Never present, will be removed."]
        #[deprecated]
        pub async_call_stack_trace_id: Option<super::super::runtime::StackTraceId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ResumedEvent(pub Option<Json>);
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptFailedToParseEvent {
        pub params: ScriptFailedToParseEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ScriptFailedToParseEventParams {
        #[doc = "Identifier of the script parsed."]
        pub script_id: super::super::runtime::ScriptId,
        #[serde(default)]
        #[doc = "URL or name of the script parsed (if any)."]
        pub url: String,
        #[serde(default)]
        #[doc = "Line offset of the script within the resource with given URL (for script tags)."]
        pub start_line: JsUInt,
        #[serde(default)]
        #[doc = "Column offset of the script within the resource with given URL."]
        pub start_column: JsUInt,
        #[serde(default)]
        #[doc = "Last line of the script."]
        pub end_line: JsUInt,
        #[serde(default)]
        #[doc = "Length of the last line of the script."]
        pub end_column: JsUInt,
        #[doc = "Specifies script creation context."]
        pub execution_context_id: super::super::runtime::ExecutionContextId,
        #[serde(default)]
        #[doc = "Content hash of the script, SHA-256."]
        pub hash: String,
        #[serde(default)]
        #[doc = "For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment."]
        pub build_id: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
        pub execution_context_aux_data: Option<Json>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "URL of source map associated with script (if any)."]
        #[serde(rename = "sourceMapURL")]
        pub source_map_url: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True, if this script has sourceURL."]
        #[serde(rename = "hasSourceURL")]
        pub has_source_url: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True, if this script is ES6 module."]
        pub is_module: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "This script length."]
        pub length: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "JavaScript top stack frame of where the script parsed event was triggered if available."]
        pub stack_trace: Option<super::super::runtime::StackTrace>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "If the scriptLanguage is WebAssembly, the code section offset in the module."]
        pub code_offset: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The language of the script."]
        pub script_language: Option<super::super::debugger::ScriptLanguage>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "The name the embedder supplied for this script."]
        pub embedder_name: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ScriptParsedEvent {
        pub params: ScriptParsedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ScriptParsedEventParams {
        #[doc = "Identifier of the script parsed."]
        pub script_id: super::super::runtime::ScriptId,
        #[serde(default)]
        #[doc = "URL or name of the script parsed (if any)."]
        pub url: String,
        #[serde(default)]
        #[doc = "Line offset of the script within the resource with given URL (for script tags)."]
        pub start_line: JsUInt,
        #[serde(default)]
        #[doc = "Column offset of the script within the resource with given URL."]
        pub start_column: JsUInt,
        #[serde(default)]
        #[doc = "Last line of the script."]
        pub end_line: JsUInt,
        #[serde(default)]
        #[doc = "Length of the last line of the script."]
        pub end_column: JsUInt,
        #[doc = "Specifies script creation context."]
        pub execution_context_id: super::super::runtime::ExecutionContextId,
        #[serde(default)]
        #[doc = "Content hash of the script, SHA-256."]
        pub hash: String,
        #[serde(default)]
        #[doc = "For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment."]
        pub build_id: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
        pub execution_context_aux_data: Option<Json>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True, if this script is generated as a result of the live edit operation."]
        pub is_live_edit: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "URL of source map associated with script (if any)."]
        #[serde(rename = "sourceMapURL")]
        pub source_map_url: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True, if this script has sourceURL."]
        #[serde(rename = "hasSourceURL")]
        pub has_source_url: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "True, if this script is ES6 module."]
        pub is_module: Option<bool>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "This script length."]
        pub length: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "JavaScript top stack frame of where the script parsed event was triggered if available."]
        pub stack_trace: Option<super::super::runtime::StackTrace>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "If the scriptLanguage is WebAssembly, the code section offset in the module."]
        pub code_offset: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The language of the script."]
        pub script_language: Option<super::super::debugger::ScriptLanguage>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "If the scriptLanguage is WebAssembly, the source of debug symbols for the module."]
        pub debug_symbols: Option<super::super::debugger::DebugSymbols>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "The name the embedder supplied for this script."]
        pub embedder_name: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "The list of set breakpoints in this script if calls to `setBreakpointByUrl`\n matches this script's URL or hash. Clients that use this list can ignore the\n `breakpointResolved` event. They are equivalent."]
        pub resolved_breakpoints: Option<Vec<super::ResolvedBreakpoint>>,
    }
}
