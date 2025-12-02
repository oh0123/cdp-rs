// Auto-generated from Chrome at version 140.0.7339.186 domain: Storage
use super::browser;
use super::network;
use super::page;
use super::target;
#[allow(unused_imports)]
use super::types::*;
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UsageForType {
    #[serde(rename = "storageType")]
    pub storage_type: StorageType,
    #[serde(default)]
    #[serde(rename = "usage")]
    pub usage: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrustTokens {
    #[serde(default)]
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: String,
    #[serde(default)]
    #[serde(rename = "count")]
    pub count: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStorageEntry {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStorageMetadata {
    #[serde(rename = "creationTime")]
    pub creation_time: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "length")]
    pub length: JsUInt,
    #[serde(default)]
    #[serde(rename = "remainingBudget")]
    pub remaining_budget: JsFloat,
    #[serde(default)]
    #[serde(rename = "bytesUsed")]
    pub bytes_used: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStoragePrivateAggregationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "aggregationCoordinatorOrigin")]
    pub aggregation_coordinator_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "contextId")]
    pub context_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "filteringIdMaxBytes")]
    pub filtering_id_max_bytes: JsUInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "maxContributions")]
    pub max_contributions: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStorageReportingMetadata {
    #[serde(default)]
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(default)]
    #[serde(rename = "reportingUrl")]
    pub reporting_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStorageUrlWithMetadata {
    #[serde(default)]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "reportingMetadata")]
    pub reporting_metadata: Vec<SharedStorageReportingMetadata>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SharedStorageAccessParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "scriptSourceUrl")]
    pub script_source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "dataOrigin")]
    pub data_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "operationId")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "keepAlive")]
    pub keep_alive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "privateAggregationConfig")]
    pub private_aggregation_config: Option<SharedStoragePrivateAggregationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "serializedData")]
    pub serialized_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "urlsWithMetadata")]
    pub urls_with_metadata: Option<Vec<SharedStorageUrlWithMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "urnUuid")]
    pub urn_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ignoreIfPresent")]
    pub ignore_if_present: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "workletOrdinal")]
    pub worklet_ordinal: Option<JsUInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "workletTargetId")]
    pub worklet_target_id: Option<target::TargetId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "withLock")]
    pub with_lock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "batchUpdateId")]
    pub batch_update_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "batchSize")]
    pub batch_size: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StorageBucket {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StorageBucketInfo {
    #[serde(rename = "bucket")]
    pub bucket: StorageBucket,
    #[serde(default)]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "expiration")]
    pub expiration: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "quota")]
    pub quota: JsFloat,
    #[serde(default)]
    #[serde(rename = "persistent")]
    pub persistent: bool,
    #[serde(rename = "durability")]
    pub durability: StorageBucketsDurability,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingFilterDataEntry {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "values")]
    pub values: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingFilterConfig {
    #[serde(rename = "filterValues")]
    pub filter_values: Vec<AttributionReportingFilterDataEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "lookbackWindow")]
    pub lookback_window: Option<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingFilterPair {
    #[serde(rename = "filters")]
    pub filters: Vec<AttributionReportingFilterConfig>,
    #[serde(rename = "notFilters")]
    pub not_filters: Vec<AttributionReportingFilterConfig>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregationKeysEntry {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: UnsignedInt128AsBase16,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingEventReportWindows {
    #[serde(default)]
    #[serde(rename = "start")]
    pub start: JsUInt,
    #[serde(default)]
    #[serde(rename = "ends")]
    pub ends: Vec<JsUInt>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableDebugReportingData {
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
    #[serde(default)]
    #[serde(rename = "types")]
    pub types: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableDebugReportingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "budget")]
    pub budget: Option<JsFloat>,
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(rename = "debugData")]
    pub debug_data: Vec<AttributionReportingAggregatableDebugReportingData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "aggregationCoordinatorOrigin")]
    pub aggregation_coordinator_origin: Option<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionScopesData {
    #[serde(default)]
    #[serde(rename = "values")]
    pub values: Vec<String>,
    #[serde(default)]
    #[serde(rename = "limit")]
    pub limit: JsFloat,
    #[serde(default)]
    #[serde(rename = "maxEventStates")]
    pub max_event_states: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingNamedBudgetDef {
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "budget")]
    pub budget: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingSourceRegistration {
    #[serde(rename = "time")]
    pub time: network::TimeSinceEpoch,
    #[serde(default)]
    #[serde(rename = "expiry")]
    pub expiry: JsUInt,
    #[serde(default)]
    #[serde(rename = "triggerData")]
    pub trigger_data: Vec<JsFloat>,
    #[serde(rename = "eventReportWindows")]
    pub event_report_windows: AttributionReportingEventReportWindows,
    #[serde(default)]
    #[serde(rename = "aggregatableReportWindow")]
    pub aggregatable_report_window: JsUInt,
    #[serde(rename = "type")]
    pub r#type: AttributionReportingSourceType,
    #[serde(default)]
    #[serde(rename = "sourceOrigin")]
    pub source_origin: String,
    #[serde(default)]
    #[serde(rename = "reportingOrigin")]
    pub reporting_origin: String,
    #[serde(default)]
    #[serde(rename = "destinationSites")]
    pub destination_sites: Vec<String>,
    #[serde(rename = "eventId")]
    pub event_id: UnsignedInt64AsBase10,
    #[serde(rename = "priority")]
    pub priority: SignedInt64AsBase10,
    #[serde(rename = "filterData")]
    pub filter_data: Vec<AttributionReportingFilterDataEntry>,
    #[serde(rename = "aggregationKeys")]
    pub aggregation_keys: Vec<AttributionReportingAggregationKeysEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "debugKey")]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "triggerDataMatching")]
    pub trigger_data_matching: AttributionReportingTriggerDataMatching,
    #[serde(rename = "destinationLimitPriority")]
    pub destination_limit_priority: SignedInt64AsBase10,
    #[serde(rename = "aggregatableDebugReportingConfig")]
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopesData")]
    pub scopes_data: Option<AttributionScopesData>,
    #[serde(default)]
    #[serde(rename = "maxEventLevelReports")]
    pub max_event_level_reports: JsUInt,
    #[serde(rename = "namedBudgets")]
    pub named_budgets: Vec<AttributionReportingNamedBudgetDef>,
    #[serde(default)]
    #[serde(rename = "debugReporting")]
    pub debug_reporting: bool,
    #[serde(default)]
    #[serde(rename = "eventLevelEpsilon")]
    pub event_level_epsilon: JsFloat,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableValueDictEntry {
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: JsFloat,
    #[serde(rename = "filteringId")]
    pub filtering_id: UnsignedInt64AsBase10,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableValueEntry {
    #[serde(rename = "values")]
    pub values: Vec<AttributionReportingAggregatableValueDictEntry>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingEventTriggerData {
    #[serde(rename = "data")]
    pub data: UnsignedInt64AsBase10,
    #[serde(rename = "priority")]
    pub priority: SignedInt64AsBase10,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dedupKey")]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableTriggerData {
    #[serde(rename = "keyPiece")]
    pub key_piece: UnsignedInt128AsBase16,
    #[serde(default)]
    #[serde(rename = "sourceKeys")]
    pub source_keys: Vec<String>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingAggregatableDedupKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dedupKey")]
    pub dedup_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingNamedBudgetCandidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttributionReportingTriggerRegistration {
    #[serde(rename = "filters")]
    pub filters: AttributionReportingFilterPair,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "debugKey")]
    pub debug_key: Option<UnsignedInt64AsBase10>,
    #[serde(rename = "aggregatableDedupKeys")]
    pub aggregatable_dedup_keys: Vec<AttributionReportingAggregatableDedupKey>,
    #[serde(rename = "eventTriggerData")]
    pub event_trigger_data: Vec<AttributionReportingEventTriggerData>,
    #[serde(rename = "aggregatableTriggerData")]
    pub aggregatable_trigger_data: Vec<AttributionReportingAggregatableTriggerData>,
    #[serde(rename = "aggregatableValues")]
    pub aggregatable_values: Vec<AttributionReportingAggregatableValueEntry>,
    #[serde(default)]
    #[serde(rename = "aggregatableFilteringIdMaxBytes")]
    pub aggregatable_filtering_id_max_bytes: JsUInt,
    #[serde(default)]
    #[serde(rename = "debugReporting")]
    pub debug_reporting: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "aggregationCoordinatorOrigin")]
    pub aggregation_coordinator_origin: Option<String>,
    #[serde(rename = "sourceRegistrationTimeConfig")]
    pub source_registration_time_config: AttributionReportingSourceRegistrationTimeConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "triggerContextId")]
    pub trigger_context_id: Option<String>,
    #[serde(rename = "aggregatableDebugReportingConfig")]
    pub aggregatable_debug_reporting_config: AttributionReportingAggregatableDebugReportingConfig,
    #[serde(default)]
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "namedBudgets")]
    pub named_budgets: Vec<AttributionReportingNamedBudgetCandidate>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RelatedWebsiteSet {
    #[serde(default)]
    #[serde(rename = "primarySites")]
    pub primary_sites: Vec<String>,
    #[serde(default)]
    #[serde(rename = "associatedSites")]
    pub associated_sites: Vec<String>,
    #[serde(default)]
    #[serde(rename = "serviceSites")]
    pub service_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStorageKeyForFrame {
    #[serde(rename = "frameId")]
    pub frame_id: page::FrameId,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDataForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(default)]
    #[serde(rename = "storageTypes")]
    pub storage_types: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearDataForStorageKey {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(default)]
    #[serde(rename = "storageTypes")]
    pub storage_types: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCookies {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetCookies {
    #[serde(rename = "cookies")]
    pub cookies: network::CookieParam,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearCookies {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: Option<browser::BrowserContextId>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetUsageAndQuota {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OverrideQuotaForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "quotaSize")]
    pub quota_size: Option<JsFloat>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackCacheStorageForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackCacheStorageForStorageKey {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackIndexedDBForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TrackIndexedDBForStorageKey {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UntrackCacheStorageForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UntrackCacheStorageForStorageKey {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UntrackIndexedDBForOrigin {
    #[serde(default)]
    #[serde(rename = "origin")]
    pub origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UntrackIndexedDBForStorageKey {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetTrustTokens(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearTrustTokens {
    #[serde(default)]
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInterestGroupDetails {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInterestGroupTracking {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetInterestGroupAuctionTracking {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSharedStorageMetadata {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSharedStorageEntries {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSharedStorageEntry {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
    #[serde(default)]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "ignoreIfPresent")]
    pub ignore_if_present: Option<bool>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteSharedStorageEntry {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(default)]
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearSharedStorageEntries {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResetSharedStorageBudget {
    #[serde(default)]
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetSharedStorageTracking {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetStorageBucketTracking {
    #[serde(default)]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeleteStorageBucket {
    #[serde(rename = "bucket")]
    pub bucket: StorageBucket,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RunBounceTrackingMitigations(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAttributionReportingLocalTestingMode {
    #[serde(default)]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetAttributionReportingTracking {
    #[serde(default)]
    #[serde(rename = "enable")]
    pub enable: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendPendingAttributionReports(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetRelatedWebsiteSets(pub Option<serde_json::Value>);
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadata {
    #[serde(default)]
    #[serde(rename = "firstPartyUrl")]
    pub first_party_url: String,
    #[serde(default)]
    #[serde(rename = "thirdPartyUrls")]
    pub third_party_urls: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetProtectedAudienceKAnonymity {
    #[serde(default)]
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(default)]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "hashes")]
    pub hashes: Vec<u8>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetStorageKeyForFrameReturnObject {
    #[serde(rename = "storageKey")]
    pub storage_key: SerializedStorageKey,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearDataForStorageKeyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetCookiesReturnObject {
    #[serde(rename = "cookies")]
    pub cookies: network::Cookie,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetCookiesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearCookiesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetUsageAndQuotaReturnObject {
    #[serde(default)]
    #[serde(rename = "usage")]
    pub usage: JsFloat,
    #[serde(default)]
    #[serde(rename = "quota")]
    pub quota: JsFloat,
    #[serde(default)]
    #[serde(rename = "overrideActive")]
    pub override_active: bool,
    #[serde(rename = "usageBreakdown")]
    pub usage_breakdown: Vec<UsageForType>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OverrideQuotaForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackCacheStorageForStorageKeyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TrackIndexedDBForStorageKeyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UntrackCacheStorageForStorageKeyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForOriginReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UntrackIndexedDBForStorageKeyReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetTrustTokensReturnObject {
    #[serde(rename = "tokens")]
    pub tokens: Vec<TrustTokens>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClearTrustTokensReturnObject {
    #[serde(default)]
    #[serde(rename = "didDeleteTokens")]
    pub did_delete_tokens: bool,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetInterestGroupDetailsReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetInterestGroupAuctionTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSharedStorageMetadataReturnObject {
    #[serde(rename = "metadata")]
    pub metadata: SharedStorageMetadata,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetSharedStorageEntriesReturnObject {
    #[serde(rename = "entries")]
    pub entries: Vec<SharedStorageEntry>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageEntryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSharedStorageEntryReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearSharedStorageEntriesReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResetSharedStorageBudgetReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetSharedStorageTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetStorageBucketTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStorageBucketReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RunBounceTrackingMitigationsReturnObject {
    #[serde(rename = "deletedSites")]
    pub deleted_sites: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributionReportingLocalTestingModeReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetAttributionReportingTrackingReturnObject {}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SendPendingAttributionReportsReturnObject {
    #[serde(default)]
    #[serde(rename = "numSent")]
    pub num_sent: JsUInt,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetRelatedWebsiteSetsReturnObject {
    #[serde(rename = "sets")]
    pub sets: Vec<RelatedWebsiteSet>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataReturnObject {
    #[serde(rename = "matchedUrls")]
    pub matched_urls: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SetProtectedAudienceKAnonymityReturnObject {}
impl Method for GetStorageKeyForFrame {
    const NAME: &'static str = "Storage.getStorageKeyForFrame";
    type ReturnObject = GetStorageKeyForFrameReturnObject;
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
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CacheStorageContentUpdatedEvent {
        pub params: CacheStorageContentUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CacheStorageContentUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "origin")]
        pub origin: String,
        #[serde(default)]
        #[serde(rename = "storageKey")]
        pub storage_key: String,
        #[serde(default)]
        #[serde(rename = "bucketId")]
        pub bucket_id: String,
        #[serde(default)]
        #[serde(rename = "cacheName")]
        pub cache_name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CacheStorageListUpdatedEvent {
        pub params: CacheStorageListUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct CacheStorageListUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "origin")]
        pub origin: String,
        #[serde(default)]
        #[serde(rename = "storageKey")]
        pub storage_key: String,
        #[serde(default)]
        #[serde(rename = "bucketId")]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBContentUpdatedEvent {
        pub params: IndexedDBContentUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBContentUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "origin")]
        pub origin: String,
        #[serde(default)]
        #[serde(rename = "storageKey")]
        pub storage_key: String,
        #[serde(default)]
        #[serde(rename = "bucketId")]
        pub bucket_id: String,
        #[serde(default)]
        #[serde(rename = "databaseName")]
        pub database_name: String,
        #[serde(default)]
        #[serde(rename = "objectStoreName")]
        pub object_store_name: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBListUpdatedEvent {
        pub params: IndexedDBListUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct IndexedDBListUpdatedEventParams {
        #[serde(default)]
        #[serde(rename = "origin")]
        pub origin: String,
        #[serde(default)]
        #[serde(rename = "storageKey")]
        pub storage_key: String,
        #[serde(default)]
        #[serde(rename = "bucketId")]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAccessedEvent {
        pub params: InterestGroupAccessedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAccessedEventParams {
        #[serde(rename = "accessTime")]
        pub access_time: super::super::network::TimeSinceEpoch,
        #[serde(rename = "type")]
        pub r#type: super::InterestGroupAccessType,
        #[serde(default)]
        #[serde(rename = "ownerOrigin")]
        pub owner_origin: String,
        #[serde(default)]
        #[serde(rename = "name")]
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "componentSellerOrigin")]
        pub component_seller_origin: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "bid")]
        pub bid: Option<JsFloat>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "bidCurrency")]
        pub bid_currency: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "uniqueAuctionId")]
        pub unique_auction_id: Option<super::InterestGroupAuctionId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionEventOccurredEvent {
        pub params: InterestGroupAuctionEventOccurredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionEventOccurredEventParams {
        #[serde(rename = "eventTime")]
        pub event_time: super::super::network::TimeSinceEpoch,
        #[serde(rename = "type")]
        pub r#type: super::InterestGroupAuctionEventType,
        #[serde(rename = "uniqueAuctionId")]
        pub unique_auction_id: super::InterestGroupAuctionId,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "parentAuctionId")]
        pub parent_auction_id: Option<super::InterestGroupAuctionId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionNetworkRequestCreatedEvent {
        pub params: InterestGroupAuctionNetworkRequestCreatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct InterestGroupAuctionNetworkRequestCreatedEventParams {
        #[serde(rename = "type")]
        pub r#type: super::InterestGroupAuctionFetchType,
        #[serde(rename = "requestId")]
        pub request_id: super::super::network::RequestId,
        #[serde(rename = "auctions")]
        pub auctions: Vec<super::InterestGroupAuctionId>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageAccessedEvent {
        pub params: SharedStorageAccessedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageAccessedEventParams {
        #[serde(rename = "accessTime")]
        pub access_time: super::super::network::TimeSinceEpoch,
        #[serde(rename = "scope")]
        pub scope: super::SharedStorageAccessScope,
        #[serde(rename = "method")]
        pub method: super::SharedStorageAccessMethod,
        #[serde(rename = "mainFrameId")]
        pub main_frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[serde(rename = "ownerOrigin")]
        pub owner_origin: String,
        #[serde(default)]
        #[serde(rename = "ownerSite")]
        pub owner_site: String,
        #[serde(rename = "params")]
        pub params: super::SharedStorageAccessParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageWorkletOperationExecutionFinishedEvent {
        pub params: SharedStorageWorkletOperationExecutionFinishedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct SharedStorageWorkletOperationExecutionFinishedEventParams {
        #[serde(rename = "finishedTime")]
        pub finished_time: super::super::network::TimeSinceEpoch,
        #[serde(default)]
        #[serde(rename = "executionTime")]
        pub execution_time: JsUInt,
        #[serde(rename = "method")]
        pub method: super::SharedStorageAccessMethod,
        #[serde(default)]
        #[serde(rename = "operationId")]
        pub operation_id: String,
        #[serde(rename = "workletTargetId")]
        pub worklet_target_id: super::super::target::TargetId,
        #[serde(rename = "mainFrameId")]
        pub main_frame_id: super::super::page::FrameId,
        #[serde(default)]
        #[serde(rename = "ownerOrigin")]
        pub owner_origin: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketCreatedOrUpdatedEvent {
        pub params: StorageBucketCreatedOrUpdatedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketCreatedOrUpdatedEventParams {
        #[serde(rename = "bucketInfo")]
        pub bucket_info: super::StorageBucketInfo,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketDeletedEvent {
        pub params: StorageBucketDeletedEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct StorageBucketDeletedEventParams {
        #[serde(default)]
        #[serde(rename = "bucketId")]
        pub bucket_id: String,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingSourceRegisteredEvent {
        pub params: AttributionReportingSourceRegisteredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingSourceRegisteredEventParams {
        #[serde(rename = "registration")]
        pub registration: super::AttributionReportingSourceRegistration,
        #[serde(rename = "result")]
        pub result: super::AttributionReportingSourceRegistrationResult,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingTriggerRegisteredEvent {
        pub params: AttributionReportingTriggerRegisteredEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingTriggerRegisteredEventParams {
        #[serde(rename = "registration")]
        pub registration: super::AttributionReportingTriggerRegistration,
        #[serde(rename = "eventLevel")]
        pub event_level: super::AttributionReportingEventLevelResult,
        #[serde(rename = "aggregatable")]
        pub aggregatable: super::AttributionReportingAggregatableResult,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingReportSentEvent {
        pub params: AttributionReportingReportSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingReportSentEventParams {
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "result")]
        pub result: super::AttributionReportingReportResult,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "netError")]
        pub net_error: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "netErrorName")]
        pub net_error_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "httpStatusCode")]
        pub http_status_code: Option<JsUInt>,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingVerboseDebugReportSentEvent {
        pub params: AttributionReportingVerboseDebugReportSentEventParams,
    }
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct AttributionReportingVerboseDebugReportSentEventParams {
        #[serde(default)]
        #[serde(rename = "url")]
        pub url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "netError")]
        pub net_error: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "netErrorName")]
        pub net_error_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(rename = "httpStatusCode")]
        pub http_status_code: Option<JsUInt>,
    }
}
