// Auto-generated from Chrome at version 146.0.7680.165 domain: SystemInfo
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes a single graphics processor (GPU)."]
pub struct GpuDevice {
    #[serde(default)]
    #[doc = "PCI ID of the GPU vendor, if available; 0 otherwise."]
    pub vendor_id: JsFloat,
    #[serde(default)]
    #[doc = "PCI ID of the GPU device, if available; 0 otherwise."]
    pub device_id: JsFloat,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Sub sys ID of the GPU, only available on Windows."]
    pub sub_sys_id: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Revision of the GPU, only available on Windows."]
    pub revision: Option<JsFloat>,
    #[serde(default)]
    #[doc = "String description of the GPU vendor, if the PCI ID is not available."]
    pub vendor_string: String,
    #[serde(default)]
    #[doc = "String description of the GPU device, if the PCI ID is not available."]
    pub device_string: String,
    #[serde(default)]
    #[doc = "String description of the GPU driver vendor."]
    pub driver_vendor: String,
    #[serde(default)]
    #[doc = "String description of the GPU driver version."]
    pub driver_version: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes the width and height dimensions of an entity."]
pub struct Size {
    #[serde(default)]
    #[doc = "Width in pixels."]
    pub width: JsUInt,
    #[serde(default)]
    #[doc = "Height in pixels."]
    pub height: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes a supported video decoding profile with its associated minimum and\n maximum resolutions."]
pub struct VideoDecodeAcceleratorCapability {
    #[serde(default)]
    #[doc = "Video codec profile that is supported, e.g. VP9 Profile 2."]
    pub profile: String,
    #[doc = "Maximum video dimensions in pixels supported for this |profile|."]
    pub max_resolution: Size,
    #[doc = "Minimum video dimensions in pixels supported for this |profile|."]
    pub min_resolution: Size,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Describes a supported video encoding profile with its associated maximum\n resolution and maximum framerate."]
pub struct VideoEncodeAcceleratorCapability {
    #[serde(default)]
    #[doc = "Video codec profile that is supported, e.g H264 Main."]
    pub profile: String,
    #[doc = "Maximum video dimensions in pixels supported for this |profile|."]
    pub max_resolution: Size,
    #[serde(default)]
    #[doc = "Maximum encoding framerate in frames per second supported for this\n |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,\n 24000/1001 fps, etc."]
    pub max_framerate_numerator: JsUInt,
    #[serde(default)]
    pub max_framerate_denominator: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Provides information about the GPU(s) on the system."]
pub struct GpuInfo {
    #[doc = "The graphics devices on the system. Element 0 is the primary GPU."]
    pub devices: Vec<GpuDevice>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An optional dictionary of additional GPU related attributes."]
    pub aux_attributes: Option<Json>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An optional dictionary of graphics features and their status."]
    pub feature_status: Option<Json>,
    #[serde(default)]
    #[doc = "An optional array of GPU driver bug workarounds."]
    pub driver_bug_workarounds: Vec<String>,
    #[doc = "Supported accelerated video decoding capabilities."]
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
    #[doc = "Supported accelerated video encoding capabilities."]
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents process info."]
pub struct ProcessInfo {
    #[serde(default)]
    #[doc = "Specifies process type."]
    pub r#type: String,
    #[serde(default)]
    #[doc = "Specifies process id."]
    pub id: JsUInt,
    #[serde(default)]
    #[doc = "Specifies cumulative CPU usage in seconds across all threads of the\n process since the process start."]
    pub cpu_time: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInfo(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about the feature state."]
pub struct GetFeatureState {
    #[serde(default)]
    pub feature_state: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetProcessInfo(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about the system."]
pub struct GetInfoReturnObject {
    #[doc = "Information about the GPUs on the system."]
    pub gpu: GpuInfo,
    #[serde(default)]
    #[doc = "A platform-dependent description of the model of the machine. On Mac OS, this is, for\n example, 'MacBookPro'. Will be the empty string if not supported."]
    pub model_name: String,
    #[serde(default)]
    #[doc = "A platform-dependent description of the version of the machine. On Mac OS, this is, for\n example, '10.1'. Will be the empty string if not supported."]
    pub model_version: String,
    #[serde(default)]
    #[doc = "The command line string used to launch the browser. Will be the empty string if not\n supported."]
    pub command_line: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about the feature state."]
pub struct GetFeatureStateReturnObject {
    #[serde(default)]
    pub feature_enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns information about all running processes."]
pub struct GetProcessInfoReturnObject {
    #[doc = "An array of process info blocks."]
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
