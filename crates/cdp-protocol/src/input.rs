// Auto-generated from Chrome at version 143.0.7499.110 domain: Input
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type TimeSinceEpoch = JsFloat;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GestureSourceType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "touch")]
    Touch,
    #[serde(rename = "mouse")]
    Mouse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MouseButton {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DispatchDragEventTypeOption {
    #[serde(rename = "dragEnter")]
    DragEnter,
    #[serde(rename = "dragOver")]
    DragOver,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "dragCancel")]
    DragCancel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DispatchKeyEventTypeOption {
    #[serde(rename = "keyDown")]
    KeyDown,
    #[serde(rename = "keyUp")]
    KeyUp,
    #[serde(rename = "rawKeyDown")]
    RawKeyDown,
    #[serde(rename = "char")]
    Char,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DispatchMouseEventTypeOption {
    #[serde(rename = "mousePressed")]
    MousePressed,
    #[serde(rename = "mouseReleased")]
    MouseReleased,
    #[serde(rename = "mouseMoved")]
    MouseMoved,
    #[serde(rename = "mouseWheel")]
    MouseWheel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DispatchMouseEventPointerTypeOption {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "pen")]
    Pen,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DispatchTouchEventTypeOption {
    #[serde(rename = "touchStart")]
    TouchStart,
    #[serde(rename = "touchEnd")]
    TouchEnd,
    #[serde(rename = "touchMove")]
    TouchMove,
    #[serde(rename = "touchCancel")]
    TouchCancel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EmulateTouchFromMouseEventTypeOption {
    #[serde(rename = "mousePressed")]
    MousePressed,
    #[serde(rename = "mouseReleased")]
    MouseReleased,
    #[serde(rename = "mouseMoved")]
    MouseMoved,
    #[serde(rename = "mouseWheel")]
    MouseWheel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TouchPoint {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "radiusX")]
    pub radius_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "radiusY")]
    pub radius_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rotationAngle")]
    pub rotation_angle: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "force")]
    pub force: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tangentialPressure")]
    pub tangential_pressure: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tiltX")]
    pub tilt_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tiltY")]
    pub tilt_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "twist")]
    pub twist: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DragDataItem {
    #[serde(default)]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(default)]
    #[serde(rename = "data")]
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "baseURL")]
    pub base_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DragData {
    #[serde(rename = "items")]
    pub items: Vec<DragDataItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "files")]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "dragOperationsMask")]
    pub drag_operations_mask: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchDragEvent {
    #[serde(rename = "type")]
    pub r#type: DispatchDragEventTypeOption,
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(rename = "data")]
    pub data: DragData,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "modifiers")]
    pub modifiers: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchKeyEvent {
    #[serde(rename = "type")]
    pub r#type: DispatchKeyEventTypeOption,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "modifiers")]
    pub modifiers: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp")]
    pub timestamp: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "unmodifiedText")]
    pub unmodified_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keyIdentifier")]
    pub key_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "windowsVirtualKeyCode")]
    pub windows_virtual_key_code: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "nativeVirtualKeyCode")]
    pub native_virtual_key_code: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "autoRepeat")]
    pub auto_repeat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isKeypad")]
    pub is_keypad: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isSystemKey")]
    pub is_system_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "location")]
    pub location: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "commands")]
    pub commands: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsertText {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImeSetComposition {
    #[serde(default)]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(default)]
    #[serde(rename = "selectionStart")]
    pub selection_start: JsUInt,
    #[serde(default)]
    #[serde(rename = "selectionEnd")]
    pub selection_end: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "replacementStart")]
    pub replacement_start: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "replacementEnd")]
    pub replacement_end: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchMouseEvent {
    #[serde(rename = "type")]
    pub r#type: DispatchMouseEventTypeOption,
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "modifiers")]
    pub modifiers: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp")]
    pub timestamp: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "button")]
    pub button: Option<MouseButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "buttons")]
    pub buttons: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "clickCount")]
    pub click_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "force")]
    pub force: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tangentialPressure")]
    pub tangential_pressure: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tiltX")]
    pub tilt_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tiltY")]
    pub tilt_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "twist")]
    pub twist: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deltaX")]
    pub delta_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deltaY")]
    pub delta_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pointerType")]
    pub pointer_type: Option<DispatchMouseEventPointerTypeOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DispatchTouchEvent {
    #[serde(rename = "type")]
    pub r#type: DispatchTouchEventTypeOption,
    #[serde(rename = "touchPoints")]
    pub touch_points: Vec<TouchPoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "modifiers")]
    pub modifiers: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp")]
    pub timestamp: Option<TimeSinceEpoch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelDragging(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulateTouchFromMouseEvent {
    #[serde(rename = "type")]
    pub r#type: EmulateTouchFromMouseEventTypeOption,
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsUInt,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsUInt,
    #[serde(rename = "button")]
    pub button: MouseButton,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp")]
    pub timestamp: Option<TimeSinceEpoch>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deltaX")]
    pub delta_x: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "deltaY")]
    pub delta_y: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "modifiers")]
    pub modifiers: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "clickCount")]
    pub click_count: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetIgnoreInputEvents {
    #[serde(default)]
    #[serde(rename = "ignore")]
    pub ignore: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInterceptDrags {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SynthesizePinchGesture {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(default)]
    #[serde(rename = "scaleFactor")]
    pub scale_factor: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "relativeSpeed")]
    pub relative_speed: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SynthesizeScrollGesture {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "xDistance")]
    pub x_distance: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "yDistance")]
    pub y_distance: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "xOverscroll")]
    pub x_overscroll: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "yOverscroll")]
    pub y_overscroll: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "preventFling")]
    pub prevent_fling: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "speed")]
    pub speed: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "repeatCount")]
    pub repeat_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "repeatDelayMs")]
    pub repeat_delay_ms: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "interactionMarkerName")]
    pub interaction_marker_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SynthesizeTapGesture {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "duration")]
    pub duration: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "tapCount")]
    pub tap_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gestureSourceType")]
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchDragEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchKeyEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ImeSetCompositionReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchMouseEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DispatchTouchEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CancelDraggingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmulateTouchFromMouseEventReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreInputEventsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInterceptDragsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizePinchGestureReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeScrollGestureReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SynthesizeTapGestureReturnObject {}
impl Method for DispatchDragEvent {
    const NAME: &'static str = "Input.dispatchDragEvent";
    type ReturnObject = DispatchDragEventReturnObject;
}
impl Method for DispatchKeyEvent {
    const NAME: &'static str = "Input.dispatchKeyEvent";
    type ReturnObject = DispatchKeyEventReturnObject;
}
impl Method for InsertText {
    const NAME: &'static str = "Input.insertText";
    type ReturnObject = InsertTextReturnObject;
}
impl Method for ImeSetComposition {
    const NAME: &'static str = "Input.imeSetComposition";
    type ReturnObject = ImeSetCompositionReturnObject;
}
impl Method for DispatchMouseEvent {
    const NAME: &'static str = "Input.dispatchMouseEvent";
    type ReturnObject = DispatchMouseEventReturnObject;
}
impl Method for DispatchTouchEvent {
    const NAME: &'static str = "Input.dispatchTouchEvent";
    type ReturnObject = DispatchTouchEventReturnObject;
}
impl Method for CancelDragging {
    const NAME: &'static str = "Input.cancelDragging";
    type ReturnObject = CancelDraggingReturnObject;
}
impl Method for EmulateTouchFromMouseEvent {
    const NAME: &'static str = "Input.emulateTouchFromMouseEvent";
    type ReturnObject = EmulateTouchFromMouseEventReturnObject;
}
impl Method for SetIgnoreInputEvents {
    const NAME: &'static str = "Input.setIgnoreInputEvents";
    type ReturnObject = SetIgnoreInputEventsReturnObject;
}
impl Method for SetInterceptDrags {
    const NAME: &'static str = "Input.setInterceptDrags";
    type ReturnObject = SetInterceptDragsReturnObject;
}
impl Method for SynthesizePinchGesture {
    const NAME: &'static str = "Input.synthesizePinchGesture";
    type ReturnObject = SynthesizePinchGestureReturnObject;
}
impl Method for SynthesizeScrollGesture {
    const NAME: &'static str = "Input.synthesizeScrollGesture";
    type ReturnObject = SynthesizeScrollGestureReturnObject;
}
impl Method for SynthesizeTapGesture {
    const NAME: &'static str = "Input.synthesizeTapGesture";
    type ReturnObject = SynthesizeTapGestureReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DragInterceptedEvent {
        pub params: DragInterceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DragInterceptedEventParams {
        #[serde(rename = "data")]
        pub data: super::DragData,
    }
}
