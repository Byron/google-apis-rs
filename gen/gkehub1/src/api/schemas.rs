use super::*;
/// Spec for App Dev Experience Feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppDevExperienceFeatureSpec { _never_set: Option<bool> }

impl client::Part for AppDevExperienceFeatureSpec {}


/// State for App Dev Exp Feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppDevExperienceFeatureState {
    /// Status of subcomponent that detects configured Service Mesh resources.
    #[serde(rename="networkingInstallSucceeded")]
    
    pub networking_install_succeeded: Option<Status>,
}

impl client::Part for AppDevExperienceFeatureState {}


/// ApplianceCluster contains information specific to GDC Edge Appliance Clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApplianceCluster {
    /// Immutable. Self-link of the GCP resource for the Appliance Cluster. For example: //transferappliance.googleapis.com/projects/my-project/locations/us-west1-a/appliances/my-appliance
    #[serde(rename="resourceLink")]
    
    pub resource_link: Option<String>,
}

impl client::Part for ApplianceCluster {}


/// Specifies the audit configuration for a service. The configuration determines which permission types are logged, and what identities, if any, are exempted from logging. An AuditConfig must have one or more AuditLogConfigs. If there are AuditConfigs for both `allServices` and a specific service, the union of the two AuditConfigs is used for that service: the log_types specified in each AuditConfig are enabled, and the exempted_members in each AuditLogConfig are exempted. Example Policy with multiple AuditConfigs: { "audit_configs": [ { "service": "allServices", "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" }, { "log_type": "ADMIN_READ" } ] }, { "service": "sampleservice.googleapis.com", "audit_log_configs": [ { "log_type": "DATA_READ" }, { "log_type": "DATA_WRITE", "exempted_members": [ "user:aliya@example.com" ] } ] } ] } For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ logging. It also exempts `jose@example.com` from DATA_READ logging, and `aliya@example.com` from DATA_WRITE logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditConfig {
    /// The configuration for logging of each type of permission.
    #[serde(rename="auditLogConfigs")]
    
    pub audit_log_configs: Option<Vec<AuditLogConfig>>,
    /// Specifies a service that will be enabled for audit logging. For example, `storage.googleapis.com`, `cloudsql.googleapis.com`. `allServices` is a special value that covers all services.
    
    pub service: Option<String>,
}

impl client::Part for AuditConfig {}


/// Provides the configuration for logging a type of permissions. Example: { "audit_log_configs": [ { "log_type": "DATA_READ", "exempted_members": [ "user:jose@example.com" ] }, { "log_type": "DATA_WRITE" } ] } This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting jose@example.com from DATA_READ logging.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuditLogConfig {
    /// Specifies the identities that do not cause logging for this type of permission. Follows the same format of Binding.members.
    #[serde(rename="exemptedMembers")]
    
    pub exempted_members: Option<Vec<String>>,
    /// The log type that this config enables.
    #[serde(rename="logType")]
    
    pub log_type: Option<String>,
}

impl client::Part for AuditLogConfig {}


/// Authority encodes how Google will recognize identities from this Membership. See the workload identity documentation for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Authority {
    /// Output only. An identity provider that reflects the `issuer` in the workload identity pool.
    #[serde(rename="identityProvider")]
    
    pub identity_provider: Option<String>,
    /// Optional. A JSON Web Token (JWT) issuer URI. `issuer` must start with `https://` and be a valid URL with length <2000 characters. If set, then Google will allow valid OIDC tokens from this issuer to authenticate within the workload_identity_pool. OIDC discovery will be performed on this URI to validate tokens from the issuer. Clearing `issuer` disables Workload Identity. `issuer` cannot be directly modified; it must be cleared (and Workload Identity disabled) before using a new issuer (and re-enabling Workload Identity).
    
    pub issuer: Option<String>,
    /// Optional. OIDC verification keys for this Membership in JWKS format (RFC 7517). When this field is set, OIDC discovery will NOT be performed on `issuer`, and instead OIDC tokens will be validated using this field.
    #[serde(rename="oidcJwks")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub oidc_jwks: Option<Vec<u8>>,
    /// Output only. The name of the workload identity pool in which `issuer` will be recognized. There is a single Workload Identity Pool per Hub that is shared between all Memberships that belong to that Hub. For a Hub hosted in {PROJECT_ID}, the workload pool format is `{PROJECT_ID}.hub.id.goog`, although this is subject to change in newer versions of this API.
    #[serde(rename="workloadIdentityPool")]
    
    pub workload_identity_pool: Option<String>,
}

impl client::Part for Authority {}


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


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// CommonFeatureSpec contains Hub-wide configuration information
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommonFeatureSpec {
    /// Appdevexperience specific spec.
    
    pub appdevexperience: Option<AppDevExperienceFeatureSpec>,
    /// FleetObservability feature spec.
    
    pub fleetobservability: Option<FleetObservabilityFeatureSpec>,
    /// Multicluster Ingress-specific spec.
    
    pub multiclusteringress: Option<MultiClusterIngressFeatureSpec>,
}

impl client::Part for CommonFeatureSpec {}


/// CommonFeatureState contains Hub-wide Feature status information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CommonFeatureState {
    /// Appdevexperience specific state.
    
    pub appdevexperience: Option<AppDevExperienceFeatureState>,
    /// FleetObservability feature state.
    
    pub fleetobservability: Option<FleetObservabilityFeatureState>,
    /// Output only. The "running state" of the Feature in this Hub.
    
    pub state: Option<FeatureState>,
}

impl client::Part for CommonFeatureState {}


/// Configuration for Config Sync
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementConfigSync {
    /// Set to true to allow the vertical scaling. Defaults to false which disallows vertical scaling. This field is deprecated.
    #[serde(rename="allowVerticalScale")]
    
    pub allow_vertical_scale: Option<bool>,
    /// Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of git field.
    
    pub enabled: Option<bool>,
    /// Git repo configuration for the cluster.
    
    pub git: Option<ConfigManagementGitConfig>,
    /// OCI repo configuration for the cluster
    
    pub oci: Option<ConfigManagementOciConfig>,
    /// Set to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts.
    #[serde(rename="preventDrift")]
    
    pub prevent_drift: Option<bool>,
    /// Specifies whether the Config Sync Repo is in "hierarchical" or "unstructured" mode.
    #[serde(rename="sourceFormat")]
    
    pub source_format: Option<String>,
}

impl client::Part for ConfigManagementConfigSync {}


