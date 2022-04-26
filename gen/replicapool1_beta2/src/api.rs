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

    /// View and manage your Google Compute Engine resources
    Compute,

    /// View your Google Compute Engine resources
    ComputeReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            Scope::Compute => "https://www.googleapis.com/auth/compute",
            Scope::ComputeReadonly => "https://www.googleapis.com/auth/compute.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ComputeReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Replicapool related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and 
/// // `client_secret`, among other things.
/// let secret: oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you, 
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about 
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().list("project", "zone")
///              .page_token("ipsum")
///              .max_results(39)
///              .filter("Lorem")
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
#[derive(Clone)]
pub struct Replicapool<> {
    pub client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for Replicapool<> {}

impl<'a, > Replicapool<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Replicapool<> {
        Replicapool {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/3.1.0".to_string(),
            _base_url: "https://www.googleapis.com/replicapool/v1beta2/projects/".to_string(),
            _root_url: "https://www.googleapis.com/".to_string(),
        }
    }

    pub fn instance_group_managers(&'a self) -> InstanceGroupManagerMethods<'a> {
        InstanceGroupManagerMethods { hub: &self }
    }
    pub fn zone_operations(&'a self) -> ZoneOperationMethods<'a> {
        ZoneOperationMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/3.1.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/replicapool/v1beta2/projects/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://www.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// An Instance Group Manager resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (none)
/// * [delete instance group managers](InstanceGroupManagerDeleteCall) (none)
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (none)
/// * [get instance group managers](InstanceGroupManagerGetCall) (response)
/// * [insert instance group managers](InstanceGroupManagerInsertCall) (request)
/// * [list instance group managers](InstanceGroupManagerListCall) (none)
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (none)
/// * [resize instance group managers](InstanceGroupManagerResizeCall) (none)
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (none)
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (none)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManager {
    /// The autohealing policy for this managed instance group. You can specify only one value.
    #[serde(rename="autoHealingPolicies")]
    pub auto_healing_policies: Option<Vec<ReplicaPoolAutoHealingPolicy>>,
    /// The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name.
    #[serde(rename="baseInstanceName")]
    pub base_instance_name: Option<String>,
    /// [Output only] The time the instance group manager was created, in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    pub creation_timestamp: Option<String>,
    /// [Output only] The number of instances that currently exist and are a part of this group. This includes instances that are starting but are not yet RUNNING, and instances that are in the process of being deleted or abandoned.
    #[serde(rename="currentSize")]
    pub current_size: Option<i32>,
    /// An optional textual description of the instance group manager.
    pub description: Option<String>,
    /// [Output only] Fingerprint of the instance group manager. This field is used for optimistic locking. An up-to-date fingerprint must be provided in order to modify the Instance Group Manager resource.
    pub fingerprint: Option<String>,
    /// [Output only] The full URL of the instance group created by the manager. This group contains all of the instances being managed, and cannot contain non-managed instances.
    pub group: Option<String>,
    /// [Output only] A server-assigned unique identifier for the resource.
    pub id: Option<String>,
    /// The full URL to an instance template from which all new instances will be created.
    #[serde(rename="instanceTemplate")]
    pub instance_template: Option<String>,
    /// [Output only] The resource type. Always replicapool#instanceGroupManager.
    pub kind: Option<String>,
    /// The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens.
    pub name: Option<String>,
    /// [Output only] The fully qualified URL for this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// The full URL of all target pools to which new instances in the group are added. Updating the target pool values does not affect existing instances.
    #[serde(rename="targetPools")]
    pub target_pools: Option<Vec<String>>,
    /// [Output only] The number of instances that the manager is attempting to maintain. Deleting or abandoning instances affects this number, as does resizing the group.
    #[serde(rename="targetSize")]
    pub target_size: Option<i32>,
}

impl client::RequestValue for InstanceGroupManager {}
impl client::Resource for InstanceGroupManager {}
impl client::ResponseResult for InstanceGroupManager {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list instance group managers](InstanceGroupManagerListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagerList {
    /// Unique identifier for the resource; defined by the server (output only).
    pub id: Option<String>,
    /// A list of instance resources.
    pub items: Option<Vec<InstanceGroupManager>>,
    /// Type of resource.
    pub kind: Option<String>,
    /// A token used to continue a truncated list request (output only).
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl client::ResponseResult for InstanceGroupManagerList {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersAbandonInstancesRequest {
    /// The names of one or more instances to abandon. For example:
    /// { 'instances': [ 'instance-c3po', 'instance-r2d2' ] }
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersAbandonInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersDeleteInstancesRequest {
    /// Names of instances to delete.
    /// 
    /// Example: 'instance-foo', 'instance-bar'
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersDeleteInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersRecreateInstancesRequest {
    /// The names of one or more instances to recreate. For example:
    /// { 'instances': [ 'instance-c3po', 'instance-r2d2' ] }
    pub instances: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersRecreateInstancesRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersSetInstanceTemplateRequest {
    /// The full URL to an Instance Template from which all new instances will be created.
    #[serde(rename="instanceTemplate")]
    pub instance_template: Option<String>,
}

impl client::RequestValue for InstanceGroupManagersSetInstanceTemplateRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (request)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InstanceGroupManagersSetTargetPoolsRequest {
    /// The current fingerprint of the Instance Group Manager resource. If this does not match the server-side fingerprint of the resource, then the request will be rejected.
    pub fingerprint: Option<String>,
    /// A list of fully-qualified URLs to existing Target Pool resources. New instances in the Instance Group Manager will be added to the specified target pools; existing instances are not affected.
    #[serde(rename="targetPools")]
    pub target_pools: Option<Vec<String>>,
}

impl client::RequestValue for InstanceGroupManagersSetTargetPoolsRequest {}


/// An operation resource, used to manage asynchronous API requests.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [abandon instances instance group managers](InstanceGroupManagerAbandonInstanceCall) (response)
/// * [delete instance group managers](InstanceGroupManagerDeleteCall) (response)
/// * [delete instances instance group managers](InstanceGroupManagerDeleteInstanceCall) (response)
/// * [insert instance group managers](InstanceGroupManagerInsertCall) (response)
/// * [recreate instances instance group managers](InstanceGroupManagerRecreateInstanceCall) (response)
/// * [resize instance group managers](InstanceGroupManagerResizeCall) (response)
/// * [set instance template instance group managers](InstanceGroupManagerSetInstanceTemplateCall) (response)
/// * [set target pools instance group managers](InstanceGroupManagerSetTargetPoolCall) (response)
/// * [get zone operations](ZoneOperationGetCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// [Output only] An optional identifier specified by the client when the mutation was initiated. Must be unique for all operation resources in the project.
    #[serde(rename="clientOperationId")]
    pub client_operation_id: Option<String>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="creationTimestamp")]
    pub creation_timestamp: Option<String>,
    /// [Output Only] The time that this operation was completed, in RFC3339 text format.
    #[serde(rename="endTime")]
    pub end_time: Option<String>,
    /// [Output Only] If errors occurred during processing of this operation, this field will be populated.
    pub error: Option<OperationError>,
    /// [Output only] If operation fails, the HTTP error message returned.
    #[serde(rename="httpErrorMessage")]
    pub http_error_message: Option<String>,
    /// [Output only] If operation fails, the HTTP error status code returned.
    #[serde(rename="httpErrorStatusCode")]
    pub http_error_status_code: Option<i32>,
    /// [Output Only] Unique identifier for the resource, generated by the server.
    pub id: Option<String>,
    /// [Output Only] The time that this operation was requested, in RFC3339 text format.
    #[serde(rename="insertTime")]
    pub insert_time: Option<String>,
    /// [Output only] Type of the resource.
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    pub name: Option<String>,
    /// [Output only] Type of the operation. Operations include insert, update, and delete.
    #[serde(rename="operationType")]
    pub operation_type: Option<String>,
    /// [Output only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess at when the operation will be complete. This number should be monotonically increasing as the operation progresses.
    pub progress: Option<i32>,
    /// [Output Only] URL of the region where the operation resides. Only available when performing regional operations.
    pub region: Option<String>,
    /// [Output Only] Server-defined fully-qualified URL for this resource.
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
    /// [Output Only] The time that this operation was started by the server, in RFC3339 text format.
    #[serde(rename="startTime")]
    pub start_time: Option<String>,
    /// [Output Only] Status of the operation.
    pub status: Option<String>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename="statusMessage")]
    pub status_message: Option<String>,
    /// [Output Only] Unique target ID which identifies a particular incarnation of the target.
    #[serde(rename="targetId")]
    pub target_id: Option<String>,
    /// [Output only] URL of the resource the operation is mutating.
    #[serde(rename="targetLink")]
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: user@example.com.
    pub user: Option<String>,
    /// [Output Only] If there are issues with this operation, a warning is returned.
    pub warnings: Option<Vec<OperationWarnings>>,
    /// [Output Only] URL of the zone where the operation resides. Only available when performing per-zone operations.
    pub zone: Option<String>,
}

impl client::ResponseResult for Operation {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list zone operations](ZoneOperationListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationList {
    /// Unique identifier for the resource; defined by the server (output only).
    pub id: Option<String>,
    /// The operation resources.
    pub items: Option<Vec<Operation>>,
    /// Type of resource.
    pub kind: Option<String>,
    /// A token used to continue a truncated list request (output only).
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Server defined URL for this resource (output only).
    #[serde(rename="selfLink")]
    pub self_link: Option<String>,
}

impl client::ResponseResult for OperationList {}


/// There is no detailed description.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReplicaPoolAutoHealingPolicy {
    /// The action to perform when an instance becomes unhealthy. Possible values are RECREATE or REBOOT. RECREATE replaces an unhealthy instance with a new instance that is based on the instance template for this managed instance group. REBOOT performs a soft reboot on an instance. If the instance cannot reboot, the instance performs a hard restart.
    #[serde(rename="actionType")]
    pub action_type: Option<String>,
    /// The URL for the HealthCheck that signals autohealing.
    #[serde(rename="healthCheck")]
    pub health_check: Option<String>,
}

impl client::Part for ReplicaPoolAutoHealingPolicy {}


/// [Output Only] If errors occurred during processing of this operation, this field will be populated.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationError {
    /// [Output Only] The array of errors encountered while processing this operation.
    pub errors: Option<Vec<OperationErrorErrors>>,
}

impl client::NestedType for OperationError {}
impl client::Part for OperationError {}


/// [Output Only] The array of errors encountered while processing this operation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationErrorErrors {
    /// [Output Only] The error type identifier for this error.
    pub code: Option<String>,
    /// [Output Only] Indicates the field in the request which caused the error. This property is optional.
    pub location: Option<String>,
    /// [Output Only] An optional, human-readable error message.
    pub message: Option<String>,
}

impl client::NestedType for OperationErrorErrors {}
impl client::Part for OperationErrorErrors {}


/// [Output Only] If there are issues with this operation, a warning is returned.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarnings {
    /// [Output only] The warning type identifier for this warning.
    pub code: Option<String>,
    /// [Output only] Metadata for this warning in key:value format.
    pub data: Option<Vec<OperationWarningsData>>,
    /// [Output only] Optional human-readable details for this warning.
    pub message: Option<String>,
}

impl client::NestedType for OperationWarnings {}
impl client::Part for OperationWarnings {}


/// [Output only] Metadata for this warning in key:value format.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OperationWarningsData {
    /// [Output Only] Metadata key for this warning.
    pub key: Option<String>,
    /// [Output Only] Metadata value for this warning.
    pub value: Option<String>,
}

