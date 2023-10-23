use super::*;
/// Request for `BindDeviceToGateway`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries bind device to gateway projects](ProjectLocationRegistryBindDeviceToGatewayCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BindDeviceToGatewayRequest {
    /// Required. The device to associate with the specified gateway. The value of `device_id` can be either the device numeric ID or the user-defined device identifier.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Required. The value of `gateway_id` can be either the device numeric ID or the user-defined device identifier.
    #[serde(rename="gatewayId")]
    
    pub gateway_id: Option<String>,
}

impl client::RequestValue for BindDeviceToGatewayRequest {}


/// Response for `BindDeviceToGateway`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries bind device to gateway projects](ProjectLocationRegistryBindDeviceToGatewayCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BindDeviceToGatewayResponse { _never_set: Option<bool> }

impl client::ResponseResult for BindDeviceToGatewayResponse {}


/// Associates `members`, or principals, with a `role`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Binding {
    /// The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the principals in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub condition: Option<Expr>,
    /// Specifies the principals requesting access for a Google Cloud resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. Does not include identities that come from external identity providers (IdPs) through identity federation. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a Google service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An identifier for a [Kubernetes service account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts). For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. 
    
    pub members: Option<Vec<String>>,
    /// Role that is assigned to the list of `members`, or principals. For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    
    pub role: Option<String>,
}

impl client::Part for Binding {}


/// The device resource.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices create projects](ProjectLocationRegistryDeviceCreateCall) (request|response)
/// * [locations registries devices get projects](ProjectLocationRegistryDeviceGetCall) (response)
/// * [locations registries devices patch projects](ProjectLocationRegistryDevicePatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    /// If a device is blocked, connections or requests from this device will fail. Can be used to temporarily prevent the device from connecting if, for example, the sensor is generating bad data and needs maintenance.
    
    pub blocked: Option<bool>,
    /// The most recent device configuration, which is eventually sent from Cloud IoT Core to the device. If not present on creation, the configuration will be initialized with an empty payload and version value of `1`. To update this field after creation, use the `DeviceManager.ModifyCloudToDeviceConfig` method.
    
    pub config: Option<DeviceConfig>,
    /// The credentials used to authenticate this device. To allow credential rotation without interruption, multiple device credentials can be bound to this device. No more than 3 credentials can be bound to a single device at a time. When new credentials are added to a device, they are verified against the registry credentials. For details, see the description of the `DeviceRegistry.credentials` field.
    
    pub credentials: Option<Vec<DeviceCredential>>,
    /// Gateway-related configuration and state.
    #[serde(rename="gatewayConfig")]
    
    pub gateway_config: Option<GatewayConfig>,
    /// The user-defined device identifier. The device ID must be unique within a device registry.
    
    pub id: Option<String>,
    /// [Output only] The last time a cloud-to-device config version acknowledgment was received from the device. This field is only for configurations sent through MQTT.
    #[serde(rename="lastConfigAckTime")]
    
    pub last_config_ack_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The last time a cloud-to-device config version was sent to the device.
    #[serde(rename="lastConfigSendTime")]
    
    pub last_config_send_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The error message of the most recent error, such as a failure to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this field. If no errors have occurred, this field has an empty message and the status code 0 == OK. Otherwise, this field is expected to have a status code other than OK.
    #[serde(rename="lastErrorStatus")]
    
    pub last_error_status: Option<Status>,
    /// [Output only] The time the most recent error occurred, such as a failure to publish to Cloud Pub/Sub. This field is the timestamp of 'last_error_status'.
    #[serde(rename="lastErrorTime")]
    
    pub last_error_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The last time a telemetry event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes.
    #[serde(rename="lastEventTime")]
    
    pub last_event_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The last time an MQTT `PINGREQ` was received. This field applies only to devices connecting through MQTT. MQTT clients usually only send `PINGREQ` messages if the connection is idle, and no other messages have been sent. Timestamps are periodically collected and written to storage; they may be stale by a few minutes.
    #[serde(rename="lastHeartbeatTime")]
    
    pub last_heartbeat_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The last time a state event was received. Timestamps are periodically collected and written to storage; they may be stale by a few minutes.
    #[serde(rename="lastStateTime")]
    
    pub last_state_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// **Beta Feature** The logging verbosity for device activity. If unspecified, DeviceRegistry.log_level will be used.
    #[serde(rename="logLevel")]
    
    pub log_level: Option<DeviceLogLevelEnum>,
    /// The metadata key-value pairs assigned to the device. This metadata is not interpreted or indexed by Cloud IoT Core. It can be used to add contextual information for the device. Keys must conform to the regular expression a-zA-Z+ and be less than 128 bytes in length. Values are free-form strings. Each value must be less than or equal to 32 KB in size. The total size of all keys and values must be less than 256 KB, and the maximum number of key-value pairs is 500.
    
    pub metadata: Option<HashMap<String, String>>,
    /// The resource path name. For example, `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`. When `name` is populated as a response from the service, it always ends in the device numeric ID.
    
    pub name: Option<String>,
    /// [Output only] A server-defined unique numeric ID for the device. This is a more compact way to identify devices, and it is globally unique.
    #[serde(rename="numId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_id: Option<u64>,
    /// [Output only] The state most recently received from the device. If no state has been reported, this field is not present.
    
    pub state: Option<DeviceState>,
}