/// The state of ConfigSync's deployment on a cluster
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementConfigSyncDeploymentState {
    /// Deployment state of admission-webhook
    #[serde(rename="admissionWebhook")]
    
    pub admission_webhook: Option<String>,
    /// Deployment state of the git-sync pod
    #[serde(rename="gitSync")]
    
    pub git_sync: Option<String>,
    /// Deployment state of the importer pod
    
    pub importer: Option<String>,
    /// Deployment state of the monitor pod
    
    pub monitor: Option<String>,
    /// Deployment state of reconciler-manager pod
    #[serde(rename="reconcilerManager")]
    
    pub reconciler_manager: Option<String>,
    /// Deployment state of root-reconciler
    #[serde(rename="rootReconciler")]
    
    pub root_reconciler: Option<String>,
    /// Deployment state of the syncer pod
    
    pub syncer: Option<String>,
}

impl client::Part for ConfigManagementConfigSyncDeploymentState {}


/// State information for ConfigSync
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementConfigSyncState {
    /// Information about the deployment of ConfigSync, including the version of the various Pods deployed
    #[serde(rename="deploymentState")]
    
    pub deployment_state: Option<ConfigManagementConfigSyncDeploymentState>,
    /// The state of ConfigSync's process to sync configs to a cluster
    #[serde(rename="syncState")]
    
    pub sync_state: Option<ConfigManagementSyncState>,
    /// The version of ConfigSync deployed
    
    pub version: Option<ConfigManagementConfigSyncVersion>,
}

impl client::Part for ConfigManagementConfigSyncState {}


/// Specific versioning information pertaining to ConfigSync's Pods
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementConfigSyncVersion {
    /// Version of the deployed admission_webhook pod
    #[serde(rename="admissionWebhook")]
    
    pub admission_webhook: Option<String>,
    /// Version of the deployed git-sync pod
    #[serde(rename="gitSync")]
    
    pub git_sync: Option<String>,
    /// Version of the deployed importer pod
    
    pub importer: Option<String>,
    /// Version of the deployed monitor pod
    
    pub monitor: Option<String>,
    /// Version of the deployed reconciler-manager pod
    #[serde(rename="reconcilerManager")]
    
    pub reconciler_manager: Option<String>,
    /// Version of the deployed reconciler container in root-reconciler pod
    #[serde(rename="rootReconciler")]
    
    pub root_reconciler: Option<String>,
    /// Version of the deployed syncer pod
    
    pub syncer: Option<String>,
}

impl client::Part for ConfigManagementConfigSyncVersion {}


/// Model for a config file in the git repo with an associated Sync error
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementErrorResource {
    /// Group/version/kind of the resource that is causing an error
    #[serde(rename="resourceGvk")]
    
    pub resource_gvk: Option<ConfigManagementGroupVersionKind>,
    /// Metadata name of the resource that is causing an error
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// Namespace of the resource that is causing an error
    #[serde(rename="resourceNamespace")]
    
    pub resource_namespace: Option<String>,
    /// Path in the git repo of the erroneous config
    #[serde(rename="sourcePath")]
    
    pub source_path: Option<String>,
}

impl client::Part for ConfigManagementErrorResource {}


/// State of Policy Controller installation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementGatekeeperDeploymentState {
    /// Status of gatekeeper-audit deployment.
    #[serde(rename="gatekeeperAudit")]
    
    pub gatekeeper_audit: Option<String>,
    /// Status of gatekeeper-controller-manager pod.
    #[serde(rename="gatekeeperControllerManagerState")]
    
    pub gatekeeper_controller_manager_state: Option<String>,
    /// Status of the pod serving the mutation webhook.
    #[serde(rename="gatekeeperMutation")]
    
    pub gatekeeper_mutation: Option<String>,
}

impl client::Part for ConfigManagementGatekeeperDeploymentState {}


/// Git repo configuration for a single cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementGitConfig {
    /// The GCP Service Account Email used for auth when secret_type is gcpServiceAccount.
    #[serde(rename="gcpServiceAccountEmail")]
    
    pub gcp_service_account_email: Option<String>,
    /// URL for the HTTPS proxy to be used when communicating with the Git repo.
    #[serde(rename="httpsProxy")]
    
    pub https_proxy: Option<String>,
    /// The path within the Git repository that represents the top level of the repo to sync. Default: the root directory of the repository.
    #[serde(rename="policyDir")]
    
    pub policy_dir: Option<String>,
    /// Type of secret configured for access to the Git repo. Must be one of ssh, cookiefile, gcenode, token, gcpserviceaccount or none. The validation of this is case-sensitive. Required.
    #[serde(rename="secretType")]
    
    pub secret_type: Option<String>,
    /// The branch of the repository to sync from. Default: master.
    #[serde(rename="syncBranch")]
    
    pub sync_branch: Option<String>,
    /// The URL of the Git repository to use as the source of truth.
    #[serde(rename="syncRepo")]
    
    pub sync_repo: Option<String>,
    /// Git revision (tag or hash) to check out. Default HEAD.
    #[serde(rename="syncRev")]
    
    pub sync_rev: Option<String>,
    /// Period in seconds between consecutive syncs. Default: 15.
    #[serde(rename="syncWaitSecs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sync_wait_secs: Option<i64>,
}

impl client::Part for ConfigManagementGitConfig {}


/// A Kubernetes object's GVK
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementGroupVersionKind {
    /// Kubernetes Group
    
    pub group: Option<String>,
    /// Kubernetes Kind
    
    pub kind: Option<String>,
    /// Kubernetes Version
    
    pub version: Option<String>,
}

impl client::Part for ConfigManagementGroupVersionKind {}


/// Configuration for Hierarchy Controller
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementHierarchyControllerConfig {
    /// Whether hierarchical resource quota is enabled in this cluster.
    #[serde(rename="enableHierarchicalResourceQuota")]
    
    pub enable_hierarchical_resource_quota: Option<bool>,
    /// Whether pod tree labels are enabled in this cluster.
    #[serde(rename="enablePodTreeLabels")]
    
    pub enable_pod_tree_labels: Option<bool>,
    /// Whether Hierarchy Controller is enabled in this cluster.
    
    pub enabled: Option<bool>,
}

impl client::Part for ConfigManagementHierarchyControllerConfig {}


/// Deployment state for Hierarchy Controller
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementHierarchyControllerDeploymentState {
    /// The deployment state for Hierarchy Controller extension (e.g. v0.7.0-hc.1)
    
    pub extension: Option<String>,
    /// The deployment state for open source HNC (e.g. v0.7.0-hc.0)
    
    pub hnc: Option<String>,
}

impl client::Part for ConfigManagementHierarchyControllerDeploymentState {}


/// State for Hierarchy Controller
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementHierarchyControllerState {
    /// The deployment state for Hierarchy Controller
    
    pub state: Option<ConfigManagementHierarchyControllerDeploymentState>,
    /// The version for Hierarchy Controller
    
    pub version: Option<ConfigManagementHierarchyControllerVersion>,
}

impl client::Part for ConfigManagementHierarchyControllerState {}


