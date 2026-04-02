// Auto-generated from Chrome at version 146.0.7680.165 domain: WebAuthn
#![allow(dead_code)]
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type AuthenticatorId = String;
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthenticatorProtocol {
    #[serde(rename = "u2f")]
    U2F,
    #[serde(rename = "ctap2")]
    Ctap2,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Ctap2Version {
    #[serde(rename = "ctap2_0")]
    Ctap20,
    #[serde(rename = "ctap2_1")]
    Ctap21,
}
#[allow(deprecated)]
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
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct VirtualAuthenticatorOptions {
    pub protocol: AuthenticatorProtocol,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Defaults to ctap2_0. Ignored if |protocol| == u2f."]
    pub ctap_2_version: Option<Ctap2Version>,
    pub transport: AuthenticatorTransport,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Defaults to false."]
    pub has_resident_key: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Defaults to false."]
    pub has_user_verification: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set to true, the authenticator will support the largeBlob extension.\n <https://w3c.github.io/webauthn#largeBlob>\n Defaults to false."]
    pub has_large_blob: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set to true, the authenticator will support the credBlob extension.\n <https://fidoalliance.org/specs/fido-v2.1-rd-20201208/fido-client-to-authenticator-protocol-v2.1-rd-20201208.html#sctn-credBlob-extension>\n Defaults to false."]
    pub has_cred_blob: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set to true, the authenticator will support the minPinLength extension.\n <https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-minpinlength-extension>\n Defaults to false."]
    pub has_min_pin_length: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set to true, the authenticator will support the prf extension.\n <https://w3c.github.io/webauthn/#prf-extension>\n Defaults to false."]
    pub has_prf: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set to true, tests of user presence will succeed immediately.\n Otherwise, they will not be resolved. Defaults to true."]
    pub automatic_presence_simulation: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Sets whether User Verification succeeds or fails for an authenticator.\n Defaults to false."]
    pub is_user_verified: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Credentials created by this authenticator will have the backup\n eligibility (BE) flag set to this value. Defaults to false.\n <https://w3c.github.io/webauthn/#sctn-credential-backup>"]
    pub default_backup_eligibility: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Credentials created by this authenticator will have the backup state\n (BS) flag set to this value. Defaults to false.\n <https://w3c.github.io/webauthn/#sctn-credential-backup>"]
    pub default_backup_state: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct Credential {
    pub credential_id: String,
    #[serde(default)]
    pub is_resident_credential: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Relying Party ID the credential is scoped to. Must be set when adding a\n credential."]
    pub rp_id: Option<String>,
    #[doc = "The ECDSA P-256 private key in PKCS#8 format."]
    pub private_key: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "An opaque byte sequence with a maximum size of 64 bytes mapping the\n credential to a specific user."]
    pub user_handle: Option<String>,
    #[serde(default)]
    #[doc = "Signature counter. This is incremented by one for each successful\n assertion.\n See <https://w3c.github.io/webauthn/#signature-counter>"]
    pub sign_count: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "The large blob associated with the credential.\n See <https://w3c.github.io/webauthn/#sctn-large-blob-extension>"]
    pub large_blob: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Assertions returned by this credential will have the backup eligibility\n (BE) flag set to this value. Defaults to the authenticator's\n defaultBackupEligibility value."]
    pub backup_eligibility: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Assertions returned by this credential will have the backup state (BS)\n flag set to this value. Defaults to the authenticator's\n defaultBackupState value."]
    pub backup_state: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The credential's user.name property. Equivalent to empty if not set.\n <https://w3c.github.io/webauthn/#dom-publickeycredentialentity-name>"]
    pub user_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The credential's user.displayName property. Equivalent to empty if\n not set.\n <https://w3c.github.io/webauthn/#dom-publickeycredentialuserentity-displayname>"]
    pub user_display_name: Option<String>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enable the WebAuthn domain and start intercepting credential storage and\n retrieval with a virtual authenticator."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether to enable the WebAuthn user interface. Enabling the UI is\n recommended for debugging and demo purposes, as it is closer to the real\n experience. Disabling the UI is recommended for automated testing.\n Supported at the embedder's discretion if UI is available.\n Defaults to false."]
    #[serde(rename = "enableUI")]
    pub enable_ui: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Creates and adds a virtual authenticator."]
