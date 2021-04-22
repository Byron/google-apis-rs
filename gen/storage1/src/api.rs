use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeMap;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;
use std::thread::sleep;

use crate::client;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash)]
pub enum Scope {
    /// View and manage your data across Google Cloud Platform services
    CloudPlatform,

    /// View your data across Google Cloud Platform services
    CloudPlatformReadOnly,

    /// Manage your data and permissions in Google Cloud Storage
    DevstorageFullControl,

    /// View your data in Google Cloud Storage
    DevstorageReadOnly,

    /// Manage your data in Google Cloud Storage
    DevstorageReadWrite,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::DevstorageFullControl => "https://www.googleapis.com/auth/devstorage.full_control",
            Scope::DevstorageReadOnly => "https://www.googleapis.com/auth/devstorage.read_only",
            Scope::DevstorageReadWrite => "https://www.googleapis.com/auth/devstorage.read_write",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Storage related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// use storage1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("et")
///              .source_generation("sed")
///              .rewrite_token("et")
///              .provisional_user_project("et")
///              .projection("vero")
///              .max_bytes_rewritten_per_call("erat")
///              .if_source_metageneration_not_match("sed")
///              .if_source_metageneration_match("duo")
///              .if_source_generation_not_match("dolore")
///              .if_source_generation_match("et")
///              .if_metageneration_not_match("voluptua.")
///              .if_metageneration_match("amet.")
///              .if_generation_not_match("consetetur")
///              .if_generation_match("diam")
///              .destination_predefined_acl("dolor")
///              .destination_kms_key_name("et")
///              .doit().await;
/// 
/// match result {
///     Err(e) => match e {
///         // The Error enum provides details about what exactly happened.
///         // You can also just use its `Debug`, `Display` or `Error` traits
///          Error::HttpError(_)
///         |Error::Io(_)
///         |Error::MissingAPIKey
///         |Error::MissingToken(_)
///         |Error::Cancelled
///         |Error::UploadSizeLimitExceeded(_, _)
///         |Error::Failure(_)
///         |Error::BadRequest(_)
///         |Error::FieldClash(_)
///         |Error::JsonDecodeError(_, _) => println!("{}", e),
///     },
///     Ok(res) => println!("Success: {:?}", res),
/// }
/// # }
/// ```
pub struct Storage<> {
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for Storage<> {}

impl<'a, > Storage<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Storage<> {
        Storage {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/2.0.5".to_string(),
            _base_url: "https://storage.googleapis.com/storage/v1/".to_string(),
            _root_url: "https://storage.googleapis.com/".to_string(),
        }
    }

