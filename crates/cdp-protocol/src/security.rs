// Auto-generated from Chrome at version 140.0.7339.186 domain: Security
use super::network;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type CertificateId = JsUInt;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MixedContentType {
    #[serde(rename = "blockable")]
    Blockable,
    #[serde(rename = "optionally-blockable")]
    OptionallyBlockable,
    #[serde(rename = "none")]
    None,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SecurityState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "insecure-broken")]
    InsecureBroken,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SafetyTipStatus {
    #[serde(rename = "badReputation")]
    BadReputation,
    #[serde(rename = "lookalike")]
    Lookalike,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CertificateErrorAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "cancel")]
    Cancel,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CertificateSecurityState {
    #[serde(default)]
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(default)]
    #[serde(rename = "keyExchange")]
    pub key_exchange: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keyExchangeGroup")]
    pub key_exchange_group: Option<String>,
    #[serde(default)]
    #[serde(rename = "cipher")]
    pub cipher: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "mac")]
    pub mac: Option<String>,
    #[serde(default)]
    #[serde(rename = "certificate")]
    pub certificate: Vec<String>,
    #[serde(default)]
    #[serde(rename = "subjectName")]
    pub subject_name: String,
    #[serde(default)]
    #[serde(rename = "issuer")]
    pub issuer: String,
    #[serde(rename = "validFrom")]
    pub valid_from: network::TimeSinceEpoch,
    #[serde(rename = "validTo")]
    pub valid_to: network::TimeSinceEpoch,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "certificateNetworkError")]
    pub certificate_network_error: Option<String>,
    #[serde(default)]
    #[serde(rename = "certificateHasWeakSignature")]
    pub certificate_has_weak_signature: bool,
    #[serde(default)]
    #[serde(rename = "certificateHasSha1Signature")]
    pub certificate_has_sha_1_signature: bool,
    #[serde(default)]
    #[serde(rename = "modernSSL")]
    pub modern_ssl: bool,
    #[serde(default)]
    #[serde(rename = "obsoleteSslProtocol")]
    pub obsolete_ssl_protocol: bool,
    #[serde(default)]
    #[serde(rename = "obsoleteSslKeyExchange")]
    pub obsolete_ssl_key_exchange: bool,
    #[serde(default)]
    #[serde(rename = "obsoleteSslCipher")]
    pub obsolete_ssl_cipher: bool,
    #[serde(default)]
    #[serde(rename = "obsoleteSslSignature")]
    pub obsolete_ssl_signature: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SafetyTipInfo {
    #[serde(rename = "safetyTipStatus")]
    pub safety_tip_status: SafetyTipStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "safeUrl")]
    pub safe_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VisibleSecurityState {
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificateSecurityState")]
    pub certificate_security_state: Option<CertificateSecurityState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "safetyTipInfo")]
    pub safety_tip_info: Option<SafetyTipInfo>,
    #[serde(default)]
    #[serde(rename = "securityStateIssueIds")]
    pub security_state_issue_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecurityStateExplanation {
    #[serde(rename = "securityState")]
    pub security_state: SecurityState,
    #[serde(default)]
    #[serde(rename = "title")]
    pub title: String,
    #[serde(default)]
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(default)]
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "mixedContentType")]
    pub mixed_content_type: MixedContentType,
    #[serde(default)]
    #[serde(rename = "certificate")]
    pub certificate: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "recommendations")]
    pub recommendations: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsecureContentStatus {
    #[serde(default)]
    #[serde(rename = "ranMixedContent")]
    pub ran_mixed_content: bool,
    #[serde(default)]
    #[serde(rename = "displayedMixedContent")]
    pub displayed_mixed_content: bool,
    #[serde(default)]
    #[serde(rename = "containedMixedForm")]
    pub contained_mixed_form: bool,
    #[serde(default)]
    #[serde(rename = "ranContentWithCertErrors")]
    pub ran_content_with_cert_errors: bool,
    #[serde(default)]
    #[serde(rename = "displayedContentWithCertErrors")]
    pub displayed_content_with_cert_errors: bool,
    #[serde(rename = "ranInsecureContentStyle")]
    pub ran_insecure_content_style: SecurityState,
    #[serde(rename = "displayedInsecureContentStyle")]
    pub displayed_insecure_content_style: SecurityState,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Enable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetIgnoreCertificateErrors {
    #[serde(default)]
    #[serde(rename = "ignore")]
    pub ignore: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HandleCertificateError {
    #[serde(default)]
    #[serde(rename = "eventId")]
    pub event_id: JsUInt,
    #[serde(rename = "action")]
    pub action: CertificateErrorAction,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetOverrideCertificateErrors {
    #[serde(default)]
    #[serde(rename = "override")]
    pub r#override: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetIgnoreCertificateErrorsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HandleCertificateErrorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetOverrideCertificateErrorsReturnObject {}
impl Method for Disable {
    const NAME: &'static str = "Security.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Security.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for SetIgnoreCertificateErrors {
    const NAME: &'static str = "Security.setIgnoreCertificateErrors";
    type ReturnObject = SetIgnoreCertificateErrorsReturnObject;
}
impl Method for HandleCertificateError {
    const NAME: &'static str = "Security.handleCertificateError";
    type ReturnObject = HandleCertificateErrorReturnObject;
}
impl Method for SetOverrideCertificateErrors {
    const NAME: &'static str = "Security.setOverrideCertificateErrors";
    type ReturnObject = SetOverrideCertificateErrorsReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CertificateErrorEvent {
        pub params: CertificateErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CertificateErrorEventParams {
        #[serde(default)]
        #[serde(rename = "eventId")]
        pub event_id: JsUInt,
        #[serde(default)]
        #[serde(rename = "errorType")]
        pub error_type: String,
        #[serde(default)]
        #[serde(rename = "requestURL")]
        pub request_url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct VisibleSecurityStateChangedEvent {
        pub params: VisibleSecurityStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct VisibleSecurityStateChangedEventParams {
        #[serde(rename = "visibleSecurityState")]
        pub visible_security_state: super::VisibleSecurityState,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SecurityStateChangedEvent {
        pub params: SecurityStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SecurityStateChangedEventParams {
        #[serde(rename = "securityState")]
        pub security_state: super::SecurityState,
        #[serde(default)]
        #[serde(rename = "schemeIsCryptographic")]
        pub scheme_is_cryptographic: bool,
        #[serde(rename = "explanations")]
        pub explanations: Vec<super::SecurityStateExplanation>,
        #[serde(rename = "insecureContentStatus")]
        pub insecure_content_status: super::InsecureContentStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "summary")]
        pub summary: Option<String>,
    }
}