impl client::RequestValue for Device {}
impl client::ResponseResult for Device {}


/// The device configuration. Eventually delivered to devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices modify cloud to device config projects](ProjectLocationRegistryDeviceModifyCloudToDeviceConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceConfig {
    /// The device configuration data.
    #[serde(rename="binaryData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub binary_data: Option<Vec<u8>>,
    /// [Output only] The time at which this configuration version was updated in Cloud IoT Core. This timestamp is set by the server.
    #[serde(rename="cloudUpdateTime")]
    
    pub cloud_update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The time at which Cloud IoT Core received the acknowledgment from the device, indicating that the device has received this configuration version. If this field is not present, the device has not yet acknowledged that it received this version. Note that when the config was sent to the device, many config versions may have been available in Cloud IoT Core while the device was disconnected, and on connection, only the latest version is sent to the device. Some versions may never be sent to the device, and therefore are never acknowledged. This timestamp is set by Cloud IoT Core.
    #[serde(rename="deviceAckTime")]
    
    pub device_ack_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// [Output only] The version of this update. The version number is assigned by the server, and is always greater than 0 after device creation. The version must be 0 on the `CreateDevice` request if a `config` is specified; the response of `CreateDevice` will always have a value of 1.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version: Option<i64>,
}

impl client::ResponseResult for DeviceConfig {}


/// A server-stored device credential used for authentication.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceCredential {
    /// [Optional] The time at which this credential becomes invalid. This credential will be ignored for new client authentication requests after this timestamp; however, it will not be automatically deleted.
    #[serde(rename="expirationTime")]
    
    pub expiration_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A public key used to verify the signature of JSON Web Tokens (JWTs). When adding a new device credential, either via device creation or via modifications, this public key credential may be required to be signed by one of the registry level certificates. More specifically, if the registry contains at least one certificate, any new device credential must be signed by one of the registry certificates. As a result, when the registry contains certificates, only X.509 certificates are accepted as device credentials. However, if the registry does not contain a certificate, self-signed certificates and public keys will be accepted. New device credentials must be different from every registry-level certificate.
    #[serde(rename="publicKey")]
    
    pub public_key: Option<PublicKeyCredential>,
}

impl client::Part for DeviceCredential {}


