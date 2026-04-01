// Auto-generated from Chrome at version 146.0.7680.165 domain: SmartCardEmulation
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ResultCode {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "removed-card")]
    RemovedCard,
    #[serde(rename = "reset-card")]
    ResetCard,
    #[serde(rename = "unpowered-card")]
    UnpoweredCard,
    #[serde(rename = "unresponsive-card")]
    UnresponsiveCard,
    #[serde(rename = "unsupported-card")]
    UnsupportedCard,
    #[serde(rename = "reader-unavailable")]
    ReaderUnavailable,
    #[serde(rename = "sharing-violation")]
    SharingViolation,
    #[serde(rename = "not-transacted")]
    NotTransacted,
    #[serde(rename = "no-smartcard")]
    NoSmartcard,
    #[serde(rename = "proto-mismatch")]
    ProtoMismatch,
    #[serde(rename = "system-cancelled")]
    SystemCancelled,
    #[serde(rename = "not-ready")]
    NotReady,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "insufficient-buffer")]
    InsufficientBuffer,
    #[serde(rename = "invalid-handle")]
    InvalidHandle,
    #[serde(rename = "invalid-parameter")]
    InvalidParameter,
    #[serde(rename = "invalid-value")]
    InvalidValue,
    #[serde(rename = "no-memory")]
    NoMemory,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "unknown-reader")]
    UnknownReader,
    #[serde(rename = "unsupported-feature")]
    UnsupportedFeature,
    #[serde(rename = "no-readers-available")]
    NoReadersAvailable,
    #[serde(rename = "service-stopped")]
    ServiceStopped,
    #[serde(rename = "no-service")]
    NoService,
    #[serde(rename = "comm-error")]
    CommError,
    #[serde(rename = "internal-error")]
    InternalError,
    #[serde(rename = "server-too-busy")]
    ServerTooBusy,
    #[serde(rename = "unexpected")]
    Unexpected,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "unknown-card")]
    UnknownCard,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ShareMode {
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "direct")]
    Direct,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Disposition {
    #[serde(rename = "leave-card")]
    LeaveCard,
    #[serde(rename = "reset-card")]
    ResetCard,
    #[serde(rename = "unpower-card")]
    UnpowerCard,
    #[serde(rename = "eject-card")]
    EjectCard,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConnectionState {
    #[serde(rename = "absent")]
    Absent,
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "swallowed")]
    Swallowed,
    #[serde(rename = "powered")]
    Powered,
    #[serde(rename = "negotiable")]
    Negotiable,
    #[serde(rename = "specific")]
    Specific,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Protocol {
    #[serde(rename = "t0")]
    T0,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "raw")]
    Raw,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Maps to the |SCARD_STATE_*| flags."]