/// Version for Hierarchy Controller
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementHierarchyControllerVersion {
    /// Version for Hierarchy Controller extension
    
    pub extension: Option<String>,
    /// Version for open source HNC
    
    pub hnc: Option<String>,
}

impl client::Part for ConfigManagementHierarchyControllerVersion {}


/// Errors pertaining to the installation of ACM
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementInstallError {
    /// A string representing the user facing error message
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
}

impl client::Part for ConfigManagementInstallError {}


/// **Anthos Config Management**: Configuration for a single cluster. Intended to parallel the ConfigManagement CR.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementMembershipSpec {
    /// Config Sync configuration for the cluster.
    #[serde(rename="configSync")]
    
    pub config_sync: Option<ConfigManagementConfigSync>,
    /// Hierarchy Controller configuration for the cluster.
    #[serde(rename="hierarchyController")]
    
    pub hierarchy_controller: Option<ConfigManagementHierarchyControllerConfig>,
    /// Policy Controller configuration for the cluster.
    #[serde(rename="policyController")]
    
    pub policy_controller: Option<ConfigManagementPolicyController>,
    /// Version of ACM installed.
    
    pub version: Option<String>,
}

impl client::Part for ConfigManagementMembershipSpec {}


/// **Anthos Config Management**: State for a single cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementMembershipState {
    /// The user-defined name for the cluster used by ClusterSelectors to group clusters together. This should match Membership's membership_name, unless the user installed ACM on the cluster manually prior to enabling the ACM hub feature. Unique within a Anthos Config Management installation.
    #[serde(rename="clusterName")]
    
    pub cluster_name: Option<String>,
    /// Current sync status
    #[serde(rename="configSyncState")]
    
    pub config_sync_state: Option<ConfigManagementConfigSyncState>,
    /// Hierarchy Controller status
    #[serde(rename="hierarchyControllerState")]
    
    pub hierarchy_controller_state: Option<ConfigManagementHierarchyControllerState>,
    /// Membership configuration in the cluster. This represents the actual state in the cluster, while the MembershipSpec in the FeatureSpec represents the intended state
    #[serde(rename="membershipSpec")]
    
    pub membership_spec: Option<ConfigManagementMembershipSpec>,
    /// Current install status of ACM's Operator
    #[serde(rename="operatorState")]
    
    pub operator_state: Option<ConfigManagementOperatorState>,
    /// PolicyController status
    #[serde(rename="policyControllerState")]
    
    pub policy_controller_state: Option<ConfigManagementPolicyControllerState>,
}

impl client::Part for ConfigManagementMembershipState {}


/// OCI repo configuration for a single cluster
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementOciConfig {
    /// The GCP Service Account Email used for auth when secret_type is gcpServiceAccount.
    #[serde(rename="gcpServiceAccountEmail")]
    
    pub gcp_service_account_email: Option<String>,
    /// The absolute path of the directory that contains the local resources. Default: the root directory of the image.
    #[serde(rename="policyDir")]
    
    pub policy_dir: Option<String>,
    /// Type of secret configured for access to the Git repo.
    #[serde(rename="secretType")]
    
    pub secret_type: Option<String>,
    /// The OCI image repository URL for the package to sync from. e.g. `LOCATION-docker.pkg.dev/PROJECT_ID/REPOSITORY_NAME/PACKAGE_NAME`.
    #[serde(rename="syncRepo")]
    
    pub sync_repo: Option<String>,
    /// Period in seconds between consecutive syncs. Default: 15.
    #[serde(rename="syncWaitSecs")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub sync_wait_secs: Option<i64>,
}

impl client::Part for ConfigManagementOciConfig {}


/// State information for an ACM's Operator
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementOperatorState {
    /// The state of the Operator's deployment
    #[serde(rename="deploymentState")]
    
    pub deployment_state: Option<String>,
    /// Install errors.
    
    pub errors: Option<Vec<ConfigManagementInstallError>>,
    /// The semenatic version number of the operator
    
    pub version: Option<String>,
}

impl client::Part for ConfigManagementOperatorState {}


/// Configuration for Policy Controller
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementPolicyController {
    /// Sets the interval for Policy Controller Audit Scans (in seconds). When set to 0, this disables audit functionality altogether.
    #[serde(rename="auditIntervalSeconds")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub audit_interval_seconds: Option<i64>,
    /// Enables the installation of Policy Controller. If false, the rest of PolicyController fields take no effect.
    
    pub enabled: Option<bool>,
    /// The set of namespaces that are excluded from Policy Controller checks. Namespaces do not need to currently exist on the cluster.
    #[serde(rename="exemptableNamespaces")]
    
    pub exemptable_namespaces: Option<Vec<String>>,
    /// Logs all denies and dry run failures.
    #[serde(rename="logDeniesEnabled")]
    
    pub log_denies_enabled: Option<bool>,
    /// Monitoring specifies the configuration of monitoring.
    
    pub monitoring: Option<ConfigManagementPolicyControllerMonitoring>,
    /// Enable or disable mutation in policy controller. If true, mutation CRDs, webhook and controller deployment will be deployed to the cluster.
    #[serde(rename="mutationEnabled")]
    
    pub mutation_enabled: Option<bool>,
    /// Enables the ability to use Constraint Templates that reference to objects other than the object currently being evaluated.
    #[serde(rename="referentialRulesEnabled")]
    
    pub referential_rules_enabled: Option<bool>,
    /// Installs the default template library along with Policy Controller.
    #[serde(rename="templateLibraryInstalled")]
    
    pub template_library_installed: Option<bool>,
}

impl client::Part for ConfigManagementPolicyController {}


/// PolicyControllerMonitoring specifies the backends Policy Controller should export metrics to. For example, to specify metrics should be exported to Cloud Monitoring and Prometheus, specify backends: ["cloudmonitoring", "prometheus"]
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementPolicyControllerMonitoring {
    /// Specifies the list of backends Policy Controller will export to. An empty list would effectively disable metrics export.
    
    pub backends: Option<Vec<String>>,
}

impl client::Part for ConfigManagementPolicyControllerMonitoring {}


/// State for PolicyControllerState.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementPolicyControllerState {
    /// The state about the policy controller installation.
    #[serde(rename="deploymentState")]
    
    pub deployment_state: Option<ConfigManagementGatekeeperDeploymentState>,
    /// The version of Gatekeeper Policy Controller deployed.
    
    pub version: Option<ConfigManagementPolicyControllerVersion>,
}

impl client::Part for ConfigManagementPolicyControllerState {}


/// The build version of Gatekeeper Policy Controller is using.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementPolicyControllerVersion {
    /// The gatekeeper image tag that is composed of ACM version, git tag, build number.
    
    pub version: Option<String>,
}

impl client::Part for ConfigManagementPolicyControllerVersion {}


