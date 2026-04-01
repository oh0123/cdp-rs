// Auto-generated from Chrome at version 146.0.7680.165 domain: Input
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct TouchPoint {
    #[serde(default)]
    #[doc = "X coordinate of the event relative to the main frame's viewport in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to\n the top of the viewport and Y increases as it proceeds towards the bottom of the viewport."]
    pub y: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "X radius of the touch area (default: 1.0)."]
    pub radius_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Y radius of the touch area (default: 1.0)."]
    pub radius_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Rotation angle (default: 0.0)."]
    pub rotation_angle: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Force (default: 1.0)."]
    pub force: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The normalized tangential pressure, which has a range of [-1,1] (default: 0)."]
    pub tangential_pressure: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)"]
    pub tilt_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0)."]
    pub tilt_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0)."]
    pub twist: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Identifier used to track touch sources between events, must be unique within an event."]
    pub id: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DragDataItem {
    #[serde(default)]
    #[doc = "Mime type of the dragged data."]
    pub mime_type: String,
    #[serde(default)]
    #[doc = "Depending of the value of `mimeType`, it contains the dragged link,\n text, HTML markup or any other data."]
    pub data: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Title associated with a link. Only valid when `mimeType` == \"text/uri-list\"."]
    pub title: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Stores the base URL for the contained markup. Only valid when `mimeType`\n == \"text/html\"."]
    #[serde(rename = "baseURL")]
    pub base_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DragData {
    pub items: Vec<DragDataItem>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "List of filenames that should be included when dropping"]
    pub files: Option<Vec<String>>,
    #[serde(default)]
    #[doc = "Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16"]
    pub drag_operations_mask: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Dispatches a drag event into the page."]
pub struct DispatchDragEvent {
    #[doc = "Type of the drag event."]
    pub r#type: DispatchDragEventTypeOption,
    #[serde(default)]
    #[doc = "X coordinate of the event relative to the main frame's viewport in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to\n the top of the viewport and Y increases as it proceeds towards the bottom of the viewport."]
    pub y: JsFloat,
    pub data: DragData,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8\n (default: 0)."]
    pub modifiers: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Dispatches a key event to the page."]
pub struct DispatchKeyEvent {
    #[doc = "Type of the key event."]
    pub r#type: DispatchKeyEventTypeOption,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8\n (default: 0)."]
    pub modifiers: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Time at which the event occurred."]
    pub timestamp: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Text as generated by processing a virtual key code with a keyboard layout. Not needed for\n for `keyUp` and `rawKeyDown` events (default: \"\")"]
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Text that would have been generated by the keyboard if no modifiers were pressed (except for\n shift). Useful for shortcut (accelerator) key handling (default: \"\")."]
    pub unmodified_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unique key identifier (e.g., 'U+0041') (default: \"\")."]
    pub key_identifier: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unique DOM defined string value for each physical key (e.g., 'KeyA') (default: \"\")."]
    pub code: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Unique DOM defined string value describing the meaning of the key in the context of active\n modifiers, keyboard layout, etc (e.g., 'AltGr') (default: \"\")."]
    pub key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Windows virtual key code (default: 0)."]
    pub windows_virtual_key_code: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Native virtual key code (default: 0)."]
    pub native_virtual_key_code: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the event was generated from auto repeat (default: false)."]
    pub auto_repeat: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the event was generated from the keypad (default: false)."]
    pub is_keypad: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the event was a system key event (default: false)."]
    pub is_system_key: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether the event was from the left or right side of the keyboard. 1=Left, 2=Right (default:\n 0)."]
    pub location: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Editing commands to send with the key event (e.g., 'selectAll') (default: []).\n These are related to but not equal the command names used in `document.execCommand` and NSStandardKeyBindingResponding.\n See https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/editing/commands/editor_command_names.h for valid command names."]
    pub commands: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This method emulates inserting text that doesn't come from a key press,\n for example an emoji keyboard or an IME."]
pub struct InsertText {
    #[serde(default)]
    #[doc = "The text to insert."]
    pub text: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "This method sets the current candidate text for IME.\n Use imeCommitComposition to commit the final text.\n Use imeSetComposition with empty string as text to cancel composition."]
pub struct ImeSetComposition {
    #[serde(default)]
    #[doc = "The text to insert"]
    pub text: String,
    #[serde(default)]
    #[doc = "selection start"]
    pub selection_start: JsUInt,
    #[serde(default)]
    #[doc = "selection end"]
    pub selection_end: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "replacement start"]
    pub replacement_start: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "replacement end"]
    pub replacement_end: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Dispatches a mouse event to the page."]
pub struct DispatchMouseEvent {
    #[doc = "Type of the mouse event."]
    pub r#type: DispatchMouseEventTypeOption,
    #[serde(default)]
    #[doc = "X coordinate of the event relative to the main frame's viewport in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to\n the top of the viewport and Y increases as it proceeds towards the bottom of the viewport."]
    pub y: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8\n (default: 0)."]
    pub modifiers: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Time at which the event occurred."]
    pub timestamp: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Mouse button (default: \"none\")."]
    pub button: Option<MouseButton>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A number indicating which buttons are pressed on the mouse when a mouse event is triggered.\n Left=1, Right=2, Middle=4, Back=8, Forward=16, None=0."]
    pub buttons: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of times the mouse button was clicked (default: 0)."]
    pub click_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The normalized pressure, which has a range of [0,1] (default: 0)."]
    pub force: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The normalized tangential pressure, which has a range of [-1,1] (default: 0)."]
    pub tangential_pressure: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)."]
    pub tilt_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0)."]
    pub tilt_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0)."]
    pub twist: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "X delta in CSS pixels for mouse wheel event (default: 0)."]
    pub delta_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Y delta in CSS pixels for mouse wheel event (default: 0)."]
    pub delta_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Pointer type (default: \"mouse\")."]
    pub pointer_type: Option<DispatchMouseEventPointerTypeOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Dispatches a touch event to the page."]
