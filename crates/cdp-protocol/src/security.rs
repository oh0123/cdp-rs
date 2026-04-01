// Auto-generated from Chrome at version 146.0.7680.165 domain: Security
use super::network;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details about the security state of the page certificate."]
pub struct CertificateSecurityState {
    #[serde(default)]
    #[doc = "Protocol name (e.g. \"TLS 1.2\" or \"QUIC\")."]
    pub protocol: String,
    #[serde(default)]
    #[doc = "Key Exchange used by the connection, or the empty string if not applicable."]
    pub key_exchange: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "(EC)DH group used by the connection, if applicable."]
    pub key_exchange_group: Option<String>,
    #[serde(default)]
    #[doc = "Cipher name."]
    pub cipher: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "TLS MAC. Note that AEAD ciphers do not have separate MACs."]
    pub mac: Option<String>,
    #[serde(default)]
    #[doc = "Page certificate."]
    pub certificate: Vec<String>,
    #[serde(default)]
    #[doc = "Certificate subject name."]
    pub subject_name: String,
    #[serde(default)]
    #[doc = "Name of the issuing CA."]
    pub issuer: String,
    #[doc = "Certificate valid from date."]
    pub valid_from: network::TimeSinceEpoch,
    #[doc = "Certificate valid to (expiration) date"]
    pub valid_to: network::TimeSinceEpoch,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The highest priority network error code, if the certificate has an error."]
    pub certificate_network_error: Option<String>,
    #[serde(default)]
    #[doc = "True if the certificate uses a weak signature algorithm."]
    pub certificate_has_weak_signature: bool,
    #[serde(default)]
    #[doc = "True if the certificate has a SHA1 signature in the chain."]
    pub certificate_has_sha_1_signature: bool,
    #[serde(default)]
    #[doc = "True if modern SSL"]
    #[serde(rename = "modernSSL")]
    pub modern_ssl: bool,
    #[serde(default)]
    #[doc = "True if the connection is using an obsolete SSL protocol."]
    pub obsolete_ssl_protocol: bool,
    #[serde(default)]
    #[doc = "True if the connection is using an obsolete SSL key exchange."]
    pub obsolete_ssl_key_exchange: bool,
    #[serde(default)]
    #[doc = "True if the connection is using an obsolete SSL cipher."]
    pub obsolete_ssl_cipher: bool,
    #[serde(default)]
    #[doc = "True if the connection is using an obsolete SSL signature."]
    pub obsolete_ssl_signature: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SafetyTipInfo {
    #[doc = "Describes whether the page triggers any safety tips or reputation warnings. Default is unknown."]
    pub safety_tip_status: SafetyTipStatus,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The URL the safety tip suggested (\"Did you mean?\"). Only filled in for lookalike matches."]
    pub safe_url: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Security state information about the page."]
pub struct VisibleSecurityState {
    #[doc = "The security level of the page."]
    pub security_state: SecurityState,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Security state details about the page certificate."]
    pub certificate_security_state: Option<CertificateSecurityState>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The type of Safety Tip triggered on the page. Note that this field will be set even if the Safety Tip UI was not actually shown."]
    pub safety_tip_info: Option<SafetyTipInfo>,
    #[serde(default)]
    #[doc = "Array of security state issues ids."]
    pub security_state_issue_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "An explanation of an factor contributing to the security state."]
pub struct SecurityStateExplanation {
    #[doc = "Security state representing the severity of the factor being explained."]
    pub security_state: SecurityState,
    #[serde(default)]
    #[doc = "Title describing the type of factor."]
    pub title: String,
    #[serde(default)]
    #[doc = "Short phrase describing the type of factor."]
    pub summary: String,
    #[serde(default)]
    #[doc = "Full text explanation of the factor."]
    pub description: String,
    #[doc = "The type of mixed content described by the explanation."]
    pub mixed_content_type: MixedContentType,
    #[serde(default)]
    #[doc = "Page certificate."]
    pub certificate: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Recommendations to fix any issues."]
    pub recommendations: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Information about insecure content on the page."]
#[deprecated]
pub struct InsecureContentStatus {
    #[serde(default)]
    #[doc = "Always false."]
    pub ran_mixed_content: bool,
    #[serde(default)]
    #[doc = "Always false."]
    pub displayed_mixed_content: bool,
    #[serde(default)]
    #[doc = "Always false."]
    pub contained_mixed_form: bool,
    #[serde(default)]
    #[doc = "Always false."]
    pub ran_content_with_cert_errors: bool,
    #[serde(default)]
    #[doc = "Always false."]
    pub displayed_content_with_cert_errors: bool,
    #[doc = "Always set to unknown."]
    pub ran_insecure_content_style: SecurityState,
    #[doc = "Always set to unknown."]
    pub displayed_insecure_content_style: SecurityState,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable/disable whether all certificate errors should be ignored."]
pub struct SetIgnoreCertificateErrors {
    #[serde(default)]
    #[doc = "If true, all certificate errors will be ignored."]
    pub ignore: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Handles a certificate error that fired a certificateError event."]
#[deprecated]
pub struct HandleCertificateError {
    #[serde(default)]
    #[doc = "The ID of the event."]
    pub event_id: JsUInt,
    #[doc = "The action to take on the certificate error."]
    pub action: CertificateErrorAction,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable/disable overriding certificate errors. If enabled, all certificate error events need to\n be handled by the DevTools client and should be answered with `handleCertificateError` commands."]
#[deprecated]
pub struct SetOverrideCertificateErrors {
    #[serde(default)]
    #[doc = "If true, certificate errors will be overridden."]
    pub r#override: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables tracking security state changes."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables tracking security state changes."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable/disable whether all certificate errors should be ignored."]
pub struct SetIgnoreCertificateErrorsReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Handles a certificate error that fired a certificateError event."]
#[deprecated]
pub struct HandleCertificateErrorReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable/disable overriding certificate errors. If enabled, all certificate error events need to\n be handled by the DevTools client and should be answered with `handleCertificateError` commands."]
#[deprecated]
pub struct SetOverrideCertificateErrorsReturnObject(pub Option<Json>);
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CertificateErrorEvent {
        pub params: CertificateErrorEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CertificateErrorEventParams {
        #[serde(default)]
        #[doc = "The ID of the event."]
        pub event_id: JsUInt,
        #[serde(default)]
        #[doc = "The type of the error."]
        pub error_type: String,
        #[serde(default)]
        #[doc = "The url that was requested."]
        #[serde(rename = "requestURL")]
        pub request_url: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct VisibleSecurityStateChangedEvent {
        pub params: VisibleSecurityStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct VisibleSecurityStateChangedEventParams {
        #[doc = "Security state information about the page."]
        pub visible_security_state: super::VisibleSecurityState,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SecurityStateChangedEvent {
        pub params: SecurityStateChangedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct SecurityStateChangedEventParams {
        #[doc = "Security state."]
        pub security_state: super::SecurityState,
        #[serde(default)]
        #[doc = "True if the page was loaded over cryptographic transport such as HTTPS."]
        #[deprecated]
        pub scheme_is_cryptographic: bool,
        #[doc = "Previously a list of explanations for the security state. Now always\n empty."]
        #[deprecated]
        pub explanations: Vec<super::SecurityStateExplanation>,
        #[doc = "Information about insecure content on the page."]
        #[deprecated]
        pub insecure_content_status: super::InsecureContentStatus,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Overrides user-visible description of the state. Always omitted."]
        #[deprecated]
        pub summary: Option<String>,
    }
}
