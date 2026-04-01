// Auto-generated from Chrome at version 146.0.7680.165 domain: Emulation
use super::dom;
use super::network;
use super::page;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type ScreenId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScreenOrientationType {
    #[serde(rename = "portraitPrimary")]
    PortraitPrimary,
    #[serde(rename = "portraitSecondary")]
    PortraitSecondary,
    #[serde(rename = "landscapePrimary")]
    LandscapePrimary,
    #[serde(rename = "landscapeSecondary")]
    LandscapeSecondary,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DisplayFeatureOrientation {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DevicePostureType {
    #[serde(rename = "continuous")]
    Continuous,
    #[serde(rename = "folded")]
    Folded,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VirtualTimePolicy {
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "pauseIfNetworkFetchesPending")]
    PauseIfNetworkFetchesPending,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SensorType {
    #[serde(rename = "absolute-orientation")]
    AbsoluteOrientation,
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "ambient-light")]
    AmbientLight,
    #[serde(rename = "gravity")]
    Gravity,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "linear-acceleration")]
    LinearAcceleration,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "relative-orientation")]
    RelativeOrientation,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PressureSource {
    #[serde(rename = "cpu")]
    Cpu,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PressureState {
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "serious")]
    Serious,
    #[serde(rename = "critical")]
    Critical,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DisabledImageType {
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "webp")]
    Webp,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetEmitTouchEventsForMouseConfigurationOption {
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetEmulatedVisionDeficiencyTypeOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "blurredVision")]
    BlurredVision,
    #[serde(rename = "reducedContrast")]
    ReducedContrast,
    #[serde(rename = "achromatopsia")]
    Achromatopsia,
    #[serde(rename = "deuteranopia")]
    Deuteranopia,
    #[serde(rename = "protanopia")]
    Protanopia,
    #[serde(rename = "tritanopia")]
    Tritanopia,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SafeAreaInsets {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-inset-top."]
    pub top: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-max-inset-top."]
    pub top_max: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-inset-left."]
    pub left: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-max-inset-left."]
    pub left_max: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-inset-bottom."]
    pub bottom: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-max-inset-bottom."]
    pub bottom_max: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-inset-right."]
    pub right: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overrides safe-area-max-inset-right."]
    pub right_max: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Screen orientation."]
pub struct ScreenOrientation {
    #[doc = "Orientation type."]
    pub r#type: ScreenOrientationType,
    #[serde(default)]
    #[doc = "Orientation angle."]
    pub angle: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DisplayFeature {
    #[doc = "Orientation of a display feature in relation to screen"]
    pub orientation: DisplayFeatureOrientation,
    #[serde(default)]
    #[doc = "The offset from the screen origin in either the x (for vertical\n orientation) or y (for horizontal orientation) direction."]
    pub offset: JsUInt,
    #[serde(default)]
    #[doc = "A display feature may mask content such that it is not physically\n displayed - this length along with the offset describes this area.\n A display feature that only splits content will have a 0 mask_length."]
    pub mask_length: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DevicePosture {
    #[doc = "Current posture of the device"]
    pub r#type: DevicePostureType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct MediaFeature {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints"]
pub struct UserAgentBrandVersion {
    #[serde(default)]
    pub brand: String,
    #[serde(default)]
    pub version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints\n Missing optional values will be filled in by the target with what it would normally use."]
pub struct UserAgentMetadata {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Brands appearing in Sec-CH-UA."]
    pub brands: Option<Vec<UserAgentBrandVersion>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Brands appearing in Sec-CH-UA-Full-Version-List."]
    pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[deprecated]
    pub full_version: Option<String>,
    #[serde(default)]
    pub platform: String,
    #[serde(default)]
    pub platform_version: String,
    #[serde(default)]
    pub architecture: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub mobile: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bitness: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub wow_64: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Used to specify User Agent form-factor values.\n See https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors"]
    pub form_factors: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SensorMetadata {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub available: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub minimum_frequency: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub maximum_frequency: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingSingle {
    #[serde(default)]
    pub value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingXyz {
    #[serde(default)]
    pub x: JsFloat,
    #[serde(default)]
    pub y: JsFloat,
    #[serde(default)]
    pub z: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SensorReadingQuaternion {
    #[serde(default)]
    pub x: JsFloat,
    #[serde(default)]
    pub y: JsFloat,
    #[serde(default)]
    pub z: JsFloat,
    #[serde(default)]
    pub w: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SensorReading {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single: Option<SensorReadingSingle>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xyz: Option<SensorReadingXyz>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quaternion: Option<SensorReadingQuaternion>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct PressureMetadata {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub available: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct WorkAreaInsets {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Work area top inset in pixels. Default is 0;"]
    pub top: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Work area left inset in pixels. Default is 0;"]
    pub left: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Work area bottom inset in pixels. Default is 0;"]
    pub bottom: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Work area right inset in pixels. Default is 0;"]
    pub right: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Screen information similar to the one returned by window.getScreenDetails() method,\n see https://w3c.github.io/window-management/#screendetailed."]
pub struct ScreenInfo {
    #[serde(default)]
    #[doc = "Offset of the left edge of the screen."]
    pub left: JsUInt,
    #[serde(default)]
    #[doc = "Offset of the top edge of the screen."]
    pub top: JsUInt,
    #[serde(default)]
    #[doc = "Width of the screen."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Height of the screen."]
    pub height: JsUInt,
    #[serde(default)]
    #[doc = "Offset of the left edge of the available screen area."]
    pub avail_left: JsUInt,
    #[serde(default)]
    #[doc = "Offset of the top edge of the available screen area."]
    pub avail_top: JsUInt,
    #[serde(default)]
    #[doc = "Width of the available screen area."]
    pub avail_width: JsUInt,
    #[serde(default)]
    #[doc = "Height of the available screen area."]
    pub avail_height: JsUInt,
    #[serde(default)]
    #[doc = "Specifies the screen's device pixel ratio."]
    pub device_pixel_ratio: JsFloat,
    #[doc = "Specifies the screen's orientation."]
    pub orientation: ScreenOrientation,
    #[serde(default)]
    #[doc = "Specifies the screen's color depth in bits."]
    pub color_depth: JsUInt,
    #[serde(default)]
    #[doc = "Indicates whether the device has multiple screens."]
    pub is_extended: bool,
    #[serde(default)]
    #[doc = "Indicates whether the screen is internal to the device or external, attached to the device."]
    pub is_internal: bool,
    #[serde(default)]
    #[doc = "Indicates whether the screen is set as the the operating system primary screen."]
    pub is_primary: bool,
    #[serde(default)]
    #[doc = "Specifies the descriptive label for the screen."]
    pub label: String,
    #[doc = "Specifies the unique identifier of the screen."]
    pub id: ScreenId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanEmulate(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDeviceMetricsOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearGeolocationOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResetPageScaleFactor(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables or disables simulating a focused and active page."]
pub struct SetFocusEmulationEnabled {
    #[serde(default)]
    #[doc = "Whether to enable to disable focus emulation."]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Automatically render all web contents using a dark theme."]
pub struct SetAutoDarkModeOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to enable or disable automatic dark mode.\n If not specified, any existing override will be cleared."]
    pub enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables CPU throttling to emulate slow CPUs."]
pub struct SetCPUThrottlingRate {
    #[serde(default)]
    #[doc = "Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc)."]
    pub rate: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\n if the content does not specify one."]
pub struct SetDefaultBackgroundColorOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "RGBA of the default background color. If not specified, any existing override will be\n cleared."]
    pub color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the\n respective variables to be undefined, even if previously overridden."]
pub struct SetSafeAreaInsetsOverride {
    pub insets: SafeAreaInsets,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\n window.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\n query results)."]
pub struct SetDeviceMetricsOverride {
    #[serde(default)]
    #[doc = "Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    pub height: JsUInt,
    #[serde(default)]
    #[doc = "Overriding device scale factor value. 0 disables the override."]
    pub device_scale_factor: JsFloat,
    #[serde(default)]
    #[doc = "Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text\n autosizing and more."]
    pub mobile: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Scale to apply to resulting view image."]
    pub scale: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding screen width value in pixels (minimum 0, maximum 10000000)."]
    pub screen_width: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding screen height value in pixels (minimum 0, maximum 10000000)."]
    pub screen_height: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding view X position on screen in pixels (minimum 0, maximum 10000000)."]
    pub position_x: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Overriding view Y position on screen in pixels (minimum 0, maximum 10000000)."]
    pub position_y: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Do not set visible view size, rely upon explicit setVisibleSize call."]
    pub dont_set_visible_size: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Screen orientation override."]
    pub screen_orientation: Option<ScreenOrientation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, the visible area of the page will be overridden to this viewport. This viewport\n change is not observed by the page, e.g. viewport-relative elements do not change positions."]
    pub viewport: Option<page::Viewport>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, the display feature of a multi-segment screen. If not set, multi-segment support\n is turned-off.\n Deprecated, use Emulation.setDisplayFeaturesOverride."]
    #[deprecated]
    pub display_feature: Option<DisplayFeature>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, the posture of a foldable device. If not set the posture is set\n to continuous.\n Deprecated, use Emulation.setDevicePostureOverride."]
    #[deprecated]
    pub device_posture: Option<DevicePosture>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Start reporting the given posture value to the Device Posture API.\n This override can also be set in setDeviceMetricsOverride()."]
pub struct SetDevicePostureOverride {
    pub posture: DevicePosture,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDevicePostureOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Start using the given display features to pupulate the Viewport Segments API.\n This override can also be set in setDeviceMetricsOverride()."]
pub struct SetDisplayFeaturesOverride {
    pub features: Vec<DisplayFeature>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDisplayFeaturesOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetScrollbarsHidden {
    #[serde(default)]
    #[doc = "Whether scrollbars should be always hidden."]
    pub hidden: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentCookieDisabled {
    #[serde(default)]
    #[doc = "Whether document.coookie API should be disabled."]
    pub disabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetEmitTouchEventsForMouse {
    #[serde(default)]
    #[doc = "Whether touch emulation based on mouse input should be enabled."]
    pub enabled: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Touch/gesture events configuration. Default: current platform."]
    pub configuration: Option<SetEmitTouchEventsForMouseConfigurationOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Emulates the given media type or media feature for CSS media queries."]
pub struct SetEmulatedMedia {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Media type to emulate. Empty string disables the override."]
    pub media: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Media features to emulate."]
    pub features: Option<Vec<MediaFeature>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Emulates the given vision deficiency."]
pub struct SetEmulatedVisionDeficiency {
    #[doc = "Vision deficiency to emulate. Order: best-effort emulations come first, followed by any\n physiologically accurate emulations for medically recognized color vision deficiencies."]
    pub r#type: SetEmulatedVisionDeficiencyTypeOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Emulates the given OS text scale."]
pub struct SetEmulatedOSTextScale {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scale: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the Geolocation Position or Error. Omitting latitude, longitude or\n accuracy emulates position unavailable."]
pub struct SetGeolocationOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock latitude"]
    pub latitude: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock longitude"]
    pub longitude: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock accuracy"]
    pub accuracy: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock altitude"]
    pub altitude: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock altitudeAccuracy"]
    pub altitude_accuracy: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock heading"]
    pub heading: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Mock speed"]
    pub speed: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformation {
    pub r#type: SensorType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides a platform sensor of a given type. If |enabled| is true, calls to\n Sensor.start() will use a virtual sensor as backend rather than fetching\n data from a real hardware sensor. Otherwise, existing virtual\n sensor-backend Sensor objects will fire an error event and new calls to\n Sensor.start() will attempt to use a real sensor instead."]
pub struct SetSensorOverrideEnabled {
    #[serde(default)]
    pub enabled: bool,
    pub r#type: SensorType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SensorMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Updates the sensor readings reported by a sensor type previously overridden\n by setSensorOverrideEnabled."]
pub struct SetSensorOverrideReadings {
    pub r#type: SensorType,
    pub reading: SensorReading,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides a pressure source of a given type, as used by the Compute\n Pressure API, so that updates to PressureObserver.observe() are provided\n via setPressureStateOverride instead of being retrieved from\n platform-provided telemetry data."]
pub struct SetPressureSourceOverrideEnabled {
    #[serde(default)]
    pub enabled: bool,
    pub source: PressureSource,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PressureMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "TODO: OBSOLETE: To remove when setPressureDataOverride is merged.\n Provides a given pressure state that will be processed and eventually be\n delivered to PressureObserver users. |source| must have been previously\n overridden by setPressureSourceOverrideEnabled."]
pub struct SetPressureStateOverride {
    pub source: PressureSource,
    pub state: PressureState,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Provides a given pressure data set that will be processed and eventually be\n delivered to PressureObserver users. |source| must have been previously\n overridden by setPressureSourceOverrideEnabled."]
pub struct SetPressureDataOverride {
    pub source: PressureSource,
    pub state: PressureState,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub own_contribution_estimate: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides the Idle state."]
pub struct SetIdleOverride {
    #[serde(default)]
    #[doc = "Mock isUserActive"]
    pub is_user_active: bool,
    #[serde(default)]
    #[doc = "Mock isScreenUnlocked"]
    pub is_screen_unlocked: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearIdleOverride(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides value returned by the javascript navigator object."]
#[deprecated]
pub struct SetNavigatorOverrides {
    #[serde(default)]
    #[doc = "The platform navigator.platform should return."]
    pub platform: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets a specified page scale factor."]
pub struct SetPageScaleFactor {
    #[serde(default)]
    #[doc = "Page scale factor."]
    pub page_scale_factor: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Switches script execution in the page."]
pub struct SetScriptExecutionDisabled {
    #[serde(default)]
    #[doc = "Whether script execution should be disabled in the page."]
    pub value: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables touch on platforms which do not support them."]
pub struct SetTouchEmulationEnabled {
    #[serde(default)]
    #[doc = "Whether the touch event emulation should be enabled."]
    pub enabled: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Maximum touch points supported. Defaults to one."]
    pub max_touch_points: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets\n the current virtual time policy.  Note this supersedes any previous time budget."]
pub struct SetVirtualTimePolicy {
    pub policy: VirtualTimePolicy,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, after this many virtual milliseconds have elapsed virtual time will be paused and a\n virtualTimeBudgetExpired event is sent."]
    pub budget: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set this specifies the maximum number of tasks that can be run before virtual is forced\n forwards to prevent deadlock."]
    pub max_virtual_time_task_starvation_count: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, base::Time::Now will be overridden to initially return this value."]
    pub initial_virtual_time: Option<network::TimeSinceEpoch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides default host system locale with the specified one."]
pub struct SetLocaleOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "ICU style C locale (e.g. \"en_US\"). If not specified or empty, disables the override and\n restores default host system locale."]
    pub locale: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Overrides default host system timezone with the specified one."]
pub struct SetTimezoneOverride {
    #[serde(default)]
    #[doc = "The timezone identifier. List of supported timezones:\n https://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt\n If empty, disables the override and restores default host system timezone."]
    pub timezone_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resizes the frame/viewport of the page. Note that this does not affect the frame's container\n (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported\n on Android."]
#[deprecated]
pub struct SetVisibleSize {
    #[serde(default)]
    #[doc = "Frame width (DIP)."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Frame height (DIP)."]
    pub height: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetDisabledImageTypes {
    #[doc = "Image types to disable."]
    pub image_types: Vec<DisabledImageType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Override the value of navigator.connection.saveData"]
pub struct SetDataSaverOverride {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Override value. Omitting the parameter disables the override."]
    pub data_saver_enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetHardwareConcurrencyOverride {
    #[serde(default)]
    #[doc = "Hardware concurrency to report"]
    pub hardware_concurrency: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows overriding user agent with the given string.\n `userAgentMetadata` must be set for Client Hint headers to be sent."]
pub struct SetUserAgentOverride {
    #[serde(default)]
    #[doc = "User agent to use."]
    pub user_agent: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Browser language to emulate."]
    pub accept_language: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The platform navigator.platform should return."]
    pub platform: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData"]
    pub user_agent_metadata: Option<UserAgentMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows overriding the automation flag."]
pub struct SetAutomationOverride {
    #[serde(default)]
    #[doc = "Whether the override should be enabled."]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows overriding the difference between the small and large viewport sizes, which determine the\n value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames."]
pub struct SetSmallViewportHeightDifferenceOverride {
    #[serde(default)]
    #[doc = "This will cause an element of size 100svh to be `difference` pixels smaller than an element\n of size 100lvh."]
    pub difference: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetScreenInfos(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Add a new screen to the device. Only supported in headless mode."]
pub struct AddScreen {
    #[serde(default)]
    #[doc = "Offset of the left edge of the screen in pixels."]
    pub left: JsUInt,
    #[serde(default)]
    #[doc = "Offset of the top edge of the screen in pixels."]
    pub top: JsUInt,
    #[serde(default)]
    #[doc = "The width of the screen in pixels."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "The height of the screen in pixels."]
    pub height: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Specifies the screen's work area. Default is entire screen."]
    pub work_area_insets: Option<WorkAreaInsets>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies the screen's device pixel ratio. Default is 1."]
    pub device_pixel_ratio: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0."]
    pub rotation: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies the screen's color depth in bits. Default is 24."]
    pub color_depth: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Specifies the descriptive label for the screen. Default is none."]
    pub label: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Indicates whether the screen is internal to the device or external, attached to the device. Default is false."]
    pub is_internal: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Remove screen from the device. Only supported in headless mode."]
pub struct RemoveScreen {
    pub screen_id: ScreenId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Tells whether emulation is supported."]
#[deprecated]
pub struct CanEmulateReturnObject {
    #[serde(default)]
    #[doc = "True if emulation is supported."]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden device metrics."]
pub struct ClearDeviceMetricsOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the overridden Geolocation Position and Error."]
pub struct ClearGeolocationOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Requests that page scale factor is reset to initial values."]
pub struct ResetPageScaleFactorReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables or disables simulating a focused and active page."]
pub struct SetFocusEmulationEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Automatically render all web contents using a dark theme."]
pub struct SetAutoDarkModeOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables CPU throttling to emulate slow CPUs."]
pub struct SetCPUThrottlingRateReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\n if the content does not specify one."]
pub struct SetDefaultBackgroundColorOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the\n respective variables to be undefined, even if previously overridden."]
pub struct SetSafeAreaInsetsOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\n window.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\n query results)."]
pub struct SetDeviceMetricsOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Start reporting the given posture value to the Device Posture API.\n This override can also be set in setDeviceMetricsOverride()."]
pub struct SetDevicePostureOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears a device posture override set with either setDeviceMetricsOverride()\n or setDevicePostureOverride() and starts using posture information from the\n platform again.\n Does nothing if no override is set."]
pub struct ClearDevicePostureOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Start using the given display features to pupulate the Viewport Segments API.\n This override can also be set in setDeviceMetricsOverride()."]
pub struct SetDisplayFeaturesOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears the display features override set with either setDeviceMetricsOverride()\n or setDisplayFeaturesOverride() and starts using display features from the\n platform again.\n Does nothing if no override is set."]
pub struct ClearDisplayFeaturesOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScrollbarsHiddenReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDocumentCookieDisabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEmitTouchEventsForMouseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Emulates the given media type or media feature for CSS media queries."]
pub struct SetEmulatedMediaReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Emulates the given vision deficiency."]
pub struct SetEmulatedVisionDeficiencyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Emulates the given OS text scale."]
pub struct SetEmulatedOSTextScaleReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the Geolocation Position or Error. Omitting latitude, longitude or\n accuracy emulates position unavailable."]
pub struct SetGeolocationOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct GetOverriddenSensorInformationReturnObject {
    #[serde(default)]
    pub requested_sampling_frequency: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides a platform sensor of a given type. If |enabled| is true, calls to\n Sensor.start() will use a virtual sensor as backend rather than fetching\n data from a real hardware sensor. Otherwise, existing virtual\n sensor-backend Sensor objects will fire an error event and new calls to\n Sensor.start() will attempt to use a real sensor instead."]
pub struct SetSensorOverrideEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Updates the sensor readings reported by a sensor type previously overridden\n by setSensorOverrideEnabled."]
pub struct SetSensorOverrideReadingsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides a pressure source of a given type, as used by the Compute\n Pressure API, so that updates to PressureObserver.observe() are provided\n via setPressureStateOverride instead of being retrieved from\n platform-provided telemetry data."]
pub struct SetPressureSourceOverrideEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "TODO: OBSOLETE: To remove when setPressureDataOverride is merged.\n Provides a given pressure state that will be processed and eventually be\n delivered to PressureObserver users. |source| must have been previously\n overridden by setPressureSourceOverrideEnabled."]
pub struct SetPressureStateOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Provides a given pressure data set that will be processed and eventually be\n delivered to PressureObserver users. |source| must have been previously\n overridden by setPressureSourceOverrideEnabled."]
pub struct SetPressureDataOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides the Idle state."]
pub struct SetIdleOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears Idle state overrides."]
pub struct ClearIdleOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides value returned by the javascript navigator object."]
#[deprecated]
pub struct SetNavigatorOverridesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets a specified page scale factor."]
pub struct SetPageScaleFactorReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Switches script execution in the page."]
pub struct SetScriptExecutionDisabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables touch on platforms which do not support them."]
pub struct SetTouchEmulationEnabledReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets\n the current virtual time policy.  Note this supersedes any previous time budget."]
pub struct SetVirtualTimePolicyReturnObject {
    #[serde(default)]
    #[doc = "Absolute timestamp at which virtual time was first enabled (up time in milliseconds)."]
    pub virtual_time_ticks_base: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides default host system locale with the specified one."]
pub struct SetLocaleOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Overrides default host system timezone with the specified one."]
pub struct SetTimezoneOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resizes the frame/viewport of the page. Note that this does not affect the frame's container\n (e.g. browser window). Can be used to produce screenshots of the specified size. Not supported\n on Android."]
#[deprecated]
pub struct SetVisibleSizeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDisabledImageTypesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Override the value of navigator.connection.saveData"]
pub struct SetDataSaverOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetHardwareConcurrencyOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows overriding user agent with the given string.\n `userAgentMetadata` must be set for Client Hint headers to be sent."]
pub struct SetUserAgentOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows overriding the automation flag."]
pub struct SetAutomationOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows overriding the difference between the small and large viewport sizes, which determine the\n value of the `svh` and `lvh` unit, respectively. Only supported for top-level frames."]
pub struct SetSmallViewportHeightDifferenceOverrideReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns device's screen configuration."]
pub struct GetScreenInfosReturnObject {
    pub screen_infos: Vec<ScreenInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Add a new screen to the device. Only supported in headless mode."]
pub struct AddScreenReturnObject {
    pub screen_info: ScreenInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Remove screen from the device. Only supported in headless mode."]
pub struct RemoveScreenReturnObject(pub Option<Json>);
impl Method for CanEmulate {
    const NAME: &'static str = "Emulation.canEmulate";
    type ReturnObject = CanEmulateReturnObject;
}
impl Method for ClearDeviceMetricsOverride {
    const NAME: &'static str = "Emulation.clearDeviceMetricsOverride";
    type ReturnObject = ClearDeviceMetricsOverrideReturnObject;
}
impl Method for ClearGeolocationOverride {
    const NAME: &'static str = "Emulation.clearGeolocationOverride";
    type ReturnObject = ClearGeolocationOverrideReturnObject;
}
impl Method for ResetPageScaleFactor {
    const NAME: &'static str = "Emulation.resetPageScaleFactor";
    type ReturnObject = ResetPageScaleFactorReturnObject;
}
impl Method for SetFocusEmulationEnabled {
    const NAME: &'static str = "Emulation.setFocusEmulationEnabled";
    type ReturnObject = SetFocusEmulationEnabledReturnObject;
}
impl Method for SetAutoDarkModeOverride {
    const NAME: &'static str = "Emulation.setAutoDarkModeOverride";
    type ReturnObject = SetAutoDarkModeOverrideReturnObject;
}
impl Method for SetCPUThrottlingRate {
    const NAME: &'static str = "Emulation.setCPUThrottlingRate";
    type ReturnObject = SetCPUThrottlingRateReturnObject;
}
impl Method for SetDefaultBackgroundColorOverride {
    const NAME: &'static str = "Emulation.setDefaultBackgroundColorOverride";
    type ReturnObject = SetDefaultBackgroundColorOverrideReturnObject;
}
impl Method for SetSafeAreaInsetsOverride {
    const NAME: &'static str = "Emulation.setSafeAreaInsetsOverride";
    type ReturnObject = SetSafeAreaInsetsOverrideReturnObject;
}
impl Method for SetDeviceMetricsOverride {
    const NAME: &'static str = "Emulation.setDeviceMetricsOverride";
    type ReturnObject = SetDeviceMetricsOverrideReturnObject;
}
impl Method for SetDevicePostureOverride {
    const NAME: &'static str = "Emulation.setDevicePostureOverride";
    type ReturnObject = SetDevicePostureOverrideReturnObject;
}
impl Method for ClearDevicePostureOverride {
    const NAME: &'static str = "Emulation.clearDevicePostureOverride";
    type ReturnObject = ClearDevicePostureOverrideReturnObject;
}
impl Method for SetDisplayFeaturesOverride {
    const NAME: &'static str = "Emulation.setDisplayFeaturesOverride";
    type ReturnObject = SetDisplayFeaturesOverrideReturnObject;
}
impl Method for ClearDisplayFeaturesOverride {
    const NAME: &'static str = "Emulation.clearDisplayFeaturesOverride";
    type ReturnObject = ClearDisplayFeaturesOverrideReturnObject;
}
impl Method for SetScrollbarsHidden {
    const NAME: &'static str = "Emulation.setScrollbarsHidden";
    type ReturnObject = SetScrollbarsHiddenReturnObject;
}
impl Method for SetDocumentCookieDisabled {
    const NAME: &'static str = "Emulation.setDocumentCookieDisabled";
    type ReturnObject = SetDocumentCookieDisabledReturnObject;
}
impl Method for SetEmitTouchEventsForMouse {
    const NAME: &'static str = "Emulation.setEmitTouchEventsForMouse";
    type ReturnObject = SetEmitTouchEventsForMouseReturnObject;
}
impl Method for SetEmulatedMedia {
    const NAME: &'static str = "Emulation.setEmulatedMedia";
    type ReturnObject = SetEmulatedMediaReturnObject;
}
impl Method for SetEmulatedVisionDeficiency {
    const NAME: &'static str = "Emulation.setEmulatedVisionDeficiency";
    type ReturnObject = SetEmulatedVisionDeficiencyReturnObject;
}
impl Method for SetEmulatedOSTextScale {
    const NAME: &'static str = "Emulation.setEmulatedOSTextScale";
    type ReturnObject = SetEmulatedOSTextScaleReturnObject;
}
impl Method for SetGeolocationOverride {
    const NAME: &'static str = "Emulation.setGeolocationOverride";
    type ReturnObject = SetGeolocationOverrideReturnObject;
}
impl Method for GetOverriddenSensorInformation {
    const NAME: &'static str = "Emulation.getOverriddenSensorInformation";
    type ReturnObject = GetOverriddenSensorInformationReturnObject;
}
impl Method for SetSensorOverrideEnabled {
    const NAME: &'static str = "Emulation.setSensorOverrideEnabled";
    type ReturnObject = SetSensorOverrideEnabledReturnObject;
}
impl Method for SetSensorOverrideReadings {
    const NAME: &'static str = "Emulation.setSensorOverrideReadings";
    type ReturnObject = SetSensorOverrideReadingsReturnObject;
}
impl Method for SetPressureSourceOverrideEnabled {
    const NAME: &'static str = "Emulation.setPressureSourceOverrideEnabled";
    type ReturnObject = SetPressureSourceOverrideEnabledReturnObject;
}
impl Method for SetPressureStateOverride {
    const NAME: &'static str = "Emulation.setPressureStateOverride";
    type ReturnObject = SetPressureStateOverrideReturnObject;
}
impl Method for SetPressureDataOverride {
    const NAME: &'static str = "Emulation.setPressureDataOverride";
    type ReturnObject = SetPressureDataOverrideReturnObject;
}
impl Method for SetIdleOverride {
    const NAME: &'static str = "Emulation.setIdleOverride";
    type ReturnObject = SetIdleOverrideReturnObject;
}
impl Method for ClearIdleOverride {
    const NAME: &'static str = "Emulation.clearIdleOverride";
    type ReturnObject = ClearIdleOverrideReturnObject;
}
impl Method for SetNavigatorOverrides {
    const NAME: &'static str = "Emulation.setNavigatorOverrides";
    type ReturnObject = SetNavigatorOverridesReturnObject;
}
impl Method for SetPageScaleFactor {
    const NAME: &'static str = "Emulation.setPageScaleFactor";
    type ReturnObject = SetPageScaleFactorReturnObject;
}
impl Method for SetScriptExecutionDisabled {
    const NAME: &'static str = "Emulation.setScriptExecutionDisabled";
    type ReturnObject = SetScriptExecutionDisabledReturnObject;
}
impl Method for SetTouchEmulationEnabled {
    const NAME: &'static str = "Emulation.setTouchEmulationEnabled";
    type ReturnObject = SetTouchEmulationEnabledReturnObject;
}
impl Method for SetVirtualTimePolicy {
    const NAME: &'static str = "Emulation.setVirtualTimePolicy";
    type ReturnObject = SetVirtualTimePolicyReturnObject;
}
impl Method for SetLocaleOverride {
    const NAME: &'static str = "Emulation.setLocaleOverride";
    type ReturnObject = SetLocaleOverrideReturnObject;
}
impl Method for SetTimezoneOverride {
    const NAME: &'static str = "Emulation.setTimezoneOverride";
    type ReturnObject = SetTimezoneOverrideReturnObject;
}
impl Method for SetVisibleSize {
    const NAME: &'static str = "Emulation.setVisibleSize";
    type ReturnObject = SetVisibleSizeReturnObject;
}
impl Method for SetDisabledImageTypes {
    const NAME: &'static str = "Emulation.setDisabledImageTypes";
    type ReturnObject = SetDisabledImageTypesReturnObject;
}
impl Method for SetDataSaverOverride {
    const NAME: &'static str = "Emulation.setDataSaverOverride";
    type ReturnObject = SetDataSaverOverrideReturnObject;
}
impl Method for SetHardwareConcurrencyOverride {
    const NAME: &'static str = "Emulation.setHardwareConcurrencyOverride";
    type ReturnObject = SetHardwareConcurrencyOverrideReturnObject;
}
impl Method for SetUserAgentOverride {
    const NAME: &'static str = "Emulation.setUserAgentOverride";
    type ReturnObject = SetUserAgentOverrideReturnObject;
}
impl Method for SetAutomationOverride {
    const NAME: &'static str = "Emulation.setAutomationOverride";
    type ReturnObject = SetAutomationOverrideReturnObject;
}
impl Method for SetSmallViewportHeightDifferenceOverride {
    const NAME: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
    type ReturnObject = SetSmallViewportHeightDifferenceOverrideReturnObject;
}
impl Method for GetScreenInfos {
    const NAME: &'static str = "Emulation.getScreenInfos";
    type ReturnObject = GetScreenInfosReturnObject;
}
impl Method for AddScreen {
    const NAME: &'static str = "Emulation.addScreen";
    type ReturnObject = AddScreenReturnObject;
}
impl Method for RemoveScreen {
    const NAME: &'static str = "Emulation.removeScreen";
    type ReturnObject = RemoveScreenReturnObject;
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
    pub struct VirtualTimeBudgetExpiredEvent(pub Option<Json>);
}
