// Auto-generated from Chrome at version 143.0.7499.110 domain: Fetch
use super::io;
use super::network;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type RequestId = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RequestStage {
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "Response")]
    Response,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthChallengeSource {
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "Proxy")]
    Proxy,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthChallengeResponseResponse {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "CancelAuth")]
    CancelAuth,
    #[serde(rename = "ProvideCredentials")]
    ProvideCredentials,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RequestPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urlPattern")]
    pub url_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType")]
    pub resource_type: Option<network::ResourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requestStage")]
    pub request_stage: Option<RequestStage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HeaderEntry {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthChallenge {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<AuthChallengeSource>,
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[serde(default)]
    #[serde(rename = "realm")]
    pub realm: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthChallengeResponse {
    #[serde(rename = "response")]
    pub response: AuthChallengeResponseResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "password")]
    pub password: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Disable(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Enable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "patterns")]
    pub patterns: Option<Vec<RequestPattern>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "handleAuthRequests")]
    pub handle_auth_requests: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FailRequest {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(rename = "errorReason")]
    pub error_reason: network::ErrorReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FulfillRequest {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(default)]
    #[serde(rename = "responseCode")]
    pub response_code: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseHeaders")]
    pub response_headers: Option<Vec<HeaderEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "binaryResponseHeaders")]
    pub binary_response_headers: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "body")]
    pub body: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "responsePhrase")]
    pub response_phrase: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContinueRequest {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "postData")]
    pub post_data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headers")]
    pub headers: Option<Vec<HeaderEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "interceptResponse")]
    pub intercept_response: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContinueWithAuth {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(rename = "authChallengeResponse")]
    pub auth_challenge_response: AuthChallengeResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContinueResponse {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "responseCode")]
    pub response_code: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "responsePhrase")]
    pub response_phrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "responseHeaders")]
    pub response_headers: Option<Vec<HeaderEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "binaryResponseHeaders")]
    pub binary_response_headers: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBody {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeResponseBodyAsStream {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DisableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FailRequestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FulfillRequestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContinueRequestReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContinueWithAuthReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponseReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetResponseBodyReturnObject {
    #[serde(default)]
    #[serde(rename = "body")]
    pub body: String,
    #[serde(default)]
    #[serde(rename = "base64Encoded")]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TakeResponseBodyAsStreamReturnObject {
    #[serde(rename = "stream")]
    pub stream: io::StreamHandle,
}
impl Method for Disable {
    const NAME: &'static str = "Fetch.disable";
    type ReturnObject = DisableReturnObject;
}
impl Method for Enable {
    const NAME: &'static str = "Fetch.enable";
    type ReturnObject = EnableReturnObject;
}
impl Method for FailRequest {
    const NAME: &'static str = "Fetch.failRequest";
    type ReturnObject = FailRequestReturnObject;
}
impl Method for FulfillRequest {
    const NAME: &'static str = "Fetch.fulfillRequest";
    type ReturnObject = FulfillRequestReturnObject;
}
impl Method for ContinueRequest {
    const NAME: &'static str = "Fetch.continueRequest";
    type ReturnObject = ContinueRequestReturnObject;
}
impl Method for ContinueWithAuth {
    const NAME: &'static str = "Fetch.continueWithAuth";
    type ReturnObject = ContinueWithAuthReturnObject;
}
impl Method for ContinueResponse {
    const NAME: &'static str = "Fetch.continueResponse";
    type ReturnObject = ContinueResponseReturnObject;
}
impl Method for GetResponseBody {
    const NAME: &'static str = "Fetch.getResponseBody";
    type ReturnObject = GetResponseBodyReturnObject;
}
impl Method for TakeResponseBodyAsStream {
    const NAME: &'static str = "Fetch.takeResponseBodyAsStream";
    type ReturnObject = TakeResponseBodyAsStreamReturnObject;
}
pub mod events {
    #[allow(unused_imports)]
    use super::super::types::*;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestPausedEvent {
        pub params: RequestPausedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestPausedEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "request")]
        pub request: super::super::network::Request,
        #[serde(rename = "frameId")]
        pub frame_id: super::super::page::FrameId,
        #[serde(rename = "resourceType")]
        pub resource_type: super::super::network::ResourceType,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "responseErrorReason")]
        pub response_error_reason: Option<super::super::network::ErrorReason>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "responseStatusCode")]
        pub response_status_code: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "responseStatusText")]
        pub response_status_text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "responseHeaders")]
        pub response_headers: Option<Vec<super::HeaderEntry>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "networkId")]
        pub network_id: Option<super::super::network::RequestId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "redirectedRequestId")]
        pub redirected_request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AuthRequiredEvent {
        pub params: AuthRequiredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AuthRequiredEventParams {
        #[serde(rename = "requestId")]
        pub request_id: super::RequestId,
        #[serde(rename = "request")]
        pub request: super::super::network::Request,
        #[serde(rename = "frameId")]
        pub frame_id: super::super::page::FrameId,
        #[serde(rename = "resourceType")]
        pub resource_type: super::super::network::ResourceType,
        #[serde(rename = "authChallenge")]
        pub auth_challenge: super::AuthChallenge,
    }
}
