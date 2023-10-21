use super::*;
/// A bucket.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete buckets](BucketDeleteCall) (none)
/// * [get buckets](BucketGetCall) (response)
/// * [get iam policy buckets](BucketGetIamPolicyCall) (none)
/// * [insert buckets](BucketInsertCall) (request|response)
/// * [list buckets](BucketListCall) (none)
/// * [lock retention policy buckets](BucketLockRetentionPolicyCall) (response)
/// * [patch buckets](BucketPatchCall) (request|response)
/// * [set iam policy buckets](BucketSetIamPolicyCall) (none)
/// * [test iam permissions buckets](BucketTestIamPermissionCall) (none)
/// * [update buckets](BucketUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Access controls on the bucket.
    
    pub acl: Option<Vec<BucketAccessControl>>,
    /// The bucket's Autoclass configuration.
    
    pub autoclass: Option<BucketAutoclass>,
    /// The bucket's billing configuration.
    
    pub billing: Option<BucketBilling>,
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    
    pub cors: Option<Vec<BucketCors>>,
    /// The bucket's custom placement configuration for Custom Dual Regions.
    #[serde(rename="customPlacementConfig")]
    
    pub custom_placement_config: Option<BucketCustomPlacementConfig>,
    /// The default value for event-based hold on newly created objects in this bucket. Event-based hold is a way to retain objects indefinitely until an event occurs, signified by the hold's release. After being released, such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false. Objects under event-based hold cannot be deleted, overwritten or archived until the hold is removed.
    #[serde(rename="defaultEventBasedHold")]
    
    pub default_event_based_hold: Option<bool>,
    /// Default access controls to apply to new objects when no ACL is provided.
    #[serde(rename="defaultObjectAcl")]
    
    pub default_object_acl: Option<Vec<ObjectAccessControl>>,
    /// Encryption configuration for a bucket.
    
    pub encryption: Option<BucketEncryption>,
    /// HTTP 1.1 Entity tag for the bucket.
    
    pub etag: Option<String>,
    /// The bucket's IAM configuration.
    #[serde(rename="iamConfiguration")]
    
    pub iam_configuration: Option<BucketIamConfiguration>,
    /// The ID of the bucket. For buckets, the id and name properties are the same.
    
    pub id: Option<String>,
    /// The kind of item this is. For buckets, this is always storage#bucket.
    
    pub kind: Option<String>,
    /// User-provided labels, in key/value pairs.
    
    pub labels: Option<HashMap<String, String>>,
    /// The bucket's lifecycle configuration. See lifecycle management for more information.
    
    pub lifecycle: Option<BucketLifecycle>,
    /// The location of the bucket. Object data for objects in the bucket resides in physical storage within this region. Defaults to US. See the developer's guide for the authoritative list.
    
    pub location: Option<String>,
    /// The type of the bucket location.
    #[serde(rename="locationType")]
    
    pub location_type: Option<String>,
    /// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
    
    pub logging: Option<BucketLogging>,
    /// The metadata generation of this bucket.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub metageneration: Option<i64>,
    /// The name of the bucket.
    
    pub name: Option<String>,
    /// The owner of the bucket. This is always the project team's owner group.
    
    pub owner: Option<BucketOwner>,
    /// The project number of the project the bucket belongs to.
    #[serde(rename="projectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_number: Option<u64>,
    /// The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error.
    #[serde(rename="retentionPolicy")]
    
    pub retention_policy: Option<BucketRetentionPolicy>,
    /// The Recovery Point Objective (RPO) of this bucket. Set to ASYNC_TURBO to turn on Turbo Replication on a bucket.
    
    pub rpo: Option<String>,
    /// Reserved for future use.
    #[serde(rename="satisfiesPZS")]
    
    pub satisfies_pzs: Option<bool>,
    /// The URI of this bucket.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The bucket's default storage class, used whenever no storageClass is specified for a newly-created object. This defines how objects in the bucket are stored and determines the SLA and the cost of storage. Values include MULTI_REGIONAL, REGIONAL, STANDARD, NEARLINE, COLDLINE, ARCHIVE, and DURABLE_REDUCED_AVAILABILITY. If this value is not specified when the bucket is created, it will default to STANDARD. For more information, see storage classes.
    #[serde(rename="storageClass")]
    
    pub storage_class: Option<String>,
    /// The creation time of the bucket in RFC 3339 format.
    #[serde(rename="timeCreated")]
    
    pub time_created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The modification time of the bucket in RFC 3339 format.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The bucket's versioning configuration.
    
    pub versioning: Option<BucketVersioning>,
    /// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information.
    
    pub website: Option<BucketWebsite>,
}

impl client::RequestValue for Bucket {}
impl client::Resource for Bucket {}
impl client::ResponseResult for Bucket {}