/// An ACM created error representing a problem syncing configurations
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementSyncError {
    /// An ACM defined error code
    
    pub code: Option<String>,
    /// A description of the error
    #[serde(rename="errorMessage")]
    
    pub error_message: Option<String>,
    /// A list of config(s) associated with the error, if any
    #[serde(rename="errorResources")]
    
    pub error_resources: Option<Vec<ConfigManagementErrorResource>>,
}

impl client::Part for ConfigManagementSyncError {}


/// State indicating an ACM's progress syncing configurations to a cluster
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigManagementSyncState {
    /// Sync status code
    
    pub code: Option<String>,
    /// A list of errors resulting from problematic configs. This list will be truncated after 100 errors, although it is unlikely for that many errors to simultaneously exist.
    
    pub errors: Option<Vec<ConfigManagementSyncError>>,
    /// Token indicating the state of the importer.
    #[serde(rename="importToken")]
    
    pub import_token: Option<String>,
    /// Deprecated: use last_sync_time instead. Timestamp of when ACM last successfully synced the repo The time format is specified in https://golang.org/pkg/time/#Time.String
    #[serde(rename="lastSync")]
    
    pub last_sync: Option<String>,
    /// Timestamp type of when ACM last successfully synced the repo
    #[serde(rename="lastSyncTime")]
    
    pub last_sync_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Token indicating the state of the repo.
    #[serde(rename="sourceToken")]
    
    pub source_token: Option<String>,
    /// Token indicating the state of the syncer.
    #[serde(rename="syncToken")]
    
    pub sync_token: Option<String>,
}

impl client::Part for ConfigManagementSyncState {}


/// ConnectAgentResource represents a Kubernetes resource manifest for Connect Agent deployment.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConnectAgentResource {
    /// YAML manifest of the resource.
    
    pub manifest: Option<String>,
    /// Kubernetes type of the resource.
    #[serde(rename="type")]
    
    pub type_: Option<TypeMeta>,
}

impl client::Part for ConnectAgentResource {}


/// EdgeCluster contains information specific to Google Edge Clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EdgeCluster {
    /// Immutable. Self-link of the GCP resource for the Edge Cluster. For example: //edgecontainer.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster
    #[serde(rename="resourceLink")]
    
    pub resource_link: Option<String>,
}

impl client::Part for EdgeCluster {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations operations delete projects](ProjectLocationOperationDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


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


/// Feature represents the settings and status of any Hub Feature.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features create projects](ProjectLocationFeatureCreateCall) (request)
/// * [locations features get projects](ProjectLocationFeatureGetCall) (response)
/// * [locations features patch projects](ProjectLocationFeaturePatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Feature {
    /// Output only. When the Feature resource was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. When the Feature resource was deleted.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// GCP labels for this Feature.
    
    pub labels: Option<HashMap<String, String>>,
    /// Optional. Membership-specific configuration for this Feature. If this Feature does not support any per-Membership configuration, this field may be unused. The keys indicate which Membership the configuration is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Membership is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature.
    #[serde(rename="membershipSpecs")]
    
    pub membership_specs: Option<HashMap<String, MembershipFeatureSpec>>,
    /// Output only. Membership-specific Feature status. If this Feature does report any per-Membership status, this field may be unused. The keys indicate which Membership the state is for, in the form: `projects/{p}/locations/{l}/memberships/{m}` Where {p} is the project number, {l} is a valid location and {m} is a valid Membership in this project at that location. {p} MUST match the Feature's project number.
    #[serde(rename="membershipStates")]
    
    pub membership_states: Option<HashMap<String, MembershipFeatureState>>,
    /// Output only. The full, unique name of this Feature resource in the format `projects/*/locations/*/features/*`.
    
    pub name: Option<String>,
    /// Output only. State of the Feature resource itself.
    #[serde(rename="resourceState")]
    
    pub resource_state: Option<FeatureResourceState>,
    /// Optional. Scope-specific configuration for this Feature. If this Feature does not support any per-Scope configuration, this field may be unused. The keys indicate which Scope the configuration is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project. {p} will always be returned as the project number, but the project ID is also accepted during input. If the same Scope is specified in the map twice (using the project ID form, and the project number form), exactly ONE of the entries will be saved, with no guarantees as to which. For this reason, it is recommended the same format be used for all entries when mutating a Feature.
    #[serde(rename="scopeSpecs")]
    
    pub scope_specs: Option<HashMap<String, ScopeFeatureSpec>>,
    /// Output only. Scope-specific Feature status. If this Feature does report any per-Scope status, this field may be unused. The keys indicate which Scope the state is for, in the form: `projects/{p}/locations/global/scopes/{s}` Where {p} is the project, {s} is a valid Scope in this project. {p} WILL match the Feature's project.
    #[serde(rename="scopeStates")]
    
    pub scope_states: Option<HashMap<String, ScopeFeatureState>>,
    /// Optional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused.
    
    pub spec: Option<CommonFeatureSpec>,
    /// Output only. The Hub-wide Feature state.
    
    pub state: Option<CommonFeatureState>,
    /// Output only. When the Feature resource was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Feature {}
impl client::ResponseResult for Feature {}


/// FeatureResourceState describes the state of a Feature *resource* in the GkeHub API. See `FeatureState` for the "running state" of the Feature in the Hub and across Memberships.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureResourceState {
    /// The current state of the Feature resource in the Hub API.
    
    pub state: Option<String>,
}

impl client::Part for FeatureResourceState {}


/// FeatureState describes the high-level state of a Feature. It may be used to describe a Feature's state at the environ-level, or per-membershop, depending on the context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FeatureState {
    /// The high-level, machine-readable status of this Feature.
    
    pub code: Option<String>,
    /// A human-readable description of the current status.
    
    pub description: Option<String>,
    /// The time this status and any related Feature-specific details were updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for FeatureState {}


/// **Fleet Observability**: The Hub-wide input for the FleetObservability feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FleetObservabilityFeatureSpec { _never_set: Option<bool> }

impl client::Part for FleetObservabilityFeatureSpec {}


/// **FleetObservability**: An empty state left as an example Hub-wide Feature state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FleetObservabilityFeatureState { _never_set: Option<bool> }

impl client::Part for FleetObservabilityFeatureState {}


/// **FleetObservability**: The membership-specific input for FleetObservability feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FleetObservabilityMembershipSpec { _never_set: Option<bool> }

impl client::Part for FleetObservabilityMembershipSpec {}


/// **FleetObservability**: An empty state left as an example membership-specific Feature state.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FleetObservabilityMembershipState { _never_set: Option<bool> }

impl client::Part for FleetObservabilityMembershipState {}


/// GenerateConnectManifestResponse contains manifest information for installing/upgrading a Connect agent.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations memberships generate connect manifest projects](ProjectLocationMembershipGenerateConnectManifestCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GenerateConnectManifestResponse {
    /// The ordered list of Kubernetes resources that need to be applied to the cluster for GKE Connect agent installation/upgrade.
    
    pub manifest: Option<Vec<ConnectAgentResource>>,
}