pub struct DispatchTouchEvent {
    #[doc = "Type of the touch event. TouchEnd and TouchCancel must not contain any touch points, while\n TouchStart and TouchMove must contains at least one."]
    pub r#type: DispatchTouchEventTypeOption,
    #[doc = "Active touch points on the touch device. One event per any changed point (compared to\n previous touch event in a sequence) is generated, emulating pressing/moving/releasing points\n one by one."]
    pub touch_points: Vec<TouchPoint>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8\n (default: 0)."]
    pub modifiers: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Time at which the event occurred."]
    pub timestamp: Option<TimeSinceEpoch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CancelDragging(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Emulates touch event from the mouse event parameters."]
pub struct EmulateTouchFromMouseEvent {
    #[doc = "Type of the mouse event."]
    pub r#type: EmulateTouchFromMouseEventTypeOption,
    #[serde(default)]
    #[doc = "X coordinate of the mouse pointer in DIP."]
    pub x: JsUInt,
    #[serde(default)]
    #[doc = "Y coordinate of the mouse pointer in DIP."]
    pub y: JsUInt,
    #[doc = "Mouse button. Only \"none\", \"left\", \"right\" are supported."]
    pub button: MouseButton,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Time at which the event occurred (default: current time)."]
    pub timestamp: Option<TimeSinceEpoch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "X delta in DIP for mouse wheel event (default: 0)."]
    pub delta_x: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Y delta in DIP for mouse wheel event (default: 0)."]
    pub delta_y: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Bit field representing pressed modifier keys. Alt=1, Ctrl=2, Meta/Command=4, Shift=8\n (default: 0)."]
    pub modifiers: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of times the mouse button was clicked (default: 0)."]
    pub click_count: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Ignores input events (useful while auditing page)."]
pub struct SetIgnoreInputEvents {
    #[serde(default)]
    #[doc = "Ignores input events processing when set to true."]
    pub ignore: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.\n Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`."]
pub struct SetInterceptDrags {
    #[serde(default)]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Synthesizes a pinch gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizePinchGesture {
    #[serde(default)]
    #[doc = "X coordinate of the start of the gesture in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the start of the gesture in CSS pixels."]
    pub y: JsFloat,
    #[serde(default)]
    #[doc = "Relative scale factor after zooming (>1.0 zooms in, <1.0 zooms out)."]
    pub scale_factor: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Relative pointer speed in pixels per second (default: 800)."]
    pub relative_speed: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Which type of input events to be generated (default: 'default', which queries the platform\n for the preferred input type)."]
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Synthesizes a scroll gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizeScrollGesture {
    #[serde(default)]
    #[doc = "X coordinate of the start of the gesture in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the start of the gesture in CSS pixels."]
    pub y: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The distance to scroll along the X axis (positive to scroll left)."]
    pub x_distance: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The distance to scroll along the Y axis (positive to scroll up)."]
    pub y_distance: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The number of additional pixels to scroll back along the X axis, in addition to the given\n distance."]
    pub x_overscroll: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The number of additional pixels to scroll back along the Y axis, in addition to the given\n distance."]
    pub y_overscroll: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Prevent fling (default: true)."]
    pub prevent_fling: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Swipe speed in pixels per second (default: 800)."]
    pub speed: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Which type of input events to be generated (default: 'default', which queries the platform\n for the preferred input type)."]
    pub gesture_source_type: Option<GestureSourceType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The number of times to repeat the gesture (default: 0)."]
    pub repeat_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The number of milliseconds delay between each repeat. (default: 250)."]
    pub repeat_delay_ms: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The name of the interaction markers to generate, if not empty (default: \"\")."]
    pub interaction_marker_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Synthesizes a tap gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizeTapGesture {
    #[serde(default)]
    #[doc = "X coordinate of the start of the gesture in CSS pixels."]
    pub x: JsFloat,
    #[serde(default)]
    #[doc = "Y coordinate of the start of the gesture in CSS pixels."]
    pub y: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Duration between touchdown and touchup events in ms (default: 50)."]
    pub duration: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of times to perform the tap (e.g. 2 for double tap, default: 1)."]
    pub tap_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Which type of input events to be generated (default: 'default', which queries the platform\n for the preferred input type)."]
    pub gesture_source_type: Option<GestureSourceType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Dispatches a drag event into the page."]
pub struct DispatchDragEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Dispatches a key event to the page."]
pub struct DispatchKeyEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "This method emulates inserting text that doesn't come from a key press,\n for example an emoji keyboard or an IME."]
pub struct InsertTextReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "This method sets the current candidate text for IME.\n Use imeCommitComposition to commit the final text.\n Use imeSetComposition with empty string as text to cancel composition."]
pub struct ImeSetCompositionReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Dispatches a mouse event to the page."]
pub struct DispatchMouseEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Dispatches a touch event to the page."]
pub struct DispatchTouchEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Cancels any active dragging in the page."]
pub struct CancelDraggingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Emulates touch event from the mouse event parameters."]
pub struct EmulateTouchFromMouseEventReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Ignores input events (useful while auditing page)."]
pub struct SetIgnoreInputEventsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Prevents default drag and drop behavior and instead emits `Input.dragIntercepted` events.\n Drag and drop behavior can be directly controlled via `Input.dispatchDragEvent`."]
pub struct SetInterceptDragsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Synthesizes a pinch gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizePinchGestureReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Synthesizes a scroll gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizeScrollGestureReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Synthesizes a tap gesture over a time period by issuing appropriate touch events."]
pub struct SynthesizeTapGestureReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DragInterceptedEvent {
        pub params: DragInterceptedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DragInterceptedEventParams {
        pub data: super::DragData,
    }
}