/// An access-control entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete bucket access controls](BucketAccessControlDeleteCall) (none)
/// * [get bucket access controls](BucketAccessControlGetCall) (response)
/// * [insert bucket access controls](BucketAccessControlInsertCall) (request|response)
/// * [list bucket access controls](BucketAccessControlListCall) (none)
/// * [patch bucket access controls](BucketAccessControlPatchCall) (request|response)
/// * [update bucket access controls](BucketAccessControlUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControl {
    /// The name of the bucket.
    
    pub bucket: Option<String>,
    /// The domain associated with the entity, if any.
    
    pub domain: Option<String>,
    /// The email address associated with the entity, if any.
    
    pub email: Option<String>,
    /// The entity holding the permission, in one of the following forms: 
    /// - user-userId 
    /// - user-email 
    /// - group-groupId 
    /// - group-email 
    /// - domain-domain 
    /// - project-team-projectId 
    /// - allUsers 
    /// - allAuthenticatedUsers Examples: 
    /// - The user liz@example.com would be user-liz@example.com. 
    /// - The group example@googlegroups.com would be group-example@googlegroups.com. 
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    
    pub entity: Option<String>,
    /// The ID for the entity, if any.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    
    pub etag: Option<String>,
    /// The ID of the access-control entry.
    
    pub id: Option<String>,
    /// The kind of item this is. For bucket access control entries, this is always storage#bucketAccessControl.
    
    pub kind: Option<String>,
    /// The project team associated with the entity, if any.
    #[serde(rename="projectTeam")]
    
    pub project_team: Option<BucketAccessControlProjectTeam>,
    /// The access permission for the entity.
    
    pub role: Option<String>,
    /// The link to this access-control entry.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for BucketAccessControl {}
impl client::Resource for BucketAccessControl {}
impl client::ResponseResult for BucketAccessControl {}


/// An access-control list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list bucket access controls](BucketAccessControlListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControls {
    /// The list of items.
    
    pub items: Option<Vec<BucketAccessControl>>,
    /// The kind of item this is. For lists of bucket access control entries, this is always storage#bucketAccessControls.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for BucketAccessControls {}


/// A list of buckets.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list buckets](BucketListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Buckets {
    /// The list of items.
    
    pub items: Option<Vec<Bucket>>,
    /// The kind of item this is. For lists of buckets, this is always storage#buckets.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for Buckets {}


/// An notification channel used to watch for resource changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [stop channels](ChannelStopCall) (request)
/// * [watch all objects](ObjectWatchAllCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    
    pub address: Option<String>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub expiration: Option<i64>,
    /// A UUID or similar unique string that identifies this channel.
    
    pub id: Option<String>,
    /// Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".
    
    pub kind: Option<String>,
    /// Additional parameters controlling delivery channel behavior. Optional.
    
    pub params: Option<HashMap<String, String>>,
    /// A Boolean value to indicate whether payload is wanted. Optional.
    
    pub payload: Option<bool>,
    /// An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<String>,
    /// A version-specific identifier for the watched resource.
    #[serde(rename="resourceUri")]
    
    pub resource_uri: Option<String>,
    /// An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.
    
    pub token: Option<String>,
    /// The type of delivery mechanism used for this channel.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::RequestValue for Channel {}
impl client::Resource for Channel {}
impl client::ResponseResult for Channel {}


/// A Compose request.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compose objects](ObjectComposeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequest {
    /// Properties of the resulting object.
    
    pub destination: Option<Object>,
    /// The kind of item this is.
    
    pub kind: Option<String>,
    /// The list of source objects that will be concatenated into a single object.
    #[serde(rename="sourceObjects")]
    
    pub source_objects: Option<Vec<ComposeRequestSourceObjects>>,
}

impl client::RequestValue for ComposeRequest {}


/// Represents an expression text. Example: title: "User account presence" description: "Determines whether the request has a user account" expression: "size(request.user) > 0"
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported.
    
    pub expression: Option<String>,
    /// An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// JSON template to produce a JSON-style HMAC Key resource for Create responses.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [hmac keys create projects](ProjectHmacKeyCreateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HmacKey {
    /// The kind of item this is. For HMAC keys, this is always storage#hmacKey.
    
    pub kind: Option<String>,
    /// Key metadata.
    
    pub metadata: Option<HmacKeyMetadata>,
    /// HMAC secret key material.
    
    pub secret: Option<String>,
}

impl client::ResponseResult for HmacKey {}


