// Auto-generated from Chrome at version 140.0.7339.186 domain: WebAuthn
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type AuthenticatorId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthenticatorProtocol {
    #[serde(rename = "u2f")]
    U2F,
    #[serde(rename = "ctap2")]
    Ctap2,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Ctap2Version {
    #[serde(rename = "ctap2_0")]
    Ctap20,
    #[serde(rename = "ctap2_1")]
    Ctap21,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthenticatorTransport {
    #[serde(rename = "usb")]
    Usb,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "ble")]
    Ble,
    #[serde(rename = "cable")]
    Cable,
    #[serde(rename = "internal")]
    Internal,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualAuthenticatorOptions {
    #[serde(rename = "protocol")]
    pub protocol: AuthenticatorProtocol,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ctap2Version")]
    pub ctap_2_version: Option<Ctap2Version>,
    #[serde(rename = "transport")]
    pub transport: AuthenticatorTransport,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasResidentKey")]
    pub has_resident_key: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasUserVerification")]
    pub has_user_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasLargeBlob")]
    pub has_large_blob: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasCredBlob")]
    pub has_cred_blob: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasMinPinLength")]
    pub has_min_pin_length: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "hasPrf")]
    pub has_prf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "automaticPresenceSimulation")]
    pub automatic_presence_simulation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isUserVerified")]
    pub is_user_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "defaultBackupEligibility")]
    pub default_backup_eligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "defaultBackupState")]
    pub default_backup_state: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Credential {
    #[serde(rename = "credentialId")]
    pub credential_id: Vec<u8>,
    #[serde(default)]
    #[serde(rename = "isResidentCredential")]
    pub is_resident_credential: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "rpId")]
    pub rp_id: Option<String>,
    #[serde(rename = "privateKey")]
    pub private_key: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userHandle")]
    pub user_handle: Option<Vec<u8>>,
    #[serde(default)]
    #[serde(rename = "signCount")]
    pub sign_count: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "largeBlob")]
    pub large_blob: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "backupEligibility")]
    pub backup_eligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "backupState")]
    pub backup_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "userDisplayName")]
    pub user_display_name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "enableUI")]
    pub enable_ui: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddVirtualAuthenticator {
    #[serde(rename = "options")]
    pub options: VirtualAuthenticatorOptions,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetResponseOverrideBits {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isBogusSignature")]
    pub is_bogus_signature: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isBadUV")]
    pub is_bad_uv: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "isBadUP")]
    pub is_bad_up: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveVirtualAuthenticator {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddCredential {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(rename = "credential")]
    pub credential: Credential,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCredential {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCredentials {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoveCredential {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearCredentials {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetUserVerified {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(default)]
    #[serde(rename = "isUserVerified")]
    pub is_user_verified: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAutomaticPresenceSimulation {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCredentialProperties {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
    #[serde(rename = "credentialId")]
    pub credential_id: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "backupEligibility")]
    pub backup_eligibility: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "backupState")]
    pub backup_state: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AddVirtualAuthenticatorReturnObject {
    #[serde(rename = "authenticatorId")]
    pub authenticator_id: AuthenticatorId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetResponseOverrideBitsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveVirtualAuthenticatorReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AddCredentialReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCredentialReturnObject {
    #[serde(rename = "credential")]
    pub credential: Credential,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCredentialsReturnObject {
    #[serde(rename = "credentials")]
    pub credentials: Vec<Credential>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RemoveCredentialReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCredentialsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetUserVerifiedReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAutomaticPresenceSimulationReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCredentialPropertiesReturnObject {}
impl Method for Enable {
    const NAME: &'static str = "WebAuthn.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "WebAuthn.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for AddVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.addVirtualAuthenticator";
    type ReturnObject = AddVirtualAuthenticatorReturnObject;
}
impl Method for SetResponseOverrideBits {
    const NAME: &'static str = "WebAuthn.setResponseOverrideBits";
    type ReturnObject = SetResponseOverrideBitsReturnObject;
}
impl Method for RemoveVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.removeVirtualAuthenticator";
    type ReturnObject = RemoveVirtualAuthenticatorReturnObject;
}
impl Method for AddCredential {
    const NAME: &'static str = "WebAuthn.addCredential";
    type ReturnObject = AddCredentialReturnObject;
}
impl Method for GetCredential {
    const NAME: &'static str = "WebAuthn.getCredential";
    type ReturnObject = GetCredentialReturnObject;
}
impl Method for GetCredentials {
    const NAME: &'static str = "WebAuthn.getCredentials";
    type ReturnObject = GetCredentialsReturnObject;
}
impl Method for RemoveCredential {
    const NAME: &'static str = "WebAuthn.removeCredential";
    type ReturnObject = RemoveCredentialReturnObject;
}
impl Method for ClearCredentials {
    const NAME: &'static str = "WebAuthn.clearCredentials";
    type ReturnObject = ClearCredentialsReturnObject;
}
impl Method for SetUserVerified {
    const NAME: &'static str = "WebAuthn.setUserVerified";
    type ReturnObject = SetUserVerifiedReturnObject;
}
impl Method for SetAutomaticPresenceSimulation {
    const NAME: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
    type ReturnObject = SetAutomaticPresenceSimulationReturnObject;
}
impl Method for SetCredentialProperties {
    const NAME: &'static str = "WebAuthn.setCredentialProperties";
    type ReturnObject = SetCredentialPropertiesReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialAddedEvent {
        pub params: CredentialAddedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialAddedEventParams {
        #[serde(rename = "authenticatorId")]
        pub authenticator_id: super::AuthenticatorId,
        #[serde(rename = "credential")]
        pub credential: super::Credential,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialDeletedEvent {
        pub params: CredentialDeletedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialDeletedEventParams {
        #[serde(rename = "authenticatorId")]
        pub authenticator_id: super::AuthenticatorId,
        #[serde(default)]
        #[serde(rename = "credentialId")]
        pub credential_id: u8,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialUpdatedEvent {
        pub params: CredentialUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialUpdatedEventParams {
        #[serde(rename = "authenticatorId")]
        pub authenticator_id: super::AuthenticatorId,
        #[serde(rename = "credential")]
        pub credential: super::Credential,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialAssertedEvent {
        pub params: CredentialAssertedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialAssertedEventParams {
        #[serde(rename = "authenticatorId")]
        pub authenticator_id: super::AuthenticatorId,
        #[serde(rename = "credential")]
        pub credential: super::Credential,
    }
}
