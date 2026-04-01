// Auto-generated from Chrome at version 146.0.7680.165 domain: Fetch
use super::io;
use super::network;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct RequestPattern {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Wildcards (`'*'` -> zero or more, `'?'` -> exactly one) are allowed. Escape character is\n backslash. Omitting is equivalent to `\"*\"`."]
    pub url_pattern: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, only requests for matching resource types will be intercepted."]
    pub resource_type: Option<network::ResourceType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Stage at which to begin intercepting requests. Default is Request."]
    pub request_stage: Option<RequestStage>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Response HTTP header entry"]
pub struct HeaderEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Authorization challenge for HTTP status code 401 or 407."]
pub struct AuthChallenge {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Source of the authentication challenge."]
    pub source: Option<AuthChallengeSource>,
    #[serde(default)]
    #[doc = "Origin of the challenger."]
    pub origin: String,
    #[serde(default)]
    #[doc = "The authentication scheme used, such as basic or digest"]
    pub scheme: String,
    #[serde(default)]
    #[doc = "The realm of the challenge. May be empty."]
    pub realm: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Response to an AuthChallenge."]
pub struct AuthChallengeResponse {
    #[doc = "The decision on what to do in response to the authorization challenge.  Default means\n deferring to the default behavior of the net stack, which will likely either the Cancel\n authentication or display a popup dialog box."]
    pub response: AuthChallengeResponseResponse,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The username to provide, possibly empty. Should only be set if response is\n ProvideCredentials."]
    pub username: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The password to provide, possibly empty. Should only be set if response is\n ProvideCredentials."]
    pub password: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Disable(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables issuing of requestPaused events. A request will be paused until client\n calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth."]
pub struct Enable {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If specified, only requests matching any of these patterns will produce\n fetchRequested event and will be paused until clients response. If not set,\n all requests will be affected."]
    pub patterns: Option<Vec<RequestPattern>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If true, authRequired events will be issued and requests will be paused\n expecting a call to continueWithAuth."]
    pub handle_auth_requests: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Causes the request to fail with specified reason."]
pub struct FailRequest {
    #[doc = "An id the client received in requestPaused event."]
    pub request_id: RequestId,
    #[doc = "Causes the request to fail with the given reason."]
    pub error_reason: network::ErrorReason,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Provides response to the request."]
pub struct FulfillRequest {
    #[doc = "An id the client received in requestPaused event."]
    pub request_id: RequestId,
    #[serde(default)]
    #[doc = "An HTTP response code."]
    pub response_code: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Response headers."]
    pub response_headers: Option<Vec<HeaderEntry>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Alternative way of specifying response headers as a \\0-separated\n series of name: value pairs. Prefer the above method unless you\n need to represent some non-UTF8 values that can't be transmitted\n over the protocol as text."]
    pub binary_response_headers: Option<Vec<u8>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "A response body. If absent, original response body will be used if\n the request is intercepted at the response stage and empty body\n will be used if the request is intercepted at the request stage."]
    pub body: Option<Vec<u8>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A textual representation of responseCode.\n If absent, a standard phrase matching responseCode is used."]
    pub response_phrase: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Continues the request, optionally modifying some of its parameters."]
pub struct ContinueRequest {
    #[doc = "An id the client received in requestPaused event."]
    pub request_id: RequestId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, the request url will be modified in a way that's not observable by page."]
    pub url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, the request method is overridden."]
    pub method: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, overrides the post data in the request."]
    pub post_data: Option<Vec<u8>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "If set, overrides the request headers. Note that the overrides do not\n extend to subsequent redirect hops, if a redirect happens. Another override\n may be applied to a different request produced by a redirect."]
    pub headers: Option<Vec<HeaderEntry>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If set, overrides response interception behavior for this request."]
    pub intercept_response: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Continues a request supplying authChallengeResponse following authRequired event."]
pub struct ContinueWithAuth {
    #[doc = "An id the client received in authRequired event."]
    pub request_id: RequestId,
    #[doc = "Response to  with an authChallenge."]
    pub auth_challenge_response: AuthChallengeResponse,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Continues loading of the paused response, optionally modifying the\n response headers. If either responseCode or headers are modified, all of them\n must be present."]
pub struct ContinueResponse {
    #[doc = "An id the client received in requestPaused event."]
    pub request_id: RequestId,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "An HTTP response code. If absent, original response code will be used."]
    pub response_code: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A textual representation of responseCode.\n If absent, a standard phrase matching responseCode is used."]
    pub response_phrase: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Response headers. If absent, original response headers will be used."]
    pub response_headers: Option<Vec<HeaderEntry>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Alternative way of specifying response headers as a \\0-separated\n series of name: value pairs. Prefer the above method unless you\n need to represent some non-UTF8 values that can't be transmitted\n over the protocol as text."]
    pub binary_response_headers: Option<Vec<u8>>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Causes the body of the response to be received from the server and\n returned as a single string. May only be issued for a request that\n is paused in the Response stage and is mutually exclusive with\n takeResponseBodyForInterceptionAsStream. Calling other methods that\n affect the request or disabling fetch domain before body is received\n results in an undefined behavior.\n Note that the response body is not available for redirects. Requests\n paused in the _redirect received_ state may be differentiated by\n `responseCode` and presence of `location` response header, see\n comments to `requestPaused` for details."]
pub struct GetResponseBody {
    #[doc = "Identifier for the intercepted request to get body for."]
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a handle to the stream representing the response body.\n The request must be paused in the HeadersReceived stage.\n Note that after this command the request can't be continued\n as is -- client either needs to cancel it or to provide the\n response body.\n The stream only supports sequential read, IO.read will fail if the position\n is specified.\n This method is mutually exclusive with getResponseBody.\n Calling other methods that affect the request or disabling fetch\n domain before body is received results in an undefined behavior."]
pub struct TakeResponseBodyAsStream {
    pub request_id: RequestId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Disables the fetch domain."]
pub struct DisableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables issuing of requestPaused events. A request will be paused until client\n calls one of failRequest, fulfillRequest or continueRequest/continueWithAuth."]
pub struct EnableReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Causes the request to fail with specified reason."]
pub struct FailRequestReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Provides response to the request."]
pub struct FulfillRequestReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Continues the request, optionally modifying some of its parameters."]
pub struct ContinueRequestReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Continues a request supplying authChallengeResponse following authRequired event."]
pub struct ContinueWithAuthReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Continues loading of the paused response, optionally modifying the\n response headers. If either responseCode or headers are modified, all of them\n must be present."]
pub struct ContinueResponseReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Causes the body of the response to be received from the server and\n returned as a single string. May only be issued for a request that\n is paused in the Response stage and is mutually exclusive with\n takeResponseBodyForInterceptionAsStream. Calling other methods that\n affect the request or disabling fetch domain before body is received\n results in an undefined behavior.\n Note that the response body is not available for redirects. Requests\n paused in the _redirect received_ state may be differentiated by\n `responseCode` and presence of `location` response header, see\n comments to `requestPaused` for details."]
pub struct GetResponseBodyReturnObject {
    #[serde(default)]
    #[doc = "Response body."]
    pub body: String,
    #[serde(default)]
    #[doc = "True, if content was sent as base64."]
    pub base_64_encoded: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a handle to the stream representing the response body.\n The request must be paused in the HeadersReceived stage.\n Note that after this command the request can't be continued\n as is -- client either needs to cancel it or to provide the\n response body.\n The stream only supports sequential read, IO.read will fail if the position\n is specified.\n This method is mutually exclusive with getResponseBody.\n Calling other methods that affect the request or disabling fetch\n domain before body is received results in an undefined behavior."]
pub struct TakeResponseBodyAsStreamReturnObject {
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
    use derive_builder::Builder;
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use serde_json::Value as Json;
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct RequestPausedEvent {
        pub params: RequestPausedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestPausedEventParams {
        #[doc = "Each request the page makes will have a unique id."]
        pub request_id: super::RequestId,
        #[doc = "The details of the request."]
        pub request: super::super::network::Request,
        #[doc = "The id of the frame that initiated the request."]
        pub frame_id: super::super::page::FrameId,
        #[doc = "How the requested resource will be used."]
        pub resource_type: super::super::network::ResourceType,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Response error if intercepted at response stage."]
        pub response_error_reason: Option<super::super::network::ErrorReason>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Response code if intercepted at response stage."]
        pub response_status_code: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Response status text if intercepted at response stage."]
        pub response_status_text: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Response headers if intercepted at the response stage."]
        pub response_headers: Option<Vec<super::HeaderEntry>>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "If the intercepted request had a corresponding Network.requestWillBeSent event fired for it,\n then this networkId will be the same as the requestId present in the requestWillBeSent event."]
        pub network_id: Option<super::super::network::RequestId>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "If the request is due to a redirect response from the server, the id of the request that\n has caused the redirect."]
        pub redirected_request_id: Option<super::RequestId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AuthRequiredEvent {
        pub params: AuthRequiredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthRequiredEventParams {
        #[doc = "Each request the page makes will have a unique id."]
        pub request_id: super::RequestId,
        #[doc = "The details of the request."]
        pub request: super::super::network::Request,
        #[doc = "The id of the frame that initiated the request."]
        pub frame_id: super::super::page::FrameId,
        #[doc = "How the requested resource will be used."]
        pub resource_type: super::super::network::ResourceType,
        #[doc = "Details of the Authorization Challenge encountered.\n If this is set, client should respond with continueRequest that\n contains AuthChallengeResponse."]
        pub auth_challenge: super::AuthChallenge,
    }
}