impl client::ResponseResult for GenerateConnectManifestResponse {}


/// GkeCluster contains information specific to GKE clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GkeCluster {
    /// Output only. If cluster_missing is set then it denotes that the GKE cluster no longer exists in the GKE Control Plane.
    #[serde(rename="clusterMissing")]
    
    pub cluster_missing: Option<bool>,
    /// Immutable. Self-link of the GCP resource for the GKE cluster. For example: //container.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster Zonal clusters are also supported.
    #[serde(rename="resourceLink")]
    
    pub resource_link: Option<String>,
}

impl client::Part for GkeCluster {}


/// The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleRpcStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    
    pub code: Option<i32>,
    /// A list of messages that carry the error details. There is a common set of message types for APIs to use.
    
    pub details: Option<Vec<HashMap<String, json::Value>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    
    pub message: Option<String>,
}

impl client::Part for GoogleRpcStatus {}


/// Configuration of an auth method for a member/cluster. Only one authentication method (e.g., OIDC and LDAP) can be set per AuthMethod.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceAuthMethod {
    /// AzureAD specific Configuration.
    #[serde(rename="azureadConfig")]
    
    pub azuread_config: Option<IdentityServiceAzureADConfig>,
    /// GoogleConfig specific configuration
    #[serde(rename="googleConfig")]
    
    pub google_config: Option<IdentityServiceGoogleConfig>,
    /// Identifier for auth config.
    
    pub name: Option<String>,
    /// OIDC specific configuration.
    #[serde(rename="oidcConfig")]
    
    pub oidc_config: Option<IdentityServiceOidcConfig>,
    /// Proxy server address to use for auth method.
    
    pub proxy: Option<String>,
}

impl client::Part for IdentityServiceAuthMethod {}


/// Configuration for the AzureAD Auth flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceAzureADConfig {
    /// ID for the registered client application that makes authentication requests to the Azure AD identity provider.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Input only. Unencrypted AzureAD client secret will be passed to the GKE Hub CLH.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<String>,
    /// Output only. Encrypted AzureAD client secret.
    #[serde(rename="encryptedClientSecret")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encrypted_client_secret: Option<Vec<u8>>,
    /// The redirect URL that kubectl uses for authorization.
    #[serde(rename="kubectlRedirectUri")]
    
    pub kubectl_redirect_uri: Option<String>,
    /// Kind of Azure AD account to be authenticated. Supported values are or for accounts belonging to a specific tenant.
    
    pub tenant: Option<String>,
}

impl client::Part for IdentityServiceAzureADConfig {}


/// Configuration for the Google Plugin Auth flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceGoogleConfig {
    /// Disable automatic configuration of Google Plugin on supported platforms.
    
    pub disable: Option<bool>,
}

impl client::Part for IdentityServiceGoogleConfig {}


/// **Anthos Identity Service**: Configuration for a single Membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceMembershipSpec {
    /// A member may support multiple auth methods.
    #[serde(rename="authMethods")]
    
    pub auth_methods: Option<Vec<IdentityServiceAuthMethod>>,
}

impl client::Part for IdentityServiceMembershipSpec {}


/// **Anthos Identity Service**: State for a single Membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceMembershipState {
    /// The reason of the failure.
    #[serde(rename="failureReason")]
    
    pub failure_reason: Option<String>,
    /// Installed AIS version. This is the AIS version installed on this member. The values makes sense iff state is OK.
    #[serde(rename="installedVersion")]
    
    pub installed_version: Option<String>,
    /// Last reconciled membership configuration
    #[serde(rename="memberConfig")]
    
    pub member_config: Option<IdentityServiceMembershipSpec>,
    /// Deployment state on this member
    
    pub state: Option<String>,
}

impl client::Part for IdentityServiceMembershipState {}


/// Configuration for OIDC Auth flow.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IdentityServiceOidcConfig {
    /// PEM-encoded CA for OIDC provider.
    #[serde(rename="certificateAuthorityData")]
    
    pub certificate_authority_data: Option<String>,
    /// ID for OIDC client application.
    #[serde(rename="clientId")]
    
    pub client_id: Option<String>,
    /// Input only. Unencrypted OIDC client secret will be passed to the GKE Hub CLH.
    #[serde(rename="clientSecret")]
    
    pub client_secret: Option<String>,
    /// Flag to denote if reverse proxy is used to connect to auth provider. This flag should be set to true when provider is not reachable by Google Cloud Console.
    #[serde(rename="deployCloudConsoleProxy")]
    
    pub deploy_cloud_console_proxy: Option<bool>,
    /// Enable access token.
    #[serde(rename="enableAccessToken")]
    
    pub enable_access_token: Option<bool>,
    /// Output only. Encrypted OIDC Client secret
    #[serde(rename="encryptedClientSecret")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub encrypted_client_secret: Option<Vec<u8>>,
    /// Comma-separated list of key-value pairs.
    #[serde(rename="extraParams")]
    
    pub extra_params: Option<String>,
    /// Prefix to prepend to group name.
    #[serde(rename="groupPrefix")]
    
    pub group_prefix: Option<String>,
    /// Claim in OIDC ID token that holds group information.
    #[serde(rename="groupsClaim")]
    
    pub groups_claim: Option<String>,
    /// URI for the OIDC provider. This should point to the level below .well-known/openid-configuration.
    #[serde(rename="issuerUri")]
    
    pub issuer_uri: Option<String>,
    /// Registered redirect uri to redirect users going through OAuth flow using kubectl plugin.
    #[serde(rename="kubectlRedirectUri")]
    
    pub kubectl_redirect_uri: Option<String>,
    /// Comma-separated list of identifiers.
    
    pub scopes: Option<String>,
    /// Claim in OIDC ID token that holds username.
    #[serde(rename="userClaim")]
    
    pub user_claim: Option<String>,
    /// Prefix to prepend to user name.
    #[serde(rename="userPrefix")]
    
    pub user_prefix: Option<String>,
}

impl client::Part for IdentityServiceOidcConfig {}


