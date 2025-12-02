// Auto-generated from Chrome at version 140.0.7339.186 domain: SystemInfo
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SubsamplingFormat {
    #[serde(rename = "yuv420")]
    Yuv420,
    #[serde(rename = "yuv422")]
    Yuv422,
    #[serde(rename = "yuv444")]
    Yuv444,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ImageType {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GpuDevice {
    #[serde(default)]
    #[serde(rename = "vendorId")]
    pub vendor_id: JsFloat,
    #[serde(default)]
    #[serde(rename = "deviceId")]
    pub device_id: JsFloat,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "subSysId")]
    pub sub_sys_id: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "revision")]
    pub revision: Option<JsFloat>,
    #[serde(default)]
    #[serde(rename = "vendorString")]
    pub vendor_string: String,
    #[serde(default)]
    #[serde(rename = "deviceString")]
    pub device_string: String,
    #[serde(default)]
    #[serde(rename = "driverVendor")]
    pub driver_vendor: String,
    #[serde(default)]
    #[serde(rename = "driverVersion")]
    pub driver_version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Size {
    #[serde(default)]
    #[serde(rename = "width")]
    pub width: JsUInt,
    #[serde(default)]
    #[serde(rename = "height")]
    pub height: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VideoDecodeAcceleratorCapability {
    #[serde(default)]
    #[serde(rename = "profile")]
    pub profile: String,
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    #[serde(rename = "minResolution")]
    pub min_resolution: Size,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VideoEncodeAcceleratorCapability {
    #[serde(default)]
    #[serde(rename = "profile")]
    pub profile: String,
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    #[serde(default)]
    #[serde(rename = "maxFramerateNumerator")]
    pub max_framerate_numerator: JsUInt,
    #[serde(default)]
    #[serde(rename = "maxFramerateDenominator")]
    pub max_framerate_denominator: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ImageDecodeAcceleratorCapability {
    #[serde(rename = "imageType")]
    pub image_type: ImageType,
    #[serde(rename = "maxDimensions")]
    pub max_dimensions: Size,
    #[serde(rename = "minDimensions")]
    pub min_dimensions: Size,
    #[serde(rename = "subsamplings")]
    pub subsamplings: Vec<SubsamplingFormat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GpuInfo {
    #[serde(rename = "devices")]
    pub devices: Vec<GpuDevice>,
    #[serde(default)]
    #[serde(rename = "driverBugWorkarounds")]
    pub driver_bug_workarounds: Vec<String>,
    #[serde(rename = "videoDecoding")]
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
    #[serde(rename = "videoEncoding")]
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
    #[serde(rename = "imageDecoding")]
    pub image_decoding: Vec<ImageDecodeAcceleratorCapability>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessInfo {
    #[serde(default)]
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: JsUInt,
    #[serde(default)]
    #[serde(rename = "cpuTime")]
    pub cpu_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetInfo(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFeatureState {
    #[serde(default)]
    #[serde(rename = "featureState")]
    pub feature_state: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetProcessInfo(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInfoReturnObject {
    #[serde(rename = "gpu")]
    pub gpu: GpuInfo,
    #[serde(default)]
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(default)]
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[serde(default)]
    #[serde(rename = "commandLine")]
    pub command_line: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetFeatureStateReturnObject {
    #[serde(default)]
    #[serde(rename = "featureEnabled")]
    pub feature_enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetProcessInfoReturnObject {
    #[serde(rename = "processInfo")]
    pub process_info: Vec<ProcessInfo>,
}
impl Method for GetInfo {
    const NAME: &'static str = "SystemInfo.getInfo";
    type ReturnObject = GetInfoReturnObject;
}
impl Method for GetFeatureState {
    const NAME: &'static str = "SystemInfo.getFeatureState";
    type ReturnObject = GetFeatureStateReturnObject;
}
impl Method for GetProcessInfo {
    const NAME: &'static str = "SystemInfo.getProcessInfo";
    type ReturnObject = GetProcessInfoReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
