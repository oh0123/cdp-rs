// Auto-generated from Chrome at version 146.0.7680.165 domain: HeadlessExperimental
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Encoding options for a screenshot."]
pub struct ScreenshotParams {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Image compression format (defaults to png)."]
    pub format: Option<ScreenshotParamsFormat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Compression quality from range [0..100] (jpeg and webp only)."]
    pub quality: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Optimize image encoding for speed, not for resulting size (defaults to false)"]
    pub optimize_for_speed: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a\n screenshot from the resulting frame. Requires that the target was created with enabled\n BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also\n https://goo.gle/chrome-headless-rendering for more background."]
pub struct BeginFrame {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,\n the current time will be used."]
    pub frame_time_ticks: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The interval between BeginFrames that is reported to the compositor, in milliseconds.\n Defaults to a 60 frames/second interval, i.e. about 16.666 milliseconds."]
    pub interval: Option<JsFloat>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether updates should not be committed and drawn onto the display. False by default. If\n true, only side effects of the BeginFrame will be run, such as layout and animations, but\n any visual updates may not be visible on the display or in screenshots."]
    pub no_display_updates: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, a screenshot of the frame will be captured and returned in the response. Otherwise,\n no screenshot will be captured. Note that capturing a screenshot can fail, for example,\n during renderer initialization. In such a case, no screenshot data will be returned."]
    pub screenshot: Option<ScreenshotParams>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a\n screenshot from the resulting frame. Requires that the target was created with enabled\n BeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also\n https://goo.gle/chrome-headless-rendering for more background."]
pub struct BeginFrameReturnObject {
    #[serde(default)]
    #[doc = "Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the\n display. Reported for diagnostic uses, may be removed in the future."]
    pub has_damage: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Base64-encoded image data of the screenshot, if one was requested and successfully taken."]
    pub screenshot_data: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables headless events for the target."]
#[deprecated]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables headless events for the target."]
#[deprecated]
pub struct EnableReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