/// KubernetesMetadata provides informational metadata for Memberships representing Kubernetes clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesMetadata {
    /// Output only. Kubernetes API server version string as reported by `/version`.
    #[serde(rename="kubernetesApiServerVersion")]
    
    pub kubernetes_api_server_version: Option<String>,
    /// Output only. The total memory capacity as reported by the sum of all Kubernetes nodes resources, defined in MB.
    #[serde(rename="memoryMb")]
    
    pub memory_mb: Option<i32>,
    /// Output only. Node count as reported by Kubernetes nodes resources.
    #[serde(rename="nodeCount")]
    
    pub node_count: Option<i32>,
    /// Output only. Node providerID as reported by the first node in the list of nodes on the Kubernetes endpoint. On Kubernetes platforms that support zero-node clusters (like GKE-on-GCP), the node_count will be zero and the node_provider_id will be empty.
    #[serde(rename="nodeProviderId")]
    
    pub node_provider_id: Option<String>,
    /// Output only. The time at which these details were last updated. This update_time is different from the Membership-level update_time since EndpointDetails are updated internally for API consumers.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. vCPU count as reported by Kubernetes nodes resources.
    #[serde(rename="vcpuCount")]
    
    pub vcpu_count: Option<i32>,
}

impl client::Part for KubernetesMetadata {}


/// KubernetesResource contains the YAML manifests and configuration for Membership Kubernetes resources in the cluster. After CreateMembership or UpdateMembership, these resources should be re-applied in the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct KubernetesResource {
    /// Output only. The Kubernetes resources for installing the GKE Connect agent This field is only populated in the Membership returned from a successful long-running operation from CreateMembership or UpdateMembership. It is not populated during normal GetMembership or ListMemberships requests. To get the resource manifest after the initial registration, the caller should make a UpdateMembership call with an empty field mask.
    #[serde(rename="connectResources")]
    
    pub connect_resources: Option<Vec<ResourceManifest>>,
    /// Input only. The YAML representation of the Membership CR. This field is ignored for GKE clusters where Hub can read the CR directly. Callers should provide the CR that is currently present in the cluster during CreateMembership or UpdateMembership, or leave this field empty if none exists. The CR manifest is used to validate the cluster has not been registered with another Membership.
    #[serde(rename="membershipCrManifest")]
    
    pub membership_cr_manifest: Option<String>,
    /// Output only. Additional Kubernetes resources that need to be applied to the cluster after Membership creation, and after every update. This field is only populated in the Membership returned from a successful long-running operation from CreateMembership or UpdateMembership. It is not populated during normal GetMembership or ListMemberships requests. To get the resource manifest after the initial registration, the caller should make a UpdateMembership call with an empty field mask.
    #[serde(rename="membershipResources")]
    
    pub membership_resources: Option<Vec<ResourceManifest>>,
    /// Optional. Options for Kubernetes resource generation.
    #[serde(rename="resourceOptions")]
    
    pub resource_options: Option<ResourceOptions>,
}

impl client::Part for KubernetesResource {}


/// Response message for the `GkeHub.ListFeatures` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features list projects](ProjectLocationFeatureListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFeaturesResponse {
    /// A token to request the next page of resources from the `ListFeatures` method. The value of an empty string means that there are no more resources to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching Features
    
    pub resources: Option<Vec<Feature>>,
}

impl client::ResponseResult for ListFeaturesResponse {}


/// The response message for Locations.ListLocations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations list projects](ProjectLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    
    pub locations: Option<Vec<Location>>,
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListLocationsResponse {}


/// Response message for the `GkeHub.ListMemberships` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations memberships list projects](ProjectLocationMembershipListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListMembershipsResponse {
    /// A token to request the next page of resources from the `ListMemberships` method. The value of an empty string means that there are no more resources to return.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of matching Memberships.
    
    pub resources: Option<Vec<Membership>>,
    /// List of locations that could not be reached while fetching this list.
    
    pub unreachable: Option<Vec<String>>,
}

impl client::ResponseResult for ListMembershipsResponse {}


/// The response message for Operations.ListOperations.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations operations list projects](ProjectLocationOperationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListOperationsResponse {
    /// The standard List next-page token.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// A list of operations that matches the specified filter in the request.
    
    pub operations: Option<Vec<Operation>>,
}

impl client::ResponseResult for ListOperationsResponse {}


/// A resource that represents Google Cloud Platform location.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations get projects](ProjectLocationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// The friendly name for this location, typically a nearby city name. For example, "Tokyo".
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"}
    
    pub labels: Option<HashMap<String, String>>,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Service-specific metadata. For example the available capacity at the given location.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"`
    
    pub name: Option<String>,
}

impl client::ResponseResult for Location {}


/// Membership contains information about a member cluster.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations memberships create projects](ProjectLocationMembershipCreateCall) (request)
/// * [locations memberships get projects](ProjectLocationMembershipGetCall) (response)
/// * [locations memberships patch projects](ProjectLocationMembershipPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Membership {
    /// Optional. How to identify workloads from this Membership. See the documentation on Workload Identity for more details: https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity
    
    pub authority: Option<Authority>,
    /// Output only. When the Membership was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. When the Membership was deleted.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Description of this membership, limited to 63 characters. Must match the regex: `a-zA-Z0-9*` This field is present for legacy purposes.
    
    pub description: Option<String>,
    /// Optional. Endpoint information to reach this member.
    
    pub endpoint: Option<MembershipEndpoint>,
    /// Optional. An externally-generated and managed ID for this Membership. This ID may be modified after creation, but this is not recommended. The ID must match the regex: `a-zA-Z0-9*` If this Membership represents a Kubernetes cluster, this value should be set to the UID of the `kube-system` namespace object.
    #[serde(rename="externalId")]
    
    pub external_id: Option<String>,
    /// Optional. GCP labels for this membership.
    
    pub labels: Option<HashMap<String, String>>,
    /// Output only. For clusters using Connect, the timestamp of the most recent connection established with Google Cloud. This time is updated every several minutes, not continuously. For clusters that do not use GKE Connect, or that have never connected successfully, this field will be unset.
    #[serde(rename="lastConnectionTime")]
    
    pub last_connection_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. The full, unique name of this Membership resource in the format `projects/*/locations/*/memberships/{membership_id}`, set during creation. `membership_id` must be a valid RFC 1123 compliant DNS label: 1. At most 63 characters in length 2. It must consist of lower case alphanumeric characters or `-` 3. It must start and end with an alphanumeric character Which can be expressed as the regex: `[a-z0-9]([-a-z0-9]*[a-z0-9])?`, with a maximum length of 63 characters.
    
    pub name: Option<String>,
    /// Output only. State of the Membership resource.
    
    pub state: Option<MembershipState>,
    /// Output only. Google-generated UUID for this resource. This is unique across all Membership resources. If a Membership resource is deleted and another resource with the same name is created, it gets a different unique_id.
    #[serde(rename="uniqueId")]
    
    pub unique_id: Option<String>,
    /// Output only. When the Membership was last updated.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for Membership {}
impl client::ResponseResult for Membership {}


