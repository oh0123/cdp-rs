// Auto-generated from Chrome at version 143.0.7499.110 domain: Emulation
use super::dom;
use super::network;
use super::page;
#[allow(unused_imports)]
use super::types::*;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SafeAreaInsets {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "top")]
    pub top: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "topMax")]
    pub top_max: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "left")]
    pub left: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "leftMax")]
    pub left_max: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bottom")]
    pub bottom: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bottomMax")]
    pub bottom_max: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "right")]
    pub right: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rightMax")]
    pub right_max: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScreenOrientation {
    #[serde(rename = "type")]
    pub r#type: ScreenOrientationType,
    #[serde(default)]
    #[serde(rename = "angle")]
    pub angle: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisplayFeature {
    #[serde(rename = "orientation")]
    pub orientation: DisplayFeatureOrientation,
    #[serde(default)]
    #[serde(rename = "offset")]
    pub offset: JsUInt,
    #[serde(default)]
    #[serde(rename = "maskLength")]
    pub mask_length: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DevicePosture {
    #[serde(rename = "type")]
    pub r#type: DevicePostureType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MediaFeature {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserAgentBrandVersion {
    #[serde(default)]
    #[serde(rename = "brand")]
    pub brand: String,
    #[serde(default)]
    #[serde(rename = "version")]
    pub version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserAgentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "brands")]
    pub brands: Option<Vec<UserAgentBrandVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fullVersionList")]
    pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "fullVersion")]
    pub full_version: Option<String>,
    #[serde(default)]
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(default)]
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    #[serde(default)]
    #[serde(rename = "architecture")]
    pub architecture: String,
    #[serde(default)]
    #[serde(rename = "model")]
    pub model: String,
    #[serde(default)]
    #[serde(rename = "mobile")]
    pub mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bitness")]
    pub bitness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "wow64")]
    pub wow_64: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "formFactors")]
    pub form_factors: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "available")]
    pub available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "minimumFrequency")]
    pub minimum_frequency: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maximumFrequency")]
    pub maximum_frequency: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensorReadingSingle {
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensorReadingXyz {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(default)]
    #[serde(rename = "z")]
    pub z: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensorReadingQuaternion {
    #[serde(default)]
    #[serde(rename = "x")]
    pub x: JsFloat,
    #[serde(default)]
    #[serde(rename = "y")]
    pub y: JsFloat,
    #[serde(default)]
    #[serde(rename = "z")]
    pub z: JsFloat,
    #[serde(default)]
    #[serde(rename = "w")]
    pub w: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensorReading {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "single")]
    pub single: Option<SensorReadingSingle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "xyz")]
    pub xyz: Option<SensorReadingXyz>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quaternion")]
    pub quaternion: Option<SensorReadingQuaternion>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PressureMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "available")]
    pub available: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WorkAreaInsets {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "top")]
    pub top: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "left")]
    pub left: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "bottom")]
    pub bottom: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "right")]
    pub right: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScreenInfo {
    #[serde(default)]
    #[serde(rename = "left")]
    pub left: JsUInt,
    #[serde(default)]
    #[serde(rename = "top")]
    pub top: JsUInt,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(default)]
    #[serde(rename = "availLeft")]
    pub avail_left: JsUInt,
    #[serde(default)]
    #[serde(rename = "availTop")]
    pub avail_top: JsUInt,
    #[serde(default)]
    #[serde(rename = "availWidth")]
    pub avail_width: JsUInt,
    #[serde(default)]
    #[serde(rename = "availHeight")]
    pub avail_height: JsUInt,
    #[serde(default)]
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: JsFloat,
    #[serde(rename = "orientation")]
    pub orientation: ScreenOrientation,
    #[serde(default)]
    #[serde(rename = "colorDepth")]
    pub color_depth: JsUInt,
    #[serde(default)]
    #[serde(rename = "isExtended")]
    pub is_extended: bool,
    #[serde(default)]
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[serde(default)]
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[serde(default)]
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "id")]
    pub id: ScreenId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CanEmulate(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceMetricsOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearGeolocationOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetPageScaleFactor(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetFocusEmulationEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAutoDarkModeOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCPUThrottlingRate {
    #[serde(default)]
    #[serde(rename = "rate")]
    pub rate: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDefaultBackgroundColorOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "color")]
    pub color: Option<dom::Rgba>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSafeAreaInsetsOverride {
    #[serde(rename = "insets")]
    pub insets: SafeAreaInsets,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDeviceMetricsOverride {
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(default)]
    #[serde(rename = "deviceScaleFactor")]
    pub device_scale_factor: JsFloat,
    #[serde(default)]
    #[serde(rename = "mobile")]
    pub mobile: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "screenWidth")]
    pub screen_width: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "screenHeight")]
    pub screen_height: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "positionX")]
    pub position_x: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "positionY")]
    pub position_y: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dontSetVisibleSize")]
    pub dont_set_visible_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewport")]
    pub viewport: Option<page::Viewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayFeature")]
    pub display_feature: Option<DisplayFeature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "devicePosture")]
    pub device_posture: Option<DevicePosture>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDevicePostureOverride {
    #[serde(rename = "posture")]
    pub posture: DevicePosture,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDevicePostureOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDisplayFeaturesOverride {
    #[serde(rename = "features")]
    pub features: Vec<DisplayFeature>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayFeaturesOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScrollbarsHidden {
    #[serde(default)]
    #[serde(rename = "hidden")]
    pub hidden: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDocumentCookieDisabled {
    #[serde(default)]
    #[serde(rename = "disabled")]
    pub disabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEmitTouchEventsForMouse {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "configuration")]
    pub configuration: Option<SetEmitTouchEventsForMouseConfigurationOption>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEmulatedMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "media")]
    pub media: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "features")]
    pub features: Option<Vec<MediaFeature>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEmulatedVisionDeficiency {
    #[serde(rename = "type")]
    pub r#type: SetEmulatedVisionDeficiencyTypeOption,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetEmulatedOSTextScale {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scale")]
    pub scale: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetGeolocationOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "latitude")]
    pub latitude: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "longitude")]
    pub longitude: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "accuracy")]
    pub accuracy: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "altitude")]
    pub altitude: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "altitudeAccuracy")]
    pub altitude_accuracy: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "heading")]
    pub heading: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "speed")]
    pub speed: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOverriddenSensorInformation {
    #[serde(rename = "type")]
    pub r#type: SensorType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSensorOverrideEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "type")]
    pub r#type: SensorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<SensorMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSensorOverrideReadings {
    #[serde(rename = "type")]
    pub r#type: SensorType,
    #[serde(rename = "reading")]
    pub reading: SensorReading,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPressureSourceOverrideEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "source")]
    pub source: PressureSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<PressureMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPressureStateOverride {
    #[serde(rename = "source")]
    pub source: PressureSource,
    #[serde(rename = "state")]
    pub state: PressureState,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPressureDataOverride {
    #[serde(rename = "source")]
    pub source: PressureSource,
    #[serde(rename = "state")]
    pub state: PressureState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ownContributionEstimate")]
    pub own_contribution_estimate: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetIdleOverride {
    #[serde(default)]
    #[serde(rename = "isUserActive")]
    pub is_user_active: bool,
    #[serde(default)]
    #[serde(rename = "isScreenUnlocked")]
    pub is_screen_unlocked: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearIdleOverride(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetNavigatorOverrides {
    #[serde(default)]
    #[serde(rename = "platform")]
    pub platform: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPageScaleFactor {
    #[serde(default)]
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetScriptExecutionDisabled {
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetTouchEmulationEnabled {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxTouchPoints")]
    pub max_touch_points: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetVirtualTimePolicy {
    #[serde(rename = "policy")]
    pub policy: VirtualTimePolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "budget")]
    pub budget: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
    pub max_virtual_time_task_starvation_count: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initialVirtualTime")]
    pub initial_virtual_time: Option<network::TimeSinceEpoch>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetLocaleOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "locale")]
    pub locale: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetTimezoneOverride {
    #[serde(default)]
    #[serde(rename = "timezoneId")]
    pub timezone_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetVisibleSize {
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDisabledImageTypes {
    #[serde(rename = "imageTypes")]
    pub image_types: Vec<DisabledImageType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetDataSaverOverride {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dataSaverEnabled")]
    pub data_saver_enabled: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetHardwareConcurrencyOverride {
    #[serde(default)]
    #[serde(rename = "hardwareConcurrency")]
    pub hardware_concurrency: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetUserAgentOverride {
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "acceptLanguage")]
    pub accept_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "platform")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userAgentMetadata")]
    pub user_agent_metadata: Option<UserAgentMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAutomationOverride {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSmallViewportHeightDifferenceOverride {
    #[serde(default)]
    #[serde(rename = "difference")]
    pub difference: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetScreenInfos(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScreen {
    #[serde(default)]
    #[serde(rename = "left")]
    pub left: JsUInt,
    #[serde(default)]
    #[serde(rename = "top")]
    pub top: JsUInt,
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workAreaInsets")]
    pub work_area_insets: Option<WorkAreaInsets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rotation")]
    pub rotation: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "colorDepth")]
    pub color_depth: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "label")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isInternal")]
    pub is_internal: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveScreen {
    #[serde(rename = "screenId")]
    pub screen_id: ScreenId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CanEmulateReturnObject {
    #[serde(default)]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDeviceMetricsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearGeolocationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetPageScaleFactorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetFocusEmulationEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoDarkModeOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCPUThrottlingRateReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultBackgroundColorOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSafeAreaInsetsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceMetricsOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDevicePostureOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDevicePostureOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDisplayFeaturesOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDisplayFeaturesOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetScrollbarsHiddenReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDocumentCookieDisabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEmitTouchEventsForMouseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedMediaReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedVisionDeficiencyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetEmulatedOSTextScaleReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetGeolocationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOverriddenSensorInformationReturnObject {
    #[serde(default)]
    #[serde(rename = "requestedSamplingFrequency")]
    pub requested_sampling_frequency: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSensorOverrideReadingsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureSourceOverrideEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureStateOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPressureDataOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetIdleOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearIdleOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetNavigatorOverridesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetPageScaleFactorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptExecutionDisabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetTouchEmulationEnabledReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetVirtualTimePolicyReturnObject {
    #[serde(default)]
    #[serde(rename = "virtualTimeTicksBase")]
    pub virtual_time_ticks_base: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetLocaleOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetTimezoneOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetVisibleSizeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDisabledImageTypesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetDataSaverOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetHardwareConcurrencyOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetUserAgentOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomationOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSmallViewportHeightDifferenceOverrideReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetScreenInfosReturnObject {
    #[serde(rename = "screenInfos")]
    pub screen_infos: Vec<ScreenInfo>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddScreenReturnObject {
    #[serde(rename = "screenInfo")]
    pub screen_info: ScreenInfo,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveScreenReturnObject {}
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct VirtualTimeBudgetExpiredEvent(pub Option<serde_json::Value>);
}