/// JSON template to produce a JSON-style HMAC Key metadata resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [hmac keys get projects](ProjectHmacKeyGetCall) (response)
/// * [hmac keys update projects](ProjectHmacKeyUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HmacKeyMetadata {
    /// The ID of the HMAC Key.
    #[serde(rename="accessId")]
    
    pub access_id: Option<String>,
    /// HTTP 1.1 Entity tag for the HMAC key.
    
    pub etag: Option<String>,
    /// The ID of the HMAC key, including the Project ID and the Access ID.
    
    pub id: Option<String>,
    /// The kind of item this is. For HMAC Key metadata, this is always storage#hmacKeyMetadata.
    
    pub kind: Option<String>,
    /// Project ID owning the service account to which the key authenticates.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The link to this resource.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The email address of the key's associated service account.
    #[serde(rename="serviceAccountEmail")]
    
    pub service_account_email: Option<String>,
    /// The state of the key. Can be one of ACTIVE, INACTIVE, or DELETED.
    
    pub state: Option<String>,
    /// The creation time of the HMAC key in RFC 3339 format.
    #[serde(rename="timeCreated")]
    
    pub time_created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The last modification time of the HMAC key metadata in RFC 3339 format.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for HmacKeyMetadata {}
impl client::ResponseResult for HmacKeyMetadata {}


/// A list of hmacKeys.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [hmac keys list projects](ProjectHmacKeyListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HmacKeysMetadata {
    /// The list of items.
    
    pub items: Option<Vec<HmacKeyMetadata>>,
    /// The kind of item this is. For lists of hmacKeys, this is always storage#hmacKeysMetadata.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for HmacKeysMetadata {}


/// A subscription to receive Google PubSub notifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete notifications](NotificationDeleteCall) (none)
/// * [get notifications](NotificationGetCall) (response)
/// * [insert notifications](NotificationInsertCall) (request|response)
/// * [list notifications](NotificationListCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    /// An optional list of additional attributes to attach to each Cloud PubSub message published for this notification subscription.
    
    pub custom_attributes: Option<HashMap<String, String>>,
    /// HTTP 1.1 Entity tag for this subscription notification.
    
    pub etag: Option<String>,
    /// If present, only send notifications about listed event types. If empty, sent notifications for all event types.
    
    pub event_types: Option<Vec<String>>,
    /// The ID of the notification.
    
    pub id: Option<String>,
    /// The kind of item this is. For notifications, this is always storage#notification.
    
    pub kind: Option<String>,
    /// If present, only apply this notification configuration to object names that begin with this prefix.
    
    pub object_name_prefix: Option<String>,
    /// The desired content of the Payload.
    
    pub payload_format: Option<String>,
    /// The canonical URL of this notification.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// The Cloud PubSub topic to which this subscription publishes. Formatted as: '//pubsub.googleapis.com/projects/{project-identifier}/topics/{my-topic}'
    
    pub topic: Option<String>,
}

impl client::RequestValue for Notification {}
impl client::Resource for Notification {}
impl client::ResponseResult for Notification {}


/// A list of notification subscriptions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list notifications](NotificationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Notifications {
    /// The list of items.
    
    pub items: Option<Vec<Notification>>,
    /// The kind of item this is. For lists of notifications, this is always storage#notifications.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for Notifications {}