/// MembershipEndpoint contains information needed to contact a Kubernetes API, endpoint and any additional Kubernetes metadata.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipEndpoint {
    /// Optional. Specific information for a GDC Edge Appliance cluster.
    #[serde(rename="applianceCluster")]
    
    pub appliance_cluster: Option<ApplianceCluster>,
    /// Optional. Specific information for a Google Edge cluster.
    #[serde(rename="edgeCluster")]
    
    pub edge_cluster: Option<EdgeCluster>,
    /// Optional. Specific information for a GKE-on-GCP cluster.
    #[serde(rename="gkeCluster")]
    
    pub gke_cluster: Option<GkeCluster>,
    /// Output only. Useful Kubernetes-specific metadata.
    #[serde(rename="kubernetesMetadata")]
    
    pub kubernetes_metadata: Option<KubernetesMetadata>,
    /// Optional. The in-cluster Kubernetes Resources that should be applied for a correctly registered cluster, in the steady state. These resources: * Ensure that the cluster is exclusively registered to one and only one Hub Membership. * Propagate Workload Pool Information available in the Membership Authority field. * Ensure proper initial configuration of default Hub Features.
    #[serde(rename="kubernetesResource")]
    
    pub kubernetes_resource: Option<KubernetesResource>,
    /// Optional. Specific information for a GKE Multi-Cloud cluster.
    #[serde(rename="multiCloudCluster")]
    
    pub multi_cloud_cluster: Option<MultiCloudCluster>,
    /// Optional. Specific information for a GKE On-Prem cluster. An onprem user-cluster who has no resourceLink is not allowed to use this field, it should have a nil "type" instead.
    #[serde(rename="onPremCluster")]
    
    pub on_prem_cluster: Option<OnPremCluster>,
}

impl client::Part for MembershipEndpoint {}


/// MembershipFeatureSpec contains configuration information for a single Membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipFeatureSpec {
    /// Config Management-specific spec.
    
    pub configmanagement: Option<ConfigManagementMembershipSpec>,
    /// True if value of `feature_spec` was inherited from a fleet-level default.
    #[serde(rename="fleetInherited")]
    
    pub fleet_inherited: Option<bool>,
    /// Fleet observability membership spec
    
    pub fleetobservability: Option<FleetObservabilityMembershipSpec>,
    /// Identity Service-specific spec.
    
    pub identityservice: Option<IdentityServiceMembershipSpec>,
    /// Anthos Service Mesh-specific spec
    
    pub mesh: Option<ServiceMeshMembershipSpec>,
}

impl client::Part for MembershipFeatureSpec {}


/// MembershipFeatureState contains Feature status information for a single Membership.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipFeatureState {
    /// Appdevexperience specific state.
    
    pub appdevexperience: Option<AppDevExperienceFeatureState>,
    /// Config Management-specific state.
    
    pub configmanagement: Option<ConfigManagementMembershipState>,
    /// Fleet observability membership state.
    
    pub fleetobservability: Option<FleetObservabilityMembershipState>,
    /// Identity Service-specific state.
    
    pub identityservice: Option<IdentityServiceMembershipState>,
    /// Service Mesh-specific state.
    
    pub servicemesh: Option<ServiceMeshMembershipState>,
    /// The high-level state of this Feature for a single membership.
    
    pub state: Option<FeatureState>,
}

impl client::Part for MembershipFeatureState {}


/// MembershipState describes the state of a Membership resource.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MembershipState {
    /// Output only. The current state of the Membership resource.
    
    pub code: Option<String>,
}

impl client::Part for MembershipState {}


/// MultiCloudCluster contains information specific to GKE Multi-Cloud clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiCloudCluster {
    /// Output only. If cluster_missing is set then it denotes that API(gkemulticloud.googleapis.com) resource for this GKE Multi-Cloud cluster no longer exists.
    #[serde(rename="clusterMissing")]
    
    pub cluster_missing: Option<bool>,
    /// Immutable. Self-link of the GCP resource for the GKE Multi-Cloud cluster. For example: //gkemulticloud.googleapis.com/projects/my-project/locations/us-west1-a/awsClusters/my-cluster //gkemulticloud.googleapis.com/projects/my-project/locations/us-west1-a/azureClusters/my-cluster //gkemulticloud.googleapis.com/projects/my-project/locations/us-west1-a/attachedClusters/my-cluster
    #[serde(rename="resourceLink")]
    
    pub resource_link: Option<String>,
}

impl client::Part for MultiCloudCluster {}


/// **Multi-cluster Ingress**: The configuration for the MultiClusterIngress feature.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiClusterIngressFeatureSpec {
    /// Fully-qualified Membership name which hosts the MultiClusterIngress CRD. Example: `projects/foo-proj/locations/global/memberships/bar`
    #[serde(rename="configMembership")]
    
    pub config_membership: Option<String>,
}

impl client::Part for MultiClusterIngressFeatureSpec {}


/// OnPremCluster contains information specific to GKE On-Prem clusters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct OnPremCluster {
    /// Immutable. Whether the cluster is an admin cluster.
    #[serde(rename="adminCluster")]
    
    pub admin_cluster: Option<bool>,
    /// Output only. If cluster_missing is set then it denotes that API(gkeonprem.googleapis.com) resource for this GKE On-Prem cluster no longer exists.
    #[serde(rename="clusterMissing")]
    
    pub cluster_missing: Option<bool>,
    /// Immutable. The on prem cluster's type.
    #[serde(rename="clusterType")]
    
    pub cluster_type: Option<String>,
    /// Immutable. Self-link of the GCP resource for the GKE On-Prem cluster. For example: //gkeonprem.googleapis.com/projects/my-project/locations/us-west1-a/vmwareClusters/my-cluster //gkeonprem.googleapis.com/projects/my-project/locations/us-west1-a/bareMetalClusters/my-cluster
    #[serde(rename="resourceLink")]
    
    pub resource_link: Option<String>,
}

impl client::Part for OnPremCluster {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features create projects](ProjectLocationFeatureCreateCall) (response)
/// * [locations features delete projects](ProjectLocationFeatureDeleteCall) (response)
/// * [locations features patch projects](ProjectLocationFeaturePatchCall) (response)
/// * [locations memberships create projects](ProjectLocationMembershipCreateCall) (response)
/// * [locations memberships delete projects](ProjectLocationMembershipDeleteCall) (response)
/// * [locations memberships patch projects](ProjectLocationMembershipPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Operation {
    /// If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available.
    
    pub done: Option<bool>,
    /// The error result of the operation in case of failure or cancellation.
    
    pub error: Option<GoogleRpcStatus>,
    /// Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any.
    
    pub metadata: Option<HashMap<String, json::Value>>,
    /// The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`.
    
    pub name: Option<String>,
    /// The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`.
    
    pub response: Option<HashMap<String, json::Value>>,
}

impl client::ResponseResult for Operation {}


