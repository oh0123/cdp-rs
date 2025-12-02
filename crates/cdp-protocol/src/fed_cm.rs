// Auto-generated from Chrome at version 140.0.7339.186 domain: FedCm
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum LoginState {
    #[serde(rename = "SignIn")]
    SignIn,
    #[serde(rename = "SignUp")]
    SignUp,
}
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DialogButton {
    #[serde(rename = "ConfirmIdpLoginContinue")]
    ConfirmIdpLoginContinue,
    #[serde(rename = "ErrorGotIt")]
    ErrorGotIt,
    #[serde(rename = "ErrorMoreDetails")]
    ErrorMoreDetails,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AccountUrlType {
    #[serde(rename = "TermsOfService")]
    TermsOfService,
    #[serde(rename = "PrivacyPolicy")]
    PrivacyPolicy,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Account {
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(default)]
    #[serde(rename = "email")]
    pub email: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(default)]
    #[serde(rename = "pictureUrl")]
    pub picture_url: String,
    #[serde(default)]
    #[serde(rename = "idpConfigUrl")]
    pub idp_config_url: String,
    #[serde(default)]
    #[serde(rename = "idpLoginUrl")]
    pub idp_login_url: String,
    #[serde(rename = "loginState")]
    pub login_state: LoginState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "termsOfServiceUrl")]
    pub terms_of_service_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "disableRejectionDelay")]
    pub disable_rejection_delay: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SelectAccount {
    #[serde(default)]
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(default)]
    #[serde(rename = "accountIndex")]
    pub account_index: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClickDialogButton {
    #[serde(default)]
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "dialogButton")]
    pub dialog_button: DialogButton,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OpenUrl {
    #[serde(default)]
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(default)]
    #[serde(rename = "accountIndex")]
    pub account_index: JsUInt,
    #[serde(rename = "accountUrlType")]
    pub account_url_type: AccountUrlType,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DismissDialog {
    #[serde(default)]
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "triggerCooldown")]
    pub trigger_cooldown: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetCooldown(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SelectAccountReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClickDialogButtonReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenUrlReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DismissDialogReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetCooldownReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "FedCm.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "FedCm.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for SelectAccount {
    const NAME: &'static str = "FedCm.selectAccount";
    type ReturnObject = SelectAccountReturnObject;
}
impl Method for ClickDialogButton {
    const NAME: &'static str = "FedCm.clickDialogButton";
    type ReturnObject = ClickDialogButtonReturnObject;
}
impl Method for OpenUrl {
    const NAME: &'static str = "FedCm.openUrl";
    type ReturnObject = OpenUrlReturnObject;
}
impl Method for DismissDialog {
    const NAME: &'static str = "FedCm.dismissDialog";
    type ReturnObject = DismissDialogReturnObject;
}
impl Method for ResetCooldown {
    const NAME: &'static str = "FedCm.resetCooldown";
    type ReturnObject = ResetCooldownReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogShownEvent {
        pub params: DialogShownEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogShownEventParams {
        #[serde(default)]
        #[serde(rename = "dialogId")]
        pub dialog_id: String,
        #[serde(rename = "dialogType")]
        pub dialog_type: super::DialogType,
        #[serde(rename = "accounts")]
        pub accounts: Vec<super::Account>,
        #[serde(default)]
        #[serde(rename = "title")]
        pub title: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "subtitle")]
        pub subtitle: Option<String>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogClosedEvent {
        pub params: DialogClosedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DialogClosedEventParams {
        #[serde(default)]
        #[serde(rename = "dialogId")]
        pub dialog_id: String,
    }
}