pub struct AddVirtualAuthenticator {
    pub options: VirtualAuthenticatorOptions,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present."]
pub struct SetResponseOverrideBits {
    pub authenticator_id: AuthenticatorId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If isBogusSignature is set, overrides the signature in the authenticator response to be zero.\n Defaults to false."]
    pub is_bogus_signature: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If isBadUV is set, overrides the UV bit in the flags in the authenticator response to\n be zero. Defaults to false."]
    #[serde(rename = "isBadUV")]
    pub is_bad_uv: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If isBadUP is set, overrides the UP bit in the flags in the authenticator response to\n be zero. Defaults to false."]
    #[serde(rename = "isBadUP")]
    pub is_bad_up: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes the given authenticator."]
pub struct RemoveVirtualAuthenticator {
    pub authenticator_id: AuthenticatorId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Adds the credential to the specified authenticator."]
pub struct AddCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential: Credential,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a single credential stored in the given virtual authenticator that\n matches the credential ID."]
pub struct GetCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: Vec<u8>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all the credentials stored in the given virtual authenticator."]
pub struct GetCredentials {
    pub authenticator_id: AuthenticatorId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes a credential from the authenticator."]
pub struct RemoveCredential {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: Vec<u8>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears all the credentials from the specified device."]
pub struct ClearCredentials {
    pub authenticator_id: AuthenticatorId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets whether User Verification succeeds or fails for an authenticator.\n The default is true."]
pub struct SetUserVerified {
    pub authenticator_id: AuthenticatorId,
    #[serde(default)]
    pub is_user_verified: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.\n The default is true."]
pub struct SetAutomaticPresenceSimulation {
    pub authenticator_id: AuthenticatorId,
    #[serde(default)]
    pub enabled: bool,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Allows setting credential properties.\n <https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties>"]
pub struct SetCredentialProperties {
    pub authenticator_id: AuthenticatorId,
    pub credential_id: Vec<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_eligibility: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backup_state: Option<bool>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enable the WebAuthn domain and start intercepting credential storage and\n retrieval with a virtual authenticator."]
pub struct EnableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disable the WebAuthn domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Creates and adds a virtual authenticator."]
pub struct AddVirtualAuthenticatorReturnObject {
    pub authenticator_id: AuthenticatorId,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resets parameters isBogusSignature, isBadUV, isBadUP to false if they are not present."]
pub struct SetResponseOverrideBitsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes the given authenticator."]
pub struct RemoveVirtualAuthenticatorReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Adds the credential to the specified authenticator."]
pub struct AddCredentialReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a single credential stored in the given virtual authenticator that\n matches the credential ID."]
pub struct GetCredentialReturnObject {
    pub credential: Credential,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all the credentials stored in the given virtual authenticator."]
pub struct GetCredentialsReturnObject {
    pub credentials: Vec<Credential>,
}
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Removes a credential from the authenticator."]
pub struct RemoveCredentialReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears all the credentials from the specified device."]
pub struct ClearCredentialsReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets whether User Verification succeeds or fails for an authenticator.\n The default is true."]
pub struct SetUserVerifiedReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets whether tests of user presence will succeed immediately (if true) or fail to resolve (if false) for an authenticator.\n The default is true."]
pub struct SetAutomaticPresenceSimulationReturnObject(pub Option<Json>);
#[allow(deprecated)]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Allows setting credential properties.\n <https://w3c.github.io/webauthn/#sctn-automation-set-credential-properties>"]
pub struct SetCredentialPropertiesReturnObject(pub Option<Json>);
#[allow(deprecated)]
impl Method for Enable {
    const NAME: &'static str = "WebAuthn.enable";
    type ReturnObject = EnableReturnObject;
}
#[allow(deprecated)]
impl Method for Disable {
    const NAME: &'static str = "WebAuthn.disable";
    type ReturnObject = DisableReturnObject;
}
#[allow(deprecated)]
impl Method for AddVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.addVirtualAuthenticator";
    type ReturnObject = AddVirtualAuthenticatorReturnObject;
}
#[allow(deprecated)]
impl Method for SetResponseOverrideBits {
    const NAME: &'static str = "WebAuthn.setResponseOverrideBits";
    type ReturnObject = SetResponseOverrideBitsReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveVirtualAuthenticator {
    const NAME: &'static str = "WebAuthn.removeVirtualAuthenticator";
    type ReturnObject = RemoveVirtualAuthenticatorReturnObject;
}
#[allow(deprecated)]
impl Method for AddCredential {
    const NAME: &'static str = "WebAuthn.addCredential";
    type ReturnObject = AddCredentialReturnObject;
}
#[allow(deprecated)]
impl Method for GetCredential {
    const NAME: &'static str = "WebAuthn.getCredential";
    type ReturnObject = GetCredentialReturnObject;
}
#[allow(deprecated)]
impl Method for GetCredentials {
    const NAME: &'static str = "WebAuthn.getCredentials";
    type ReturnObject = GetCredentialsReturnObject;
}
#[allow(deprecated)]
impl Method for RemoveCredential {
    const NAME: &'static str = "WebAuthn.removeCredential";
    type ReturnObject = RemoveCredentialReturnObject;
}
#[allow(deprecated)]
impl Method for ClearCredentials {
    const NAME: &'static str = "WebAuthn.clearCredentials";
    type ReturnObject = ClearCredentialsReturnObject;
}
#[allow(deprecated)]
impl Method for SetUserVerified {
    const NAME: &'static str = "WebAuthn.setUserVerified";
    type ReturnObject = SetUserVerifiedReturnObject;
}
#[allow(deprecated)]
impl Method for SetAutomaticPresenceSimulation {
    const NAME: &'static str = "WebAuthn.setAutomaticPresenceSimulation";
    type ReturnObject = SetAutomaticPresenceSimulationReturnObject;
}
#[allow(deprecated)]
impl Method for SetCredentialProperties {
    const NAME: &'static str = "WebAuthn.setCredentialProperties";
    type ReturnObject = SetCredentialPropertiesReturnObject;
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
    pub struct CredentialAddedEvent {
        pub params: CredentialAddedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CredentialAddedEventParams {
        pub authenticator_id: super::AuthenticatorId,
        pub credential: super::Credential,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialDeletedEvent {
        pub params: CredentialDeletedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CredentialDeletedEventParams {
        pub authenticator_id: super::AuthenticatorId,
        #[serde(default)]
        pub credential_id: String,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialUpdatedEvent {
        pub params: CredentialUpdatedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CredentialUpdatedEventParams {
        pub authenticator_id: super::AuthenticatorId,
        pub credential: super::Credential,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CredentialAssertedEvent {
        pub params: CredentialAssertedEventParams,
    }
    #[allow(deprecated)]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct CredentialAssertedEventParams {
        pub authenticator_id: super::AuthenticatorId,
        pub credential: super::Credential,
    }
}
