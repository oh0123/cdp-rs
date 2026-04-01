// Auto-generated from Chrome at version 146.0.7680.165 domain: Storage
use super::browser;
use super::network;
use super::page;
use super::target;
#[allow(unused_imports)]
use super::types::*;
#[allow(unused_imports)]
use derive_builder::Builder;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value as Json;
pub type SerializedStorageKey = String;
pub type InterestGroupAuctionId = String;
pub type UnsignedInt64AsBase10 = String;
pub type UnsignedInt128AsBase16 = String;
pub type SignedInt64AsBase10 = String;
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StorageType {
    #[serde(rename = "cookies")]
    Cookies,
    #[serde(rename = "file_systems")]
    FileSystems,
    #[serde(rename = "indexeddb")]
    Indexeddb,
    #[serde(rename = "local_storage")]
    LocalStorage,
    #[serde(rename = "shader_cache")]
    ShaderCache,
    #[serde(rename = "websql")]
    Websql,
    #[serde(rename = "service_workers")]
    ServiceWorkers,
    #[serde(rename = "cache_storage")]
    CacheStorage,
    #[serde(rename = "interest_groups")]
    InterestGroups,
    #[serde(rename = "shared_storage")]
    SharedStorage,
    #[serde(rename = "storage_buckets")]
    StorageBuckets,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "other")]
    Other,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InterestGroupAccessType {
    #[serde(rename = "join")]
    Join,
    #[serde(rename = "leave")]
    Leave,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "additionalBid")]
    AdditionalBid,
    #[serde(rename = "additionalBidWin")]
    AdditionalBidWin,
    #[serde(rename = "topLevelBid")]
    TopLevelBid,
    #[serde(rename = "topLevelAdditionalBid")]
    TopLevelAdditionalBid,
    #[serde(rename = "clear")]
    Clear,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InterestGroupAuctionEventType {
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "configResolved")]
    ConfigResolved,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum InterestGroupAuctionFetchType {
    #[serde(rename = "bidderJs")]
    BidderJs,
    #[serde(rename = "bidderWasm")]
    BidderWasm,
    #[serde(rename = "sellerJs")]
    SellerJs,
    #[serde(rename = "bidderTrustedSignals")]
    BidderTrustedSignals,
    #[serde(rename = "sellerTrustedSignals")]
    SellerTrustedSignals,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SharedStorageAccessScope {
    #[serde(rename = "window")]
    Window,
    #[serde(rename = "sharedStorageWorklet")]
    SharedStorageWorklet,
    #[serde(rename = "protectedAudienceWorklet")]
    ProtectedAudienceWorklet,
    #[serde(rename = "header")]
    Header,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SharedStorageAccessMethod {
    #[serde(rename = "addModule")]
    AddModule,
    #[serde(rename = "createWorklet")]
    CreateWorklet,
    #[serde(rename = "selectURL")]
    SelectUrl,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "batchUpdate")]
    BatchUpdate,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "append")]
    Append,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "keys")]
    Keys,
    #[serde(rename = "values")]
    Values,
    #[serde(rename = "entries")]
    Entries,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "remainingBudget")]
    RemainingBudget,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum StorageBucketsDurability {
    #[serde(rename = "relaxed")]
    Relaxed,
    #[serde(rename = "strict")]
    Strict,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingSourceType {
    #[serde(rename = "navigation")]
    Navigation,
    #[serde(rename = "event")]
    Event,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingTriggerDataMatching {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "modulus")]
    Modulus,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingSourceRegistrationResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "insufficientSourceCapacity")]
    InsufficientSourceCapacity,
    #[serde(rename = "insufficientUniqueDestinationCapacity")]
    InsufficientUniqueDestinationCapacity,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "successNoised")]
    SuccessNoised,
    #[serde(rename = "destinationReportingLimitReached")]
    DestinationReportingLimitReached,
    #[serde(rename = "destinationGlobalLimitReached")]
    DestinationGlobalLimitReached,
    #[serde(rename = "destinationBothLimitsReached")]
    DestinationBothLimitsReached,
    #[serde(rename = "reportingOriginsPerSiteLimitReached")]
    ReportingOriginsPerSiteLimitReached,
    #[serde(rename = "exceedsMaxChannelCapacity")]
    ExceedsMaxChannelCapacity,
    #[serde(rename = "exceedsMaxScopesChannelCapacity")]
    ExceedsMaxScopesChannelCapacity,
    #[serde(rename = "exceedsMaxTriggerStateCardinality")]
    ExceedsMaxTriggerStateCardinality,
    #[serde(rename = "exceedsMaxEventStatesLimit")]
    ExceedsMaxEventStatesLimit,
    #[serde(rename = "destinationPerDayReportingLimitReached")]
    DestinationPerDayReportingLimitReached,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingSourceRegistrationTimeConfig {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingEventLevelResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "successDroppedLowerPriority")]
    SuccessDroppedLowerPriority,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "noCapacityForAttributionDestination")]
    NoCapacityForAttributionDestination,
    #[serde(rename = "noMatchingSources")]
    NoMatchingSources,
    #[serde(rename = "deduplicated")]
    Deduplicated,
    #[serde(rename = "excessiveAttributions")]
    ExcessiveAttributions,
    #[serde(rename = "priorityTooLow")]
    PriorityTooLow,
    #[serde(rename = "neverAttributedSource")]
    NeverAttributedSource,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "noMatchingSourceFilterData")]
    NoMatchingSourceFilterData,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "noMatchingConfigurations")]
    NoMatchingConfigurations,
    #[serde(rename = "excessiveReports")]
    ExcessiveReports,
    #[serde(rename = "falselyAttributedSource")]
    FalselyAttributedSource,
    #[serde(rename = "reportWindowPassed")]
    ReportWindowPassed,
    #[serde(rename = "notRegistered")]
    NotRegistered,
    #[serde(rename = "reportWindowNotStarted")]
    ReportWindowNotStarted,
    #[serde(rename = "noMatchingTriggerData")]
    NoMatchingTriggerData,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingAggregatableResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "internalError")]
    InternalError,
    #[serde(rename = "noCapacityForAttributionDestination")]
    NoCapacityForAttributionDestination,
    #[serde(rename = "noMatchingSources")]
    NoMatchingSources,
    #[serde(rename = "excessiveAttributions")]
    ExcessiveAttributions,
    #[serde(rename = "excessiveReportingOrigins")]
    ExcessiveReportingOrigins,
    #[serde(rename = "noHistograms")]
    NoHistograms,
    #[serde(rename = "insufficientBudget")]
    InsufficientBudget,
    #[serde(rename = "insufficientNamedBudget")]
    InsufficientNamedBudget,
    #[serde(rename = "noMatchingSourceFilterData")]
    NoMatchingSourceFilterData,
    #[serde(rename = "notRegistered")]
    NotRegistered,
    #[serde(rename = "prohibitedByBrowserPolicy")]
    ProhibitedByBrowserPolicy,
    #[serde(rename = "deduplicated")]
    Deduplicated,
    #[serde(rename = "reportWindowPassed")]
    ReportWindowPassed,
    #[serde(rename = "excessiveReports")]
    ExcessiveReports,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttributionReportingReportResult {
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "prohibited")]
    Prohibited,
    #[serde(rename = "failedToAssemble")]
    FailedToAssemble,
    #[serde(rename = "expired")]
    Expired,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Usage for a storage type."]
