use super::*;
/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [locations keys create projects](ProjectLocationKeyCreateCall) (response)
/// * [locations keys delete projects](ProjectLocationKeyDeleteCall) (response)
/// * [locations keys patch projects](ProjectLocationKeyPatchCall) (response)
/// * [locations keys undelete projects](ProjectLocationKeyUndeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<Status>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::Resource for Operation {}
impl client::ResponseResult for Operation {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for Status {}


/// Identifier of an Android application for key use.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2AndroidApplication {
    /// The package name of the application.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// The SHA1 fingerprint of the application. For example, both sha1 formats are acceptable : DA:39:A3:EE:5E:6B:4B:0D:32:55:BF:EF:95:60:18:90:AF:D8:07:09 or DA39A3EE5E6B4B0D3255BFEF95601890AFD80709. Output format is the latter.
    #[serde(rename="sha1Fingerprint")]
    
    pub sha1_fingerprint: Option<String>,
}

impl client::Part for V2AndroidApplication {}


/// The Android apps that are allowed to use the key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2AndroidKeyRestrictions {
    /// A list of Android applications that are allowed to make API calls with this key.
    #[serde(rename="allowedApplications")]
    
    pub allowed_applications: Option<Vec<V2AndroidApplication>>,
}

impl client::Part for V2AndroidKeyRestrictions {}


/// A restriction for a specific service and optionally one or multiple specific methods. Both fields are case insensitive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2ApiTarget {
    /// Optional. List of one or more methods that can be called. If empty, all methods for the service are allowed. A wildcard (*) can be used as the last symbol. Valid examples: `google.cloud.translate.v2.TranslateService.GetSupportedLanguage` `TranslateText` `Get*` `translate.googleapis.com.Get*`
    
    pub methods: Option<Vec<String>>,
    /// The service for this restriction. It should be the canonical service name, for example: `translate.googleapis.com`. You can use [`gcloud services list`](https://cloud.google.com/sdk/gcloud/reference/services/list) to get a list of services that are enabled in the project.
    
    pub service: Option<String>,
}

impl client::Part for V2ApiTarget {}


/// The HTTP referrers (websites) that are allowed to use the key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2BrowserKeyRestrictions {
    /// A list of regular expressions for the referrer URLs that are allowed to make API calls with this key.
    #[serde(rename="allowedReferrers")]
    
    pub allowed_referrers: Option<Vec<String>>,
}

impl client::Part for V2BrowserKeyRestrictions {}


/// Response message for `GetKeyString` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations keys get key string projects](ProjectLocationKeyGetKeyStringCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2GetKeyStringResponse {
    /// An encrypted and signed value of the key.
    #[serde(rename="keyString")]
    
    pub key_string: Option<String>,
}

impl client::ResponseResult for V2GetKeyStringResponse {}


/// The iOS apps that are allowed to use the key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2IosKeyRestrictions {
    /// A list of bundle IDs that are allowed when making API calls with this key.
    #[serde(rename="allowedBundleIds")]
    
    pub allowed_bundle_ids: Option<Vec<String>>,
}

impl client::Part for V2IosKeyRestrictions {}


/// The representation of a key managed by the API Keys API.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations keys create projects](ProjectLocationKeyCreateCall) (request)
/// * [locations keys get projects](ProjectLocationKeyGetCall) (response)
/// * [locations keys patch projects](ProjectLocationKeyPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2Key {
    /// Annotations is an unstructured key-value map stored with a policy that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. A timestamp identifying the time this key was originally created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. A timestamp when this key was deleted. If the resource is not deleted, this must be empty.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human-readable display name of this key that you can modify. The maximum length is 63 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. A checksum computed by the server based on the current value of the Key resource. This may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding. See https://google.aip.dev/154.
    
    pub etag: Option<String>,
    /// Output only. An encrypted and signed value held by this key. This field can be accessed only through the `GetKeyString` method.
    #[serde(rename="keyString")]
    
    pub key_string: Option<String>,
    /// Output only. The resource name of the key. The `name` has the form: `projects//locations/global/keys/`. For example: `projects/123456867718/locations/global/keys/b7ff1f9f-8275-410a-94dd-3855ee9b5dd2` NOTE: Key is a global resource; hence the only supported value for location is `global`.
    
    pub name: Option<String>,
    /// Key restrictions.
    
    pub restrictions: Option<V2Restrictions>,
    /// Output only. Unique id in UUID4 format.
    
    pub uid: Option<String>,
    /// Output only. A timestamp identifying the time this key was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for V2Key {}
impl client::ResponseResult for V2Key {}


/// Response message for `ListKeys` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations keys list projects](ProjectLocationKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2ListKeysResponse {
    /// A list of API keys.
    
    pub keys: Option<Vec<V2Key>>,
    /// The pagination token for the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for V2ListKeysResponse {}


/// Response message for `LookupKey` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [lookup key keys](KeyLookupKeyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2LookupKeyResponse {
    /// The resource name of the API key. If the API key has been purged, resource name is empty.
    
    pub name: Option<String>,
    /// The project that owns the key with the value specified in the request.
    
    pub parent: Option<String>,
}

impl client::ResponseResult for V2LookupKeyResponse {}


/// Describes the restrictions on the key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2Restrictions {
    /// The Android apps that are allowed to use the key.
    #[serde(rename="androidKeyRestrictions")]
    
    pub android_key_restrictions: Option<V2AndroidKeyRestrictions>,
    /// A restriction for a specific service and optionally one or more specific methods. Requests are allowed if they match any of these restrictions. If no restrictions are specified, all targets are allowed.
    #[serde(rename="apiTargets")]
    
    pub api_targets: Option<Vec<V2ApiTarget>>,
    /// The HTTP referrers (websites) that are allowed to use the key.
    #[serde(rename="browserKeyRestrictions")]
    
    pub browser_key_restrictions: Option<V2BrowserKeyRestrictions>,
    /// The iOS apps that are allowed to use the key.
    #[serde(rename="iosKeyRestrictions")]
    
    pub ios_key_restrictions: Option<V2IosKeyRestrictions>,
    /// The IP addresses of callers that are allowed to use the key.
    #[serde(rename="serverKeyRestrictions")]
    
    pub server_key_restrictions: Option<V2ServerKeyRestrictions>,
}

impl client::Part for V2Restrictions {}


/// The IP addresses of callers that are allowed to use the key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2ServerKeyRestrictions {
    /// A list of the caller IP addresses that are allowed to make API calls with this key.
    #[serde(rename="allowedIps")]
    
    pub allowed_ips: Option<Vec<String>>,
}

impl client::Part for V2ServerKeyRestrictions {}


/// Request message for `UndeleteKey` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations keys undelete projects](ProjectLocationKeyUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct V2UndeleteKeyRequest { _never_set: Option<bool> }

impl client::RequestValue for V2UndeleteKeyRequest {}