/// A container for a group of devices.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries create projects](ProjectLocationRegistryCreateCall) (request|response)
/// * [locations registries get projects](ProjectLocationRegistryGetCall) (response)
/// * [locations registries patch projects](ProjectLocationRegistryPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceRegistry {
    /// The credentials used to verify the device credentials. No more than 10 credentials can be bound to a single registry at a time. The verification process occurs at the time of device creation or update. If this field is empty, no verification is performed. Otherwise, the credentials of a newly created device or added credentials of an updated device should be signed with one of these registry credentials. Note, however, that existing devices will never be affected by modifications to this list of credentials: after a device has been successfully created in a registry, it should be able to connect even if its registry credentials are revoked, deleted, or modified.
    
    pub credentials: Option<Vec<RegistryCredential>>,
    /// The configuration for notification of telemetry events received from the device. All telemetry events that were successfully published by the device and acknowledged by Cloud IoT Core are guaranteed to be delivered to Cloud Pub/Sub. If multiple configurations match a message, only the first matching configuration is used. If you try to publish a device telemetry event using MQTT without specifying a Cloud Pub/Sub topic for the device's registry, the connection closes automatically. If you try to do so using an HTTP connection, an error is returned. Up to 10 configurations may be provided.
    #[serde(rename="eventNotificationConfigs")]
    
    pub event_notification_configs: Option<Vec<EventNotificationConfig>>,
    /// The DeviceService (HTTP) configuration for this device registry.
    #[serde(rename="httpConfig")]
    
    pub http_config: Option<HttpConfig>,
    /// The identifier of this device registry. For example, `myRegistry`.
    
    pub id: Option<String>,
    /// **Beta Feature** The default logging verbosity for activity from devices in this registry. The verbosity level can be overridden by Device.log_level.
    #[serde(rename="logLevel")]
    
    pub log_level: Option<DeviceRegistryLogLevelEnum>,
    /// The MQTT configuration for this device registry.
    #[serde(rename="mqttConfig")]
    
    pub mqtt_config: Option<MqttConfig>,
    /// The resource path name. For example, `projects/example-project/locations/us-central1/registries/my-registry`.
    
    pub name: Option<String>,
    /// The configuration for notification of new states received from the device. State updates are guaranteed to be stored in the state history, but notifications to Cloud Pub/Sub are not guaranteed. For example, if permissions are misconfigured or the specified topic doesn't exist, no notification will be published but the state will still be stored in Cloud IoT Core.
    #[serde(rename="stateNotificationConfig")]
    
    pub state_notification_config: Option<StateNotificationConfig>,
}

impl client::RequestValue for DeviceRegistry {}
impl client::ResponseResult for DeviceRegistry {}


/// The device state, as reported by the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceState {
    /// The device state data.
    #[serde(rename="binaryData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub binary_data: Option<Vec<u8>>,
    /// [Output only] The time at which this state version was updated in Cloud IoT Core.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for DeviceState {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices delete projects](ProjectLocationRegistryDeviceDeleteCall) (response)
/// * [locations registries delete projects](ProjectLocationRegistryDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// The configuration for forwarding telemetry events.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EventNotificationConfig {
    /// A Cloud Pub/Sub topic name. For example, `projects/myProject/topics/deviceEvents`.
    #[serde(rename="pubsubTopicName")]
    
    pub pubsub_topic_name: Option<String>,
    /// If the subfolder name matches this string exactly, this configuration will be used. The string must not include the leading '/' character. If empty, all strings are matched. This field is used only for telemetry events; subfolders are not supported for state changes.
    #[serde(rename="subfolderMatches")]
    
    pub subfolder_matches: Option<String>,
}

impl client::Part for EventNotificationConfig {}


/// Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: "Summary size limit" description: "Determines if a summary is less than 100 chars" expression: "document.summary.size() < 100" Example (Equality): title: "Requestor is owner" description: "Determines if requestor is the document owner" expression: "document.owner == request.auth.claims.email" Example (Logic): title: "Public documents" description: "Determine whether the document should be publicly visible" expression: "document.type != 'private' && document.type != 'internal'" Example (Data Manipulation): title: "Notification string" description: "Create a notification string with a timestamp." expression: "'New message received at ' + string(document.create_time)" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    /// Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax.
    
    pub expression: Option<String>,
    /// Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    
    pub location: Option<String>,
    /// Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    
    pub title: Option<String>,
}

impl client::Part for Expr {}


/// Gateway-related configuration and state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Indicates how to authorize and/or authenticate devices to access the gateway.
    #[serde(rename="gatewayAuthMethod")]
    
    pub gateway_auth_method: Option<GatewayConfigGatewayAuthMethodEnum>,
    /// Indicates whether the device is a gateway.
    #[serde(rename="gatewayType")]
    
    pub gateway_type: Option<GatewayConfigGatewayTypeEnum>,
    /// [Output only] The ID of the gateway the device accessed most recently.
    #[serde(rename="lastAccessedGatewayId")]
    
    pub last_accessed_gateway_id: Option<String>,
    /// [Output only] The most recent time at which the device accessed the gateway specified in `last_accessed_gateway`.
    #[serde(rename="lastAccessedGatewayTime")]
    
    pub last_accessed_gateway_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for GatewayConfig {}


/// Request message for `GetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries groups get iam policy projects](ProjectLocationRegistryGroupGetIamPolicyCall) (request)
/// * [locations registries get iam policy projects](ProjectLocationRegistryGetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetIamPolicyRequest {
    /// OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`.
    
    pub options: Option<GetPolicyOptions>,
}

impl client::RequestValue for GetIamPolicyRequest {}