/// An object.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [compose objects](ObjectComposeCall) (response)
/// * [copy objects](ObjectCopyCall) (request|response)
/// * [delete objects](ObjectDeleteCall) (none)
/// * [get objects](ObjectGetCall) (response)
/// * [get iam policy objects](ObjectGetIamPolicyCall) (none)
/// * [insert objects](ObjectInsertCall) (request|response)
/// * [list objects](ObjectListCall) (none)
/// * [patch objects](ObjectPatchCall) (request|response)
/// * [rewrite objects](ObjectRewriteCall) (request)
/// * [set iam policy objects](ObjectSetIamPolicyCall) (none)
/// * [test iam permissions objects](ObjectTestIamPermissionCall) (none)
/// * [update objects](ObjectUpdateCall) (request|response)
/// * [watch all objects](ObjectWatchAllCall) (none)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Object {
    /// Access controls on the object.
    
    pub acl: Option<Vec<ObjectAccessControl>>,
    /// The name of the bucket containing this object.
    
    pub bucket: Option<String>,
    /// Cache-Control directive for the object data. If omitted, and the object is accessible to all anonymous users, the default will be public, max-age=3600.
    #[serde(rename="cacheControl")]
    
    pub cache_control: Option<String>,
    /// Number of underlying components that make up this object. Components are accumulated by compose operations.
    #[serde(rename="componentCount")]
    
    pub component_count: Option<i32>,
    /// Content-Disposition of the object data.
    #[serde(rename="contentDisposition")]
    
    pub content_disposition: Option<String>,
    /// Content-Encoding of the object data.
    #[serde(rename="contentEncoding")]
    
    pub content_encoding: Option<String>,
    /// Content-Language of the object data.
    #[serde(rename="contentLanguage")]
    
    pub content_language: Option<String>,
    /// Content-Type of the object data. If an object is stored without a Content-Type, it is served as application/octet-stream.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best Practices.
    
    pub crc32c: Option<String>,
    /// A timestamp in RFC 3339 format specified by the user for an object.
    #[serde(rename="customTime")]
    
    pub custom_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    #[serde(rename="customerEncryption")]
    
    pub customer_encryption: Option<ObjectCustomerEncryption>,
    /// HTTP 1.1 Entity tag for the object.
    
    pub etag: Option<String>,
    /// Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false.
    #[serde(rename="eventBasedHold")]
    
    pub event_based_hold: Option<bool>,
    /// The content generation of this object. Used for object versioning.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// The ID of the object, including the bucket name, object name, and generation number.
    
    pub id: Option<String>,
    /// The kind of item this is. For objects, this is always storage#object.
    
    pub kind: Option<String>,
    /// Not currently supported. Specifying the parameter causes the request to fail with status code 400 - Bad Request.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// MD5 hash of the data; encoded using base64. For more information about using the MD5 hash, see Hashes and ETags: Best Practices.
    #[serde(rename="md5Hash")]
    
    pub md5_hash: Option<String>,
    /// Media download link.
    #[serde(rename="mediaLink")]
    
    pub media_link: Option<String>,
    /// User-provided metadata, in key/value pairs.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The version of the metadata for this object at this generation. Used for preconditions and for detecting changes in metadata. A metageneration number is only meaningful in the context of a particular generation of a particular object.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub metageneration: Option<i64>,
    /// The name of the object. Required if not specified by URL parameter.
    
    pub name: Option<String>,
    /// The owner of the object. This will always be the uploader of the object.
    
    pub owner: Option<ObjectOwner>,
    /// A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold).
    #[serde(rename="retentionExpirationTime")]
    
    pub retention_expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The link to this object.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
    /// Content-Length of the data in bytes.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub size: Option<u64>,
    /// Storage class of the object.
    #[serde(rename="storageClass")]
    
    pub storage_class: Option<String>,
    /// Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object.
    #[serde(rename="temporaryHold")]
    
    pub temporary_hold: Option<bool>,
    /// The creation time of the object in RFC 3339 format.
    #[serde(rename="timeCreated")]
    
    pub time_created: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.
    #[serde(rename="timeDeleted")]
    
    pub time_deleted: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated.
    #[serde(rename="timeStorageClassUpdated")]
    
    pub time_storage_class_updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The modification time of the object metadata in RFC 3339 format. Set initially to object creation time and then updated whenever any metadata of the object changes. This includes changes made by a requester, such as modifying custom metadata, as well as changes made by Cloud Storage on behalf of a requester, such as changing the storage class based on an Object Lifecycle Configuration.
    
    pub updated: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Object {}
impl client::Resource for Object {}
impl client::ResponseResult for Object {}


/// An access-control entry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get default object access controls](DefaultObjectAccessControlGetCall) (response)
/// * [insert default object access controls](DefaultObjectAccessControlInsertCall) (request|response)
/// * [patch default object access controls](DefaultObjectAccessControlPatchCall) (request|response)
/// * [update default object access controls](DefaultObjectAccessControlUpdateCall) (request|response)
/// * [delete object access controls](ObjectAccessControlDeleteCall) (none)
/// * [get object access controls](ObjectAccessControlGetCall) (response)
/// * [insert object access controls](ObjectAccessControlInsertCall) (request|response)
/// * [list object access controls](ObjectAccessControlListCall) (none)
/// * [patch object access controls](ObjectAccessControlPatchCall) (request|response)
/// * [update object access controls](ObjectAccessControlUpdateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControl {
    /// The name of the bucket.
    
    pub bucket: Option<String>,
    /// The domain associated with the entity, if any.
    
    pub domain: Option<String>,
    /// The email address associated with the entity, if any.
    
    pub email: Option<String>,
    /// The entity holding the permission, in one of the following forms: 
    /// - user-userId 
    /// - user-email 
    /// - group-groupId 
    /// - group-email 
    /// - domain-domain 
    /// - project-team-projectId 
    /// - allUsers 
    /// - allAuthenticatedUsers Examples: 
    /// - The user liz@example.com would be user-liz@example.com. 
    /// - The group example@googlegroups.com would be group-example@googlegroups.com. 
    /// - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    
    pub entity: Option<String>,
    /// The ID for the entity, if any.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    
    pub etag: Option<String>,
    /// The content generation of the object, if applied to an object.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// The ID of the access-control entry.
    
    pub id: Option<String>,
    /// The kind of item this is. For object access control entries, this is always storage#objectAccessControl.
    
    pub kind: Option<String>,
    /// The name of the object, if applied to an object.
    
    pub object: Option<String>,
    /// The project team associated with the entity, if any.
    #[serde(rename="projectTeam")]
    
    pub project_team: Option<ObjectAccessControlProjectTeam>,
    /// The access permission for the entity.
    
    pub role: Option<String>,
    /// The link to this access-control entry.
    #[serde(rename="selfLink")]
    
    pub self_link: Option<String>,
}