pub struct ReaderStateFlags {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unaware: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub changed: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unknown: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unavailable: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub empty: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub present: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exclusive: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inuse: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mute: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unpowered: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Maps to the |SCARD_PROTOCOL_*| flags."]
pub struct ProtocolSet {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_0: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub t_1: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub raw: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateIn {
    #[serde(default)]
    pub reader: String,
    pub current_state: ReaderStateFlags,
    #[serde(default)]
    pub current_insertion_count: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct ReaderStateOut {
    #[serde(default)]
    pub reader: String,
    pub event_state: ReaderStateFlags,
    #[serde(default)]
    pub event_count: JsUInt,
    pub atr: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardEstablishContext| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext"]
pub struct ReportEstablishContextResult {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub context_id: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardReleaseContext| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext"]
pub struct ReportReleaseContextResult {
    #[serde(default)]
    pub request_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardListReaders| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa"]
pub struct ReportListReadersResult {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub readers: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardGetStatusChange| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea"]
pub struct ReportGetStatusChangeResult {
    #[serde(default)]
    pub request_id: String,
    pub reader_states: Vec<ReaderStateOut>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the result of a |SCardBeginTransaction| call.\n On success, this creates a new transaction object.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction"]
pub struct ReportBeginTransactionResult {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub handle: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a call that returns only a result code.\n Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.\n \n This maps to:\n 1. SCardCancel\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel\n \n 2. SCardDisconnect\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect\n \n 3. SCardSetAttrib\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib\n \n 4. SCardEndTransaction\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction"]
pub struct ReportPlainResult {
    #[serde(default)]
    pub request_id: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardConnect| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta"]
pub struct ReportConnectResult {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub handle: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_protocol: Option<Protocol>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a call that sends back data on success.\n Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.\n \n This maps to:\n 1. SCardTransmit\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit\n \n 2. SCardControl\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol\n \n 3. SCardGetAttrib\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib"]
pub struct ReportDataResult {
    #[serde(default)]
    pub request_id: String,
    pub data: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports the successful result of a |SCardStatus| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa"]
pub struct ReportStatusResult {
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub reader_name: String,
    pub state: ConnectionState,
    pub atr: Vec<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Reports an error result for the given request."]
pub struct ReportError {
    #[serde(default)]
    pub request_id: String,
    pub result_code: ResultCode,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables the |SmartCardEmulation| domain."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the |SmartCardEmulation| domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardEstablishContext| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaa1b8970169fd4883a6dc4a8f43f19b67\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardestablishcontext"]
pub struct ReportEstablishContextResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardReleaseContext| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga6aabcba7744c5c9419fdd6404f73a934\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardreleasecontext"]
pub struct ReportReleaseContextResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardListReaders| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga93b07815789b3cf2629d439ecf20f0d9\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardlistreadersa"]
pub struct ReportListReadersResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardGetStatusChange| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga33247d5d1257d59e55647c3bb717db24\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetstatuschangea"]
pub struct ReportGetStatusChangeResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the result of a |SCardBeginTransaction| call.\n On success, this creates a new transaction object.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaddb835dce01a0da1d6ca02d33ee7d861\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardbegintransaction"]
pub struct ReportBeginTransactionResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a call that returns only a result code.\n Used for: |SCardCancel|, |SCardDisconnect|, |SCardSetAttrib|, |SCardEndTransaction|.\n \n This maps to:\n 1. SCardCancel\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacbbc0c6d6c0cbbeb4f4debf6fbeeee6\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcancel\n \n 2. SCardDisconnect\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4be198045c73ec0deb79e66c0ca1738a\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scarddisconnect\n \n 3. SCardSetAttrib\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga060f0038a4ddfd5dd2b8fadf3c3a2e4f\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardsetattrib\n \n 4. SCardEndTransaction\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae8742473b404363e5c587f570d7e2f3b\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardendtransaction"]
pub struct ReportPlainResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardConnect| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga4e515829752e0a8dbc4d630696a8d6a5\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardconnecta"]
pub struct ReportConnectResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a call that sends back data on success.\n Used for |SCardTransmit|, |SCardControl|, and |SCardGetAttrib|.\n \n This maps to:\n 1. SCardTransmit\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#ga9a2d77242a271310269065e64633ab99\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardtransmit\n \n 2. SCardControl\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gac3454d4657110fd7f753b2d3d8f4e32f\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardcontrol\n \n 3. SCardGetAttrib\n    PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gaacfec51917255b7a25b94c5104961602\n    Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardgetattrib"]
pub struct ReportDataResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports the successful result of a |SCardStatus| call.\n \n This maps to:\n PC/SC Lite: https://pcsclite.apdu.fr/api/group__API.html#gae49c3c894ad7ac12a5b896bde70d0382\n Microsoft: https://learn.microsoft.com/en-us/windows/win32/api/winscard/nf-winscard-scardstatusa"]
pub struct ReportStatusResultReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Reports an error result for the given request."]
pub struct ReportErrorReturnObject(pub Option<Json>);
impl Method for Enable {
    const NAME: &'static str = "SmartCardEmulation.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for Disable {
    const NAME: &'static str = "SmartCardEmulation.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for ReportEstablishContextResult {
    const NAME: &'static str = "SmartCardEmulation.reportEstablishContextResult";
    type ReturnObject = ReportEstablishContextResultReturnObject;
}
impl Method for ReportReleaseContextResult {
    const NAME: &'static str = "SmartCardEmulation.reportReleaseContextResult";
    type ReturnObject = ReportReleaseContextResultReturnObject;
}
impl Method for ReportListReadersResult {
    const NAME: &'static str = "SmartCardEmulation.reportListReadersResult";
    type ReturnObject = ReportListReadersResultReturnObject;
}
impl Method for ReportGetStatusChangeResult {
    const NAME: &'static str = "SmartCardEmulation.reportGetStatusChangeResult";
    type ReturnObject = ReportGetStatusChangeResultReturnObject;
}
impl Method for ReportBeginTransactionResult {
    const NAME: &'static str = "SmartCardEmulation.reportBeginTransactionResult";
    type ReturnObject = ReportBeginTransactionResultReturnObject;
}
impl Method for ReportPlainResult {
    const NAME: &'static str = "SmartCardEmulation.reportPlainResult";
    type ReturnObject = ReportPlainResultReturnObject;
}
impl Method for ReportConnectResult {
    const NAME: &'static str = "SmartCardEmulation.reportConnectResult";
    type ReturnObject = ReportConnectResultReturnObject;
}
impl Method for ReportDataResult {
    const NAME: &'static str = "SmartCardEmulation.reportDataResult";
    type ReturnObject = ReportDataResultReturnObject;
}
impl Method for ReportStatusResult {
    const NAME: &'static str = "SmartCardEmulation.reportStatusResult";
    type ReturnObject = ReportStatusResultReturnObject;
}
impl Method for ReportError {
    const NAME: &'static str = "SmartCardEmulation.reportError";
    type ReturnObject = ReportErrorReturnObject;
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
    pub struct EstablishContextRequestedEvent {
        pub params: EstablishContextRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct EstablishContextRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ReleaseContextRequestedEvent {
        pub params: ReleaseContextRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ReleaseContextRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub context_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ListReadersRequestedEvent {
        pub params: ListReadersRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ListReadersRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub context_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct GetStatusChangeRequestedEvent {
        pub params: GetStatusChangeRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct GetStatusChangeRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub context_id: JsUInt,
        pub reader_states: Vec<super::ReaderStateIn>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "in milliseconds, if absent, it means \"infinite\""]
        pub timeout: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CancelRequestedEvent {
        pub params: CancelRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct CancelRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub context_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ConnectRequestedEvent {
        pub params: ConnectRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ConnectRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub context_id: JsUInt,
        #[serde(default)]
        pub reader: String,
        pub share_mode: super::ShareMode,
        pub preferred_protocols: super::ProtocolSet,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct DisconnectRequestedEvent {
        pub params: DisconnectRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct DisconnectRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        pub disposition: super::Disposition,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TransmitRequestedEvent {
        pub params: TransmitRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct TransmitRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        #[serde(default)]
        pub data: u8,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub protocol: Option<super::Protocol>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct ControlRequestedEvent {
        pub params: ControlRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct ControlRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        #[serde(default)]
        pub control_code: JsUInt,
        #[serde(default)]
        pub data: u8,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct GetAttribRequestedEvent {
        pub params: GetAttribRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct GetAttribRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        #[serde(default)]
        pub attrib_id: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SetAttribRequestedEvent {
        pub params: SetAttribRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct SetAttribRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        #[serde(default)]
        pub attrib_id: JsUInt,
        #[serde(default)]
        pub data: u8,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StatusRequestedEvent {
        pub params: StatusRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct StatusRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BeginTransactionRequestedEvent {
        pub params: BeginTransactionRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct BeginTransactionRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct EndTransactionRequestedEvent {
        pub params: EndTransactionRequestedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct EndTransactionRequestedEventParams {
        #[serde(default)]
        pub request_id: String,
        #[serde(default)]
        pub handle: JsUInt,
        pub disposition: super::Disposition,
    }
}