/// An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members`, or principals, to a single `role`. Principals can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { “bindings”: \[ { “role”: “roles/resourcemanager.organizationAdmin”, “members”: \[ “user:mike@example.com”, “group:admins@example.com”, “domain:google.com”, “serviceAccount:my-project-id@appspot.gserviceaccount.com” \] }, { “role”: “roles/resourcemanager.organizationViewer”, “members”: \[ “user:eve@example.com” \], “condition”: { “title”: “expirable access”, “description”: “Does not grant access after Sep 2020”, “expression”: “request.time \< timestamp(‘2020-10-01T00:00:00.000Z’)”, } } \], “etag”: “BwWWja0YfJA=”, “version”: 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time \< timestamp(‘2020-10-01T00:00:00.000Z’) etag: BwWWja0YfJA= version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features get iam policy projects](ProjectLocationFeatureGetIamPolicyCall) (response)
/// * [locations features set iam policy projects](ProjectLocationFeatureSetIamPolicyCall) (response)
/// * [locations memberships get iam policy projects](ProjectLocationMembershipGetIamPolicyCall) (response)
/// * [locations memberships set iam policy projects](ProjectLocationMembershipSetIamPolicyCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Specifies cloud audit logging configuration for this policy.
    #[serde(rename="auditConfigs")]
    
    pub audit_configs: Option<Vec<AuditConfig>>,
    /// Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`.
    
    pub bindings: Option<Vec<Binding>>,
    /// `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub etag: Option<Vec<u8>>,
    /// Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    
    pub version: Option<i32>,
}

impl client::ResponseResult for Policy {}


/// ResourceManifest represents a single Kubernetes resource to be applied to the cluster.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceManifest {
    /// Whether the resource provided in the manifest is `cluster_scoped`. If unset, the manifest is assumed to be namespace scoped. This field is used for REST mapping when applying the resource in a cluster.
    #[serde(rename="clusterScoped")]
    
    pub cluster_scoped: Option<bool>,
    /// YAML manifest of the resource.
    
    pub manifest: Option<String>,
}

impl client::Part for ResourceManifest {}


/// ResourceOptions represent options for Kubernetes resource generation.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ResourceOptions {
    /// Optional. The Connect agent version to use for connect_resources. Defaults to the latest GKE Connect version. The version must be a currently supported version, obsolete versions will be rejected.
    #[serde(rename="connectVersion")]
    
    pub connect_version: Option<String>,
    /// Optional. Major version of the Kubernetes cluster. This is only used to determine which version to use for the CustomResourceDefinition resources, `apiextensions/v1beta1` or`apiextensions/v1`.
    #[serde(rename="k8sVersion")]
    
    pub k8s_version: Option<String>,
    /// Optional. Use `apiextensions/v1beta1` instead of `apiextensions/v1` for CustomResourceDefinition resources. This option should be set for clusters with Kubernetes apiserver versions <1.16.
    #[serde(rename="v1beta1Crd")]
    
    pub v1beta1_crd: Option<bool>,
}

impl client::Part for ResourceOptions {}


/// ScopeFeatureSpec contains feature specs for a fleet scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScopeFeatureSpec { _never_set: Option<bool> }

impl client::Part for ScopeFeatureSpec {}


/// ScopeFeatureState contains Scope-wide Feature status information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ScopeFeatureState {
    /// Output only. The "running state" of the Feature in this Scope.
    
    pub state: Option<FeatureState>,
}

impl client::Part for ScopeFeatureState {}


/// Status of control plane management.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceMeshControlPlaneManagement {
    /// Explanation of state.
    
    pub details: Option<Vec<ServiceMeshStatusDetails>>,
    /// LifecycleState of control plane management.
    
    pub state: Option<String>,
}

impl client::Part for ServiceMeshControlPlaneManagement {}


/// Status of data plane management. Only reported per-member.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceMeshDataPlaneManagement {
    /// Explanation of the status.
    
    pub details: Option<Vec<ServiceMeshStatusDetails>>,
    /// Lifecycle status of data plane management.
    
    pub state: Option<String>,
}

impl client::Part for ServiceMeshDataPlaneManagement {}


/// **Service Mesh**: Spec for a single Membership for the servicemesh feature
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceMeshMembershipSpec {
    /// Enables automatic control plane management.
    #[serde(rename="controlPlane")]
    
    pub control_plane: Option<String>,
    /// Enables automatic Service Mesh management.
    
    pub management: Option<String>,
}

impl client::Part for ServiceMeshMembershipSpec {}


/// **Service Mesh**: State for a single Membership, as analyzed by the Service Mesh Hub Controller.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceMeshMembershipState {
    /// Output only. Status of control plane management
    #[serde(rename="controlPlaneManagement")]
    
    pub control_plane_management: Option<ServiceMeshControlPlaneManagement>,
    /// Output only. Status of data plane management.
    #[serde(rename="dataPlaneManagement")]
    
    pub data_plane_management: Option<ServiceMeshDataPlaneManagement>,
}

impl client::Part for ServiceMeshMembershipState {}


/// Structured and human-readable details for a status.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceMeshStatusDetails {
    /// A machine-readable code that further describes a broad status.
    
    pub code: Option<String>,
    /// Human-readable explanation of code.
    
    pub details: Option<String>,
}

impl client::Part for ServiceMeshStatusDetails {}


/// Request message for `SetIamPolicy` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features set iam policy projects](ProjectLocationFeatureSetIamPolicyCall) (request)
/// * [locations memberships set iam policy projects](ProjectLocationMembershipSetIamPolicyCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them.
    
    pub policy: Option<Policy>,
    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"`
    #[serde(rename="updateMask")]
    
    pub update_mask: Option<client::FieldMask>,
}

impl client::RequestValue for SetIamPolicyRequest {}


/// Status specifies state for the subcomponent.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    /// Code specifies AppDevExperienceFeature's subcomponent ready state.
    
    pub code: Option<String>,
    /// Description is populated if Code is Failed, explaining why it has failed.
    
    pub description: Option<String>,
}

impl client::Part for Status {}


/// Request message for `TestIamPermissions` method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations features test iam permissions projects](ProjectLocationFeatureTestIamPermissionCall) (request)
/// * [locations memberships test iam permissions projects](ProjectLocationMembershipTestIamPermissionCall) (request)
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
/// * [locations features test iam permissions projects](ProjectLocationFeatureTestIamPermissionCall) (response)
/// * [locations memberships test iam permissions projects](ProjectLocationMembershipTestIamPermissionCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is allowed.
    
    pub permissions: Option<Vec<String>>,
}

impl client::ResponseResult for TestIamPermissionsResponse {}


/// TypeMeta is the type information needed for content unmarshalling of Kubernetes resources in the manifest.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TypeMeta {
    /// APIVersion of the resource (e.g. v1).
    #[serde(rename="apiVersion")]
    
    pub api_version: Option<String>,
    /// Kind of the resource (e.g. Deployment).
    
    pub kind: Option<String>,
}

impl client::Part for TypeMeta {}