impl client::RequestValue for ObjectAccessControl {}
impl client::Resource for ObjectAccessControl {}
impl client::ResponseResult for ObjectAccessControl {}


/// An access-control list.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list default object access controls](DefaultObjectAccessControlListCall) (response)
/// * [list object access controls](ObjectAccessControlListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControls {
    /// The list of items.
    
    pub items: Option<Vec<ObjectAccessControl>>,
    /// The kind of item this is. For lists of object access control entries, this is always storage#objectAccessControls.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ObjectAccessControls {}


/// A list of objects.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list objects](ObjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Objects {
    /// The list of items.
    
    pub items: Option<Vec<Object>>,
    /// The kind of item this is. For lists of objects, this is always storage#objects.
    
    pub kind: Option<String>,
    /// The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of prefixes of objects matching-but-not-listed up to and including the requested delimiter.
    
    pub prefixes: Option<Vec<String>>,
}

impl client::ResponseResult for Objects {}


/// A bucket/object IAM policy.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get iam policy buckets](BucketGetIamPolicyCall) (response)
/// * [set iam policy buckets](BucketSetIamPolicyCall) (request|response)
/// * [get iam policy objects](ObjectGetIamPolicyCall) (response)
/// * [set iam policy objects](ObjectSetIamPolicyCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// An association between a role, which comes with a set of permissions, and members who may assume that role.
    
    pub bindings: Option<Vec<PolicyBindings>>,
    /// HTTP 1.1  Entity tag for the policy.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// The kind of item this is. For policies, this is always storage#policy. This field is ignored on input.
    
    pub kind: Option<String>,
    /// The ID of the resource to which this policy belongs. Will be of the form projects/_/buckets/bucket for buckets, and projects/_/buckets/bucket/objects/object for objects. A specific generation may be specified by appending #generationNumber to the end of the object name, e.g. projects/_/buckets/my-bucket/objects/data.txt#17. The current generation can be denoted with #0. This field is ignored on input.
    #[serde(rename="resourceId")]
    
    pub resource_id: Option<String>,
    /// The IAM policy format version.
    
    pub version: Option<i32>,
}

impl client::RequestValue for Policy {}
impl client::ResponseResult for Policy {}


/// A rewrite response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [rewrite objects](ObjectRewriteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RewriteResponse {
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response.
    
    pub done: Option<bool>,
    /// The kind of item this is.
    
    pub kind: Option<String>,
    /// The total size of the object being copied in bytes. This property is always present in the response.
    #[serde(rename="objectSize")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub object_size: Option<i64>,
    /// A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes.
    
    pub resource: Option<Object>,
    /// A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy.
    #[serde(rename="rewriteToken")]
    
    pub rewrite_token: Option<String>,
    /// The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response.
    #[serde(rename="totalBytesRewritten")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub total_bytes_rewritten: Option<i64>,
}

impl client::ResponseResult for RewriteResponse {}


/// A subscription to receive Google PubSub notifications.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [service account get projects](ProjectServiceAccountGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// The ID of the notification.
    
    pub email_address: Option<String>,
    /// The kind of item this is. For notifications, this is always storage#notification.
    
    pub kind: Option<String>,
}

impl client::ResponseResult for ServiceAccount {}


/// A storage.(buckets|objects).testIamPermissions response.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [test iam permissions buckets](BucketTestIamPermissionCall) (response)
/// * [test iam permissions objects](ObjectTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// The kind of item this is.
    
    pub kind: Option<String>,
    /// The permissions held by the caller. Permissions are always of the format storage.resource.capability, where resource is one of buckets or objects. The supported permissions are as follows:  
    /// - storage.buckets.delete — Delete bucket.  
    /// - storage.buckets.get — Read bucket metadata.  
    /// - storage.buckets.getIamPolicy — Read bucket IAM policy.  
    /// - storage.buckets.create — Create bucket.  
    /// - storage.buckets.list — List buckets.  
    /// - storage.buckets.setIamPolicy — Update bucket IAM policy.  
    /// - storage.buckets.update — Update bucket metadata.  
    /// - storage.objects.delete — Delete object.  
    /// - storage.objects.get — Read object data and metadata.  
    /// - storage.objects.getIamPolicy — Read object IAM policy.  
    /// - storage.objects.create — Create object.  
    /// - storage.objects.list — List objects.  
    /// - storage.objects.setIamPolicy — Update object IAM policy.  
    /// - storage.objects.update — Update object metadata.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// The bucket's Autoclass configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAutoclass {
    /// Whether or not Autoclass is enabled on this bucket
    
    pub enabled: Option<bool>,
    /// A date and time in RFC 3339 format representing the instant at which "enabled" was last toggled.
    #[serde(rename="toggleTime")]
    
    pub toggle_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::NestedType for BucketAutoclass {}
