// Auto-generated from Chrome at version 146.0.7680.165 domain: FedCm
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LoginState {
    #[serde(rename = "SignIn")]
    SignIn,
    #[serde(rename = "SignUp")]
    SignUp,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DialogType {
    #[serde(rename = "AccountChooser")]
    AccountChooser,
    #[serde(rename = "AutoReauthn")]
    AutoReauthn,
    #[serde(rename = "ConfirmIdpLogin")]
    ConfirmIdpLogin,
    #[serde(rename = "Error")]
    Error,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DialogButton {
    #[serde(rename = "ConfirmIdpLoginContinue")]
    ConfirmIdpLoginContinue,
    #[serde(rename = "ErrorGotIt")]
    ErrorGotIt,
    #[serde(rename = "ErrorMoreDetails")]
    ErrorMoreDetails,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AccountUrlType {
    #[serde(rename = "TermsOfService")]
    TermsOfService,
    #[serde(rename = "PrivacyPolicy")]
    PrivacyPolicy,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Corresponds to IdentityRequestAccount"]
pub struct Account {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub given_name: String,
    #[serde(default)]
    pub picture_url: String,
    #[serde(default)]
    pub idp_config_url: String,
    #[serde(default)]
    pub idp_login_url: String,
    pub login_state: LoginState,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "These two are only set if the loginState is signUp"]
    pub terms_of_service_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_policy_url: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Allows callers to disable the promise rejection delay that would\n normally happen, if this is unimportant to what's being tested.\n (step 4 of <https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in>)"]
    pub disable_rejection_delay: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SelectAccount {
    #[serde(default)]
    pub dialog_id: String,
    #[serde(default)]
    pub account_index: JsUInt,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ClickDialogButton {
    #[serde(default)]
    pub dialog_id: String,
    pub dialog_button: DialogButton,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct OpenUrl {
    #[serde(default)]
    pub dialog_id: String,
    #[serde(default)]
    pub account_index: JsUInt,
    pub account_url_type: AccountUrlType,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct DismissDialog {
    #[serde(default)]
    pub dialog_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trigger_cooldown: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResetCooldown(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SelectAccountReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClickDialogButtonReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OpenUrlReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DismissDialogReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resets the cooldown time, if any, to allow the next FedCM call to show\n a dialog even if one was recently dismissed by the user."]
pub struct ResetCooldownReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "FedCm.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "FedCm.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for SelectAccount {
    const NAME: &'static str = "FedCm.selectAccount";
    type ReturnObject = SelectAccountReturnObject;
}
#[allow(deprecated)]
impl Method for ClickDialogButton {
    const NAME: &'static str = "FedCm.clickDialogButton";
    type ReturnObject = ClickDialogButtonReturnObject;
}
#[allow(deprecated)]
impl Method for OpenUrl {
    const NAME: &'static str = "FedCm.openUrl";
    type ReturnObject = OpenUrlReturnObject;
}
#[allow(deprecated)]
impl Method for DismissDialog {
    const NAME: &'static str = "FedCm.dismissDialog";
    type ReturnObject = DismissDialogReturnObject;
}
#[allow(deprecated)]
impl Method for ResetCooldown {
    const NAME: &'static str = "FedCm.resetCooldown";
    type ReturnObject = ResetCooldownReturnObject;
}
#[allow(dead_code)]
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogShownEvent {
        pub params: DialogShownEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogShownEventParams {
        #[serde(default)]
        pub dialog_id: String,
        pub dialog_type: super::DialogType,
        pub accounts: Vec<super::Account>,
        #[serde(default)]
        #[doc = "These exist primarily so that the caller can verify the\n RP context was used appropriately."]
        pub title: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub subtitle: Option<String>,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogClosedEvent {
        pub params: DialogClosedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogClosedEventParams {
        #[serde(default)]
        pub dialog_id: String,
    }
}