    pub fn bucket_access_controls(&'a self) -> BucketAccessControlMethods<'a> {
        BucketAccessControlMethods { hub: &self }
    }
    pub fn buckets(&'a self) -> BucketMethods<'a> {
        BucketMethods { hub: &self }
    }
    pub fn channels(&'a self) -> ChannelMethods<'a> {
        ChannelMethods { hub: &self }
    }
    pub fn default_object_access_controls(&'a self) -> DefaultObjectAccessControlMethods<'a> {
        DefaultObjectAccessControlMethods { hub: &self }
    }
    pub fn notifications(&'a self) -> NotificationMethods<'a> {
        NotificationMethods { hub: &self }
    }
    pub fn object_access_controls(&'a self) -> ObjectAccessControlMethods<'a> {
        ObjectAccessControlMethods { hub: &self }
    }
    pub fn objects(&'a self) -> ObjectMethods<'a> {
        ObjectMethods { hub: &self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://storage.googleapis.com/storage/v1/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://storage.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    /// Access controls on the bucket.
    pub acl: Option<Vec<BucketAccessControl>>,
    /// The bucket's billing configuration.
    pub billing: Option<BucketBilling>,
    /// The bucket's Cross-Origin Resource Sharing (CORS) configuration.
    pub cors: Option<Vec<BucketCors>>,
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
    pub metageneration: Option<String>,
    /// The name of the bucket.
    pub name: Option<String>,
    /// The owner of the bucket. This is always the project team's owner group.
    pub owner: Option<BucketOwner>,
    /// The project number of the project the bucket belongs to.
    #[serde(rename="projectNumber")]
    pub project_number: Option<String>,
    /// The bucket's retention policy. The retention policy enforces a minimum retention time for all objects contained in the bucket, based on their creation time. Any attempt to overwrite or delete objects younger than the retention period will result in a PERMISSION_DENIED error. An unlocked retention policy can be modified or removed from the bucket via a storage.buckets.update operation. A locked retention policy cannot be removed or shortened in duration for the lifetime of the bucket. Attempting to remove or decrease period of a locked retention policy will result in a PERMISSION_DENIED error.
    #[serde(rename="retentionPolicy")]
    pub retention_policy: Option<BucketRetentionPolicy>,
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
    pub time_created: Option<String>,
    /// The modification time of the bucket in RFC 3339 format.
    pub updated: Option<String>,
    /// The bucket's versioning configuration.
    pub versioning: Option<BucketVersioning>,
    /// The bucket's website configuration, controlling how the service behaves when accessing bucket contents as a web site. See the Static Website Examples for more information.
    pub website: Option<BucketWebsite>,
    /// The zone or zones from which the bucket is intended to use zonal quota. Requests for data from outside the specified affinities are still allowed but won't be able to use zonal quota. The zone or zones need to be within the bucket location otherwise the requests will fail with a 400 Bad Request response.
    #[serde(rename="zoneAffinity")]
    pub zone_affinity: Option<Vec<String>>,
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
/// 
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
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    /// The address where notifications are delivered for this channel.
    pub address: Option<String>,
    /// Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.
    pub expiration: Option<String>,
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
/// 
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
/// 
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
/// 
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
    pub time_created: Option<String>,
    /// The last modification time of the HMAC key metadata in RFC 3339 format.
    pub updated: Option<String>,
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
/// 
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
/// 
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
/// 
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
/// 
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
    pub custom_time: Option<String>,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    #[serde(rename="customerEncryption")]
    pub customer_encryption: Option<ObjectCustomerEncryption>,
    /// HTTP 1.1 Entity tag for the object.
    pub etag: Option<String>,
    /// Whether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any). One sample use case of this flag is for banks to hold loan documents for at least 3 years after loan is paid in full. Here, bucket-level retention is 3 years and the event is the loan being paid in full. In this example, these objects will be held intact for any number of years until the event has occurred (event-based hold on the object is released) and then 3 more years after that. That means retention duration of the objects begins from the moment event-based hold transitioned from true to false.
    #[serde(rename="eventBasedHold")]
    pub event_based_hold: Option<bool>,
    /// The content generation of this object. Used for object versioning.
    pub generation: Option<String>,
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
    pub metageneration: Option<String>,
    /// The name of the object. Required if not specified by URL parameter.
    pub name: Option<String>,
    /// The owner of the object. This will always be the uploader of the object.
    pub owner: Option<ObjectOwner>,
    /// A server-determined value that specifies the earliest time that the object's retention period expires. This value is in RFC 3339 format. Note 1: This field is not provided for objects with an active event-based hold, since retention expiration is unknown until the hold is removed. Note 2: This value can be provided even when temporary hold is set (so that the user can reason about policy without having to first unset the temporary hold).
    #[serde(rename="retentionExpirationTime")]
    pub retention_expiration_time: Option<String>,
    /// The link to this object.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// Content-Length of the data in bytes.
    pub size: Option<String>,
    /// Storage class of the object.
    #[serde(rename="storageClass")]
    pub storage_class: Option<String>,
    /// Whether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites. A common use case of this flag is regulatory investigations where objects need to be retained while the investigation is ongoing. Note that unlike event-based hold, temporary hold does not impact retention expiration time of an object.
    #[serde(rename="temporaryHold")]
    pub temporary_hold: Option<bool>,
    /// The creation time of the object in RFC 3339 format.
    #[serde(rename="timeCreated")]
    pub time_created: Option<String>,
    /// The deletion time of the object in RFC 3339 format. Will be returned if and only if this version of the object has been deleted.
    #[serde(rename="timeDeleted")]
    pub time_deleted: Option<String>,
    /// The time at which the object's storage class was last changed. When the object is initially created, it will be set to timeCreated.
    #[serde(rename="timeStorageClassUpdated")]
    pub time_storage_class_updated: Option<String>,
    /// The modification time of the object metadata in RFC 3339 format.
    pub updated: Option<String>,
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
/// 
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
    pub generation: Option<String>,
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
/// 
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
/// 
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// An association between a role, which comes with a set of permissions, and members who may assume that role.
    pub bindings: Option<Vec<PolicyBindings>>,
    /// HTTP 1.1  Entity tag for the policy.
    pub etag: Option<String>,
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
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RewriteResponse {
    /// true if the copy is finished; otherwise, false if the copy is in progress. This property is always present in the response.
    pub done: Option<bool>,
    /// The kind of item this is.
    pub kind: Option<String>,
    /// The total size of the object being copied in bytes. This property is always present in the response.
    #[serde(rename="objectSize")]
    pub object_size: Option<String>,
    /// A resource containing the metadata for the copied-to object. This property is present in the response only when copying completes.
    pub resource: Option<Object>,
    /// A token to use in subsequent requests to continue copying data. This token is present in the response only when there is more data to copy.
    #[serde(rename="rewriteToken")]
    pub rewrite_token: Option<String>,
    /// The total bytes written so far, which can be used to provide a waiting user with a progress indicator. This property is always present in the response.
    #[serde(rename="totalBytesRewritten")]
    pub total_bytes_rewritten: Option<String>,
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
/// 
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
/// 
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


/// The bucket's billing configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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


/// Encryption configuration for a bucket.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfiguration {
    /// The bucket's uniform bucket-level access configuration. The feature was formerly known as Bucket Policy Only. For backward compatibility, this field will be populated with identical information as the uniformBucketLevelAccess field. We recommend using the uniformBucketLevelAccess field to enable and disable the feature.
    #[serde(rename="bucketPolicyOnly")]
    pub bucket_policy_only: Option<BucketIamConfigurationBucketPolicyOnly>,
    /// The bucket's Public Access Prevention configuration. Currently, 'unspecified' and 'enforced' are supported.
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfigurationBucketPolicyOnly {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.bucketPolicyOnly.enabled from true to false in RFC 3339 format. iamConfiguration.bucketPolicyOnly.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename="lockedTime")]
    pub locked_time: Option<String>,
}

impl client::NestedType for BucketIamConfigurationBucketPolicyOnly {}
impl client::Part for BucketIamConfigurationBucketPolicyOnly {}


/// The bucket's uniform bucket-level access configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketIamConfigurationUniformBucketLevelAccess {
    /// If set, access is controlled only by bucket-level or above IAM policies.
    pub enabled: Option<bool>,
    /// The deadline for changing iamConfiguration.uniformBucketLevelAccess.enabled from true to false in RFC 3339  format. iamConfiguration.uniformBucketLevelAccess.enabled may be changed from true to false until the locked time, after which the field is immutable.
    #[serde(rename="lockedTime")]
    pub locked_time: Option<String>,
}

impl client::NestedType for BucketIamConfigurationUniformBucketLevelAccess {}
impl client::Part for BucketIamConfigurationUniformBucketLevelAccess {}


/// The bucket's lifecycle configuration. See lifecycle management for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleAction {
    /// Target storage class. Required iff the type of the action is SetStorageClass.
    #[serde(rename="storageClass")]
    pub storage_class: Option<String>,
    /// Type of the action. Currently, only Delete and SetStorageClass are supported.
    #[serde(rename="type")]
    pub type_: Option<String>,
}

impl client::NestedType for BucketLifecycleRuleAction {}
impl client::Part for BucketLifecycleRuleAction {}


/// The condition(s) under which the action will be taken.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketLifecycleRuleCondition {
    /// Age of an object (in days). This condition is satisfied when an object reaches the specified age.
    pub age: Option<i32>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when an object is created before midnight of the specified date in UTC.
    #[serde(rename="createdBefore")]
    pub created_before: Option<String>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when the custom time on an object is before this date in UTC.
    #[serde(rename="customTimeBefore")]
    pub custom_time_before: Option<String>,
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
    /// Objects having any of the storage classes specified by this condition will be matched. Values include MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE, STANDARD, and DURABLE_REDUCED_AVAILABILITY.
    #[serde(rename="matchesStorageClass")]
    pub matches_storage_class: Option<Vec<String>>,
    /// A date in RFC 3339 format with only the date part (for instance, "2013-01-15"). This condition is satisfied when the noncurrent time on an object is before this date in UTC. This condition is relevant only for versioned objects.
    #[serde(rename="noncurrentTimeBefore")]
    pub noncurrent_time_before: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BucketRetentionPolicy {
    /// Server-determined value that indicates the time from which policy was enforced and effective. This value is in RFC 3339 format.
    #[serde(rename="effectiveTime")]
    pub effective_time: Option<String>,
    /// Once locked, an object retention policy cannot be modified.
    #[serde(rename="isLocked")]
    pub is_locked: Option<bool>,
    /// The duration in seconds that objects need to be retained. Retention duration must be greater than zero and less than 100 years. Note that enforcement of retention periods less than a day is not guaranteed. Such periods should only be used for testing purposes.
    #[serde(rename="retentionPeriod")]
    pub retention_period: Option<String>,
}

impl client::NestedType for BucketRetentionPolicy {}
impl client::Part for BucketRetentionPolicy {}


/// The bucket's versioning configuration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjects {
    /// The generation of this object to use as the source.
    pub generation: Option<String>,
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
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ComposeRequestSourceObjectsObjectPreconditions {
    /// Only perform the composition if the generation of the source object that would be used matches this value. If this value and a generation are both specified, they must be the same value or the call will fail.
    #[serde(rename="ifGenerationMatch")]
    pub if_generation_match: Option<String>,
}

impl client::NestedType for ComposeRequestSourceObjectsObjectPreconditions {}
impl client::Part for ComposeRequestSourceObjectsObjectPreconditions {}


/// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
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



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *bucketAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.bucket_access_controls();
/// # }
/// ```
pub struct BucketAccessControlMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for BucketAccessControlMethods<'a> {}

impl<'a> BucketAccessControlMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> BucketAccessControlDeleteCall<'a> {
        BucketAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> BucketAccessControlGetCall<'a> {
        BucketAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: BucketAccessControl, bucket: &str) -> BucketAccessControlInsertCall<'a> {
        BucketAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> BucketAccessControlListCall<'a> {
        BucketAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlPatchCall<'a> {
        BucketAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: BucketAccessControl, bucket: &str, entity: &str) -> BucketAccessControlUpdateCall<'a> {
        BucketAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *bucket* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `lock_retention_policy(...)`, `patch(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.buckets();
/// # }
/// ```
pub struct BucketMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for BucketMethods<'a> {}

impl<'a> BucketMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes an empty bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn delete(&self, bucket: &str) -> BucketDeleteCall<'a> {
        BucketDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get(&self, bucket: &str) -> BucketGetCall<'a> {
        BucketGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn get_iam_policy(&self, bucket: &str) -> BucketGetIamPolicyCall<'a> {
        BucketGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _options_requested_policy_version: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - A valid API project identifier.
    pub fn insert(&self, request: Bucket, project: &str) -> BucketInsertCall<'a> {
        BucketInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of buckets for a given project.
    /// 
    /// # Arguments
    ///
    /// * `project` - A valid API project identifier.
    pub fn list(&self, project: &str) -> BucketListCall<'a> {
        BucketListCall {
            hub: self.hub,
            _project: project.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Locks retention policy on a bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `ifMetagenerationMatch` - Makes the operation conditional on whether bucket's current metageneration matches the given value.
    pub fn lock_retention_policy(&self, bucket: &str, if_metageneration_match: &str) -> BucketLockRetentionPolicyCall<'a> {
        BucketLockRetentionPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _if_metageneration_match: if_metageneration_match.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn patch(&self, request: Bucket, bucket: &str) -> BucketPatchCall<'a> {
        BucketPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an IAM policy for the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str) -> BucketSetIamPolicyCall<'a> {
        BucketSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests a set of permissions on the given bucket to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, permissions: &Vec<String>) -> BucketTestIamPermissionCall<'a> {
        BucketTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn update(&self, request: Bucket, bucket: &str) -> BucketUpdateCall<'a> {
        BucketUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_default_object_acl: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *channel* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `stop(...)`
/// // to build up your call.
/// let rb = hub.channels();
/// # }
/// ```
pub struct ChannelMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for ChannelMethods<'a> {}

impl<'a> ChannelMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stop watching resources through this channel
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn stop(&self, request: Channel) -> ChannelStopCall<'a> {
        ChannelStopCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *defaultObjectAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.default_object_access_controls();
/// # }
/// ```
pub struct DefaultObjectAccessControlMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for DefaultObjectAccessControlMethods<'a> {}

impl<'a> DefaultObjectAccessControlMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlDeleteCall<'a> {
        DefaultObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the default object ACL entry for the specified entity on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, entity: &str) -> DefaultObjectAccessControlGetCall<'a> {
        DefaultObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str) -> DefaultObjectAccessControlInsertCall<'a> {
        DefaultObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves default object ACL entries on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    pub fn list(&self, bucket: &str) -> DefaultObjectAccessControlListCall<'a> {
        DefaultObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches a default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlPatchCall<'a> {
        DefaultObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates a default object ACL entry on the specified bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, entity: &str) -> DefaultObjectAccessControlUpdateCall<'a> {
        DefaultObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *notification* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.notifications();
/// # }
/// ```
pub struct NotificationMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for NotificationMethods<'a> {}

impl<'a> NotificationMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes a notification subscription.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - ID of the notification to delete.
    pub fn delete(&self, bucket: &str, notification: &str) -> NotificationDeleteCall<'a> {
        NotificationDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// View a notification configuration.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - The parent bucket of the notification.
    /// * `notification` - Notification ID
    pub fn get(&self, bucket: &str, notification: &str) -> NotificationGetCall<'a> {
        NotificationGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _notification: notification.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a notification subscription for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - The parent bucket of the notification.
    pub fn insert(&self, request: Notification, bucket: &str) -> NotificationInsertCall<'a> {
        NotificationInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of notification subscriptions for a given bucket.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a Google Cloud Storage bucket.
    pub fn list(&self, bucket: &str) -> NotificationListCall<'a> {
        NotificationListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *objectAccessControl* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `insert(...)`, `list(...)`, `patch(...)` and `update(...)`
/// // to build up your call.
/// let rb = hub.object_access_controls();
/// # }
/// ```
pub struct ObjectAccessControlMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for ObjectAccessControlMethods<'a> {}

impl<'a> ObjectAccessControlMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn delete(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlDeleteCall<'a> {
        ObjectAccessControlDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the ACL entry for the specified entity on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn get(&self, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlGetCall<'a> {
        ObjectAccessControlGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn insert(&self, request: ObjectAccessControl, bucket: &str, object: &str) -> ObjectAccessControlInsertCall<'a> {
        ObjectAccessControlInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves ACL entries on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn list(&self, bucket: &str, object: &str) -> ObjectAccessControlListCall<'a> {
        ObjectAccessControlListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn patch(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlPatchCall<'a> {
        ObjectAccessControlPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an ACL entry on the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of a bucket.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `entity` - The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub fn update(&self, request: ObjectAccessControl, bucket: &str, object: &str, entity: &str) -> ObjectAccessControlUpdateCall<'a> {
        ObjectAccessControlUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _entity: entity.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *object* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `compose(...)`, `copy(...)`, `delete(...)`, `get(...)`, `get_iam_policy(...)`, `insert(...)`, `list(...)`, `patch(...)`, `rewrite(...)`, `set_iam_policy(...)`, `test_iam_permissions(...)`, `update(...)` and `watch_all(...)`
/// // to build up your call.
/// let rb = hub.objects();
/// # }
/// ```
pub struct ObjectMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for ObjectMethods<'a> {}

impl<'a> ObjectMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Concatenates a list of existing objects into a new object in the same bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `destinationBucket` - Name of the bucket containing the source objects. The destination object is stored in this bucket.
    /// * `destinationObject` - Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn compose(&self, request: ComposeRequest, destination_bucket: &str, destination_object: &str) -> ObjectComposeCall<'a> {
        ObjectComposeCall {
            hub: self.hub,
            _request: request,
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _kms_key_name: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Copies a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.
    pub fn copy(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectCopyCall<'a> {
        ObjectCopyCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _source_generation: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _if_source_metageneration_not_match: Default::default(),
            _if_source_metageneration_match: Default::default(),
            _if_source_generation_not_match: Default::default(),
            _if_source_generation_match: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _destination_kms_key_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn delete(&self, bucket: &str, object: &str) -> ObjectDeleteCall<'a> {
        ObjectDeleteCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an object or its metadata.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get(&self, bucket: &str, object: &str) -> ObjectGetCall<'a> {
        ObjectGetCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn get_iam_policy(&self, bucket: &str, object: &str) -> ObjectGetIamPolicyCall<'a> {
        ObjectGetIamPolicyCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Stores a new object and metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    pub fn insert(&self, request: Object, bucket: &str) -> ObjectInsertCall<'a> {
        ObjectInsertCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _name: Default::default(),
            _kms_key_name: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _content_encoding: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of objects matching the criteria.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn list(&self, bucket: &str) -> ObjectListCall<'a> {
        ObjectListCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
            _start_offset: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_trailing_delimiter: Default::default(),
            _end_offset: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Patches an object's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn patch(&self, request: Object, bucket: &str, object: &str) -> ObjectPatchCall<'a> {
        ObjectPatchCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Rewrites a source object to a destination object. Optionally overrides metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `sourceBucket` - Name of the bucket in which to find the source object.
    /// * `sourceObject` - Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `destinationBucket` - Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    /// * `destinationObject` - Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn rewrite(&self, request: Object, source_bucket: &str, source_object: &str, destination_bucket: &str, destination_object: &str) -> ObjectRewriteCall<'a> {
        ObjectRewriteCall {
            hub: self.hub,
            _request: request,
            _source_bucket: source_bucket.to_string(),
            _source_object: source_object.to_string(),
            _destination_bucket: destination_bucket.to_string(),
            _destination_object: destination_object.to_string(),
            _user_project: Default::default(),
            _source_generation: Default::default(),
            _rewrite_token: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _max_bytes_rewritten_per_call: Default::default(),
            _if_source_metageneration_not_match: Default::default(),
            _if_source_metageneration_match: Default::default(),
            _if_source_generation_not_match: Default::default(),
            _if_source_generation_match: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _destination_predefined_acl: Default::default(),
            _destination_kms_key_name: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an IAM policy for the specified object.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn set_iam_policy(&self, request: Policy, bucket: &str, object: &str) -> ObjectSetIamPolicyCall<'a> {
        ObjectSetIamPolicyCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Tests a set of permissions on the given object to see which, if any, are held by the caller.
    /// 
    /// # Arguments
    ///
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    /// * `permissions` - Permissions to test.
    pub fn test_iam_permissions(&self, bucket: &str, object: &str, permissions: &Vec<String>) -> ObjectTestIamPermissionCall<'a> {
        ObjectTestIamPermissionCall {
            hub: self.hub,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _permissions: permissions.clone(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates an object's metadata.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which the object resides.
    /// * `object` - Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    pub fn update(&self, request: Object, bucket: &str, object: &str) -> ObjectUpdateCall<'a> {
        ObjectUpdateCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _object: object.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _predefined_acl: Default::default(),
            _if_metageneration_not_match: Default::default(),
            _if_metageneration_match: Default::default(),
            _if_generation_not_match: Default::default(),
            _if_generation_match: Default::default(),
            _generation: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Watch for changes on all objects in a bucket.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `bucket` - Name of the bucket in which to look for objects.
    pub fn watch_all(&self, request: Channel, bucket: &str) -> ObjectWatchAllCall<'a> {
        ObjectWatchAllCall {
            hub: self.hub,
            _request: request,
            _bucket: bucket.to_string(),
            _versions: Default::default(),
            _user_project: Default::default(),
            _start_offset: Default::default(),
            _provisional_user_project: Default::default(),
            _projection: Default::default(),
            _prefix: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _include_trailing_delimiter: Default::default(),
            _end_offset: Default::default(),
            _delimiter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Storage` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_storage1 as storage1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use storage1::Storage;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `hmac_keys_create(...)`, `hmac_keys_delete(...)`, `hmac_keys_get(...)`, `hmac_keys_list(...)`, `hmac_keys_update(...)` and `service_account_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a>
    where  {

    hub: &'a Storage<>,
}

impl<'a> client::MethodsBuilder for ProjectMethods<'a> {}

impl<'a> ProjectMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new HMAC key for the specified service account.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the service account.
    /// * `serviceAccountEmail` - Email address of the service account.
    pub fn hmac_keys_create(&self, project_id: &str, service_account_email: &str) -> ProjectHmacKeyCreateCall<'a> {
        ProjectHmacKeyCreateCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _service_account_email: service_account_email.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes an HMAC key.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the requested key
    /// * `accessId` - Name of the HMAC key to be deleted.
    pub fn hmac_keys_delete(&self, project_id: &str, access_id: &str) -> ProjectHmacKeyDeleteCall<'a> {
        ProjectHmacKeyDeleteCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves an HMAC key's metadata
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID owning the service account of the requested key.
    /// * `accessId` - Name of the HMAC key.
    pub fn hmac_keys_get(&self, project_id: &str, access_id: &str) -> ProjectHmacKeyGetCall<'a> {
        ProjectHmacKeyGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves a list of HMAC keys matching the criteria.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Name of the project in which to look for HMAC keys.
    pub fn hmac_keys_list(&self, project_id: &str) -> ProjectHmacKeyListCall<'a> {
        ProjectHmacKeyListCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _user_project: Default::default(),
            _show_deleted_keys: Default::default(),
            _service_account_email: Default::default(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - Project ID owning the service account of the updated key.
    /// * `accessId` - Name of the HMAC key being updated.
    pub fn hmac_keys_update(&self, request: HmacKeyMetadata, project_id: &str, access_id: &str) -> ProjectHmacKeyUpdateCall<'a> {
        ProjectHmacKeyUpdateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _access_id: access_id.to_string(),
            _user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get the email address of this project's Google Cloud Storage service account.
    /// 
    /// # Arguments
    ///
    /// * `projectId` - Project ID
    pub fn service_account_get(&self, project_id: &str) -> ProjectServiceAccountGetCall<'a> {
        ProjectServiceAccountGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _user_project: Default::default(),
            _provisional_user_project: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Permanently deletes the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().delete("bucket", "entity")
///              .user_project("Stet")
///              .provisional_user_project("dolor")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlDeleteCall<'a> {}

impl<'a> BucketAccessControlDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().get("bucket", "entity")
///              .user_project("vero")
///              .provisional_user_project("invidunt")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlGetCall<'a> {}

impl<'a> BucketAccessControlGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlGetCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new ACL entry on the specified bucket.
///
/// A builder for the *insert* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::BucketAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().insert(req, "bucket")
///              .user_project("vero")
///              .provisional_user_project("elitr")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: BucketAccessControl,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlInsertCall<'a> {}

impl<'a> BucketAccessControlInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().list("bucket")
///              .user_project("diam")
///              .provisional_user_project("no")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlListCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlListCall<'a> {}

impl<'a> BucketAccessControlListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BucketAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlListCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Patches an ACL entry on the specified bucket.
///
/// A builder for the *patch* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::BucketAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().patch(req, "bucket", "entity")
///              .user_project("takimata")
///              .provisional_user_project("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlPatchCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlPatchCall<'a> {}

impl<'a> BucketAccessControlPatchCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlPatchCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlPatchCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlPatchCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlPatchCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlPatchCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *bucketAccessControl* resource.
/// It is not used directly, but through a `BucketAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::BucketAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = BucketAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.bucket_access_controls().update(req, "bucket", "entity")
///              .user_project("erat")
///              .provisional_user_project("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct BucketAccessControlUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: BucketAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketAccessControlUpdateCall<'a> {}

impl<'a> BucketAccessControlUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, BucketAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.bucketAccessControls.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: BucketAccessControl) -> BucketAccessControlUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketAccessControlUpdateCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketAccessControlUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketAccessControlUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketAccessControlUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes an empty bucket.
///
/// A builder for the *delete* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().delete("bucket")
///              .user_project("sed")
///              .provisional_user_project("takimata")
///              .if_metageneration_not_match("dolores")
///              .if_metageneration_match("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct BucketDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketDeleteCall<'a> {}

impl<'a> BucketDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["bucket", "userProject", "provisionalUserProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If set, only deletes the bucket if its metageneration does not match this value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketDeleteCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// If set, only deletes the bucket if its metageneration matches this value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketDeleteCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns metadata for the specified bucket.
///
/// A builder for the *get* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().get("bucket")
///              .user_project("accusam")
///              .provisional_user_project("voluptua.")
///              .projection("dolore")
///              .if_metageneration_not_match("dolore")
///              .if_metageneration_match("dolore")
///              .doit().await;
/// # }
/// ```
pub struct BucketGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketGetCall<'a> {}

impl<'a> BucketGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketGetCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns an IAM policy for the specified bucket.
///
/// A builder for the *getIamPolicy* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().get_iam_policy("bucket")
///              .user_project("amet.")
///              .provisional_user_project("ea")
///              .options_requested_policy_version(-95)
///              .doit().await;
/// # }
/// ```
pub struct BucketGetIamPolicyCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _options_requested_policy_version: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketGetIamPolicyCall<'a> {}

impl<'a> BucketGetIamPolicyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.getIamPolicy",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._options_requested_policy_version {
            params.push(("optionsRequestedPolicyVersion", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "optionsRequestedPolicyVersion"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketGetIamPolicyCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketGetIamPolicyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketGetIamPolicyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The IAM policy format version to be returned. If the optionsRequestedPolicyVersion is for an older version that doesn't support part of the requested IAM policy, the request fails.
    ///
    /// Sets the *options requested policy version* query property to the given value.
    pub fn options_requested_policy_version(mut self, new_value: i32) -> BucketGetIamPolicyCall<'a> {
        self._options_requested_policy_version = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketGetIamPolicyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketGetIamPolicyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketGetIamPolicyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new bucket.
///
/// A builder for the *insert* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Bucket;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().insert(req, "project")
///              .user_project("invidunt")
///              .provisional_user_project("no")
///              .projection("est")
///              .predefined_default_object_acl("At")
///              .predefined_acl("sed")
///              .doit().await;
/// # }
/// ```
pub struct BucketInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Bucket,
    _project: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketInsertCall<'a> {}

impl<'a> BucketInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        for &field in ["alt", "project", "userProject", "provisionalUserProject", "projection", "predefinedDefaultObjectAcl", "predefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Bucket) -> BucketInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// A valid API project identifier.
    ///
    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the bucket resource specifies acl or defaultObjectAcl properties, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketInsertCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of buckets for a given project.
///
/// A builder for the *list* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().list("project")
///              .user_project("et")
///              .provisional_user_project("tempor")
///              .projection("aliquyam")
///              .prefix("ipsum")
///              .page_token("et")
///              .max_results(93)
///              .doit().await;
/// # }
/// ```
pub struct BucketListCall<'a>
    where  {

    hub: &'a Storage<>,
    _project: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketListCall<'a> {}

impl<'a> BucketListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Buckets)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "project", "userProject", "provisionalUserProject", "projection", "prefix", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// A valid API project identifier.
    ///
    /// Sets the *project* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> BucketListCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketListCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to buckets whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> BucketListCall<'a> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> BucketListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of buckets to return in a single response. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> BucketListCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Locks retention policy on a bucket.
///
/// A builder for the *lockRetentionPolicy* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().lock_retention_policy("bucket", "ifMetagenerationMatch")
///              .user_project("sed")
///              .provisional_user_project("diam")
///              .doit().await;
/// # }
/// ```
pub struct BucketLockRetentionPolicyCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _if_metageneration_match: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketLockRetentionPolicyCall<'a> {}

impl<'a> BucketLockRetentionPolicyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.lockRetentionPolicy",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("ifMetagenerationMatch", self._if_metageneration_match.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "ifMetagenerationMatch", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/lockRetentionPolicy";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketLockRetentionPolicyCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Makes the operation conditional on whether bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketLockRetentionPolicyCall<'a> {
        self._if_metageneration_match = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketLockRetentionPolicyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketLockRetentionPolicyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketLockRetentionPolicyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketLockRetentionPolicyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketLockRetentionPolicyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Patches a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
///
/// A builder for the *patch* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Bucket;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().patch(req, "bucket")
///              .user_project("dolores")
///              .provisional_user_project("et")
///              .projection("sed")
///              .predefined_default_object_acl("no")
///              .predefined_acl("et")
///              .if_metageneration_not_match("elitr")
///              .if_metageneration_match("sed")
///              .doit().await;
/// # }
/// ```
pub struct BucketPatchCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Bucket,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketPatchCall<'a> {}

impl<'a> BucketPatchCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Bucket) -> BucketPatchCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketPatchCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketPatchCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketPatchCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketPatchCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an IAM policy for the specified bucket.
///
/// A builder for the *setIamPolicy* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Policy;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Policy::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().set_iam_policy(req, "bucket")
///              .user_project("nonumy")
///              .provisional_user_project("At")
///              .doit().await;
/// # }
/// ```
pub struct BucketSetIamPolicyCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Policy,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketSetIamPolicyCall<'a> {}

impl<'a> BucketSetIamPolicyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.setIamPolicy",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Policy) -> BucketSetIamPolicyCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketSetIamPolicyCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketSetIamPolicyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketSetIamPolicyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketSetIamPolicyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketSetIamPolicyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketSetIamPolicyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Tests a set of permissions on the given bucket to see which, if any, are held by the caller.
///
/// A builder for the *testIamPermissions* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().test_iam_permissions("bucket", &vec!["aliquyam".into()])
///              .user_project("dolores")
///              .provisional_user_project("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct BucketTestIamPermissionCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _permissions: Vec<String>,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketTestIamPermissionCall<'a> {}

impl<'a> BucketTestIamPermissionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.testIamPermissions",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if self._permissions.len() > 0 {
            for f in self._permissions.iter() {
                params.push(("permissions", f.to_string()));
            }
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "permissions", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/iam/testPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Permissions to test.
    ///
    /// Append the given value to the *permissions* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_permissions(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a> {
        self._permissions.push(new_value.to_string());
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketTestIamPermissionCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketTestIamPermissionCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketTestIamPermissionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketTestIamPermissionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a bucket. Changes to the bucket will be readable immediately after writing, but configuration changes may take time to propagate.
///
/// A builder for the *update* method supported by a *bucket* resource.
/// It is not used directly, but through a `BucketMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Bucket;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Bucket::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.buckets().update(req, "bucket")
///              .user_project("aliquyam")
///              .provisional_user_project("amet")
///              .projection("est")
///              .predefined_default_object_acl("et")
///              .predefined_acl("sea")
///              .if_metageneration_not_match("consetetur")
///              .if_metageneration_match("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct BucketUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Bucket,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_default_object_acl: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for BucketUpdateCall<'a> {}

impl<'a> BucketUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Bucket)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.buckets.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_default_object_acl {
            params.push(("predefinedDefaultObjectAcl", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "projection", "predefinedDefaultObjectAcl", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Bucket) -> BucketUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of default object access controls to this bucket.
    ///
    /// Sets the *predefined default object acl* query property to the given value.
    pub fn predefined_default_object_acl(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._predefined_default_object_acl = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this bucket.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the return of the bucket metadata conditional on whether the bucket's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> BucketUpdateCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> BucketUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> BucketUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> BucketUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Stop watching resources through this channel
///
/// A builder for the *stop* method supported by a *channel* resource.
/// It is not used directly, but through a `ChannelMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Channel;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.channels().stop(req)
///              .doit().await;
/// # }
/// ```
pub struct ChannelStopCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Channel,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ChannelStopCall<'a> {}

impl<'a> ChannelStopCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.channels.stop",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(2 + self._additional_params.len());
        for &field in [].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "channels/stop";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }


        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ChannelStopCall<'a> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ChannelStopCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ChannelStopCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ChannelStopCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *delete* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().delete("bucket", "entity")
///              .user_project("aliquyam")
///              .provisional_user_project("elitr")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlDeleteCall<'a> {}

impl<'a> DefaultObjectAccessControlDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the default object ACL entry for the specified entity on the specified bucket.
///
/// A builder for the *get* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().get("bucket", "entity")
///              .user_project("est")
///              .provisional_user_project("sit")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlGetCall<'a> {}

impl<'a> DefaultObjectAccessControlGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new default object ACL entry on the specified bucket.
///
/// A builder for the *insert* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().insert(req, "bucket")
///              .user_project("eos")
///              .provisional_user_project("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlInsertCall<'a> {}

impl<'a> DefaultObjectAccessControlInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves default object ACL entries on the specified bucket.
///
/// A builder for the *list* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().list("bucket")
///              .user_project("Stet")
///              .provisional_user_project("dolores")
///              .if_metageneration_not_match("eos")
///              .if_metageneration_match("et")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlListCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlListCall<'a> {}

impl<'a> DefaultObjectAccessControlListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, only return default ACL listing if the bucket's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// If present, only return default ACL listing if the bucket's current metageneration matches this value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> DefaultObjectAccessControlListCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Patches a default object ACL entry on the specified bucket.
///
/// A builder for the *patch* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().patch(req, "bucket", "entity")
///              .user_project("At")
///              .provisional_user_project("dolore")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlPatchCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlPatchCall<'a> {}

impl<'a> DefaultObjectAccessControlPatchCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlPatchCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlPatchCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlPatchCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlPatchCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlPatchCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates a default object ACL entry on the specified bucket.
///
/// A builder for the *update* method supported by a *defaultObjectAccessControl* resource.
/// It is not used directly, but through a `DefaultObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.default_object_access_controls().update(req, "bucket", "entity")
///              .user_project("accusam")
///              .provisional_user_project("amet")
///              .doit().await;
/// # }
/// ```
pub struct DefaultObjectAccessControlUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for DefaultObjectAccessControlUpdateCall<'a> {}

impl<'a> DefaultObjectAccessControlUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.defaultObjectAccessControls.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "entity", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/defaultObjectAcl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["entity", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DefaultObjectAccessControlUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> DefaultObjectAccessControlUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> DefaultObjectAccessControlUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes a notification subscription.
///
/// A builder for the *delete* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().delete("bucket", "notification")
///              .user_project("erat")
///              .provisional_user_project("accusam")
///              .doit().await;
/// # }
/// ```
pub struct NotificationDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _notification: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NotificationDeleteCall<'a> {}

impl<'a> NotificationDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.notifications.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("notification", self._notification.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["bucket", "notification", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs/{notification}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{notification}", "notification")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["notification", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// ID of the notification to delete.
    ///
    /// Sets the *notification* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification(mut self, new_value: &str) -> NotificationDeleteCall<'a> {
        self._notification = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> NotificationDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NotificationDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> NotificationDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// View a notification configuration.
///
/// A builder for the *get* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().get("bucket", "notification")
///              .user_project("Lorem")
///              .provisional_user_project("et")
///              .doit().await;
/// # }
/// ```
pub struct NotificationGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _notification: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NotificationGetCall<'a> {}

impl<'a> NotificationGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.notifications.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("notification", self._notification.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "notification", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs/{notification}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{notification}", "notification")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["notification", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Notification ID
    ///
    /// Sets the *notification* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn notification(mut self, new_value: &str) -> NotificationGetCall<'a> {
        self._notification = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> NotificationGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NotificationGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> NotificationGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a notification subscription for a given bucket.
///
/// A builder for the *insert* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Notification;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Notification::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().insert(req, "bucket")
///              .user_project("dolor")
///              .provisional_user_project("et")
///              .doit().await;
/// # }
/// ```
pub struct NotificationInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Notification,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NotificationInsertCall<'a> {}

impl<'a> NotificationInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Notification)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.notifications.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Notification) -> NotificationInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// The parent bucket of the notification.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationInsertCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> NotificationInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NotificationInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> NotificationInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of notification subscriptions for a given bucket.
///
/// A builder for the *list* method supported by a *notification* resource.
/// It is not used directly, but through a `NotificationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.notifications().list("bucket")
///              .user_project("erat")
///              .provisional_user_project("sea")
///              .doit().await;
/// # }
/// ```
pub struct NotificationListCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for NotificationListCall<'a> {}

impl<'a> NotificationListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Notifications)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.notifications.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/notificationConfigs";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a Google Cloud Storage bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> NotificationListCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> NotificationListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> NotificationListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> NotificationListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> NotificationListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> NotificationListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Permanently deletes the ACL entry for the specified entity on the specified object.
///
/// A builder for the *delete* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().delete("bucket", "object", "entity")
///              .user_project("justo")
///              .provisional_user_project("sea")
///              .generation("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlDeleteCall<'a> {}

impl<'a> ObjectAccessControlDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "entity", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["entity", "object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlDeleteCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the ACL entry for the specified entity on the specified object.
///
/// A builder for the *get* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().get("bucket", "object", "entity")
///              .user_project("At")
///              .provisional_user_project("dolores")
///              .generation("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlGetCall<'a> {}

impl<'a> ObjectAccessControlGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["entity", "object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlGetCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new ACL entry on the specified object.
///
/// A builder for the *insert* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().insert(req, "bucket", "object")
///              .user_project("aliquyam")
///              .provisional_user_project("no")
///              .generation("amet.")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlInsertCall<'a> {}

impl<'a> ObjectAccessControlInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlInsertCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves ACL entries on the specified object.
///
/// A builder for the *list* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().list("bucket", "object")
///              .user_project("accusam")
///              .provisional_user_project("gubergren")
///              .generation("sadipscing")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlListCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlListCall<'a> {}

impl<'a> ObjectAccessControlListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControls)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlListCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlListCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlListCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Patches an ACL entry on the specified object.
///
/// A builder for the *patch* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().patch(req, "bucket", "object", "entity")
///              .user_project("sit")
///              .provisional_user_project("magna")
///              .generation("et")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlPatchCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlPatchCall<'a> {}

impl<'a> ObjectAccessControlPatchCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["entity", "object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlPatchCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlPatchCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlPatchCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlPatchCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlPatchCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an ACL entry on the specified object.
///
/// A builder for the *update* method supported by a *objectAccessControl* resource.
/// It is not used directly, but through a `ObjectAccessControlMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ObjectAccessControl;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ObjectAccessControl::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.object_access_controls().update(req, "bucket", "object", "entity")
///              .user_project("justo")
///              .provisional_user_project("amet.")
///              .generation("no")
///              .doit().await;
/// # }
/// ```
pub struct ObjectAccessControlUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ObjectAccessControl,
    _bucket: String,
    _object: String,
    _entity: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectAccessControlUpdateCall<'a> {}

impl<'a> ObjectAccessControlUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ObjectAccessControl)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objectAccessControls.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(9 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        params.push(("entity", self._entity.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "entity", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/acl/{entity}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object"), ("{entity}", "entity")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(3);
            for param_name in ["entity", "object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ObjectAccessControl) -> ObjectAccessControlUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of a bucket.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    ///
    /// Sets the *entity* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn entity(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._entity = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectAccessControlUpdateCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectAccessControlUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectAccessControlUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectAccessControlUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Concatenates a list of existing objects into a new object in the same bucket.
///
/// A builder for the *compose* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::ComposeRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ComposeRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().compose(req, "destinationBucket", "destinationObject")
///              .user_project("kasd")
///              .provisional_user_project("Lorem")
///              .kms_key_name("sanctus")
///              .if_metageneration_match("nonumy")
///              .if_generation_match("rebum.")
///              .destination_predefined_acl("tempor")
///              .doit().await;
/// # }
/// ```
pub struct ObjectComposeCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: ComposeRequest,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _kms_key_name: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectComposeCall<'a> {}

impl<'a> ObjectComposeCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.compose",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._kms_key_name {
            params.push(("kmsKeyName", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        for &field in ["alt", "destinationBucket", "destinationObject", "userProject", "provisionalUserProject", "kmsKeyName", "ifMetagenerationMatch", "ifGenerationMatch", "destinationPredefinedAcl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{destinationBucket}/o/{destinationObject}/compose";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{destinationBucket}", "destinationBucket"), ("{destinationObject}", "destinationObject")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["destinationObject", "destinationBucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ComposeRequest) -> ObjectComposeCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket containing the source objects. The destination object is stored in this bucket.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any.
    ///
    /// Sets the *kms key name* query property to the given value.
    pub fn kms_key_name(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._kms_key_name = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectComposeCall<'a> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectComposeCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectComposeCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectComposeCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Copies a source object to a destination object. Optionally overrides metadata.
///
/// A builder for the *copy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().copy(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("amet")
///              .source_generation("ut")
///              .provisional_user_project("At")
///              .projection("sit")
///              .if_source_metageneration_not_match("vero")
///              .if_source_metageneration_match("duo")
///              .if_source_generation_not_match("sadipscing")
///              .if_source_generation_match("ut")
///              .if_metageneration_not_match("rebum.")
///              .if_metageneration_match("duo")
///              .if_generation_not_match("kasd")
///              .if_generation_match("sadipscing")
///              .destination_predefined_acl("tempor")
///              .destination_kms_key_name("sea")
///              .doit().await;
/// # }
/// ```
pub struct ObjectCopyCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Object,
    _source_bucket: String,
    _source_object: String,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
    _source_generation: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _if_source_metageneration_not_match: Option<String>,
    _if_source_metageneration_match: Option<String>,
    _if_source_generation_not_match: Option<String>,
    _if_source_generation_match: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _destination_kms_key_name: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectCopyCall<'a> {}

impl<'a> ObjectCopyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.copy",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(21 + self._additional_params.len());
        params.push(("sourceBucket", self._source_bucket.to_string()));
        params.push(("sourceObject", self._source_object.to_string()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._source_generation {
            params.push(("sourceGeneration", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_not_match {
            params.push(("ifSourceMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_match {
            params.push(("ifSourceMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_not_match {
            params.push(("ifSourceGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_match {
            params.push(("ifSourceGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        if let Some(value) = self._destination_kms_key_name {
            params.push(("destinationKmsKeyName", value.to_string()));
        }
        for &field in ["alt", "sourceBucket", "sourceObject", "destinationBucket", "destinationObject", "userProject", "sourceGeneration", "provisionalUserProject", "projection", "ifSourceMetagenerationNotMatch", "ifSourceMetagenerationMatch", "ifSourceGenerationNotMatch", "ifSourceGenerationMatch", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "destinationPredefinedAcl", "destinationKmsKeyName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{sourceBucket}/o/{sourceObject}/copyTo/b/{destinationBucket}/o/{destinationObject}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{sourceBucket}", "sourceBucket"), ("{sourceObject}", "sourceObject"), ("{destinationBucket}", "destinationBucket"), ("{destinationObject}", "destinationObject")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(4);
            for param_name in ["destinationObject", "destinationBucket", "sourceObject", "sourceBucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectCopyCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to find the source object.
    ///
    /// Sets the *source bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._source_bucket = new_value.to_string();
        self
    }
    /// Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *source object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_object(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._source_object = new_value.to_string();
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    ///
    /// Sets the *source generation* query property to the given value.
    pub fn source_generation(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._source_generation = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    ///
    /// Sets the *if source metageneration not match* query property to the given value.
    pub fn if_source_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_source_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.
    ///
    /// Sets the *if source metageneration match* query property to the given value.
    pub fn if_source_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_source_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current generation does not match the given value.
    ///
    /// Sets the *if source generation not match* query property to the given value.
    pub fn if_source_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_source_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current generation matches the given value.
    ///
    /// Sets the *if source generation match* query property to the given value.
    pub fn if_source_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_source_generation_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any.
    ///
    /// Sets the *destination kms key name* query property to the given value.
    pub fn destination_kms_key_name(mut self, new_value: &str) -> ObjectCopyCall<'a> {
        self._destination_kms_key_name = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectCopyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectCopyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectCopyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes an object and its metadata. Deletions are permanent if versioning is not enabled for the bucket, or if the generation parameter is used.
///
/// A builder for the *delete* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().delete("bucket", "object")
///              .user_project("magna")
///              .provisional_user_project("takimata")
///              .if_metageneration_not_match("rebum.")
///              .if_metageneration_match("At")
///              .if_generation_not_match("invidunt")
///              .if_generation_match("clita")
///              .generation("Stet")
///              .doit().await;
/// # }
/// ```
pub struct ObjectDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectDeleteCall<'a> {}

impl<'a> ObjectDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "userProject", "provisionalUserProject", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, permanently deletes a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectDeleteCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves an object or its metadata.
///
/// This method supports **media download**. To enable it, adjust the builder like this:
/// `.param("alt", "media")`.
/// Please note that due to missing multi-part support on the server side, you will only receive the media,
/// but not the `Object` structure that you would usually get. The latter will be a default value.
///
/// A builder for the *get* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().get("bucket", "object")
///              .user_project("sit")
///              .provisional_user_project("vero")
///              .projection("rebum.")
///              .if_metageneration_not_match("dolores")
///              .if_metageneration_match("consetetur")
///              .if_generation_not_match("dolores")
///              .if_generation_match("sed")
///              .generation("invidunt")
///              .doit().await;
/// # }
/// ```
pub struct ObjectGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectGetCall<'a> {}

impl<'a> ObjectGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(11 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["bucket", "object", "userProject", "provisionalUserProject", "projection", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        let (json_field_missing, enable_resource_parsing) = {
            let mut enable = true;
            let mut field_present = true;
            for &(name, ref value) in params.iter() {
                if name == "alt" {
                    field_present = false;
                    if <String as AsRef<str>>::as_ref(&value) != "json" {
                        enable = false;
                    }
                    break;
                }
            }
            (field_present, enable)
        };
        if json_field_missing {
            params.push(("alt", "json".to_string()));
        }

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = if enable_resource_parsing {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    } else { (res, Default::default()) };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectGetCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns an IAM policy for the specified object.
///
/// A builder for the *getIamPolicy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().get_iam_policy("bucket", "object")
///              .user_project("aliquyam")
///              .provisional_user_project("magna")
///              .generation("diam")
///              .doit().await;
/// # }
/// ```
pub struct ObjectGetIamPolicyCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectGetIamPolicyCall<'a> {}

impl<'a> ObjectGetIamPolicyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.getIamPolicy",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectGetIamPolicyCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectGetIamPolicyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectGetIamPolicyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectGetIamPolicyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Stores a new object and metadata.
///
/// A builder for the *insert* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload_resumable(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().insert(req, "bucket")
///              .user_project("et")
///              .provisional_user_project("sanctus")
///              .projection("accusam")
///              .predefined_acl("tempor")
///              .name("sed")
///              .kms_key_name("est")
///              .if_metageneration_not_match("takimata")
///              .if_metageneration_match("dolor")
///              .if_generation_not_match("diam")
///              .if_generation_match("At")
///              .content_encoding("erat")
///              .upload_resumable(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct ObjectInsertCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Object,
    _bucket: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _name: Option<String>,
    _kms_key_name: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _content_encoding: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectInsertCall<'a> {}

impl<'a> ObjectInsertCall<'a> {


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> client::Result<(hyper::Response<hyper::body::Body>, Object)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._name {
            params.push(("name", value.to_string()));
        }
        if let Some(value) = self._kms_key_name {
            params.push(("kmsKeyName", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._content_encoding {
            params.push(("contentEncoding", value.to_string()));
        }
        for &field in ["alt", "bucket", "userProject", "provisionalUserProject", "projection", "predefinedAcl", "name", "kmsKeyName", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "contentEncoding"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/storage/v1/b/{bucket}/o", "resumable")
            } else if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/storage/v1/b/{bucket}/o", "multipart")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Localtion", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let mut mp_reader: client::MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut dyn io::Read, (CONTENT_TYPE, mime_type.to_string()))
                        },
                        _ => (&mut request_value_reader as &mut dyn io::Read, (CONTENT_TYPE, json_mime_type.to_string())),
                    };
                    let client = &self.hub.client;
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                            .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));
    
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let mut body_reader_bytes = vec![];
                            body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                            let request = req_builder
                            .header(content_type.0, content_type.1.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes));
    
                    client.request(request.unwrap()).await
                    
                }
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &self.hub.client,
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &self.hub.auth,
                                user_agent: &self.hub._user_agent,
                                auth_header: format!("Bearer {}", token.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 0kb
    /// * *valid mime types*: '*/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Object)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable").await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 0kb
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, Object)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, "simple").await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Name of the object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any.
    ///
    /// Sets the *kms key name* query property to the given value.
    pub fn kms_key_name(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._kms_key_name = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If set, sets the contentEncoding property of the final object to this value. Setting this parameter is equivalent to setting the contentEncoding metadata property. This can be useful when uploading an object with uploadType=media to indicate the encoding of the content being uploaded.
    ///
    /// Sets the *content encoding* query property to the given value.
    pub fn content_encoding(mut self, new_value: &str) -> ObjectInsertCall<'a> {
        self._content_encoding = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectInsertCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectInsertCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of objects matching the criteria.
///
/// A builder for the *list* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().list("bucket")
///              .versions(false)
///              .user_project("dolores")
///              .start_offset("consetetur")
///              .provisional_user_project("no")
///              .projection("justo")
///              .prefix("sadipscing")
///              .page_token("diam")
///              .max_results(91)
///              .include_trailing_delimiter(true)
///              .end_offset("Stet")
///              .delimiter("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct ObjectListCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _versions: Option<bool>,
    _user_project: Option<String>,
    _start_offset: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _include_trailing_delimiter: Option<bool>,
    _end_offset: Option<String>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectListCall<'a> {}

impl<'a> ObjectListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Objects)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._start_offset {
            params.push(("startOffset", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._include_trailing_delimiter {
            params.push(("includeTrailingDelimiter", value.to_string()));
        }
        if let Some(value) = self._end_offset {
            params.push(("endOffset", value.to_string()));
        }
        if let Some(value) = self._delimiter {
            params.push(("delimiter", value.to_string()));
        }
        for &field in ["alt", "bucket", "versions", "userProject", "startOffset", "provisionalUserProject", "projection", "prefix", "pageToken", "maxResults", "includeTrailingDelimiter", "endOffset", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which to look for objects.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning.
    ///
    /// Sets the *versions* query property to the given value.
    pub fn versions(mut self, new_value: bool) -> ObjectListCall<'a> {
        self._versions = Some(new_value);
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names are lexicographically equal to or after startOffset. If endOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive).
    ///
    /// Sets the *start offset* query property to the given value.
    pub fn start_offset(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._start_offset = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ObjectListCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes.
    ///
    /// Sets the *include trailing delimiter* query property to the given value.
    pub fn include_trailing_delimiter(mut self, new_value: bool) -> ObjectListCall<'a> {
        self._include_trailing_delimiter = Some(new_value);
        self
    }
    /// Filter results to objects whose names are lexicographically before endOffset. If startOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive).
    ///
    /// Sets the *end offset* query property to the given value.
    pub fn end_offset(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._end_offset = Some(new_value.to_string());
        self
    }
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.
    ///
    /// Sets the *delimiter* query property to the given value.
    pub fn delimiter(mut self, new_value: &str) -> ObjectListCall<'a> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Patches an object's metadata.
///
/// A builder for the *patch* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().patch(req, "bucket", "object")
///              .user_project("sit")
///              .provisional_user_project("kasd")
///              .projection("amet")
///              .predefined_acl("Lorem")
///              .if_metageneration_not_match("justo")
///              .if_metageneration_match("invidunt")
///              .if_generation_not_match("sed")
///              .if_generation_match("nonumy")
///              .generation("sea")
///              .doit().await;
/// # }
/// ```
pub struct ObjectPatchCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Object,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectPatchCall<'a> {}

impl<'a> ObjectPatchCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.patch",
                               http_method: hyper::Method::PATCH });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PATCH).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectPatchCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request, for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectPatchCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectPatchCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectPatchCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectPatchCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Rewrites a source object to a destination object. Optionally overrides metadata.
///
/// A builder for the *rewrite* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().rewrite(req, "sourceBucket", "sourceObject", "destinationBucket", "destinationObject")
///              .user_project("At")
///              .source_generation("erat")
///              .rewrite_token("clita")
///              .provisional_user_project("vero")
///              .projection("invidunt")
///              .max_bytes_rewritten_per_call("nonumy")
///              .if_source_metageneration_not_match("erat")
///              .if_source_metageneration_match("erat")
///              .if_source_generation_not_match("dolores")
///              .if_source_generation_match("ipsum")
///              .if_metageneration_not_match("voluptua.")
///              .if_metageneration_match("eos")
///              .if_generation_not_match("duo")
///              .if_generation_match("elitr")
///              .destination_predefined_acl("consetetur")
///              .destination_kms_key_name("et")
///              .doit().await;
/// # }
/// ```
pub struct ObjectRewriteCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Object,
    _source_bucket: String,
    _source_object: String,
    _destination_bucket: String,
    _destination_object: String,
    _user_project: Option<String>,
    _source_generation: Option<String>,
    _rewrite_token: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _max_bytes_rewritten_per_call: Option<String>,
    _if_source_metageneration_not_match: Option<String>,
    _if_source_metageneration_match: Option<String>,
    _if_source_generation_not_match: Option<String>,
    _if_source_generation_match: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _destination_predefined_acl: Option<String>,
    _destination_kms_key_name: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectRewriteCall<'a> {}

impl<'a> ObjectRewriteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, RewriteResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.rewrite",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(23 + self._additional_params.len());
        params.push(("sourceBucket", self._source_bucket.to_string()));
        params.push(("sourceObject", self._source_object.to_string()));
        params.push(("destinationBucket", self._destination_bucket.to_string()));
        params.push(("destinationObject", self._destination_object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._source_generation {
            params.push(("sourceGeneration", value.to_string()));
        }
        if let Some(value) = self._rewrite_token {
            params.push(("rewriteToken", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._max_bytes_rewritten_per_call {
            params.push(("maxBytesRewrittenPerCall", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_not_match {
            params.push(("ifSourceMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_metageneration_match {
            params.push(("ifSourceMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_not_match {
            params.push(("ifSourceGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_source_generation_match {
            params.push(("ifSourceGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._destination_predefined_acl {
            params.push(("destinationPredefinedAcl", value.to_string()));
        }
        if let Some(value) = self._destination_kms_key_name {
            params.push(("destinationKmsKeyName", value.to_string()));
        }
        for &field in ["alt", "sourceBucket", "sourceObject", "destinationBucket", "destinationObject", "userProject", "sourceGeneration", "rewriteToken", "provisionalUserProject", "projection", "maxBytesRewrittenPerCall", "ifSourceMetagenerationNotMatch", "ifSourceMetagenerationMatch", "ifSourceGenerationNotMatch", "ifSourceGenerationMatch", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "destinationPredefinedAcl", "destinationKmsKeyName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{sourceBucket}/o/{sourceObject}/rewriteTo/b/{destinationBucket}/o/{destinationObject}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{sourceBucket}", "sourceBucket"), ("{sourceObject}", "sourceObject"), ("{destinationBucket}", "destinationBucket"), ("{destinationObject}", "destinationObject")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(4);
            for param_name in ["destinationObject", "destinationBucket", "sourceObject", "sourceBucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectRewriteCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to find the source object.
    ///
    /// Sets the *source bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_bucket(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._source_bucket = new_value.to_string();
        self
    }
    /// Name of the source object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *source object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn source_object(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._source_object = new_value.to_string();
        self
    }
    /// Name of the bucket in which to store the new object. Overrides the provided object metadata's bucket value, if any.
    ///
    /// Sets the *destination bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_bucket(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._destination_bucket = new_value.to_string();
        self
    }
    /// Name of the new object. Required when the object metadata is not otherwise provided. Overrides the object metadata's name value, if any. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *destination object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn destination_object(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._destination_object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of the source object (as opposed to the latest version, the default).
    ///
    /// Sets the *source generation* query property to the given value.
    pub fn source_generation(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._source_generation = Some(new_value.to_string());
        self
    }
    /// Include this field (from the previous rewrite response) on each rewrite request after the first one, until the rewrite response 'done' flag is true. Calls that provide a rewriteToken can omit all other request fields, but if included those fields must match the values provided in the first rewrite request.
    ///
    /// Sets the *rewrite token* query property to the given value.
    pub fn rewrite_token(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._rewrite_token = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl, unless the object resource specifies the acl property, when it defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// The maximum number of bytes that will be rewritten per rewrite request. Most callers shouldn't need to specify this parameter - it is primarily in place to support testing. If specified the value must be an integral multiple of 1 MiB (1048576). Also, this only applies to requests where the source and destination span locations and/or storage classes. Finally, this value must not change across rewrite calls else you'll get an error that the rewriteToken is invalid.
    ///
    /// Sets the *max bytes rewritten per call* query property to the given value.
    pub fn max_bytes_rewritten_per_call(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._max_bytes_rewritten_per_call = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration does not match the given value.
    ///
    /// Sets the *if source metageneration not match* query property to the given value.
    pub fn if_source_metageneration_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_source_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current metageneration matches the given value.
    ///
    /// Sets the *if source metageneration match* query property to the given value.
    pub fn if_source_metageneration_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_source_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current generation does not match the given value.
    ///
    /// Sets the *if source generation not match* query property to the given value.
    pub fn if_source_generation_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_source_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the source object's current generation matches the given value.
    ///
    /// Sets the *if source generation match* query property to the given value.
    pub fn if_source_generation_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_source_generation_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the destination object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to the destination object.
    ///
    /// Sets the *destination predefined acl* query property to the given value.
    pub fn destination_predefined_acl(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._destination_predefined_acl = Some(new_value.to_string());
        self
    }
    /// Resource name of the Cloud KMS key, of the form projects/my-project/locations/global/keyRings/my-kr/cryptoKeys/my-key, that will be used to encrypt the object. Overrides the object metadata's kms_key_name value, if any.
    ///
    /// Sets the *destination kms key name* query property to the given value.
    pub fn destination_kms_key_name(mut self, new_value: &str) -> ObjectRewriteCall<'a> {
        self._destination_kms_key_name = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectRewriteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectRewriteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectRewriteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an IAM policy for the specified object.
///
/// A builder for the *setIamPolicy* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Policy;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Policy::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().set_iam_policy(req, "bucket", "object")
///              .user_project("takimata")
///              .provisional_user_project("erat")
///              .generation("diam")
///              .doit().await;
/// # }
/// ```
pub struct ObjectSetIamPolicyCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Policy,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectSetIamPolicyCall<'a> {}

impl<'a> ObjectSetIamPolicyCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Policy)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.setIamPolicy",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Policy) -> ObjectSetIamPolicyCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectSetIamPolicyCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectSetIamPolicyCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectSetIamPolicyCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectSetIamPolicyCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Tests a set of permissions on the given object to see which, if any, are held by the caller.
///
/// A builder for the *testIamPermissions* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().test_iam_permissions("bucket", "object", &vec!["At".into()])
///              .user_project("diam")
///              .provisional_user_project("diam")
///              .generation("sed")
///              .doit().await;
/// # }
/// ```
pub struct ObjectTestIamPermissionCall<'a>
    where  {

    hub: &'a Storage<>,
    _bucket: String,
    _object: String,
    _permissions: Vec<String>,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectTestIamPermissionCall<'a> {}

impl<'a> ObjectTestIamPermissionCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, TestIamPermissionsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.testIamPermissions",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if self._permissions.len() > 0 {
            for f in self._permissions.iter() {
                params.push(("permissions", f.to_string()));
            }
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "permissions", "userProject", "provisionalUserProject", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}/iam/testPermissions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// Permissions to test.
    ///
    /// Append the given value to the *permissions* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_permissions(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._permissions.push(new_value.to_string());
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectTestIamPermissionCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectTestIamPermissionCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectTestIamPermissionCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectTestIamPermissionCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates an object's metadata.
///
/// A builder for the *update* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Object;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Object::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().update(req, "bucket", "object")
///              .user_project("dolore")
///              .provisional_user_project("ipsum")
///              .projection("ea")
///              .predefined_acl("At")
///              .if_metageneration_not_match("sit")
///              .if_metageneration_match("sit")
///              .if_generation_not_match("Lorem")
///              .if_generation_match("Stet")
///              .generation("duo")
///              .doit().await;
/// # }
/// ```
pub struct ObjectUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Object,
    _bucket: String,
    _object: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _predefined_acl: Option<String>,
    _if_metageneration_not_match: Option<String>,
    _if_metageneration_match: Option<String>,
    _if_generation_not_match: Option<String>,
    _if_generation_match: Option<String>,
    _generation: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectUpdateCall<'a> {}

impl<'a> ObjectUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Object)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(14 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        params.push(("object", self._object.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._predefined_acl {
            params.push(("predefinedAcl", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_not_match {
            params.push(("ifMetagenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_metageneration_match {
            params.push(("ifMetagenerationMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_not_match {
            params.push(("ifGenerationNotMatch", value.to_string()));
        }
        if let Some(value) = self._if_generation_match {
            params.push(("ifGenerationMatch", value.to_string()));
        }
        if let Some(value) = self._generation {
            params.push(("generation", value.to_string()));
        }
        for &field in ["alt", "bucket", "object", "userProject", "provisionalUserProject", "projection", "predefinedAcl", "ifMetagenerationNotMatch", "ifMetagenerationMatch", "ifGenerationNotMatch", "ifGenerationMatch", "generation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/{object}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket"), ("{object}", "object")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["object", "bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Object) -> ObjectUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which the object resides.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// Name of the object. For information about how to URL encode object names to be path safe, see Encoding URI Path Parts.
    ///
    /// Sets the *object* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn object(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._object = new_value.to_string();
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to full.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Apply a predefined set of access controls to this object.
    ///
    /// Sets the *predefined acl* query property to the given value.
    pub fn predefined_acl(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._predefined_acl = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration does not match the given value.
    ///
    /// Sets the *if metageneration not match* query property to the given value.
    pub fn if_metageneration_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._if_metageneration_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current metageneration matches the given value.
    ///
    /// Sets the *if metageneration match* query property to the given value.
    pub fn if_metageneration_match(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._if_metageneration_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation does not match the given value. If no live object exists, the precondition fails. Setting to 0 makes the operation succeed only if there is a live version of the object.
    ///
    /// Sets the *if generation not match* query property to the given value.
    pub fn if_generation_not_match(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._if_generation_not_match = Some(new_value.to_string());
        self
    }
    /// Makes the operation conditional on whether the object's current generation matches the given value. Setting to 0 makes the operation succeed only if there are no live versions of the object.
    ///
    /// Sets the *if generation match* query property to the given value.
    pub fn if_generation_match(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._if_generation_match = Some(new_value.to_string());
        self
    }
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    ///
    /// Sets the *generation* query property to the given value.
    pub fn generation(mut self, new_value: &str) -> ObjectUpdateCall<'a> {
        self._generation = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Watch for changes on all objects in a bucket.
///
/// A builder for the *watchAll* method supported by a *object* resource.
/// It is not used directly, but through a `ObjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::Channel;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Channel::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.objects().watch_all(req, "bucket")
///              .versions(false)
///              .user_project("et")
///              .start_offset("Lorem")
///              .provisional_user_project("rebum.")
///              .projection("et")
///              .prefix("sed")
///              .page_token("Stet")
///              .max_results(19)
///              .include_trailing_delimiter(false)
///              .end_offset("sit")
///              .delimiter("kasd")
///              .doit().await;
/// # }
/// ```
pub struct ObjectWatchAllCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: Channel,
    _bucket: String,
    _versions: Option<bool>,
    _user_project: Option<String>,
    _start_offset: Option<String>,
    _provisional_user_project: Option<String>,
    _projection: Option<String>,
    _prefix: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _include_trailing_delimiter: Option<bool>,
    _end_offset: Option<String>,
    _delimiter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ObjectWatchAllCall<'a> {}

impl<'a> ObjectWatchAllCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Channel)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.objects.watchAll",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(15 + self._additional_params.len());
        params.push(("bucket", self._bucket.to_string()));
        if let Some(value) = self._versions {
            params.push(("versions", value.to_string()));
        }
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._start_offset {
            params.push(("startOffset", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        if let Some(value) = self._projection {
            params.push(("projection", value.to_string()));
        }
        if let Some(value) = self._prefix {
            params.push(("prefix", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._include_trailing_delimiter {
            params.push(("includeTrailingDelimiter", value.to_string()));
        }
        if let Some(value) = self._end_offset {
            params.push(("endOffset", value.to_string()));
        }
        if let Some(value) = self._delimiter {
            params.push(("delimiter", value.to_string()));
        }
        for &field in ["alt", "bucket", "versions", "userProject", "startOffset", "provisionalUserProject", "projection", "prefix", "pageToken", "maxResults", "includeTrailingDelimiter", "endOffset", "delimiter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "b/{bucket}/o/watch";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{bucket}", "bucket")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["bucket"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Channel) -> ObjectWatchAllCall<'a> {
        self._request = new_value;
        self
    }
    /// Name of the bucket in which to look for objects.
    ///
    /// Sets the *bucket* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn bucket(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._bucket = new_value.to_string();
        self
    }
    /// If true, lists all versions of an object as distinct results. The default is false. For more information, see Object Versioning.
    ///
    /// Sets the *versions* query property to the given value.
    pub fn versions(mut self, new_value: bool) -> ObjectWatchAllCall<'a> {
        self._versions = Some(new_value);
        self
    }
    /// The project to be billed for this request. Required for Requester Pays buckets.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names are lexicographically equal to or after startOffset. If endOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive).
    ///
    /// Sets the *start offset* query property to the given value.
    pub fn start_offset(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._start_offset = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// Set of properties to return. Defaults to noAcl.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// Filter results to objects whose names begin with this prefix.
    ///
    /// Sets the *prefix* query property to the given value.
    pub fn prefix(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._prefix = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items plus prefixes to return in a single page of responses. As duplicate prefixes are omitted, fewer total results may be returned than requested. The service will use this parameter or 1,000 items, whichever is smaller.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ObjectWatchAllCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// If true, objects that end in exactly one instance of delimiter will have their metadata included in items in addition to prefixes.
    ///
    /// Sets the *include trailing delimiter* query property to the given value.
    pub fn include_trailing_delimiter(mut self, new_value: bool) -> ObjectWatchAllCall<'a> {
        self._include_trailing_delimiter = Some(new_value);
        self
    }
    /// Filter results to objects whose names are lexicographically before endOffset. If startOffset is also set, the objects listed will have names between startOffset (inclusive) and endOffset (exclusive).
    ///
    /// Sets the *end offset* query property to the given value.
    pub fn end_offset(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._end_offset = Some(new_value.to_string());
        self
    }
    /// Returns results in a directory-like mode. items will contain only objects whose names, aside from the prefix, do not contain delimiter. Objects whose names, aside from the prefix, contain delimiter will have their name, truncated after the delimiter, returned in prefixes. Duplicate prefixes are omitted.
    ///
    /// Sets the *delimiter* query property to the given value.
    pub fn delimiter(mut self, new_value: &str) -> ObjectWatchAllCall<'a> {
        self._delimiter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ObjectWatchAllCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ObjectWatchAllCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ObjectWatchAllCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates a new HMAC key for the specified service account.
///
/// A builder for the *hmacKeys.create* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().hmac_keys_create("projectId", "serviceAccountEmail")
///              .user_project("amet")
///              .doit().await;
/// # }
/// ```
pub struct ProjectHmacKeyCreateCall<'a>
    where  {

    hub: &'a Storage<>,
    _project_id: String,
    _service_account_email: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectHmacKeyCreateCall<'a> {}

impl<'a> ProjectHmacKeyCreateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, HmacKey)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.hmacKeys.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("serviceAccountEmail", self._service_account_email.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "projectId", "serviceAccountEmail", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/hmacKeys";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID owning the service account.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectHmacKeyCreateCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// Email address of the service account.
    ///
    /// Sets the *service account email* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn service_account_email(mut self, new_value: &str) -> ProjectHmacKeyCreateCall<'a> {
        self._service_account_email = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectHmacKeyCreateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectHmacKeyCreateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectHmacKeyCreateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectHmacKeyCreateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes an HMAC key.
///
/// A builder for the *hmacKeys.delete* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().hmac_keys_delete("projectId", "accessId")
///              .user_project("sea")
///              .doit().await;
/// # }
/// ```
pub struct ProjectHmacKeyDeleteCall<'a>
    where  {

    hub: &'a Storage<>,
    _project_id: String,
    _access_id: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectHmacKeyDeleteCall<'a> {}

impl<'a> ProjectHmacKeyDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<hyper::Response<hyper::body::Body>> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.hmacKeys.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("accessId", self._access_id.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["projectId", "accessId", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }


        let mut url = self.hub._base_url.clone() + "projects/{projectId}/hmacKeys/{accessId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{accessId}", "accessId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["accessId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::DELETE).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = res;

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID owning the requested key
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectHmacKeyDeleteCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// Name of the HMAC key to be deleted.
    ///
    /// Sets the *access id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn access_id(mut self, new_value: &str) -> ProjectHmacKeyDeleteCall<'a> {
        self._access_id = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectHmacKeyDeleteCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectHmacKeyDeleteCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectHmacKeyDeleteCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectHmacKeyDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves an HMAC key's metadata
///
/// A builder for the *hmacKeys.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().hmac_keys_get("projectId", "accessId")
///              .user_project("et")
///              .doit().await;
/// # }
/// ```
pub struct ProjectHmacKeyGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _project_id: String,
    _access_id: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectHmacKeyGetCall<'a> {}

impl<'a> ProjectHmacKeyGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, HmacKeyMetadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.hmacKeys.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("accessId", self._access_id.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "projectId", "accessId", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/hmacKeys/{accessId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{accessId}", "accessId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["accessId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID owning the service account of the requested key.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectHmacKeyGetCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// Name of the HMAC key.
    ///
    /// Sets the *access id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn access_id(mut self, new_value: &str) -> ProjectHmacKeyGetCall<'a> {
        self._access_id = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectHmacKeyGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectHmacKeyGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectHmacKeyGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectHmacKeyGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves a list of HMAC keys matching the criteria.
///
/// A builder for the *hmacKeys.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().hmac_keys_list("projectId")
///              .user_project("eirmod")
///              .show_deleted_keys(false)
///              .service_account_email("dolor")
///              .page_token("dolor")
///              .max_results(55)
///              .doit().await;
/// # }
/// ```
pub struct ProjectHmacKeyListCall<'a>
    where  {

    hub: &'a Storage<>,
    _project_id: String,
    _user_project: Option<String>,
    _show_deleted_keys: Option<bool>,
    _service_account_email: Option<String>,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectHmacKeyListCall<'a> {}

impl<'a> ProjectHmacKeyListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, HmacKeysMetadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.hmacKeys.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._show_deleted_keys {
            params.push(("showDeletedKeys", value.to_string()));
        }
        if let Some(value) = self._service_account_email {
            params.push(("serviceAccountEmail", value.to_string()));
        }
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        for &field in ["alt", "projectId", "userProject", "showDeletedKeys", "serviceAccountEmail", "pageToken", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/hmacKeys";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Name of the project in which to look for HMAC keys.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectHmacKeyListCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectHmacKeyListCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// Whether or not to show keys in the DELETED state.
    ///
    /// Sets the *show deleted keys* query property to the given value.
    pub fn show_deleted_keys(mut self, new_value: bool) -> ProjectHmacKeyListCall<'a> {
        self._show_deleted_keys = Some(new_value);
        self
    }
    /// If present, only keys for the given service account are returned.
    ///
    /// Sets the *service account email* query property to the given value.
    pub fn service_account_email(mut self, new_value: &str) -> ProjectHmacKeyListCall<'a> {
        self._service_account_email = Some(new_value.to_string());
        self
    }
    /// A previously-returned page token representing part of the larger set of results to view.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectHmacKeyListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of items to return in a single page of responses. The service uses this parameter or 250 items, whichever is smaller. The max number of items per page will also be limited by the number of distinct service accounts in the response. If the number of service accounts in a single response is too high, the page will truncated and a next page token will be returned.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ProjectHmacKeyListCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectHmacKeyListCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectHmacKeyListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectHmacKeyListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Updates the state of an HMAC key. See the HMAC Key resource descriptor for valid states.
///
/// A builder for the *hmacKeys.update* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// use storage1::api::HmacKeyMetadata;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = HmacKeyMetadata::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().hmac_keys_update(req, "projectId", "accessId")
///              .user_project("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct ProjectHmacKeyUpdateCall<'a>
    where  {

    hub: &'a Storage<>,
    _request: HmacKeyMetadata,
    _project_id: String,
    _access_id: String,
    _user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectHmacKeyUpdateCall<'a> {}

impl<'a> ProjectHmacKeyUpdateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, HmacKeyMetadata)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.hmacKeys.update",
                               http_method: hyper::Method::PUT });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        params.push(("accessId", self._access_id.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        for &field in ["alt", "projectId", "accessId", "userProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/hmacKeys/{accessId}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId"), ("{accessId}", "accessId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(2);
            for param_name in ["accessId", "projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
        let mut request_value_reader =
            {
                let mut value = json::value::to_value(&self._request).expect("serde to work");
                client::remove_json_null_values(&mut value);
                let mut dst = io::Cursor::new(Vec::with_capacity(128));
                json::to_writer(&mut dst, &value).unwrap();
                dst
            };
        let request_size = request_value_reader.seek(io::SeekFrom::End(0)).unwrap();
        request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();


        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::PUT).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .header(CONTENT_TYPE, format!("{}", json_mime_type.to_string()))
                        .header(CONTENT_LENGTH, request_size as u64)
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: HmacKeyMetadata) -> ProjectHmacKeyUpdateCall<'a> {
        self._request = new_value;
        self
    }
    /// Project ID owning the service account of the updated key.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectHmacKeyUpdateCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// Name of the HMAC key being updated.
    ///
    /// Sets the *access id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn access_id(mut self, new_value: &str) -> ProjectHmacKeyUpdateCall<'a> {
        self._access_id = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectHmacKeyUpdateCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectHmacKeyUpdateCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectHmacKeyUpdateCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectHmacKeyUpdateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Get the email address of this project's Google Cloud Storage service account.
///
/// A builder for the *serviceAccount.get* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_storage1 as storage1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use storage1::Storage;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Storage::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().service_account_get("projectId")
///              .user_project("diam")
///              .provisional_user_project("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct ProjectServiceAccountGetCall<'a>
    where  {

    hub: &'a Storage<>,
    _project_id: String,
    _user_project: Option<String>,
    _provisional_user_project: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectServiceAccountGetCall<'a> {}

impl<'a> ProjectServiceAccountGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ServiceAccount)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "storage.projects.serviceAccount.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("projectId", self._project_id.to_string()));
        if let Some(value) = self._user_project {
            params.push(("userProject", value.to_string()));
        }
        if let Some(value) = self._provisional_user_project {
            params.push(("provisionalUserProject", value.to_string()));
        }
        for &field in ["alt", "projectId", "userProject", "provisionalUserProject"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "projects/{projectId}/serviceAccount";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["projectId"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();



        loop {
            let token = match self.hub.auth.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
                Ok(token) => token.clone(),
                Err(err) => {
                    match  dlg.token(&err) {
                        Some(token) => token,
                        None => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(err))
                        }
                    }
                }
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.request(request.unwrap()).await
                
            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(client::Error::HttpError(err))
                }
                Ok(mut res) => {
                    if !res.status().is_success() {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        let res_body_string = client::get_body_as_string(res.body_mut()).await;

                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (res, decoded),
                            Err(err) => {
                                dlg.response_json_decode_error(&res_body_string, &err);
                                return Err(client::Error::JsonDecodeError(res_body_string, err));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value)
                }
            }
        }
    }


    /// Project ID
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectServiceAccountGetCall<'a> {
        self._project_id = new_value.to_string();
        self
    }
    /// The project to be billed for this request.
    ///
    /// Sets the *user project* query property to the given value.
    pub fn user_project(mut self, new_value: &str) -> ProjectServiceAccountGetCall<'a> {
        self._user_project = Some(new_value.to_string());
        self
    }
    /// The project to be billed for this request if the target bucket is requester-pays bucket.
    ///
    /// Sets the *provisional user project* query property to the given value.
    pub fn provisional_user_project(mut self, new_value: &str) -> ProjectServiceAccountGetCall<'a> {
        self._provisional_user_project = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectServiceAccountGetCall<'a> {
        self._delegate = Some(new_value);
        self
    }

    /// Set any additional parameter of the query string used in the request.
    /// It should be used to set parameters which are not yet available through their own
    /// setters.
    ///
    /// Please note that this method must not be used to set any of the known parameters
    /// which have their own setter method. If done anyway, the request will fail.
    ///
    /// # Additional Parameters
    ///
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    /// * *userIp* (query-string) - Deprecated. Please use quotaUser instead.
    pub fn param<T>(mut self, name: T, value: T) -> ProjectServiceAccountGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::CloudPlatform`.
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    /// If `None` is specified, then all scopes will be removed and no default scope will be used either.
    /// In that case, you have to specify your API-key using the `key` parameter (see the `param()`
    /// function for details).
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectServiceAccountGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