impl client::Part for BucketAutoclass {}


/// The bucket's billing configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketBilling {
    /// When set to true, Requester Pays is enabled for this bucket.
    #[serde(rename="requesterPays")]
    
    pub requester_pays: Option<bool>,
}

impl client::NestedType for BucketBilling {}
impl client::Part for BucketBilling {}


/// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketCors {
    /// The value, in seconds, to return in the  Access-Control-Max-Age header used in preflight responses.
    #[serde(rename="maxAgeSeconds")]
    
    pub max_age_seconds: Option<i32>,
    /// The list of HTTP methods on which to include CORS response headers, (GET, OPTIONS, POST, etc) Note: "*" is permitted in the list of methods, and means "any method".
    
    pub method: Option<Vec<String>>,
    /// The list of Origins eligible to receive CORS response headers. Note: "*" is permitted in the list of origins, and means "any Origin".
    
    pub origin: Option<Vec<String>>,
    /// The list of HTTP headers other than the simple response headers to give permission for the user-agent to share across domains.
    #[serde(rename="responseHeader")]
    
    pub response_header: Option<Vec<String>>,
}

impl client::NestedType for BucketCors {}
impl client::Part for BucketCors {}


/// The bucket's custom placement configuration for Custom Dual Regions.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketCustomPlacementConfig {
    /// The list of regional locations in which data is placed.
    #[serde(rename="dataLocations")]
    
    pub data_locations: Option<Vec<String>>,
}

impl client::NestedType for BucketCustomPlacementConfig {}
impl client::Part for BucketCustomPlacementConfig {}


/// Encryption configuration for a bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketEncryption {
    /// A Cloud KMS key that will be used to encrypt objects inserted into this bucket, if no encryption method is specified.
    #[serde(rename="defaultKmsKeyName")]
    
    pub default_kms_key_name: Option<String>,
}

impl client::NestedType for BucketEncryption {}
impl client::Part for BucketEncryption {}


/// The bucket's IAM configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfiguration {
    /// The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature.
    #[serde(rename="bucketPolicyOnly")]
    
    pub bucket_policy_only: Option<BucketIamConfigurationBucketPolicyOnly>,
    /// The bucket's Public Access Prevention configuration. Currently, 'inherited' and 'enforced' are supported.
    #[serde(rename="publicAccessPrevention")]
    
    pub public_access_prevention: Option<String>,
    /// The bucket's uniform bucket-level access configuration.
    #[serde(rename="uniformBucketLevelAccess")]
    
    pub uniform_bucket_level_access: Option<BucketIamConfigurationUniformBucketLevelAccess>,
}

impl client::NestedType for BucketIamConfiguration {}
impl client::Part for BucketIamConfiguration {}


/// The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfigurationBucketPolicyOnly {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename="lockedTime")]
    
    pub locked_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::NestedType for BucketIamConfigurationBucketPolicyOnly {}
impl client::Part for BucketIamConfigurationBucketPolicyOnly {}


/// The bucket's uniform bucket-level access configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfigurationUniformBucketLevelAccess {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename="lockedTime")]
    
    pub locked_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::NestedType for BucketIamConfigurationUniformBucketLevelAccess {}
impl client::Part for BucketIamConfigurationUniformBucketLevelAccess {}


/// The bucket's lifecycle configuration. See lifecycle management for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycle {
    /// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
    
    pub rule: Option<Vec<BucketLifecycleRule>>,
}

impl client::NestedType for BucketLifecycle {}
impl client::Part for BucketLifecycle {}


/// A lifecycle management rule, which is made of an action to take and the condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRule {
    /// The action to take.
    
    pub action: Option<BucketLifecycleRuleAction>,
    /// The condition(s) under which the action will be taken.
    
    pub condition: Option<BucketLifecycleRuleCondition>,
}

impl client::NestedType for BucketLifecycleRule {}
impl client::Part for BucketLifecycleRule {}


/// The action to take.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleAction {
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    #[serde(rename="storageClass")]
    
    pub storage_class: Option<String>,
    /// Type of the action. Currently, only Delete, SetStorageClass, and AbortIncompleteMultipartUpload are supported.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
}

impl client::NestedType for BucketLifecycleRuleAction {}
impl client::Part for BucketLifecycleRuleAction {}