/// Encapsulates settings provided to GetIamPolicy.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetPolicyOptions {
    /// Optional. The maximum policy version that will be used to format the policy. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional role bindings must specify version 3. Policies with no conditional role bindings may specify any valid value or leave the field unset. The policy in the response might use the policy version that you specified, or it might use a lower policy version. For example, if you specify version 3, but the policy has no conditional role bindings, the response uses version 1. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    #[serde(rename="requestedPolicyVersion")]
    
    pub requested_policy_version: Option<i32>,
}

impl client::Part for GetPolicyOptions {}


/// The configuration of the HTTP bridge for a device registry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpConfig {
    /// If enabled, allows devices to use DeviceService via the HTTP protocol. Otherwise, any requests to DeviceService will fail for this registry.
    #[serde(rename="httpEnabledState")]
    
    pub http_enabled_state: Option<HttpConfigHttpEnabledStateEnum>,
}

impl client::Part for HttpConfig {}


/// Response for `ListDeviceConfigVersions`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices config versions list projects](ProjectLocationRegistryDeviceConfigVersionListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDeviceConfigVersionsResponse {
    /// The device configuration for the last few versions. Versions are listed in decreasing order, starting from the most recent one.
    #[serde(rename="deviceConfigs")]
    
    pub device_configs: Option<Vec<DeviceConfig>>,
}

impl client::ResponseResult for ListDeviceConfigVersionsResponse {}


/// Response for `ListDeviceRegistries`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries list projects](ProjectLocationRegistryListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDeviceRegistriesResponse {
    /// The registries that matched the query.
    #[serde(rename="deviceRegistries")]
    
    pub device_registries: Option<Vec<DeviceRegistry>>,
    /// If not empty, indicates that there may be more registries that match the request; this value should be passed in a new `ListDeviceRegistriesRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDeviceRegistriesResponse {}


/// Response for `ListDeviceStates`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices states list projects](ProjectLocationRegistryDeviceStateListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDeviceStatesResponse {
    /// The last few device states. States are listed in descending order of server update time, starting from the most recent one.
    #[serde(rename="deviceStates")]
    
    pub device_states: Option<Vec<DeviceState>>,
}

impl client::ResponseResult for ListDeviceStatesResponse {}


/// Response for `ListDevices`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices list projects](ProjectLocationRegistryDeviceListCall) (response)
/// * [locations registries groups devices list projects](ProjectLocationRegistryGroupDeviceListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDevicesResponse {
    /// The devices that match the request.
    
    pub devices: Option<Vec<Device>>,
    /// If not empty, indicates that there may be more devices that match the request; this value should be passed in a new `ListDevicesRequest`.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListDevicesResponse {}


/// Request for `ModifyCloudToDeviceConfig`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices modify cloud to device config projects](ProjectLocationRegistryDeviceModifyCloudToDeviceConfigCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyCloudToDeviceConfigRequest {
    /// Required. The configuration data for the device.
    #[serde(rename="binaryData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub binary_data: Option<Vec<u8>>,
    /// The version number to update. If this value is zero, it will not check the version number of the server and will always update the current version; otherwise, this update will fail if the version number found on the server does not match this version number. This is used to support multiple simultaneous updates without losing data.
    #[serde(rename="versionToUpdate")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_to_update: Option<i64>,
}

impl client::RequestValue for ModifyCloudToDeviceConfigRequest {}


/// The configuration of MQTT for a device registry.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MqttConfig {
    /// If enabled, allows connections using the MQTT protocol. Otherwise, MQTT connections to this registry will fail.
    #[serde(rename="mqttEnabledState")]
    
    pub mqtt_enabled_state: Option<MqttConfigMqttEnabledStateEnum>,
}

impl client::Part for MqttConfig {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries groups get iam policy projects](ProjectLocationRegistryGroupGetIamPolicyCall) (response)
/// * [locations registries groups set iam policy projects](ProjectLocationRegistryGroupSetIamPolicyCall) (response)
/// * [locations registries get iam policy projects](ProjectLocationRegistryGetIamPolicyCall) (response)
/// * [locations registries set iam policy projects](ProjectLocationRegistrySetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// A public key certificate format and data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKeyCertificate {
    /// The certificate data.
    
    pub certificate: Option<String>,
    /// The certificate format.
    
    pub format: Option<PublicKeyCertificateFormatEnum>,
    /// [Output only] The certificate details. Used only for X.509 certificates.
    #[serde(rename="x509Details")]
    
    pub x509_details: Option<X509CertificateDetails>,
}

