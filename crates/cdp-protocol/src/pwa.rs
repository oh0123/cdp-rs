// Auto-generated from Chrome at version 146.0.7680.165 domain: PWA
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "The following types are the replica of\n https://crsrc.org/c/chrome/browser/web_applications/proto/web_app_os_integration_state.proto;drc=9910d3be894c8f142c977ba1023f30a656bc13fc;l=67"]
pub struct FileHandlerAccept {
    #[serde(default)]
    #[doc = "New name of the mimetype according to\n https://www.iana.org/assignments/media-types/media-types.xhtml"]
    pub media_type: String,
    #[serde(default)]
    pub file_extensions: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct FileHandler {
    #[serde(default)]
    pub action: String,
    pub accepts: Vec<FileHandlerAccept>,
    #[serde(default)]
    pub display_name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the following OS state for the given manifest id."]
pub struct GetOsAppState {
    #[serde(default)]
    #[doc = "The id from the webapp's manifest file, commonly it's the url of the\n site installing the webapp. See\n https://web.dev/learn/pwa/web-app-manifest."]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Installs the given manifest identity, optionally using the given installUrlOrBundleUrl\n \n IWA-specific install description:\n manifestId corresponds to isolated-app:// + web_package::SignedWebBundleId\n \n File installation mode:\n The installUrlOrBundleUrl can be either file:// or http(s):// pointing\n to a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to\n The .swbn file's signing key.\n \n Dev proxy installation mode:\n installUrlOrBundleUrl must be http(s):// that serves dev mode IWA.\n web_package::SignedWebBundleId must be of type dev proxy.\n \n The advantage of dev proxy mode is that all changes to IWA\n automatically will be reflected in the running app without\n reinstallation.\n \n To generate bundle id for proxy mode:\n 1. Generate 32 random bytes.\n 2. Add a specific suffix at the end following the documentation\n    https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix\n 3. Encode the entire sequence using Base32 without padding.\n \n If Chrome is not in IWA dev\n mode, the installation will fail, regardless of the state of the allowlist."]
pub struct Install {
    #[serde(default)]
    pub manifest_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The location of the app or bundle overriding the one derived from the\n manifestId."]
    pub install_url_or_bundle_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Uninstalls the given manifest_id and closes any opened app windows."]
pub struct Uninstall {
    #[serde(default)]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Launches the installed web app, or an url in the same web app instead of the\n default start url if it is provided. Returns a page Target.TargetID which\n can be used to attach to via Target.attachToTarget or similar APIs."]
pub struct Launch {
    #[serde(default)]
    pub manifest_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Opens one or more local files from an installed web app identified by its\n manifestId. The web app needs to have file handlers registered to process\n the files. The API returns one or more page Target.TargetIDs which can be\n used to attach to via Target.attachToTarget or similar APIs.\n If some files in the parameters cannot be handled by the web app, they will\n be ignored. If none of the files can be handled, this API returns an error.\n If no files are provided as the parameter, this API also returns an error.\n \n According to the definition of the file handlers in the manifest file, one\n Target.TargetID may represent a page handling one or more files. The order\n of the returned Target.TargetIDs is not guaranteed.\n \n TODO(crbug.com/339454034): Check the existences of the input files."]
pub struct LaunchFilesInApp {
    #[serde(default)]
    pub manifest_id: String,
    #[serde(default)]
    pub files: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Opens the current page in its web app identified by the manifest id, needs\n to be called on a page target. This function returns immediately without\n waiting for the app to finish loading."]
pub struct OpenCurrentPageInApp {
    #[serde(default)]
    pub manifest_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Changes user settings of the web app identified by its manifestId. If the\n app was not installed, this command returns an error. Unset parameters will\n be ignored; unrecognized values will cause an error.\n \n Unlike the ones defined in the manifest files of the web apps, these\n settings are provided by the browser and controlled by the users, they\n impact the way the browser handling the web apps.\n \n See the comment of each parameter."]
pub struct ChangeAppUserSettings {
    #[serde(default)]
    pub manifest_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If user allows the links clicked on by the user in the app's scope, or\n extended scope if the manifest has scope extensions and the flags\n `DesktopPWAsLinkCapturingWithScopeExtensions` and\n `WebAppEnableScopeExtensions` are enabled.\n \n Note, the API does not support resetting the linkCapturing to the\n initial value, uninstalling and installing the web app again will reset\n it.\n \n TODO(crbug.com/339453269): Setting this value on ChromeOS is not\n supported yet."]
    pub link_capturing: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_mode: Option<DisplayMode>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the following OS state for the given manifest id."]
pub struct GetOsAppStateReturnObject {
    #[serde(default)]
    pub badge_count: JsUInt,
    pub file_handlers: Vec<FileHandler>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Installs the given manifest identity, optionally using the given installUrlOrBundleUrl\n \n IWA-specific install description:\n manifestId corresponds to isolated-app:// + web_package::SignedWebBundleId\n \n File installation mode:\n The installUrlOrBundleUrl can be either file:// or http(s):// pointing\n to a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to\n The .swbn file's signing key.\n \n Dev proxy installation mode:\n installUrlOrBundleUrl must be http(s):// that serves dev mode IWA.\n web_package::SignedWebBundleId must be of type dev proxy.\n \n The advantage of dev proxy mode is that all changes to IWA\n automatically will be reflected in the running app without\n reinstallation.\n \n To generate bundle id for proxy mode:\n 1. Generate 32 random bytes.\n 2. Add a specific suffix at the end following the documentation\n    https://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix\n 3. Encode the entire sequence using Base32 without padding.\n \n If Chrome is not in IWA dev\n mode, the installation will fail, regardless of the state of the allowlist."]
pub struct InstallReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Uninstalls the given manifest_id and closes any opened app windows."]
pub struct UninstallReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Launches the installed web app, or an url in the same web app instead of the\n default start url if it is provided. Returns a page Target.TargetID which\n can be used to attach to via Target.attachToTarget or similar APIs."]
pub struct LaunchReturnObject {
    #[doc = "ID of the tab target created as a result."]
    pub target_id: target::TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Opens one or more local files from an installed web app identified by its\n manifestId. The web app needs to have file handlers registered to process\n the files. The API returns one or more page Target.TargetIDs which can be\n used to attach to via Target.attachToTarget or similar APIs.\n If some files in the parameters cannot be handled by the web app, they will\n be ignored. If none of the files can be handled, this API returns an error.\n If no files are provided as the parameter, this API also returns an error.\n \n According to the definition of the file handlers in the manifest file, one\n Target.TargetID may represent a page handling one or more files. The order\n of the returned Target.TargetIDs is not guaranteed.\n \n TODO(crbug.com/339454034): Check the existences of the input files."]
pub struct LaunchFilesInAppReturnObject {
    #[doc = "IDs of the tab targets created as the result."]
    pub target_ids: target::TargetId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Opens the current page in its web app identified by the manifest id, needs\n to be called on a page target. This function returns immediately without\n waiting for the app to finish loading."]
pub struct OpenCurrentPageInAppReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Changes user settings of the web app identified by its manifestId. If the\n app was not installed, this command returns an error. Unset parameters will\n be ignored; unrecognized values will cause an error.\n \n Unlike the ones defined in the manifest files of the web apps, these\n settings are provided by the browser and controlled by the users, they\n impact the way the browser handling the web apps.\n \n See the comment of each parameter."]
pub struct ChangeAppUserSettingsReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
}