/// The condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleCondition {
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified age.
    
    pub age: Option<i32>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when an object is created before midnight of the specified date in UTC.
    #[serde(rename="createdBefore")]
    
    pub created_before: Option<client::chrono::NaiveDate>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when the custom time on an object is before this date in UTC.
    #[serde(rename="customTimeBefore")]
    
    pub custom_time_before: Option<client::chrono::NaiveDate>,
    /// Number of days elapsed since the user-specified timestamp set on an object. The condition is satisfied if the days elapsed is at least this number. If no custom timestamp is specified on an object, the condition does not apply.
    #[serde(rename="daysSinceCustomTime")]
    
    pub days_since_custom_time: Option<i32>,
    /// Number of days elapsed since the noncurrent timestamp of an object. The condition is satisfied if the days elapsed is at least this number. This condition is relevant only for versioned objects. The value of the field must be a nonnegative integer. If it's zero, the object version will become eligible for Lifecycle action as soon as it becomes noncurrent.
    #[serde(rename="daysSinceNoncurrentTime")]
    
    pub days_since_noncurrent_time: Option<i32>,
    /// Relevant only for versioned objects. If the value is true, this condition matches live objects; if the value is false, it matches archived objects.
    #[serde(rename="isLive")]
    
    pub is_live: Option<bool>,
    /// A regular expression that satisfies the RE2 syntax. This condition is satisfied when the name of the object matches the RE2 pattern. Note: This feature is currently in the "Early Access" launch stage and is only available to a whitelisted set of users; that means that this feature may be changed in backward-incompatible ways and that it is not guaranteed to be released.
    #[serde(rename="matchesPattern")]
    
    pub matches_pattern: Option<String>,
    /// List of object name prefixes. This condition will be satisfied when at least one of the prefixes exactly matches the beginning of the object name.
    #[serde(rename="matchesPrefix")]
    
    pub matches_prefix: Option<Vec<String>>,
    /// Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, and DURABLE_REDUCED_AVAILABILITY.
    #[serde(rename="matchesStorageClass")]
    
    pub matches_storage_class: Option<Vec<String>>,
    /// List of object name suffixes. This condition will be satisfied when at least one of the suffixes exactly matches the end of the object name.
    #[serde(rename="matchesSuffix")]
    
    pub matches_suffix: Option<Vec<String>>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when the noncurrent time on an object is before this date in UTC. This condition is relevant only for versioned objects.
    #[serde(rename="noncurrentTimeBefore")]
    
    pub noncurrent_time_before: Option<client::chrono::NaiveDate>,
    /// Relevant only for versioned objects. If the value is N, this condition is satisfied when there are at least N versions (including the live version) newer than this version of the object.
    #[serde(rename="numNewerVersions")]
    
    pub num_newer_versions: Option<i32>,
}

impl client::NestedType for BucketLifecycleRuleCondition {}
impl client::Part for BucketLifecycleRuleCondition {}


/// The bucket's logging configuration, which defines the destination bucket and optional name prefix for the current bucket's logs.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLogging {
    /// The destination bucket where the current bucket's logs should be placed.
    #[serde(rename="logBucket")]
    
    pub log_bucket: Option<String>,
    /// A prefix for log object names.
    #[serde(rename="logObjectPrefix")]
    
    pub log_object_prefix: Option<String>,
}

impl client::NestedType for BucketLogging {}
impl client::Part for BucketLogging {}


/// The owner of the bucket. This is always the project team's owner group.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketOwner {
    /// The entity, in the form project-owner-projectId.
    
    pub entity: Option<String>,
    /// The ID for the entity.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
}

impl client::NestedType for BucketOwner {}
impl client::Part for BucketOwner {}


/// The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketRetentionPolicy {
    /// Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format.
    #[serde(rename="effectiveTime")]
    
    pub effective_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Once locked, an object retention policy cannot be modified.
    #[serde(rename="isLocked")]
    
    pub is_locked: Option<bool>,
    /// The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes.
    #[serde(rename="retentionPeriod")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub retention_period: Option<i64>,
}

impl client::NestedType for BucketRetentionPolicy {}
impl client::Part for BucketRetentionPolicy {}


/// The bucket's versioning configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketVersioning {
    /// While set to true, versioning is fully enabled for this bucket.
    
    pub enabled: Option<bool>,
}

impl client::NestedType for BucketVersioning {}
impl client::Part for BucketVersioning {}


/// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketWebsite {
    /// If the requested object path is missing, the service will ensure the path has a trailing '/', append this suffix, and attempt to retrieve the resulting object. This allows the creation of index.html objects to represent directory pages.
    #[serde(rename="mainPageSuffix")]
    
    pub main_page_suffix: Option<String>,
    /// If the requested object path is missing, and any mainPageSuffix object is missing, if applicable, the service will return the named object from this bucket as the content for a 404 Not Found result.
    #[serde(rename="notFoundPage")]
    
    pub not_found_page: Option<String>,
}

impl client::NestedType for BucketWebsite {}
impl client::Part for BucketWebsite {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketAccessControlProjectTeam {
    /// The project number.
    #[serde(rename="projectNumber")]
    
    pub project_number: Option<String>,
    /// The team.
    
    pub team: Option<String>,
}

impl client::NestedType for BucketAccessControlProjectTeam {}
impl client::Part for BucketAccessControlProjectTeam {}


/// The list of source objects that will be concatenated into a single object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjects {
    /// The generation of this object to use as the source.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// The source object's name. All source objects must reside in the same bucket.
    