impl client::Part for PublicKeyCertificate {}


/// A public key format and data.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PublicKeyCredential {
    /// The format of the key.
    
    pub format: Option<PublicKeyCredentialFormatEnum>,
    /// The key data.
    
    pub key: Option<String>,
}

impl client::Part for PublicKeyCredential {}


/// A server-stored registry credential used to validate device credentials.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegistryCredential {
    /// A public key certificate used to verify the device credentials.
    #[serde(rename="publicKeyCertificate")]
    
    pub public_key_certificate: Option<PublicKeyCertificate>,
}

impl client::Part for RegistryCredential {}


/// Request for `SendCommandToDevice`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices send command to device projects](ProjectLocationRegistryDeviceSendCommandToDeviceCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendCommandToDeviceRequest {
    /// Required. The command data to send to the device.
    #[serde(rename="binaryData")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub binary_data: Option<Vec<u8>>,
    /// Optional subfolder for the command. If empty, the command will be delivered to the /devices/{device-id}/commands topic, otherwise it will be delivered to the /devices/{device-id}/commands/{subfolder} topic. Multi-level subfolders are allowed. This field must not have more than 256 characters, and must not contain any MQTT wildcards ("+" or "#") or null characters.
    
    pub subfolder: Option<String>,
}

impl client::RequestValue for SendCommandToDeviceRequest {}


/// Response for `SendCommandToDevice`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries devices send command to device projects](ProjectLocationRegistryDeviceSendCommandToDeviceCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SendCommandToDeviceResponse { _never_set: Option<bool> }

impl client::ResponseResult for SendCommandToDeviceResponse {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries groups set iam policy projects](ProjectLocationRegistryGroupSetIamPolicyCall) (request)
/// * [locations registries set iam policy projects](ProjectLocationRegistrySetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// The configuration for notification of new states received from the device.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StateNotificationConfig {
    /// A Cloud Pub/Sub topic name. For example, `projects/myProject/topics/deviceEvents`.
    #[serde(rename="pubsubTopicName")]
    
    pub pubsub_topic_name: Option<String>,
}

impl client::Part for StateNotificationConfig {}


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


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries groups test iam permissions projects](ProjectLocationRegistryGroupTestIamPermissionCall) (request)
/// * [locations registries test iam permissions projects](ProjectLocationRegistryTestIamPermissionCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with wildcards (such as `*` or `storage.*`) are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    
    pub permissions: Option<Vec<String>>,
}

impl client::RequestValue for TestIamPermissionsRequest {}


/// Response message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries groups test iam permissions projects](ProjectLocationRegistryGroupTestIamPermissionCall) (response)
/// * [locations registries test iam permissions projects](ProjectLocationRegistryTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// Request for `UnbindDeviceFromGateway`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries unbind device from gateway projects](ProjectLocationRegistryUnbindDeviceFromGatewayCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnbindDeviceFromGatewayRequest {
    /// Required. The device to disassociate from the specified gateway. The value of `device_id` can be either the device numeric ID or the user-defined device identifier.
    #[serde(rename="deviceId")]
    
    pub device_id: Option<String>,
    /// Required. The value of `gateway_id` can be either the device numeric ID or the user-defined device identifier.
    #[serde(rename="gatewayId")]
    
    pub gateway_id: Option<String>,
}

impl client::RequestValue for UnbindDeviceFromGatewayRequest {}


/// Response for `UnbindDeviceFromGateway`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations registries unbind device from gateway projects](ProjectLocationRegistryUnbindDeviceFromGatewayCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnbindDeviceFromGatewayResponse { _never_set: Option<bool> }

impl client::ResponseResult for UnbindDeviceFromGatewayResponse {}


/// Details of an X.509 certificate. For informational purposes only.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct X509CertificateDetails {
    /// The time the certificate becomes invalid.
    #[serde(rename="expiryTime")]
    
    pub expiry_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The entity that signed the certificate.
    
    pub issuer: Option<String>,
    /// The type of public key in the certificate.
    #[serde(rename="publicKeyType")]
    
    pub public_key_type: Option<String>,
    /// The algorithm used to sign the certificate.
    #[serde(rename="signatureAlgorithm")]
    
    pub signature_algorithm: Option<String>,
    /// The time the certificate becomes valid.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// The entity the certificate and public key belong to.
    
    pub subject: Option<String>,
}

impl client::Part for X509CertificateDetails {}