impl client::NestedType for OperationWarningsData {}
impl client::Part for OperationWarningsData {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *instanceGroupManager* resources.
/// It is not used directly, but through the `Replicapool` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `abandon_instances(...)`, `delete(...)`, `delete_instances(...)`, `get(...)`, `insert(...)`, `list(...)`, `recreate_instances(...)`, `resize(...)`, `set_instance_template(...)` and `set_target_pools(...)`
/// // to build up your call.
/// let rb = hub.instance_group_managers();
/// # }
/// ```
pub struct InstanceGroupManagerMethods<'a>
    where  {

    hub: &'a Replicapool<>,
}

impl<'a> client::MethodsBuilder for InstanceGroupManagerMethods<'a> {}

impl<'a> InstanceGroupManagerMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Removes the specified instances from the managed instance group, and from any target pools of which they were members, without deleting the instances.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn abandon_instances(&self, request: InstanceGroupManagersAbandonInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerAbandonInstanceCall<'a> {
        InstanceGroupManagerAbandonInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the instance group manager and all instances contained within. If you'd like to delete the manager without deleting the instances, you must first abandon the instances to remove them from the group.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - Name of the Instance Group Manager resource to delete.
    pub fn delete(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteCall<'a> {
        InstanceGroupManagerDeleteCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes the specified instances. The instances are deleted, then removed from the instance group and any target pools of which they were a member. The targetSize of the instance group manager is reduced by the number of instances deleted.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn delete_instances(&self, request: InstanceGroupManagersDeleteInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerDeleteInstanceCall<'a> {
        InstanceGroupManagerDeleteInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the specified Instance Group Manager resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - Name of the instance resource to return.
    pub fn get(&self, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerGetCall<'a> {
        InstanceGroupManagerGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates an instance group manager, as well as the instance group and the specified number of instances.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `size` - Number of instances that should exist.
    pub fn insert(&self, request: InstanceGroupManager, project: &str, zone: &str, size: i32) -> InstanceGroupManagerInsertCall<'a> {
        InstanceGroupManagerInsertCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _size: size,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of Instance Group Manager resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    pub fn list(&self, project: &str, zone: &str) -> InstanceGroupManagerListCall<'a> {
        InstanceGroupManagerListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Recreates the specified instances. The instances are deleted, then recreated using the instance group manager's current instance template.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn recreate_instances(&self, request: InstanceGroupManagersRecreateInstancesRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerRecreateInstanceCall<'a> {
        InstanceGroupManagerRecreateInstanceCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Resizes the managed instance group up or down. If resized up, new instances are created using the current instance template. If resized down, instances are removed in the order outlined in Resizing a managed instance group.
    /// 
    /// # Arguments
    ///
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    /// * `size` - Number of instances that should exist in this Instance Group Manager.
    pub fn resize(&self, project: &str, zone: &str, instance_group_manager: &str, size: i32) -> InstanceGroupManagerResizeCall<'a> {
        InstanceGroupManagerResizeCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _size: size,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Sets the instance template to use when creating new instances in this group. Existing instances are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn set_instance_template(&self, request: InstanceGroupManagersSetInstanceTemplateRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
        InstanceGroupManagerSetInstanceTemplateCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Modifies the target pools to which all new instances in this group are assigned. Existing instances in the group are not affected.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `project` - The Google Developers Console project name.
    /// * `zone` - The name of the zone in which the instance group manager resides.
    /// * `instanceGroupManager` - The name of the instance group manager.
    pub fn set_target_pools(&self, request: InstanceGroupManagersSetTargetPoolsRequest, project: &str, zone: &str, instance_group_manager: &str) -> InstanceGroupManagerSetTargetPoolCall<'a> {
        InstanceGroupManagerSetTargetPoolCall {
            hub: self.hub,
            _request: request,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _instance_group_manager: instance_group_manager.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *zoneOperation* resources.
/// It is not used directly, but through the `Replicapool` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.zone_operations();
/// # }
/// ```
pub struct ZoneOperationMethods<'a>
    where  {

    hub: &'a Replicapool<>,
}

impl<'a> client::MethodsBuilder for ZoneOperationMethods<'a> {}

impl<'a> ZoneOperationMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the specified zone-specific operation resource.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    /// * `operation` - Name of the operation resource to return.
    pub fn get(&self, project: &str, zone: &str, operation: &str) -> ZoneOperationGetCall<'a> {
        ZoneOperationGetCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _operation: operation.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of operation resources contained within the specified zone.
    /// 
    /// # Arguments
    ///
    /// * `project` - Name of the project scoping this request.
    /// * `zone` - Name of the zone scoping this request.
    pub fn list(&self, project: &str, zone: &str) -> ZoneOperationListCall<'a> {
        ZoneOperationListCall {
            hub: self.hub,
            _project: project.to_string(),
            _zone: zone.to_string(),
            _page_token: Default::default(),
            _max_results: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Removes the specified instances from the managed instance group, and from any target pools of which they were members, without deleting the instances.
///
/// A builder for the *abandonInstances* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManagersAbandonInstancesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManagersAbandonInstancesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().abandon_instances(req, "project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerAbandonInstanceCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManagersAbandonInstancesRequest,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerAbandonInstanceCall<'a> {}

impl<'a> InstanceGroupManagerAbandonInstanceCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.abandonInstances",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/abandonInstances";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManagersAbandonInstancesRequest) -> InstanceGroupManagerAbandonInstanceCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerAbandonInstanceCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerAbandonInstanceCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerAbandonInstanceCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerAbandonInstanceCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerAbandonInstanceCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerAbandonInstanceCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the instance group manager and all instances contained within. If you'd like to delete the manager without deleting the instances, you must first abandon the instances to remove them from the group.
///
/// A builder for the *delete* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().delete("project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerDeleteCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerDeleteCall<'a> {}

impl<'a> InstanceGroupManagerDeleteCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.delete",
                               http_method: hyper::Method::DELETE });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerDeleteCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerDeleteCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Name of the Instance Group Manager resource to delete.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerDeleteCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerDeleteCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerDeleteCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerDeleteCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Deletes the specified instances. The instances are deleted, then removed from the instance group and any target pools of which they were a member. The targetSize of the instance group manager is reduced by the number of instances deleted.
///
/// A builder for the *deleteInstances* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManagersDeleteInstancesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManagersDeleteInstancesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().delete_instances(req, "project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerDeleteInstanceCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManagersDeleteInstancesRequest,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerDeleteInstanceCall<'a> {}

impl<'a> InstanceGroupManagerDeleteInstanceCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.deleteInstances",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/deleteInstances";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManagersDeleteInstancesRequest) -> InstanceGroupManagerDeleteInstanceCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerDeleteInstanceCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerDeleteInstanceCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerDeleteInstanceCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerDeleteInstanceCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerDeleteInstanceCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerDeleteInstanceCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Returns the specified Instance Group Manager resource.
///
/// A builder for the *get* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().get("project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerGetCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerGetCall<'a> {}

impl<'a> InstanceGroupManagerGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InstanceGroupManager)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ComputeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerGetCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerGetCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Name of the instance resource to return.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerGetCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerGetCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerGetCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ComputeReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Creates an instance group manager, as well as the instance group and the specified number of instances.
///
/// A builder for the *insert* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManager;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManager::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().insert(req, "project", "zone", -50)
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerInsertCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManager,
    _project: String,
    _zone: String,
    _size: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerInsertCall<'a> {}

impl<'a> InstanceGroupManagerInsertCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.insert",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("size", self._size.to_string()));
        for &field in ["alt", "project", "zone", "size"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone")].iter() {
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
            for param_name in ["zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManager) -> InstanceGroupManagerInsertCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerInsertCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerInsertCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Number of instances that should exist.
    ///
    /// Sets the *size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size(mut self, new_value: i32) -> InstanceGroupManagerInsertCall<'a> {
        self._size = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerInsertCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerInsertCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerInsertCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the list of Instance Group Manager resources contained within the specified zone.
///
/// A builder for the *list* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().list("project", "zone")
///              .page_token("gubergren")
///              .max_results(84)
///              .filter("dolor")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerListCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerListCall<'a> {}

impl<'a> InstanceGroupManagerListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, InstanceGroupManagerList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "project", "zone", "pageToken", "maxResults", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ComputeReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone")].iter() {
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
            for param_name in ["zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerListCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerListCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Optional. Tag returned by a previous list request truncated by maxResults. Used to continue a previous list request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> InstanceGroupManagerListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum count of results to be returned. Maximum value is 500 and default value is 500.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> InstanceGroupManagerListCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// Optional. Filter expression for filtering listed resources.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> InstanceGroupManagerListCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerListCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerListCall<'a>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ComputeReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Recreates the specified instances. The instances are deleted, then recreated using the instance group manager's current instance template.
///
/// A builder for the *recreateInstances* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManagersRecreateInstancesRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManagersRecreateInstancesRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().recreate_instances(req, "project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerRecreateInstanceCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManagersRecreateInstancesRequest,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerRecreateInstanceCall<'a> {}

impl<'a> InstanceGroupManagerRecreateInstanceCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.recreateInstances",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/recreateInstances";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManagersRecreateInstancesRequest) -> InstanceGroupManagerRecreateInstanceCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerRecreateInstanceCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerRecreateInstanceCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerRecreateInstanceCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerRecreateInstanceCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerRecreateInstanceCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerRecreateInstanceCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Resizes the managed instance group up or down. If resized up, new instances are created using the current instance template. If resized down, instances are removed in the order outlined in Resizing a managed instance group.
///
/// A builder for the *resize* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().resize("project", "zone", "instanceGroupManager", -61)
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerResizeCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _size: i32,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerResizeCall<'a> {}

impl<'a> InstanceGroupManagerResizeCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.resize",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        params.push(("size", self._size.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager", "size"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/resize";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerResizeCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerResizeCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerResizeCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// Number of instances that should exist in this Instance Group Manager.
    ///
    /// Sets the *size* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn size(mut self, new_value: i32) -> InstanceGroupManagerResizeCall<'a> {
        self._size = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerResizeCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerResizeCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerResizeCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Sets the instance template to use when creating new instances in this group. Existing instances are not affected.
///
/// A builder for the *setInstanceTemplate* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManagersSetInstanceTemplateRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManagersSetInstanceTemplateRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().set_instance_template(req, "project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerSetInstanceTemplateCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManagersSetInstanceTemplateRequest,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerSetInstanceTemplateCall<'a> {}

impl<'a> InstanceGroupManagerSetInstanceTemplateCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.setInstanceTemplate",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/setInstanceTemplate";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManagersSetInstanceTemplateRequest) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerSetInstanceTemplateCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerSetInstanceTemplateCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerSetInstanceTemplateCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Modifies the target pools to which all new instances in this group are assigned. Existing instances in the group are not affected.
///
/// A builder for the *setTargetPools* method supported by a *instanceGroupManager* resource.
/// It is not used directly, but through a `InstanceGroupManagerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// use replicapool1_beta2::api::InstanceGroupManagersSetTargetPoolsRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = InstanceGroupManagersSetTargetPoolsRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.instance_group_managers().set_target_pools(req, "project", "zone", "instanceGroupManager")
///              .doit().await;
/// # }
/// ```
pub struct InstanceGroupManagerSetTargetPoolCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _request: InstanceGroupManagersSetTargetPoolsRequest,
    _project: String,
    _zone: String,
    _instance_group_manager: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for InstanceGroupManagerSetTargetPoolCall<'a> {}

impl<'a> InstanceGroupManagerSetTargetPoolCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.instanceGroupManagers.setTargetPools",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(6 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("instanceGroupManager", self._instance_group_manager.to_string()));
        for &field in ["alt", "project", "zone", "instanceGroupManager"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/instanceGroupManagers/{instanceGroupManager}/setTargetPools";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{instanceGroupManager}", "instanceGroupManager")].iter() {
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
            for param_name in ["instanceGroupManager", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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
    pub fn request(mut self, new_value: InstanceGroupManagersSetTargetPoolsRequest) -> InstanceGroupManagerSetTargetPoolCall<'a> {
        self._request = new_value;
        self
    }
    /// The Google Developers Console project name.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> InstanceGroupManagerSetTargetPoolCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// The name of the zone in which the instance group manager resides.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> InstanceGroupManagerSetTargetPoolCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// The name of the instance group manager.
    ///
    /// Sets the *instance group manager* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn instance_group_manager(mut self, new_value: &str) -> InstanceGroupManagerSetTargetPoolCall<'a> {
        self._instance_group_manager = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> InstanceGroupManagerSetTargetPoolCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> InstanceGroupManagerSetTargetPoolCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> InstanceGroupManagerSetTargetPoolCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the specified zone-specific operation resource.
///
/// A builder for the *get* method supported by a *zoneOperation* resource.
/// It is not used directly, but through a `ZoneOperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.zone_operations().get("project", "zone", "operation")
///              .doit().await;
/// # }
/// ```
pub struct ZoneOperationGetCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _operation: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ZoneOperationGetCall<'a> {}

impl<'a> ZoneOperationGetCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Operation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.zoneOperations.get",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        params.push(("operation", self._operation.to_string()));
        for &field in ["alt", "project", "zone", "operation"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/operations/{operation}";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone"), ("{operation}", "operation")].iter() {
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
            for param_name in ["operation", "zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// Name of the project scoping this request.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> ZoneOperationGetCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// Name of the zone scoping this request.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> ZoneOperationGetCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Name of the operation resource to return.
    ///
    /// Sets the *operation* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn operation(mut self, new_value: &str) -> ZoneOperationGetCall<'a> {
        self._operation = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ZoneOperationGetCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> ZoneOperationGetCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ZoneOperationGetCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Retrieves the list of operation resources contained within the specified zone.
///
/// A builder for the *list* method supported by a *zoneOperation* resource.
/// It is not used directly, but through a `ZoneOperationMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_replicapool1_beta2 as replicapool1_beta2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use replicapool1_beta2::{Replicapool, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Replicapool::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.zone_operations().list("project", "zone")
///              .page_token("et")
///              .max_results(73)
///              .filter("amet.")
///              .doit().await;
/// # }
/// ```
pub struct ZoneOperationListCall<'a>
    where  {

    hub: &'a Replicapool<>,
    _project: String,
    _zone: String,
    _page_token: Option<String>,
    _max_results: Option<u32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ZoneOperationListCall<'a> {}

impl<'a> ZoneOperationListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, OperationList)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "replicapool.zoneOperations.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("project", self._project.to_string()));
        params.push(("zone", self._zone.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._max_results {
            params.push(("maxResults", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "project", "zone", "pageToken", "maxResults", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "{project}/zones/{zone}/operations";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{project}", "project"), ("{zone}", "zone")].iter() {
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
            for param_name in ["zone", "project"].iter() {
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
                        let (parts, _) = res.into_parts();
                        let body = hyper::Body::from(res_body_string.clone());
                        let restored_response = hyper::Response::from_parts(parts, body);

                        let server_response = json::from_str::<serde_json::Value>(&res_body_string).ok();

                        if let client::Retry::After(d) = dlg.http_failure(&restored_response, server_response.clone()) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return match server_response {
                            Some(error_value) => Err(client::Error::BadRequest(error_value)),
                            None => Err(client::Error::Failure(restored_response)),
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


    /// Name of the project scoping this request.
    ///
    /// Sets the *project* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project(mut self, new_value: &str) -> ZoneOperationListCall<'a> {
        self._project = new_value.to_string();
        self
    }
    /// Name of the zone scoping this request.
    ///
    /// Sets the *zone* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn zone(mut self, new_value: &str) -> ZoneOperationListCall<'a> {
        self._zone = new_value.to_string();
        self
    }
    /// Optional. Tag returned by a previous list request truncated by maxResults. Used to continue a previous list request.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ZoneOperationListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. Maximum count of results to be returned. Maximum value is 500 and default value is 500.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ZoneOperationListCall<'a> {
        self._max_results = Some(new_value);
        self
    }
    /// Optional. Filter expression for filtering listed resources.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ZoneOperationListCall<'a> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ZoneOperationListCall<'a> {
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
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> ZoneOperationListCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ZoneOperationListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


