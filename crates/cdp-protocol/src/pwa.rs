// Auto-generated from Chrome at version 143.0.7499.110 domain: PWA
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DisplayMode {
    #[serde(rename = "standalone")]
    Standalone,
    #[serde(rename = "browser")]
    Browser,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileHandlerAccept {
    #[serde(default)]
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(default)]
    #[serde(rename = "fileExtensions")]
    pub file_extensions: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FileHandler {
    #[serde(default)]
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "accepts")]
    pub accepts: Vec<FileHandlerAccept>,
    #[serde(default)]
    #[serde(rename = "displayName")]
    pub display_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOsAppState {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Install {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "installUrlOrBundleUrl")]
    pub install_url_or_bundle_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Uninstall {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Launch {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LaunchFilesInApp {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(default)]
    #[serde(rename = "files")]
    pub files: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OpenCurrentPageInApp {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChangeAppUserSettings {
    #[serde(default)]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "linkCapturing")]
    pub link_capturing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayMode")]
    pub display_mode: Option<DisplayMode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetOsAppStateReturnObject {
    #[serde(default)]
    #[serde(rename = "badgeCount")]
    pub badge_count: JsUInt,
    #[serde(rename = "fileHandlers")]
    pub file_handlers: Vec<FileHandler>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InstallReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UninstallReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LaunchReturnObject {
    #[serde(rename = "targetId")]
    pub target_id: target::TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LaunchFilesInAppReturnObject {
    #[serde(rename = "targetIds")]
    pub target_ids: target::TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenCurrentPageInAppReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAppUserSettingsReturnObject {}
impl Method for GetOsAppState {
    const NAME: &'static str = "PWA.getOsAppState";
    type ReturnObject = GetOsAppStateReturnObject;
}
impl Method for Install {
    const NAME: &'static str = "PWA.install";
    type ReturnObject = InstallReturnObject;
}
impl Method for Uninstall {
    const NAME: &'static str = "PWA.uninstall";
    type ReturnObject = UninstallReturnObject;
}
impl Method for Launch {
    const NAME: &'static str = "PWA.launch";
    type ReturnObject = LaunchReturnObject;
}
impl Method for LaunchFilesInApp {
    const NAME: &'static str = "PWA.launchFilesInApp";
    type ReturnObject = LaunchFilesInAppReturnObject;
}
impl Method for OpenCurrentPageInApp {
    const NAME: &'static str = "PWA.openCurrentPageInApp";
    type ReturnObject = OpenCurrentPageInAppReturnObject;
}
impl Method for ChangeAppUserSettings {
    const NAME: &'static str = "PWA.changeAppUserSettings";
    type ReturnObject = ChangeAppUserSettingsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
}