    pub name: Option<String>,
    /// Conditions that must be met for this operation to execute.
    #[serde(rename="objectPreconditions")]
    
    pub object_preconditions: Option<ComposeRequestSourceObjectsObjectPreconditions>,
}

impl client::NestedType for ComposeRequestSourceObjects {}
impl client::Part for ComposeRequestSourceObjects {}


/// Conditions that must be met for this operation to execute.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjectsObjectPreconditions {
    /// Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail.
    #[serde(rename="ifGenerationMatch")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub if_generation_match: Option<i64>,
}

impl client::NestedType for ComposeRequestSourceObjectsObjectPreconditions {}
impl client::Part for ComposeRequestSourceObjectsObjectPreconditions {}


/// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectCustomerEncryption {
    /// The encryption algorithm.
    #[serde(rename="encryptionAlgorithm")]
    
    pub encryption_algorithm: Option<String>,
    /// SHA256 hash value of the encryption key.
    #[serde(rename="keySha256")]
    
    pub key_sha256: Option<String>,
}

impl client::NestedType for ObjectCustomerEncryption {}
impl client::Part for ObjectCustomerEncryption {}


/// The owner of the object. This will always be the uploader of the object.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectOwner {
    /// The entity, in the form user-userId.
    
    pub entity: Option<String>,
    /// The ID for the entity.
    #[serde(rename="entityId")]
    
    pub entity_id: Option<String>,
}

impl client::NestedType for ObjectOwner {}
impl client::Part for ObjectOwner {}


/// The project team associated with the entity, if any.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ObjectAccessControlProjectTeam {
    /// The project number.
    #[serde(rename="projectNumber")]
    
    pub project_number: Option<String>,
    /// The team.
    
    pub team: Option<String>,
}

impl client::NestedType for ObjectAccessControlProjectTeam {}
impl client::Part for ObjectAccessControlProjectTeam {}


/// An association between a role, which comes with a set of permissions, and members who may assume that role.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PolicyBindings {
    /// The condition that is associated with this binding. NOTE: an unsatisfied condition will not allow user access via current binding. Different bindings, including their conditions, are examined independently.
    
    pub condition: Option<Expr>,
    /// A collection of identifiers for members who may assume the provided role. Recognized identifiers are as follows:  
    /// - allUsers — A special identifier that represents anyone on the internet; with or without a Google account.  
    /// - allAuthenticatedUsers — A special identifier that represents anyone who is authenticated with a Google account or a service account.  
    /// - user:emailid — An email address that represents a specific account. For example, user:alice@gmail.com or user:joe@example.com.  
    /// - serviceAccount:emailid — An email address that represents a service account. For example,  serviceAccount:my-other-app@appspot.gserviceaccount.com .  
    /// - group:emailid — An email address that represents a Google group. For example, group:admins@example.com.  
    /// - domain:domain — A Google Apps domain name that represents all the users of that domain. For example, domain:google.com or domain:example.com.  
    /// - projectOwner:projectid — Owners of the given project. For example, projectOwner:my-example-project  
    /// - projectEditor:projectid — Editors of the given project. For example, projectEditor:my-example-project  
    /// - projectViewer:projectid — Viewers of the given project. For example, projectViewer:my-example-project
    
    pub members: Option<Vec<String>>,
    /// The role to which members belong. Two types of roles are supported: new IAM roles, which grant permissions that do not map directly to those provided by ACLs, and legacy IAM roles, which do map directly to ACL permissions. All roles are of the format roles/storage.specificRole.
    /// The new IAM roles are:  
    /// - roles/storage.admin — Full control of Google Cloud Storage resources.  
    /// - roles/storage.objectViewer — Read-Only access to Google Cloud Storage objects.  
    /// - roles/storage.objectCreator — Access to create objects in Google Cloud Storage.  
    /// - roles/storage.objectAdmin — Full control of Google Cloud Storage objects.   The legacy IAM roles are:  
    /// - roles/storage.legacyObjectReader — Read-only access to objects without listing. Equivalent to an ACL entry on an object with the READER role.  
    /// - roles/storage.legacyObjectOwner — Read/write access to existing objects without listing. Equivalent to an ACL entry on an object with the OWNER role.  
    /// - roles/storage.legacyBucketReader — Read access to buckets with object listing. Equivalent to an ACL entry on a bucket with the READER role.  
    /// - roles/storage.legacyBucketWriter — Read access to buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the WRITER role.  
    /// - roles/storage.legacyBucketOwner — Read and write access to existing buckets with object listing/creation/deletion. Equivalent to an ACL entry on a bucket with the OWNER role.
    
    pub role: Option<String>,
}

impl client::NestedType for PolicyBindings {}
impl client::Part for PolicyBindings {}


