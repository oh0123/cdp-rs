// Auto-generated from Chrome at version 140.0.7339.186 domain: HeadlessExperimental
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ScreenshotParamsFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScreenshotParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "format")]
    pub format: Option<ScreenshotParamsFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "quality")]
    pub quality: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "optimizeForSpeed")]
    pub optimize_for_speed: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BeginFrame {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "frameTimeTicks")]
    pub frame_time_ticks: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "interval")]
    pub interval: Option<JsFloat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "noDisplayUpdates")]
    pub no_display_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "screenshot")]
    pub screenshot: Option<ScreenshotParams>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BeginFrameReturnObject {
    #[serde(default)]
    #[serde(rename = "hasDamage")]
    pub has_damage: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "screenshotData")]
    pub screenshot_data: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
impl Method for BeginFrame {
    const NAME: &'static str = "HeadlessExperimental.beginFrame";
    type ReturnObject = BeginFrameReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "HeadlessExperimental.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "HeadlessExperimental.enable";
    type ReturnObject = EnableReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