pub struct UsageForType {
    #[doc = "Name of storage type."]
    pub storage_type: StorageType,
    #[serde(default)]
    #[doc = "Storage usage (bytes)."]
    pub usage: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Pair of issuer origin and number of available (signed, but not used) Trust\n Tokens from that issuer."]
pub struct TrustTokens {
    #[serde(default)]
    pub issuer_origin: String,
    #[serde(default)]
    pub count: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Struct for a single key-value pair in an origin's shared storage."]
pub struct SharedStorageEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Details for an origin's shared storage."]
pub struct SharedStorageMetadata {
    #[doc = "Time when the origin's shared storage was last created."]
    pub creation_time: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "Number of key-value pairs stored in origin's shared storage."]
    pub length: JsUInt,
    #[serde(default)]
    #[doc = "Current amount of bits of entropy remaining in the navigation budget."]
    pub remaining_budget: JsFloat,
    #[serde(default)]
    #[doc = "Total number of bytes stored as key-value pairs in origin's shared\n storage."]
    pub bytes_used: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Represents a dictionary object passed in as privateAggregationConfig to\n run or selectURL."]
pub struct SharedStoragePrivateAggregationConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The chosen aggregation service deployment."]
    pub aggregation_coordinator_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The context ID provided."]
    pub context_id: Option<String>,
    #[serde(default)]
    #[doc = "Configures the maximum size allowed for filtering IDs."]
    pub filtering_id_max_bytes: JsUInt,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The limit on the number of contributions in the final report."]
    pub max_contributions: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Pair of reporting metadata details for a candidate URL for `selectURL()`."]
pub struct SharedStorageReportingMetadata {
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub reporting_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Bundles a candidate URL with its reporting metadata."]
pub struct SharedStorageUrlWithMetadata {
    #[serde(default)]
    #[doc = "Spec of candidate URL."]
    pub url: String,
    #[doc = "Any associated reporting metadata."]
    pub reporting_metadata: Vec<SharedStorageReportingMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Bundles the parameters for shared storage access events whose\n presence/absence can vary according to SharedStorageAccessType."]
pub struct SharedStorageAccessParams {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Spec of the module script URL.\n Present only for SharedStorageAccessMethods: addModule and\n createWorklet."]
    pub script_source_url: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "String denoting \"context-origin\", \"script-origin\", or a custom\n origin to be used as the worklet's data origin.\n Present only for SharedStorageAccessMethod: createWorklet."]
    pub data_origin: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Name of the registered operation to be run.\n Present only for SharedStorageAccessMethods: run and selectURL."]
    pub operation_name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "ID of the operation call.\n Present only for SharedStorageAccessMethods: run and selectURL."]
    pub operation_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to keep the worket alive for future run or selectURL\n calls.\n Present only for SharedStorageAccessMethods: run and selectURL."]
    pub keep_alive: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Configures the private aggregation options.\n Present only for SharedStorageAccessMethods: run and selectURL."]
    pub private_aggregation_config: Option<SharedStoragePrivateAggregationConfig>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The operation's serialized data in bytes (converted to a string).\n Present only for SharedStorageAccessMethods: run and selectURL.\n TODO(crbug.com/401011862): Consider updating this parameter to binary."]
    pub serialized_data: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Array of candidate URLs' specs, along with any associated metadata.\n Present only for SharedStorageAccessMethod: selectURL."]
    pub urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Spec of the URN:UUID generated for a selectURL call.\n Present only for SharedStorageAccessMethod: selectURL."]
    pub urn_uuid: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Key for a specific entry in an origin's shared storage.\n Present only for SharedStorageAccessMethods: set, append, delete, and\n get."]
    pub key: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Value for a specific entry in an origin's shared storage.\n Present only for SharedStorageAccessMethods: set and append."]
    pub value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Whether or not to set an entry for a key if that key is already present.\n Present only for SharedStorageAccessMethod: set."]
    pub ignore_if_present: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "A number denoting the (0-based) order of the worklet's\n creation relative to all other shared storage worklets created by\n documents using the current storage partition.\n Present only for SharedStorageAccessMethods: addModule, createWorklet."]
    pub worklet_ordinal: Option<JsUInt>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Hex representation of the DevTools token used as the TargetID for the\n associated shared storage worklet.\n Present only for SharedStorageAccessMethods: addModule, createWorklet,\n run, selectURL, and any other SharedStorageAccessMethod when the\n SharedStorageAccessScope is sharedStorageWorklet."]
    pub worklet_target_id: Option<target::TargetId>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Name of the lock to be acquired, if present.\n Optionally present only for SharedStorageAccessMethods: batchUpdate,\n set, append, delete, and clear."]
    pub with_lock: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If the method has been called as part of a batchUpdate, then this\n number identifies the batch to which it belongs.\n Optionally present only for SharedStorageAccessMethods:\n batchUpdate (required), set, append, delete, and clear."]
    pub batch_update_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "Number of modifier methods sent in batch.\n Present only for SharedStorageAccessMethod: batchUpdate."]
    pub batch_size: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StorageBucket {
    pub storage_key: SerializedStorageKey,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If not specified, it is the default bucket of the storageKey."]
    pub name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct StorageBucketInfo {
    pub bucket: StorageBucket,
    #[serde(default)]
    pub id: String,
    pub expiration: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "Storage quota (bytes)."]
    pub quota: JsFloat,
    #[serde(default)]
    pub persistent: bool,
    pub durability: StorageBucketsDurability,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingFilterDataEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub values: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingFilterConfig {
    pub filter_values: Vec<AttributionReportingFilterDataEntry>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "duration in seconds"]
    pub lookback_window: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingFilterPair {
    pub filters: Vec<AttributionReportingFilterConfig>,
    pub not_filters: Vec<AttributionReportingFilterConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregationKeysEntry {
    #[serde(default)]
    pub key: String,
    pub value: UnsignedInt128AsBase16,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingEventReportWindows {
    #[serde(default)]
    #[doc = "duration in seconds"]
    pub start: JsUInt,
    #[serde(default)]
    #[doc = "duration in seconds"]
    pub ends: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableDebugReportingData {
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(default)]
    #[doc = "number instead of integer because not all uint32 can be represented by\n int"]
    pub value: JsFloat,
    #[serde(default)]
    pub types: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableDebugReportingConfig {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "number instead of integer because not all uint32 can be represented by\n int, only present for source registrations"]
    pub budget: Option<JsFloat>,
    pub key_piece: UnsignedInt128AsBase16,
    pub debug_data: Vec<AttributionReportingAggregatableDebugReportingData>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aggregation_coordinator_origin: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionScopesData {
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    #[doc = "number instead of integer because not all uint32 can be represented by\n int"]
    pub limit: JsFloat,
    #[serde(default)]
    pub max_event_states: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingNamedBudgetDef {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub budget: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingSourceRegistration {
    pub time: network::TimeSinceEpoch,
    #[serde(default)]
    #[doc = "duration in seconds"]
    pub expiry: JsUInt,
    #[serde(default)]
    #[doc = "number instead of integer because not all uint32 can be represented by\n int"]
    pub trigger_data: Vec<JsFloat>,
    pub event_report_windows: AttributionReportingEventReportWindows,
    #[serde(default)]
    #[doc = "duration in seconds"]
    pub aggregatable_report_window: JsUInt,
    pub r#type: AttributionReportingSourceType,
    #[serde(default)]
    pub source_origin: String,
    #[serde(default)]
    pub reporting_origin: String,
    #[serde(default)]
    pub destination_sites: Vec<String>,
    pub event_id: UnsignedInt64AsBase10,
    pub priority: SignedInt64AsBase10,
    pub filter_data: Vec<AttributionReportingFilterDataEntry>,
    pub aggregation_keys: Vec<AttributionReportingAggregationKeysEntry>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    pub trigger_data_matching: AttributionReportingTriggerDataMatching,
    pub destination_limit_priority: SignedInt64AsBase10,
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes_data: Option<AttributionScopesData>,
    #[serde(default)]
    pub max_event_level_reports: JsUInt,
    pub named_budgets: Vec<AttributionReportingNamedBudgetDef>,
    #[serde(default)]
    pub debug_reporting: bool,
    #[serde(default)]
    pub event_level_epsilon: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableValueDictEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    #[doc = "number instead of integer because not all uint32 can be represented by\n int"]
    pub value: JsFloat,
    pub filtering_id: UnsignedInt64AsBase10,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableValueEntry {
    pub values: Vec<AttributionReportingAggregatableValueDictEntry>,
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingEventTriggerData {
    pub data: UnsignedInt64AsBase10,
    pub priority: SignedInt64AsBase10,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableTriggerData {
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(default)]
    pub source_keys: Vec<String>,
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingAggregatableDedupKey {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingNamedBudgetCandidate {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct AttributionReportingTriggerRegistration {
    pub filters: AttributionReportingFilterPair,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    pub aggregatable_dedup_keys: Vec<AttributionReportingAggregatableDedupKey>,
    pub event_trigger_data: Vec<AttributionReportingEventTriggerData>,
    pub aggregatable_trigger_data: Vec<AttributionReportingAggregatableTriggerData>,
    pub aggregatable_values: Vec<AttributionReportingAggregatableValueEntry>,
    #[serde(default)]
    pub aggregatable_filtering_id_max_bytes: JsUInt,
    #[serde(default)]
    pub debug_reporting: bool,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aggregation_coordinator_origin: Option<String>,
    pub source_registration_time_config: AttributionReportingSourceRegistrationTimeConfig,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trigger_context_id: Option<String>,
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[serde(default)]
    pub scopes: Vec<String>,
    pub named_budgets: Vec<AttributionReportingNamedBudgetCandidate>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "A single Related Website Set object."]
pub struct RelatedWebsiteSet {
    #[serde(default)]
    #[doc = "The primary site of this set, along with the ccTLDs if there is any."]
    pub primary_sites: Vec<String>,
    #[serde(default)]
    #[doc = "The associated sites of this set, along with the ccTLDs if there is any."]
    pub associated_sites: Vec<String>,
    #[serde(default)]
    #[doc = "The service sites of this set, along with the ccTLDs if there is any."]
    pub service_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a storage key given a frame id.\n Deprecated. Please use Storage.getStorageKey instead."]
#[deprecated]
pub struct GetStorageKeyForFrame {
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns storage key for the given frame. If no frame ID is provided,\n the storage key of the target executing this command is returned."]
pub struct GetStorageKey {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<page::FrameId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears storage for origin."]
pub struct ClearDataForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
    #[serde(default)]
    #[doc = "Comma separated list of StorageType to clear."]
    pub storage_types: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears storage for storage key."]
pub struct ClearDataForStorageKey {
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: String,
    #[serde(default)]
    #[doc = "Comma separated list of StorageType to clear."]
    pub storage_types: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser cookies."]
pub struct GetCookies {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Browser context to use when called on the browser endpoint."]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets given cookies."]
pub struct SetCookies {
    #[doc = "Cookies to be set."]
    pub cookies: network::CookieParam,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Browser context to use when called on the browser endpoint."]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears cookies."]
pub struct ClearCookies {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[doc = "Browser context to use when called on the browser endpoint."]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns usage and quota in bytes."]
pub struct GetUsageAndQuota {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Override quota for the specified origin"]
pub struct OverrideQuotaForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "The quota size (in bytes) to override the original quota with.\n If this is called multiple times, the overridden quota will be equal to\n the quotaSize provided in the final call. If this is called without\n specifying a quotaSize, the quota will be reset to the default value for\n the specified origin. If this is called multiple times with different\n origins, the override will be maintained for each origin until it is\n disabled (called without a quotaSize)."]
    pub quota_size: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Registers origin to be notified when an update occurs to its cache storage list."]
pub struct TrackCacheStorageForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Registers storage key to be notified when an update occurs to its cache storage list."]
pub struct TrackCacheStorageForStorageKey {
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Registers origin to be notified when an update occurs to its IndexedDB."]
pub struct TrackIndexedDBForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Registers storage key to be notified when an update occurs to its IndexedDB."]
pub struct TrackIndexedDBForStorageKey {
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Unregisters origin from receiving notifications for cache storage."]
pub struct UntrackCacheStorageForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Unregisters storage key from receiving notifications for cache storage."]
pub struct UntrackCacheStorageForStorageKey {
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Unregisters origin from receiving notifications for IndexedDB."]
pub struct UntrackIndexedDBForOrigin {
    #[serde(default)]
    #[doc = "Security origin."]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Unregisters storage key from receiving notifications for IndexedDB."]
pub struct UntrackIndexedDBForStorageKey {
    #[serde(default)]
    #[doc = "Storage key."]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTrustTokens(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Removes all Trust Tokens issued by the provided issuerOrigin.\n Leaves other stored data, including the issuer's Redemption Records, intact."]
pub struct ClearTrustTokens {
    #[serde(default)]
    pub issuer_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets details for a named interest group."]
pub struct GetInterestGroupDetails {
    #[serde(default)]
    pub owner_origin: String,
    #[serde(default)]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables/Disables issuing of interestGroupAccessed events."]
pub struct SetInterestGroupTracking {
    #[serde(default)]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables/Disables issuing of interestGroupAuctionEventOccurred and\n interestGroupAuctionNetworkRequestCreated."]
pub struct SetInterestGroupAuctionTracking {
    #[serde(default)]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets metadata for an origin's shared storage."]
pub struct GetSharedStorageMetadata {
    #[serde(default)]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the entries in an given origin's shared storage."]
pub struct GetSharedStorageEntries {
    #[serde(default)]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Sets entry with `key` and `value` for a given origin's shared storage."]
pub struct SetSharedStorageEntry {
    #[serde(default)]
    pub owner_origin: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[doc = "If `ignoreIfPresent` is included and true, then only sets the entry if\n `key` doesn't already exist."]
    pub ignore_if_present: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes entry for `key` (if it exists) for a given origin's shared storage."]
pub struct DeleteSharedStorageEntry {
    #[serde(default)]
    pub owner_origin: String,
    #[serde(default)]
    pub key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Clears all entries for a given origin's shared storage."]
pub struct ClearSharedStorageEntries {
    #[serde(default)]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Resets the budget for `ownerOrigin` by clearing all budget withdrawals."]
pub struct ResetSharedStorageBudget {
    #[serde(default)]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables/disables issuing of sharedStorageAccessed events."]
pub struct SetSharedStorageTracking {
    #[serde(default)]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Set tracking for a storage key's buckets."]
pub struct SetStorageBucketTracking {
    #[serde(default)]
    pub storage_key: String,
    #[serde(default)]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes the Storage Bucket with the given storage key and bucket name."]
pub struct DeleteStorageBucket {
    pub bucket: StorageBucket,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RunBounceTrackingMitigations(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "https://wicg.github.io/attribution-reporting-api/"]
pub struct SetAttributionReportingLocalTestingMode {
    #[serde(default)]
    #[doc = "If enabled, noise is suppressed and reports are sent immediately."]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Enables/disables issuing of Attribution Reporting events."]
pub struct SetAttributionReportingTracking {
    #[serde(default)]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SendPendingAttributionReports(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRelatedWebsiteSets(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the list of URLs from a page and its embedded resources that match\n existing grace period URL pattern rules.\n https://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period"]
pub struct GetAffectedUrlsForThirdPartyCookieMetadata {
    #[serde(default)]
    #[doc = "The URL of the page currently being visited."]
    pub first_party_url: String,
    #[serde(default)]
    #[doc = "The list of embedded resource URLs from the page."]
    pub third_party_urls: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[builder(setter(into, strip_option))]
#[serde(rename_all = "camelCase")]
pub struct SetProtectedAudienceKAnonymity {
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub hashes: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns a storage key given a frame id.\n Deprecated. Please use Storage.getStorageKey instead."]
#[deprecated]
pub struct GetStorageKeyForFrameReturnObject {
    pub storage_key: SerializedStorageKey,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns storage key for the given frame. If no frame ID is provided,\n the storage key of the target executing this command is returned."]
pub struct GetStorageKeyReturnObject {
    pub storage_key: SerializedStorageKey,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears storage for origin."]
pub struct ClearDataForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears storage for storage key."]
pub struct ClearDataForStorageKeyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns all browser cookies."]
pub struct GetCookiesReturnObject {
    #[doc = "Array of cookie objects."]
    pub cookies: network::Cookie,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets given cookies."]
pub struct SetCookiesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears cookies."]
pub struct ClearCookiesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns usage and quota in bytes."]
pub struct GetUsageAndQuotaReturnObject {
    #[serde(default)]
    #[doc = "Storage usage (bytes)."]
    pub usage: JsFloat,
    #[serde(default)]
    #[doc = "Storage quota (bytes)."]
    pub quota: JsFloat,
    #[serde(default)]
    #[doc = "Whether or not the origin has an active storage quota override"]
    pub override_active: bool,
    #[doc = "Storage usage per type (bytes)."]
    pub usage_breakdown: Vec<UsageForType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Override quota for the specified origin"]
pub struct OverrideQuotaForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Registers origin to be notified when an update occurs to its cache storage list."]
pub struct TrackCacheStorageForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Registers storage key to be notified when an update occurs to its cache storage list."]
pub struct TrackCacheStorageForStorageKeyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Registers origin to be notified when an update occurs to its IndexedDB."]
pub struct TrackIndexedDBForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Registers storage key to be notified when an update occurs to its IndexedDB."]
pub struct TrackIndexedDBForStorageKeyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Unregisters origin from receiving notifications for cache storage."]
pub struct UntrackCacheStorageForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Unregisters storage key from receiving notifications for cache storage."]
pub struct UntrackCacheStorageForStorageKeyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Unregisters origin from receiving notifications for IndexedDB."]
pub struct UntrackIndexedDBForOriginReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Unregisters storage key from receiving notifications for IndexedDB."]
pub struct UntrackIndexedDBForStorageKeyReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the number of stored Trust Tokens per issuer for the\n current browsing context."]
pub struct GetTrustTokensReturnObject {
    pub tokens: Vec<TrustTokens>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Removes all Trust Tokens issued by the provided issuerOrigin.\n Leaves other stored data, including the issuer's Redemption Records, intact."]
pub struct ClearTrustTokensReturnObject {
    #[serde(default)]
    #[doc = "True if any tokens were deleted, false otherwise."]
    pub did_delete_tokens: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets details for a named interest group."]
pub struct GetInterestGroupDetailsReturnObject {
    #[serde(default)]
    #[doc = "This largely corresponds to:\n https://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup\n but has absolute expirationTime instead of relative lifetimeMs and\n also adds joiningOrigin."]
    pub details: Json,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables/Disables issuing of interestGroupAccessed events."]
pub struct SetInterestGroupTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables/Disables issuing of interestGroupAuctionEventOccurred and\n interestGroupAuctionNetworkRequestCreated."]
pub struct SetInterestGroupAuctionTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets metadata for an origin's shared storage."]
pub struct GetSharedStorageMetadataReturnObject {
    pub metadata: SharedStorageMetadata,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Gets the entries in an given origin's shared storage."]
pub struct GetSharedStorageEntriesReturnObject {
    pub entries: Vec<SharedStorageEntry>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Sets entry with `key` and `value` for a given origin's shared storage."]
pub struct SetSharedStorageEntryReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes entry for `key` (if it exists) for a given origin's shared storage."]
pub struct DeleteSharedStorageEntryReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Clears all entries for a given origin's shared storage."]
pub struct ClearSharedStorageEntriesReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Resets the budget for `ownerOrigin` by clearing all budget withdrawals."]
pub struct ResetSharedStorageBudgetReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables/disables issuing of sharedStorageAccessed events."]
pub struct SetSharedStorageTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Set tracking for a storage key's buckets."]
pub struct SetStorageBucketTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Deletes the Storage Bucket with the given storage key and bucket name."]
pub struct DeleteStorageBucketReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Deletes state for sites identified as potential bounce trackers, immediately."]
pub struct RunBounceTrackingMitigationsReturnObject {
    pub deleted_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "https://wicg.github.io/attribution-reporting-api/"]
pub struct SetAttributionReportingLocalTestingModeReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[doc = "Enables/disables issuing of Attribution Reporting events."]
pub struct SetAttributionReportingTrackingReturnObject(pub Option<Json>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Sends all pending Attribution Reports immediately, regardless of their\n scheduled report time."]
pub struct SendPendingAttributionReportsReturnObject {
    #[serde(default)]
    #[doc = "The number of reports that were sent."]
    pub num_sent: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the effective Related Website Sets in use by this profile for the browser\n session. The effective Related Website Sets will not change during a browser session."]
pub struct GetRelatedWebsiteSetsReturnObject {
    pub sets: Vec<RelatedWebsiteSet>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
#[doc = "Returns the list of URLs from a page and its embedded resources that match\n existing grace period URL pattern rules.\n https://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period"]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataReturnObject {
    #[doc = "Array of matching URLs. If there is a primary pattern match for the first-\n party URL, only the first-party URL is returned in the array."]
    pub matched_urls: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetProtectedAudienceKAnonymityReturnObject(pub Option<Json>);
impl Method for GetStorageKeyForFrame {
    const NAME: &'static str = "Storage.getStorageKeyForFrame";
    type ReturnObject = GetStorageKeyForFrameReturnObject;
}
impl Method for GetStorageKey {
    const NAME: &'static str = "Storage.getStorageKey";
    type ReturnObject = GetStorageKeyReturnObject;
}
impl Method for ClearDataForOrigin {
    const NAME: &'static str = "Storage.clearDataForOrigin";
    type ReturnObject = ClearDataForOriginReturnObject;
}
impl Method for ClearDataForStorageKey {
    const NAME: &'static str = "Storage.clearDataForStorageKey";
    type ReturnObject = ClearDataForStorageKeyReturnObject;
}
impl Method for GetCookies {
    const NAME: &'static str = "Storage.getCookies";
    type ReturnObject = GetCookiesReturnObject;
}
impl Method for SetCookies {
    const NAME: &'static str = "Storage.setCookies";
    type ReturnObject = SetCookiesReturnObject;
}
impl Method for ClearCookies {
    const NAME: &'static str = "Storage.clearCookies";
    type ReturnObject = ClearCookiesReturnObject;
}
impl Method for GetUsageAndQuota {
    const NAME: &'static str = "Storage.getUsageAndQuota";
    type ReturnObject = GetUsageAndQuotaReturnObject;
}
impl Method for OverrideQuotaForOrigin {
    const NAME: &'static str = "Storage.overrideQuotaForOrigin";
    type ReturnObject = OverrideQuotaForOriginReturnObject;
}
impl Method for TrackCacheStorageForOrigin {
    const NAME: &'static str = "Storage.trackCacheStorageForOrigin";
    type ReturnObject = TrackCacheStorageForOriginReturnObject;
}
impl Method for TrackCacheStorageForStorageKey {
    const NAME: &'static str = "Storage.trackCacheStorageForStorageKey";
    type ReturnObject = TrackCacheStorageForStorageKeyReturnObject;
}
impl Method for TrackIndexedDBForOrigin {
    const NAME: &'static str = "Storage.trackIndexedDBForOrigin";
    type ReturnObject = TrackIndexedDBForOriginReturnObject;
}
impl Method for TrackIndexedDBForStorageKey {
    const NAME: &'static str = "Storage.trackIndexedDBForStorageKey";
    type ReturnObject = TrackIndexedDBForStorageKeyReturnObject;
}
impl Method for UntrackCacheStorageForOrigin {
    const NAME: &'static str = "Storage.untrackCacheStorageForOrigin";
    type ReturnObject = UntrackCacheStorageForOriginReturnObject;
}
impl Method for UntrackCacheStorageForStorageKey {
    const NAME: &'static str = "Storage.untrackCacheStorageForStorageKey";
    type ReturnObject = UntrackCacheStorageForStorageKeyReturnObject;
}
impl Method for UntrackIndexedDBForOrigin {
    const NAME: &'static str = "Storage.untrackIndexedDBForOrigin";
    type ReturnObject = UntrackIndexedDBForOriginReturnObject;
}
impl Method for UntrackIndexedDBForStorageKey {
    const NAME: &'static str = "Storage.untrackIndexedDBForStorageKey";
    type ReturnObject = UntrackIndexedDBForStorageKeyReturnObject;
}
impl Method for GetTrustTokens {
    const NAME: &'static str = "Storage.getTrustTokens";
    type ReturnObject = GetTrustTokensReturnObject;
}
impl Method for ClearTrustTokens {
    const NAME: &'static str = "Storage.clearTrustTokens";
    type ReturnObject = ClearTrustTokensReturnObject;
}
impl Method for GetInterestGroupDetails {
    const NAME: &'static str = "Storage.getInterestGroupDetails";
    type ReturnObject = GetInterestGroupDetailsReturnObject;
}
impl Method for SetInterestGroupTracking {
    const NAME: &'static str = "Storage.setInterestGroupTracking";
    type ReturnObject = SetInterestGroupTrackingReturnObject;
}
impl Method for SetInterestGroupAuctionTracking {
    const NAME: &'static str = "Storage.setInterestGroupAuctionTracking";
    type ReturnObject = SetInterestGroupAuctionTrackingReturnObject;
}
impl Method for GetSharedStorageMetadata {
    const NAME: &'static str = "Storage.getSharedStorageMetadata";
    type ReturnObject = GetSharedStorageMetadataReturnObject;
}
impl Method for GetSharedStorageEntries {
    const NAME: &'static str = "Storage.getSharedStorageEntries";
    type ReturnObject = GetSharedStorageEntriesReturnObject;
}
impl Method for SetSharedStorageEntry {
    const NAME: &'static str = "Storage.setSharedStorageEntry";
    type ReturnObject = SetSharedStorageEntryReturnObject;
}
impl Method for DeleteSharedStorageEntry {
    const NAME: &'static str = "Storage.deleteSharedStorageEntry";
    type ReturnObject = DeleteSharedStorageEntryReturnObject;
}
impl Method for ClearSharedStorageEntries {
    const NAME: &'static str = "Storage.clearSharedStorageEntries";
    type ReturnObject = ClearSharedStorageEntriesReturnObject;
}
impl Method for ResetSharedStorageBudget {
    const NAME: &'static str = "Storage.resetSharedStorageBudget";
    type ReturnObject = ResetSharedStorageBudgetReturnObject;
}
impl Method for SetSharedStorageTracking {
    const NAME: &'static str = "Storage.setSharedStorageTracking";
    type ReturnObject = SetSharedStorageTrackingReturnObject;
}
impl Method for SetStorageBucketTracking {
    const NAME: &'static str = "Storage.setStorageBucketTracking";
    type ReturnObject = SetStorageBucketTrackingReturnObject;
}
impl Method for DeleteStorageBucket {
    const NAME: &'static str = "Storage.deleteStorageBucket";
    type ReturnObject = DeleteStorageBucketReturnObject;
}
impl Method for RunBounceTrackingMitigations {
    const NAME: &'static str = "Storage.runBounceTrackingMitigations";
    type ReturnObject = RunBounceTrackingMitigationsReturnObject;
}
impl Method for SetAttributionReportingLocalTestingMode {
    const NAME: &'static str = "Storage.setAttributionReportingLocalTestingMode";
    type ReturnObject = SetAttributionReportingLocalTestingModeReturnObject;
}
impl Method for SetAttributionReportingTracking {
    const NAME: &'static str = "Storage.setAttributionReportingTracking";
    type ReturnObject = SetAttributionReportingTrackingReturnObject;
}
impl Method for SendPendingAttributionReports {
    const NAME: &'static str = "Storage.sendPendingAttributionReports";
    type ReturnObject = SendPendingAttributionReportsReturnObject;
}
impl Method for GetRelatedWebsiteSets {
    const NAME: &'static str = "Storage.getRelatedWebsiteSets";
    type ReturnObject = GetRelatedWebsiteSetsReturnObject;
}
impl Method for GetAffectedUrlsForThirdPartyCookieMetadata {
    const NAME: &'static str = "Storage.getAffectedUrlsForThirdPartyCookieMetadata";
    type ReturnObject = GetAffectedUrlsForThirdPartyCookieMetadataReturnObject;
}
impl Method for SetProtectedAudienceKAnonymity {
    const NAME: &'static str = "Storage.setProtectedAudienceKAnonymity";
    type ReturnObject = SetProtectedAudienceKAnonymityReturnObject;
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
    pub struct CacheStorageContentUpdatedEvent {
        pub params: CacheStorageContentUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct CacheStorageContentUpdatedEventParams {
        #[serde(default)]
        #[doc = "Origin to update."]
        pub origin: String,
        #[serde(default)]
        #[doc = "Storage key to update."]
        pub storage_key: String,
        #[serde(default)]
        #[doc = "Storage bucket to update."]
        pub bucket_id: String,
        #[serde(default)]
        #[doc = "Name of cache in origin."]
        pub cache_name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CacheStorageListUpdatedEvent {
        pub params: CacheStorageListUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct CacheStorageListUpdatedEventParams {
        #[serde(default)]
        #[doc = "Origin to update."]
        pub origin: String,
        #[serde(default)]
        #[doc = "Storage key to update."]
        pub storage_key: String,
        #[serde(default)]
        #[doc = "Storage bucket to update."]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBContentUpdatedEvent {
        pub params: IndexedDBContentUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct IndexedDBContentUpdatedEventParams {
        #[serde(default)]
        #[doc = "Origin to update."]
        pub origin: String,
        #[serde(default)]
        #[doc = "Storage key to update."]
        pub storage_key: String,
        #[serde(default)]
        #[doc = "Storage bucket to update."]
        pub bucket_id: String,
        #[serde(default)]
        #[doc = "Database to update."]
        pub database_name: String,
        #[serde(default)]
        #[doc = "ObjectStore to update."]
        pub object_store_name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBListUpdatedEvent {
        pub params: IndexedDBListUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct IndexedDBListUpdatedEventParams {
        #[serde(default)]
        #[doc = "Origin to update."]
        pub origin: String,
        #[serde(default)]
        #[doc = "Storage key to update."]
        pub storage_key: String,
        #[serde(default)]
        #[doc = "Storage bucket to update."]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAccessedEvent {
        pub params: InterestGroupAccessedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct InterestGroupAccessedEventParams {
        pub access_time: super::super::network::TimeSinceEpoch,
        pub r#type: super::InterestGroupAccessType,
        #[serde(default)]
        pub owner_origin: String,
        #[serde(default)]
        pub name: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "For topLevelBid/topLevelAdditionalBid, and when appropriate,\n win and additionalBidWin"]
        pub component_seller_origin: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "For bid or somethingBid event, if done locally and not on a server."]
        pub bid: Option<JsFloat>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub bid_currency: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "For non-global events --- links to interestGroupAuctionEvent"]
        pub unique_auction_id: Option<super::InterestGroupAuctionId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionEventOccurredEvent {
        pub params: InterestGroupAuctionEventOccurredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct InterestGroupAuctionEventOccurredEventParams {
        pub event_time: super::super::network::TimeSinceEpoch,
        pub r#type: super::InterestGroupAuctionEventType,
        pub unique_auction_id: super::InterestGroupAuctionId,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Set for child auctions."]
        pub parent_auction_id: Option<super::InterestGroupAuctionId>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "Set for started and configResolved"]
        pub auction_config: Option<Json>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionNetworkRequestCreatedEvent {
        pub params: InterestGroupAuctionNetworkRequestCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct InterestGroupAuctionNetworkRequestCreatedEventParams {
        pub r#type: super::InterestGroupAuctionFetchType,
        pub request_id: super::super::network::RequestId,
        #[doc = "This is the set of the auctions using the worklet that issued this\n request.  In the case of trusted signals, it's possible that only some of\n them actually care about the keys being queried."]
        pub auctions: Vec<super::InterestGroupAuctionId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageAccessedEvent {
        pub params: SharedStorageAccessedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct SharedStorageAccessedEventParams {
        #[doc = "Time of the access."]
        pub access_time: super::super::network::TimeSinceEpoch,
        #[doc = "Enum value indicating the access scope."]
        pub scope: super::SharedStorageAccessScope,
        #[doc = "Enum value indicating the Shared Storage API method invoked."]
        pub method: super::SharedStorageAccessMethod,
        #[doc = "DevTools Frame Token for the primary frame tree's root."]
        pub main_frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[doc = "Serialization of the origin owning the Shared Storage data."]
        pub owner_origin: String,
        #[serde(default)]
        #[doc = "Serialization of the site owning the Shared Storage data."]
        pub owner_site: String,
        #[doc = "The sub-parameters wrapped by `params` are all optional and their\n presence/absence depends on `type`."]
        pub params: super::SharedStorageAccessParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageWorkletOperationExecutionFinishedEvent {
        pub params: SharedStorageWorkletOperationExecutionFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct SharedStorageWorkletOperationExecutionFinishedEventParams {
        #[doc = "Time that the operation finished."]
        pub finished_time: super::super::network::TimeSinceEpoch,
        #[serde(default)]
        #[doc = "Time, in microseconds, from start of shared storage JS API call until\n end of operation execution in the worklet."]
        pub execution_time: JsUInt,
        #[doc = "Enum value indicating the Shared Storage API method invoked."]
        pub method: super::SharedStorageAccessMethod,
        #[serde(default)]
        #[doc = "ID of the operation call."]
        pub operation_id: String,
        #[doc = "Hex representation of the DevTools token used as the TargetID for the\n associated shared storage worklet."]
        pub worklet_target_id: super::super::target::TargetId,
        #[doc = "DevTools Frame Token for the primary frame tree's root."]
        pub main_frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[doc = "Serialization of the origin owning the Shared Storage data."]
        pub owner_origin: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketCreatedOrUpdatedEvent {
        pub params: StorageBucketCreatedOrUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct StorageBucketCreatedOrUpdatedEventParams {
        pub bucket_info: super::StorageBucketInfo,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketDeletedEvent {
        pub params: StorageBucketDeletedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct StorageBucketDeletedEventParams {
        #[serde(default)]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingSourceRegisteredEvent {
        pub params: AttributionReportingSourceRegisteredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct AttributionReportingSourceRegisteredEventParams {
        pub registration: super::AttributionReportingSourceRegistration,
        pub result: super::AttributionReportingSourceRegistrationResult,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingTriggerRegisteredEvent {
        pub params: AttributionReportingTriggerRegisteredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct AttributionReportingTriggerRegisteredEventParams {
        pub registration: super::AttributionReportingTriggerRegistration,
        pub event_level: super::AttributionReportingEventLevelResult,
        pub aggregatable: super::AttributionReportingAggregatableResult,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingReportSentEvent {
        pub params: AttributionReportingReportSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct AttributionReportingReportSentEventParams {
        #[serde(default)]
        pub url: String,
        #[serde(default)]
        pub body: Json,
        pub result: super::AttributionReportingReportResult,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[doc = "If result is `sent`, populated with net/HTTP status."]
        pub net_error: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub net_error_name: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub http_status_code: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingVerboseDebugReportSentEvent {
        pub params: AttributionReportingVerboseDebugReportSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Builder)]
    pub struct AttributionReportingVerboseDebugReportSentEventParams {
        #[serde(default)]
        pub url: String,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub body: Option<Vec<Json>>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub net_error: Option<JsUInt>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub net_error_name: Option<String>,
        #[builder(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub http_status_code: Option<JsUInt>,
    }
}
