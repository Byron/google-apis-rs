use super::*;
/// ApprovalConfig describes configuration for manual approval of a build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalConfig {
    /// Whether or not approval is needed. If this is set on a build, it will become pending when created, and will need to be explicitly approved to start.
    #[serde(rename="approvalRequired")]
    
    pub approval_required: Option<bool>,
}

impl client::Part for ApprovalConfig {}


/// ApprovalResult describes the decision and associated metadata of a manual approval of a build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalResult {
    /// Output only. The time when the approval decision was made.
    #[serde(rename="approvalTime")]
    
    pub approval_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Email of the user that called the ApproveBuild API to approve or reject a build at the time that the API was called.
    #[serde(rename="approverAccount")]
    
    pub approver_account: Option<String>,
    /// Optional. An optional comment for this manual approval result.
    
    pub comment: Option<String>,
    /// Required. The decision of this manual approval.
    
    pub decision: Option<ApprovalResultDecisionEnum>,
    /// Optional. An optional URL tied to this manual approval result. This field is essentially the same as comment, except that it will be rendered by the UI differently. An example use case is a link to an external job that approved this Build.
    
    pub url: Option<String>,
}

impl client::Part for ApprovalResult {}


/// Request to approve or reject a pending build.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [builds approve projects](ProjectBuildApproveCall) (request)
/// * [locations builds approve projects](ProjectLocationBuildApproveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ApproveBuildRequest {
    /// Approval decision and metadata.
    #[serde(rename="approvalResult")]
    
    pub approval_result: Option<ApprovalResult>,
}

impl client::RequestValue for ApproveBuildRequest {}


/// Files in the workspace to upload to Cloud Storage upon successful completion of all build steps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactObjects {
    /// Cloud Storage bucket and optional object path, in the form "gs://bucket/path/to/somewhere/". (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Files in the workspace matching any path pattern will be uploaded to Cloud Storage with this location as a prefix.
    
    pub location: Option<String>,
    /// Path globs used to match files in the build's workspace.
    
    pub paths: Option<Vec<String>>,
    /// Output only. Stores timing information for pushing all artifact objects.
    
    pub timing: Option<TimeSpan>,
}

impl client::Part for ArtifactObjects {}


/// Artifacts produced by a build that should be uploaded upon successful completion of all build steps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Artifacts {
    /// A list of images to be pushed upon the successful completion of all build steps. The images will be pushed using the builder service account's credentials. The digests of the pushed images will be stored in the Build resource's results field. If any of the images fail to be pushed, the build is marked FAILURE.
    
    pub images: Option<Vec<String>>,
    /// A list of Maven artifacts to be uploaded to Artifact Registry upon successful completion of all build steps. Artifacts in the workspace matching specified paths globs will be uploaded to the specified Artifact Registry repository using the builder service account's credentials. If any artifacts fail to be pushed, the build is marked FAILURE.
    #[serde(rename="mavenArtifacts")]
    
    pub maven_artifacts: Option<Vec<MavenArtifact>>,
    /// A list of objects to be uploaded to Cloud Storage upon successful completion of all build steps. Files in the workspace matching specified paths globs will be uploaded to the specified Cloud Storage location using the builder service account's credentials. The location and generation of the uploaded objects will be stored in the Build resource's results field. If any objects fail to be pushed, the build is marked FAILURE.
    
    pub objects: Option<ArtifactObjects>,
    /// A list of Python packages to be uploaded to Artifact Registry upon successful completion of all build steps. The build service account credentials will be used to perform the upload. If any objects fail to be pushed, the build is marked FAILURE.
    #[serde(rename="pythonPackages")]
    
    pub python_packages: Option<Vec<PythonPackage>>,
}

impl client::Part for Artifacts {}


/// RPC request object accepted by BatchCreateBitbucketServerConnectedRepositories RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations bitbucket server configs connected repositories batch create projects](ProjectLocationBitbucketServerConfigConnectedRepositoryBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateBitbucketServerConnectedRepositoriesRequest {
    /// Required. Requests to connect Bitbucket Server repositories.
    
    pub requests: Option<Vec<CreateBitbucketServerConnectedRepositoryRequest>>,
}

impl client::RequestValue for BatchCreateBitbucketServerConnectedRepositoriesRequest {}


/// RPC request object accepted by BatchCreateGitLabConnectedRepositories RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations git lab configs connected repositories batch create projects](ProjectLocationGitLabConfigConnectedRepositoryBatchCreateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BatchCreateGitLabConnectedRepositoriesRequest {
    /// Required. Requests to connect GitLab repositories.
    
    pub requests: Option<Vec<CreateGitLabConnectedRepositoryRequest>>,
}

impl client::RequestValue for BatchCreateGitLabConnectedRepositoriesRequest {}


/// BitbucketServerConfig represents the configuration for a Bitbucket Server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations bitbucket server configs create projects](ProjectLocationBitbucketServerConfigCreateCall) (request)
/// * [locations bitbucket server configs get projects](ProjectLocationBitbucketServerConfigGetCall) (response)
/// * [locations bitbucket server configs patch projects](ProjectLocationBitbucketServerConfigPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerConfig {
    /// Required. Immutable. API Key that will be attached to webhook. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig.
    #[serde(rename="apiKey")]
    
    pub api_key: Option<String>,
    /// Output only. Connected Bitbucket Server repositories for this config.
    #[serde(rename="connectedRepositories")]
    
    pub connected_repositories: Option<Vec<BitbucketServerRepositoryId>>,
    /// Time when the config was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Required. Immutable. The URI of the Bitbucket Server host. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig.
    #[serde(rename="hostUri")]
    
    pub host_uri: Option<String>,
    /// The resource name for the config.
    
    pub name: Option<String>,
    /// Optional. The network to be used when reaching out to the Bitbucket Server instance. The VPC network must be enabled for private service connection. This should be set if the Bitbucket Server instance is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the Bitbucket Server instance will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project.
    #[serde(rename="peeredNetwork")]
    
    pub peered_network: Option<String>,
    /// Required. Secret Manager secrets needed by the config.
    
    pub secrets: Option<BitbucketServerSecrets>,
    /// Optional. SSL certificate to use for requests to Bitbucket Server. The format should be PEM format but the extension can be one of .pem, .cer, or .crt.
    #[serde(rename="sslCa")]
    
    pub ssl_ca: Option<String>,
    /// Username of the account Cloud Build will use on Bitbucket Server.
    
    pub username: Option<String>,
    /// Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config.
    #[serde(rename="webhookKey")]
    
    pub webhook_key: Option<String>,
}

impl client::RequestValue for BitbucketServerConfig {}
impl client::ResponseResult for BitbucketServerConfig {}


/// / BitbucketServerConnectedRepository represents a connected Bitbucket Server / repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerConnectedRepository {
    /// The name of the `BitbucketServerConfig` that added connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`
    
    pub parent: Option<String>,
    /// The Bitbucket Server repositories to connect.
    
    pub repo: Option<BitbucketServerRepositoryId>,
    /// Output only. The status of the repo connection request.
    
    pub status: Option<Status>,
}

impl client::Part for BitbucketServerConnectedRepository {}


/// BitbucketServerRepository represents a repository hosted on a Bitbucket Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerRepository {
    /// Link to the browse repo page on the Bitbucket Server instance.
    #[serde(rename="browseUri")]
    
    pub browse_uri: Option<String>,
    /// Description of the repository.
    
    pub description: Option<String>,
    /// Display name of the repository.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the repository.
    
    pub name: Option<String>,
    /// Identifier for a repository hosted on a Bitbucket Server.
    #[serde(rename="repoId")]
    
    pub repo_id: Option<BitbucketServerRepositoryId>,
}

impl client::Part for BitbucketServerRepository {}


/// BitbucketServerRepositoryId identifies a specific repository hosted on a Bitbucket Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerRepositoryId {
    /// Required. Identifier for the project storing the repository.
    #[serde(rename="projectKey")]
    
    pub project_key: Option<String>,
    /// Required. Identifier for the repository.
    #[serde(rename="repoSlug")]
    
    pub repo_slug: Option<String>,
    /// Output only. The ID of the webhook that was created for receiving events from this repo. We only create and manage a single webhook for each repo.
    #[serde(rename="webhookId")]
    
    pub webhook_id: Option<i32>,
}

impl client::Part for BitbucketServerRepositoryId {}


/// BitbucketServerSecrets represents the secrets in Secret Manager for a Bitbucket Server.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerSecrets {
    /// Required. The resource name for the admin access token's secret version.
    #[serde(rename="adminAccessTokenVersionName")]
    
    pub admin_access_token_version_name: Option<String>,
    /// Required. The resource name for the read access token's secret version.
    #[serde(rename="readAccessTokenVersionName")]
    
    pub read_access_token_version_name: Option<String>,
    /// Required. Immutable. The resource name for the webhook secret's secret version. Once this field has been set, it cannot be changed. If you need to change it, please create another BitbucketServerConfig.
    #[serde(rename="webhookSecretVersionName")]
    
    pub webhook_secret_version_name: Option<String>,
}

impl client::Part for BitbucketServerSecrets {}


/// BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BitbucketServerTriggerConfig {
    /// Output only. The BitbucketServerConfig specified in the bitbucket_server_config_resource field.
    #[serde(rename="bitbucketServerConfig")]
    
    pub bitbucket_server_config: Option<BitbucketServerConfig>,
    /// Required. The Bitbucket server config resource that this trigger config maps to.
    #[serde(rename="bitbucketServerConfigResource")]
    
    pub bitbucket_server_config_resource: Option<String>,
    /// Required. Key of the project that the repo is in. For example: The key for https://mybitbucket.server/projects/TEST/repos/test-repo is "TEST".
    #[serde(rename="projectKey")]
    
    pub project_key: Option<String>,
    /// Filter to match changes in pull requests.
    #[serde(rename="pullRequest")]
    
    pub pull_request: Option<PullRequestFilter>,
    /// Filter to match changes in refs like branches, tags.
    
    pub push: Option<PushFilter>,
    /// Required. Slug of the repository. A repository slug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL. For example, if the repository name is 'test repo', in the URL it would become 'test-repo' as in https://mybitbucket.server/projects/TEST/repos/test-repo.
    #[serde(rename="repoSlug")]
    
    pub repo_slug: Option<String>,
}

impl client::Part for BitbucketServerTriggerConfig {}


/// A build resource in the Cloud Build API. At a high level, a `Build` describes where to find source code, how to build it (for example, the builder image to run on the source), and where to store the built artifacts. Fields can include the following variables, which will be expanded when the build is created: - $PROJECT_ID: the project ID of the build. - $PROJECT_NUMBER: the project number of the build. - $LOCATION: the location/region of the build. - $BUILD_ID: the autogenerated ID of the build. - $REPO_NAME: the source repository name specified by RepoSource. - $BRANCH_NAME: the branch name specified by RepoSource. - $TAG_NAME: the tag name specified by RepoSource. - $REVISION_ID or $COMMIT_SHA: the commit SHA specified by RepoSource or resolved from the specified branch or tag. - $SHORT_SHA: first 7 characters of $REVISION_ID or $COMMIT_SHA.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [builds cancel projects](ProjectBuildCancelCall) (response)
/// * [builds create projects](ProjectBuildCreateCall) (request)
/// * [builds get projects](ProjectBuildGetCall) (response)
/// * [locations builds cancel projects](ProjectLocationBuildCancelCall) (response)
/// * [locations builds create projects](ProjectLocationBuildCreateCall) (request)
/// * [locations builds get projects](ProjectLocationBuildGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Build {
    /// Output only. Describes this build's approval configuration, status, and result.
    
    pub approval: Option<BuildApproval>,
    /// Artifacts produced by the build that should be uploaded upon successful completion of all build steps.
    
    pub artifacts: Option<Artifacts>,
    /// Secrets and secret environment variables.
    #[serde(rename="availableSecrets")]
    
    pub available_secrets: Option<Secrets>,
    /// Output only. The ID of the `BuildTrigger` that triggered this build, if it was triggered automatically.
    #[serde(rename="buildTriggerId")]
    
    pub build_trigger_id: Option<String>,
    /// Output only. Time at which the request to create the build was received.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Contains information about the build when status=FAILURE.
    #[serde(rename="failureInfo")]
    
    pub failure_info: Option<FailureInfo>,
    /// Output only. Time at which execution of the build was finished. The difference between finish_time and start_time is the duration of the build's execution.
    #[serde(rename="finishTime")]
    
    pub finish_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Unique identifier of the build.
    
    pub id: Option<String>,
    /// A list of images to be pushed upon the successful completion of all build steps. The images are pushed using the builder service account's credentials. The digests of the pushed images will be stored in the `Build` resource's results field. If any of the images fail to be pushed, the build status is marked `FAILURE`.
    
    pub images: Option<Vec<String>>,
    /// Output only. URL to logs for this build in Google Cloud Console.
    #[serde(rename="logUrl")]
    
    pub log_url: Option<String>,
    /// Google Cloud Storage bucket where logs should be written (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)). Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`.
    #[serde(rename="logsBucket")]
    
    pub logs_bucket: Option<String>,
    /// Output only. The 'Build' name with format: `projects/{project}/locations/{location}/builds/{build}`, where {build} is a unique identifier generated by the service.
    
    pub name: Option<String>,
    /// Special options for this build.
    
    pub options: Option<BuildOptions>,
    /// Output only. ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// TTL in queue for this build. If provided and the build is enqueued longer than this value, the build will expire and the build status will be `EXPIRED`. The TTL starts ticking from create_time.
    #[serde(rename="queueTtl")]
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub queue_ttl: Option<client::chrono::Duration>,
    /// Output only. Results of the build.
    
    pub results: Option<Results>,
    /// Secrets to decrypt using Cloud Key Management Service. Note: Secret Manager is the recommended technique for managing sensitive data with Cloud Build. Use `available_secrets` to configure builds to access secrets from Secret Manager. For instructions, see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets
    
    pub secrets: Option<Vec<Secret>>,
    /// IAM service account whose credentials will be used at build runtime. Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`. ACCOUNT can be email address or uniqueId of the service account. 
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// The location of the source files to build.
    
    pub source: Option<Source>,
    /// Output only. A permanent fixed identifier for source.
    #[serde(rename="sourceProvenance")]
    
    pub source_provenance: Option<SourceProvenance>,
    /// Output only. Time at which execution of the build was started.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Status of the build.
    
    pub status: Option<BuildStatusEnum>,
    /// Output only. Customer-readable message about the current status.
    #[serde(rename="statusDetail")]
    
    pub status_detail: Option<String>,
    /// Required. The operations to be performed on the workspace.
    
    pub steps: Option<Vec<BuildStep>>,
    /// Substitutions data for `Build` resource.
    
    pub substitutions: Option<HashMap<String, String>>,
    /// Tags for annotation of a `Build`. These are not docker tags.
    
    pub tags: Option<Vec<String>>,
    /// Amount of time that this build should be allowed to run, to second granularity. If this amount of time elapses, work on the build will cease and the build status will be `TIMEOUT`. `timeout` starts ticking from `startTime`. Default time is 60 minutes.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
    /// Output only. Stores timing information for phases of the build. Valid keys are: * BUILD: time to execute all build steps. * PUSH: time to push all artifacts including docker images and non docker artifacts. * FETCHSOURCE: time to fetch source. * SETUPBUILD: time to set up build. If the build does not specify source or images, these keys will not be included.
    
    pub timing: Option<HashMap<String, TimeSpan>>,
    /// Output only. Non-fatal problems encountered during the execution of the build.
    
    pub warnings: Option<Vec<Warning>>,
}

impl client::RequestValue for Build {}
impl client::ResponseResult for Build {}


/// BuildApproval describes a build's approval configuration, state, and result.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildApproval {
    /// Output only. Configuration for manual approval of this build.
    
    pub config: Option<ApprovalConfig>,
    /// Output only. Result of manual approval for this Build.
    
    pub result: Option<ApprovalResult>,
    /// Output only. The state of this build's approval.
    
    pub state: Option<BuildApprovalStateEnum>,
}

impl client::Part for BuildApproval {}


/// Optional arguments to enable specific features of builds.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildOptions {
    /// Requested disk size for the VM that runs the build. Note that this is *NOT* "disk free"; some of the space will be used by the operating system and build utilities. Also note that this is the minimum disk size that will be allocated for the build -- the build may run with a larger disk than requested. At present, the maximum disk size is 2000GB; builds that request more than the maximum are rejected with an error.
    #[serde(rename="diskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_size_gb: Option<i64>,
    /// Option to specify whether or not to apply bash style string operations to the substitutions. NOTE: this is always enabled for triggered builds and cannot be overridden in the build configuration file.
    #[serde(rename="dynamicSubstitutions")]
    
    pub dynamic_substitutions: Option<bool>,
    /// A list of global environment variable definitions that will exist for all build steps in this build. If a variable is defined in both globally and in a build step, the variable will use the build step value. The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    
    pub env: Option<Vec<String>>,
    /// Option to define build log streaming behavior to Google Cloud Storage.
    #[serde(rename="logStreamingOption")]
    
    pub log_streaming_option: Option<BuildOptionLogStreamingOptionEnum>,
    /// Option to specify the logging mode, which determines if and where build logs are stored.
    
    pub logging: Option<BuildOptionLoggingEnum>,
    /// Compute Engine machine type on which to run the build.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<BuildOptionMachineTypeEnum>,
    /// Optional. Specification for execution on a `WorkerPool`. See [running builds in a private pool](https://cloud.google.com/build/docs/private-pools/run-builds-in-private-pool) for more information.
    
    pub pool: Option<PoolOption>,
    /// Requested verifiability options.
    #[serde(rename="requestedVerifyOption")]
    
    pub requested_verify_option: Option<BuildOptionRequestedVerifyOptionEnum>,
    /// A list of global environment variables, which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`. These variables will be available to all build steps in this build.
    #[serde(rename="secretEnv")]
    
    pub secret_env: Option<Vec<String>>,
    /// Requested hash for SourceProvenance.
    #[serde(rename="sourceProvenanceHash")]
    
    pub source_provenance_hash: Option<Vec<BuildOptionSourceProvenanceHashEnum>>,
    /// Option to specify behavior when there is an error in the substitution checks. NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot be overridden in the build configuration file.
    #[serde(rename="substitutionOption")]
    
    pub substitution_option: Option<BuildOptionSubstitutionOptionEnum>,
    /// Global list of volumes to mount for ALL build steps Each volume is created as an empty volume prior to starting the build process. Upon completion of the build, volumes and their contents are discarded. Global volume names and paths cannot conflict with the volumes defined a build step. Using a global volume in a build with only one step is not valid as it is indicative of a build request with an incorrect configuration.
    
    pub volumes: Option<Vec<Volume>>,
    /// This field deprecated; please use `pool.name` instead.
    #[serde(rename="workerPool")]
    
    pub worker_pool: Option<String>,
}

impl client::Part for BuildOptions {}


/// A step in the build pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildStep {
    /// Allow this build step to fail without failing the entire build if and only if the exit code is one of the specified codes. If allow_failure is also specified, this field will take precedence.
    #[serde(rename="allowExitCodes")]
    
    pub allow_exit_codes: Option<Vec<i32>>,
    /// Allow this build step to fail without failing the entire build. If false, the entire build will fail if this step fails. Otherwise, the build will succeed, but this step will still have a failure status. Error information will be reported in the failure_detail field.
    #[serde(rename="allowFailure")]
    
    pub allow_failure: Option<bool>,
    /// A list of arguments that will be presented to the step when it is started. If the image used to run the step's container has an entrypoint, the `args` are used as arguments to that entrypoint. If the image does not define an entrypoint, the first element in args is used as the entrypoint, and the remainder will be used as arguments.
    
    pub args: Option<Vec<String>>,
    /// Working directory to use when running this step's container. If this value is a relative path, it is relative to the build's working directory. If this value is absolute, it may be outside the build's working directory, in which case the contents of the path may not be persisted across build step executions, unless a `volume` for that path is specified. If the build specifies a `RepoSource` with `dir` and a step with a `dir`, which specifies an absolute path, the `RepoSource` `dir` is ignored for the step's execution.
    
    pub dir: Option<String>,
    /// Entrypoint to be used instead of the build step image's default entrypoint. If unset, the image's default entrypoint is used.
    
    pub entrypoint: Option<String>,
    /// A list of environment variable definitions to be used when running a step. The elements are of the form "KEY=VALUE" for the environment variable "KEY" being given the value "VALUE".
    
    pub env: Option<Vec<String>>,
    /// Output only. Return code from running the step.
    #[serde(rename="exitCode")]
    
    pub exit_code: Option<i32>,
    /// Unique identifier for this build step, used in `wait_for` to reference this build step as a dependency.
    
    pub id: Option<String>,
    /// Required. The name of the container image that will run this particular build step. If the image is available in the host's Docker daemon's cache, it will be run directly. If not, the host will attempt to pull the image first, using the builder service account's credentials if necessary. The Docker daemon's cache will already have the latest versions of all of the officially supported build steps ([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)). The Docker daemon will also have cached many of the layers for some popular images, like "ubuntu", "debian", but they will be refreshed at the time you attempt to use them. If you built an image in a previous build step, it will be stored in the host's Docker daemon's cache and is available to use as the name for a later build step.
    
    pub name: Option<String>,
    /// Output only. Stores timing information for pulling this build step's builder image only.
    #[serde(rename="pullTiming")]
    
    pub pull_timing: Option<TimeSpan>,
    /// A shell script to be executed in the step. When script is provided, the user cannot specify the entrypoint or args.
    
    pub script: Option<String>,
    /// A list of environment variables which are encrypted using a Cloud Key Management Service crypto key. These values must be specified in the build's `Secret`.
    #[serde(rename="secretEnv")]
    
    pub secret_env: Option<Vec<String>>,
    /// Output only. Status of the build step. At this time, build step status is only updated on build completion; step status is not updated in real-time as the build progresses.
    
    pub status: Option<BuildStepStatusEnum>,
    /// Time limit for executing this build step. If not defined, the step has no time limit and will be allowed to continue to run until either it completes or the build itself times out.
    
    #[serde_as(as = "Option<::client::serde::duration::Wrapper>")]
    pub timeout: Option<client::chrono::Duration>,
    /// Output only. Stores timing information for executing this build step.
    
    pub timing: Option<TimeSpan>,
    /// List of volumes to mount into the build step. Each volume is created as an empty volume prior to execution of the build step. Upon completion of the build, volumes and their contents are discarded. Using a named volume in only one step is not valid as it is indicative of a build request with an incorrect configuration.
    
    pub volumes: Option<Vec<Volume>>,
    /// The ID(s) of the step(s) that this build step depends on. This build step will not start until all the build steps in `wait_for` have completed successfully. If `wait_for` is empty, this build step will start when all previous build steps in the `Build.Steps` list have completed successfully.
    #[serde(rename="waitFor")]
    
    pub wait_for: Option<Vec<String>>,
}

impl client::Part for BuildStep {}


/// Configuration for an automated build in response to source repository changes.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations triggers create projects](ProjectLocationTriggerCreateCall) (request|response)
/// * [locations triggers get projects](ProjectLocationTriggerGetCall) (response)
/// * [locations triggers patch projects](ProjectLocationTriggerPatchCall) (request|response)
/// * [triggers create projects](ProjectTriggerCreateCall) (request|response)
/// * [triggers get projects](ProjectTriggerGetCall) (response)
/// * [triggers patch projects](ProjectTriggerPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuildTrigger {
    /// Configuration for manual approval to start a build invocation of this BuildTrigger.
    #[serde(rename="approvalConfig")]
    
    pub approval_config: Option<ApprovalConfig>,
    /// Autodetect build configuration. The following precedence is used (case insensitive): 1. cloudbuild.yaml 2. cloudbuild.yml 3. cloudbuild.json 4. Dockerfile Currently only available for GitHub App Triggers.
    
    pub autodetect: Option<bool>,
    /// BitbucketServerTriggerConfig describes the configuration of a trigger that creates a build whenever a Bitbucket Server event is received.
    #[serde(rename="bitbucketServerTriggerConfig")]
    
    pub bitbucket_server_trigger_config: Option<BitbucketServerTriggerConfig>,
    /// Contents of the build template.
    
    pub build: Option<Build>,
    /// Output only. Time when the trigger was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Human-readable description of this trigger.
    
    pub description: Option<String>,
    /// If true, the trigger will never automatically execute a build.
    
    pub disabled: Option<bool>,
    /// EventType allows the user to explicitly set the type of event to which this BuildTrigger should respond. This field will be validated against the rest of the configuration if it is set.
    #[serde(rename="eventType")]
    
    pub event_type: Option<BuildTriggerEventTypeEnum>,
    /// Path, from the source root, to the build configuration file (i.e. cloudbuild.yaml).
    
    pub filename: Option<String>,
    /// A Common Expression Language string.
    
    pub filter: Option<String>,
    /// The file source describing the local or remote Build template.
    #[serde(rename="gitFileSource")]
    
    pub git_file_source: Option<GitFileSource>,
    /// GitHubEventsConfig describes the configuration of a trigger that creates a build whenever a GitHub event is received. Mutually exclusive with `trigger_template`.
    
    pub github: Option<GitHubEventsConfig>,
    /// GitLabEnterpriseEventsConfig describes the configuration of a trigger that creates a build whenever a GitLab Enterprise event is received.
    #[serde(rename="gitlabEnterpriseEventsConfig")]
    
    pub gitlab_enterprise_events_config: Option<GitLabEventsConfig>,
    /// Output only. Unique identifier of the trigger.
    
    pub id: Option<String>,
    /// ignored_files and included_files are file glob matches using https://golang.org/pkg/path/filepath/#Match extended with support for "**". If ignored_files and changed files are both empty, then they are not used to determine whether or not to trigger a build. If ignored_files is not empty, then we ignore any files that match any of the ignored_file globs. If the change has no files that are outside of the ignored_files globs, then we do not trigger a build.
    #[serde(rename="ignoredFiles")]
    
    pub ignored_files: Option<Vec<String>>,
    /// If set to INCLUDE_BUILD_LOGS_WITH_STATUS, log url will be shown on GitHub page when build status is final. Setting this field to INCLUDE_BUILD_LOGS_WITH_STATUS for non GitHub triggers results in INVALID_ARGUMENT error.
    #[serde(rename="includeBuildLogs")]
    
    pub include_build_logs: Option<BuildTriggerIncludeBuildLogsEnum>,
    /// If any of the files altered in the commit pass the ignored_files filter and included_files is empty, then as far as this filter is concerned, we should trigger the build. If any of the files altered in the commit pass the ignored_files filter and included_files is not empty, then we make sure that at least one of those files matches a included_files glob. If not, then we do not trigger a build.
    #[serde(rename="includedFiles")]
    
    pub included_files: Option<Vec<String>>,
    /// User-assigned name of the trigger. Must be unique within the project. Trigger names must meet the following requirements: + They must contain only alphanumeric characters and dashes. + They can be 1-64 characters long. + They must begin and end with an alphanumeric character.
    
    pub name: Option<String>,
    /// PubsubConfig describes the configuration of a trigger that creates a build whenever a Pub/Sub message is published.
    #[serde(rename="pubsubConfig")]
    
    pub pubsub_config: Option<PubsubConfig>,
    /// The configuration of a trigger that creates a build whenever an event from Repo API is received.
    #[serde(rename="repositoryEventConfig")]
    
    pub repository_event_config: Option<RepositoryEventConfig>,
    /// The `Trigger` name with format: `projects/{project}/locations/{location}/triggers/{trigger}`, where {trigger} is a unique identifier generated by the service.
    #[serde(rename="resourceName")]
    
    pub resource_name: Option<String>,
    /// The service account used for all user-controlled operations including UpdateBuildTrigger, RunBuildTrigger, CreateBuild, and CancelBuild. If no service account is set, then the standard Cloud Build service account ([PROJECT_NUM]@system.gserviceaccount.com) will be used instead. Format: `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_ID_OR_EMAIL}`
    #[serde(rename="serviceAccount")]
    
    pub service_account: Option<String>,
    /// The repo and ref of the repository from which to build. This field is used only for those triggers that do not respond to SCM events. Triggers that respond to such events build source at whatever commit caused the event. This field is currently only used by Webhook, Pub/Sub, Manual, and Cron triggers.
    #[serde(rename="sourceToBuild")]
    
    pub source_to_build: Option<GitRepoSource>,
    /// Substitutions for Build resource. The keys must match the following regular expression: `^_[A-Z0-9_]+$`.
    
    pub substitutions: Option<HashMap<String, String>>,
    /// Tags for annotation of a `BuildTrigger`
    
    pub tags: Option<Vec<String>>,
    /// Template describing the types of source changes to trigger a build. Branch and tag names in trigger templates are interpreted as regular expressions. Any branch or tag change that matches that regular expression will trigger a build. Mutually exclusive with `github`.
    #[serde(rename="triggerTemplate")]
    
    pub trigger_template: Option<RepoSource>,
    /// WebhookConfig describes the configuration of a trigger that creates a build whenever a webhook is sent to a trigger's webhook URL.
    #[serde(rename="webhookConfig")]
    
    pub webhook_config: Option<WebhookConfig>,
}

impl client::RequestValue for BuildTrigger {}
impl client::ResponseResult for BuildTrigger {}


/// An image built by the pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BuiltImage {
    /// Docker Registry 2.0 digest.
    
    pub digest: Option<String>,
    /// Name used to push the container image to Google Container Registry, as presented to `docker push`.
    
    pub name: Option<String>,
    /// Output only. Stores timing information for pushing the specified image.
    #[serde(rename="pushTiming")]
    
    pub push_timing: Option<TimeSpan>,
}

impl client::Part for BuiltImage {}


/// Request to cancel an ongoing build.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [builds cancel projects](ProjectBuildCancelCall) (request)
/// * [locations builds cancel projects](ProjectLocationBuildCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelBuildRequest {
    /// Required. ID of the build.
    
    pub id: Option<String>,
    /// The name of the `Build` to cancel. Format: `projects/{project}/locations/{location}/builds/{build}`
    
    pub name: Option<String>,
    /// Required. ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::RequestValue for CancelBuildRequest {}


/// The request message for Operations.CancelOperation.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (request)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CancelOperationRequest { _never_set: Option<bool> }

impl client::RequestValue for CancelOperationRequest {}


/// Request to connect a repository from a connected Bitbucket Server host.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateBitbucketServerConnectedRepositoryRequest {
    /// Required. The Bitbucket Server repository to connect.
    #[serde(rename="bitbucketServerConnectedRepository")]
    
    pub bitbucket_server_connected_repository: Option<BitbucketServerConnectedRepository>,
    /// Required. The name of the `BitbucketServerConfig` that added connected repository. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{config}`
    
    pub parent: Option<String>,
}

impl client::Part for CreateBitbucketServerConnectedRepositoryRequest {}


/// Request to connect a repository from a connected GitLab host.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CreateGitLabConnectedRepositoryRequest {
    /// Required. The GitLab repository to connect.
    #[serde(rename="gitlabConnectedRepository")]
    
    pub gitlab_connected_repository: Option<GitLabConnectedRepository>,
    /// Required. The name of the `GitLabConfig` that adds connected repository. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`
    
    pub parent: Option<String>,
}

impl client::Part for CreateGitLabConnectedRepositoryRequest {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [receive github dot com webhook](GithubDotComWebhookReceiveCall) (response)
/// * [regional webhook locations](LocationRegionalWebhookCall) (response)
/// * [cancel operations](OperationCancelCall) (response)
/// * [locations bitbucket server configs remove bitbucket server connected repository projects](ProjectLocationBitbucketServerConfigRemoveBitbucketServerConnectedRepositoryCall) (response)
/// * [locations git lab configs remove git lab connected repository projects](ProjectLocationGitLabConfigRemoveGitLabConnectedRepositoryCall) (response)
/// * [locations operations cancel projects](ProjectLocationOperationCancelCall) (response)
/// * [locations triggers delete projects](ProjectLocationTriggerDeleteCall) (response)
/// * [triggers delete projects](ProjectTriggerDeleteCall) (response)
/// * [webhook](MethodWebhookCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// A fatal problem encountered during the execution of the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FailureInfo {
    /// Explains the failure issue in more detail using hard-coded text.
    
    pub detail: Option<String>,
    /// The name of the failure.
    #[serde(rename="type")]
    
    pub type_: Option<FailureInfoTypeEnum>,
}

impl client::Part for FailureInfo {}


/// Container message for hashes of byte content of files, used in SourceProvenance messages to verify integrity of source input to the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FileHashes {
    /// Collection of file hashes.
    #[serde(rename="fileHash")]
    
    pub file_hash: Option<Vec<Hash>>,
}

impl client::Part for FileHashes {}


/// GitFileSource describes a file within a (possibly remote) code repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitFileSource {
    /// The full resource name of the bitbucket server config. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{id}`.
    #[serde(rename="bitbucketServerConfig")]
    
    pub bitbucket_server_config: Option<String>,
    /// The full resource name of the github enterprise config. Format: `projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}`. `projects/{project}/githubEnterpriseConfigs/{id}`.
    #[serde(rename="githubEnterpriseConfig")]
    
    pub github_enterprise_config: Option<String>,
    /// The path of the file, with the repo root as the root of the path.
    
    pub path: Option<String>,
    /// See RepoType above.
    #[serde(rename="repoType")]
    
    pub repo_type: Option<GitFileSourceRepoTypeEnum>,
    /// The branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the filename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions If unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path.
    
    pub revision: Option<String>,
    /// The URI of the repo. Either uri or repository can be specified. If unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path.
    
    pub uri: Option<String>,
}

impl client::Part for GitFileSource {}


/// GitHubEnterpriseConfig represents a configuration for a GitHub Enterprise server.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [github enterprise configs create projects](ProjectGithubEnterpriseConfigCreateCall) (request)
/// * [github enterprise configs get projects](ProjectGithubEnterpriseConfigGetCall) (response)
/// * [github enterprise configs patch projects](ProjectGithubEnterpriseConfigPatchCall) (request)
/// * [locations github enterprise configs create projects](ProjectLocationGithubEnterpriseConfigCreateCall) (request)
/// * [locations github enterprise configs get projects](ProjectLocationGithubEnterpriseConfigGetCall) (response)
/// * [locations github enterprise configs patch projects](ProjectLocationGithubEnterpriseConfigPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitHubEnterpriseConfig {
    /// Required. The GitHub app id of the Cloud Build app on the GitHub Enterprise server.
    #[serde(rename="appId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub app_id: Option<i64>,
    /// Output only. Time when the installation was associated with the project.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Name to display for this config.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The URL of the github enterprise host the configuration is for.
    #[serde(rename="hostUrl")]
    
    pub host_url: Option<String>,
    /// Optional. The full resource name for the GitHubEnterpriseConfig For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    
    pub name: Option<String>,
    /// Optional. The network to be used when reaching out to the GitHub Enterprise server. The VPC network must be enabled for private service connection. This should be set if the GitHub Enterprise server is hosted on-premises and not reachable by public internet. If this field is left empty, no network peering will occur and calls to the GitHub Enterprise server will be made over the public internet. Must be in the format `projects/{project}/global/networks/{network}`, where {project} is a project number or id and {network} is the name of a VPC network in the project.
    #[serde(rename="peeredNetwork")]
    
    pub peered_network: Option<String>,
    /// Names of secrets in Secret Manager.
    
    pub secrets: Option<GitHubEnterpriseSecrets>,
    /// Optional. SSL certificate to use for requests to GitHub Enterprise.
    #[serde(rename="sslCa")]
    
    pub ssl_ca: Option<String>,
    /// The key that should be attached to webhook calls to the ReceiveWebhook endpoint.
    #[serde(rename="webhookKey")]
    
    pub webhook_key: Option<String>,
}

impl client::RequestValue for GitHubEnterpriseConfig {}
impl client::ResponseResult for GitHubEnterpriseConfig {}


/// GitHubEnterpriseSecrets represents the names of all necessary secrets in Secret Manager for a GitHub Enterprise server. Format is: projects//secrets/.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitHubEnterpriseSecrets {
    /// The resource name for the OAuth client ID secret in Secret Manager.
    #[serde(rename="oauthClientIdName")]
    
    pub oauth_client_id_name: Option<String>,
    /// The resource name for the OAuth client ID secret version in Secret Manager.
    #[serde(rename="oauthClientIdVersionName")]
    
    pub oauth_client_id_version_name: Option<String>,
    /// The resource name for the OAuth secret in Secret Manager.
    #[serde(rename="oauthSecretName")]
    
    pub oauth_secret_name: Option<String>,
    /// The resource name for the OAuth secret secret version in Secret Manager.
    #[serde(rename="oauthSecretVersionName")]
    
    pub oauth_secret_version_name: Option<String>,
    /// The resource name for the private key secret.
    #[serde(rename="privateKeyName")]
    
    pub private_key_name: Option<String>,
    /// The resource name for the private key secret version.
    #[serde(rename="privateKeyVersionName")]
    
    pub private_key_version_name: Option<String>,
    /// The resource name for the webhook secret in Secret Manager.
    #[serde(rename="webhookSecretName")]
    
    pub webhook_secret_name: Option<String>,
    /// The resource name for the webhook secret secret version in Secret Manager.
    #[serde(rename="webhookSecretVersionName")]
    
    pub webhook_secret_version_name: Option<String>,
}

impl client::Part for GitHubEnterpriseSecrets {}


/// GitHubEventsConfig describes the configuration of a trigger that creates a build whenever a GitHub event is received.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitHubEventsConfig {
    /// Optional. The resource name of the github enterprise config that should be applied to this installation. For example: "projects/{$project_id}/locations/{$location_id}/githubEnterpriseConfigs/{$config_id}"
    #[serde(rename="enterpriseConfigResourceName")]
    
    pub enterprise_config_resource_name: Option<String>,
    /// The installationID that emits the GitHub event.
    #[serde(rename="installationId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub installation_id: Option<i64>,
    /// Name of the repository. For example: The name for https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    
    pub name: Option<String>,
    /// Owner of the repository. For example: The owner for https://github.com/googlecloudplatform/cloud-builders is "googlecloudplatform".
    
    pub owner: Option<String>,
    /// filter to match changes in pull requests.
    #[serde(rename="pullRequest")]
    
    pub pull_request: Option<PullRequestFilter>,
    /// filter to match changes in refs like branches, tags.
    
    pub push: Option<PushFilter>,
}

impl client::Part for GitHubEventsConfig {}


/// GitLabConfig represents the configuration for a GitLab integration.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations git lab configs create projects](ProjectLocationGitLabConfigCreateCall) (request)
/// * [locations git lab configs get projects](ProjectLocationGitLabConfigGetCall) (response)
/// * [locations git lab configs patch projects](ProjectLocationGitLabConfigPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabConfig {
    /// Connected GitLab.com or GitLabEnterprise repositories for this config.
    #[serde(rename="connectedRepositories")]
    
    pub connected_repositories: Option<Vec<GitLabRepositoryId>>,
    /// Output only. Time when the config was created.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Optional. GitLabEnterprise config.
    #[serde(rename="enterpriseConfig")]
    
    pub enterprise_config: Option<GitLabEnterpriseConfig>,
    /// The resource name for the config.
    
    pub name: Option<String>,
    /// Required. Secret Manager secrets needed by the config.
    
    pub secrets: Option<GitLabSecrets>,
    /// Username of the GitLab.com or GitLab Enterprise account Cloud Build will use.
    
    pub username: Option<String>,
    /// Output only. UUID included in webhook requests. The UUID is used to look up the corresponding config.
    #[serde(rename="webhookKey")]
    
    pub webhook_key: Option<String>,
}

impl client::RequestValue for GitLabConfig {}
impl client::ResponseResult for GitLabConfig {}


/// GitLabConnectedRepository represents a GitLab connected repository request response.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabConnectedRepository {
    /// The name of the `GitLabConfig` that added connected repository. Format: `projects/{project}/locations/{location}/gitLabConfigs/{config}`
    
    pub parent: Option<String>,
    /// The GitLab repositories to connect.
    
    pub repo: Option<GitLabRepositoryId>,
    /// Output only. The status of the repo connection request.
    
    pub status: Option<Status>,
}

impl client::Part for GitLabConnectedRepository {}


/// GitLabEnterpriseConfig represents the configuration for a GitLabEnterprise integration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabEnterpriseConfig {
    /// Immutable. The URI of the GitlabEnterprise host.
    #[serde(rename="hostUri")]
    
    pub host_uri: Option<String>,
    /// The Service Directory configuration to be used when reaching out to the GitLab Enterprise instance.
    #[serde(rename="serviceDirectoryConfig")]
    
    pub service_directory_config: Option<ServiceDirectoryConfig>,
    /// The SSL certificate to use in requests to GitLab Enterprise instances.
    #[serde(rename="sslCa")]
    
    pub ssl_ca: Option<String>,
}

impl client::Part for GitLabEnterpriseConfig {}


/// GitLabEventsConfig describes the configuration of a trigger that creates a build whenever a GitLab event is received.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabEventsConfig {
    /// Output only. The GitLabConfig specified in the gitlab_config_resource field.
    #[serde(rename="gitlabConfig")]
    
    pub gitlab_config: Option<GitLabConfig>,
    /// The GitLab config resource that this trigger config maps to.
    #[serde(rename="gitlabConfigResource")]
    
    pub gitlab_config_resource: Option<String>,
    /// Namespace of the GitLab project.
    #[serde(rename="projectNamespace")]
    
    pub project_namespace: Option<String>,
    /// Filter to match changes in pull requests.
    #[serde(rename="pullRequest")]
    
    pub pull_request: Option<PullRequestFilter>,
    /// Filter to match changes in refs like branches, tags.
    
    pub push: Option<PushFilter>,
}

impl client::Part for GitLabEventsConfig {}


/// Proto Representing a GitLabRepository
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabRepository {
    /// Link to the browse repo page on the GitLab instance
    #[serde(rename="browseUri")]
    
    pub browse_uri: Option<String>,
    /// Description of the repository
    
    pub description: Option<String>,
    /// Display name of the repository
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the repository
    
    pub name: Option<String>,
    /// Identifier for a repository
    #[serde(rename="repositoryId")]
    
    pub repository_id: Option<GitLabRepositoryId>,
}

impl client::Part for GitLabRepository {}


/// GitLabRepositoryId identifies a specific repository hosted on GitLab.com or GitLabEnterprise
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabRepositoryId {
    /// Required. Identifier for the repository. example: "namespace/project-slug", namespace is usually the username or group ID
    
    pub id: Option<String>,
    /// Output only. The ID of the webhook that was created for receiving events from this repo. We only create and manage a single webhook for each repo.
    #[serde(rename="webhookId")]
    
    pub webhook_id: Option<i32>,
}

impl client::Part for GitLabRepositoryId {}


/// GitLabSecrets represents the secrets in Secret Manager for a GitLab integration.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitLabSecrets {
    /// Required. The resource name for the api access token’s secret version
    #[serde(rename="apiAccessTokenVersion")]
    
    pub api_access_token_version: Option<String>,
    /// Required. Immutable. API Key that will be attached to webhook requests from GitLab to Cloud Build.
    #[serde(rename="apiKeyVersion")]
    
    pub api_key_version: Option<String>,
    /// Required. The resource name for the read access token’s secret version
    #[serde(rename="readAccessTokenVersion")]
    
    pub read_access_token_version: Option<String>,
    /// Required. Immutable. The resource name for the webhook secret’s secret version. Once this field has been set, it cannot be changed. If you need to change it, please create another GitLabConfig.
    #[serde(rename="webhookSecretVersion")]
    
    pub webhook_secret_version: Option<String>,
}

impl client::Part for GitLabSecrets {}


/// GitRepoSource describes a repo and ref of a code repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitRepoSource {
    /// The full resource name of the bitbucket server config. Format: `projects/{project}/locations/{location}/bitbucketServerConfigs/{id}`.
    #[serde(rename="bitbucketServerConfig")]
    
    pub bitbucket_server_config: Option<String>,
    /// The full resource name of the github enterprise config. Format: `projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}`. `projects/{project}/githubEnterpriseConfigs/{id}`.
    #[serde(rename="githubEnterpriseConfig")]
    
    pub github_enterprise_config: Option<String>,
    /// The branch or tag to use. Must start with "refs/" (required).
    #[serde(rename="ref")]
    
    pub ref_: Option<String>,
    /// See RepoType below.
    #[serde(rename="repoType")]
    
    pub repo_type: Option<GitRepoSourceRepoTypeEnum>,
    /// The URI of the repo. Either uri or repository can be specified and is required.
    
    pub uri: Option<String>,
}

impl client::Part for GitRepoSource {}


/// Container message for hash values.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Hash {
    /// The type of hash that was performed.
    #[serde(rename="type")]
    
    pub type_: Option<HashTypeEnum>,
    /// The hash value.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub value: Option<Vec<u8>>,
}

impl client::Part for Hash {}


/// Message that represents an arbitrary HTTP body. It should only be used for payload formats that can’t be represented as JSON, such as raw binary or an HTML page. This message can be used both in streaming and non-streaming API methods in the request as well as the response. It can be used as a top-level request field, which is convenient if one wants to extract parameters from either the URL or HTTP template into the request fields and also want access to the raw HTTP body. Example: message GetResourceRequest { // A unique request id. string request_id = 1; // The raw HTTP body is bound to this field. google.api.HttpBody http_body = 2; } service ResourceService { rpc GetResource(GetResourceRequest) returns (google.api.HttpBody); rpc UpdateResource(google.api.HttpBody) returns (google.protobuf.Empty); } Example with streaming methods: service CaldavService { rpc GetCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); rpc UpdateCalendar(stream google.api.HttpBody) returns (stream google.api.HttpBody); } Use of this type only changes how the request and response bodies are handled, all other features will continue to work unchanged.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [receive github dot com webhook](GithubDotComWebhookReceiveCall) (request)
/// * [regional webhook locations](LocationRegionalWebhookCall) (request)
/// * [locations triggers webhook projects](ProjectLocationTriggerWebhookCall) (request)
/// * [triggers webhook projects](ProjectTriggerWebhookCall) (request)
/// * [webhook](MethodWebhookCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HttpBody {
    /// The HTTP Content-Type header value specifying the content type of the body.
    #[serde(rename="contentType")]
    
    pub content_type: Option<String>,
    /// The HTTP request/response body as raw binary.
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub data: Option<Vec<u8>>,
    /// Application specific response metadata. Must be set in the first response for streaming APIs.
    
    pub extensions: Option<Vec<HashMap<String, json::Value>>>,
}

impl client::RequestValue for HttpBody {}


/// Pairs a set of secret environment variables mapped to encrypted values with the Cloud KMS key to use to decrypt the value.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct InlineSecret {
    /// Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets.
    #[serde(rename="envMap")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde::urlsafe_base64::Wrapper>>")]
    pub env_map: Option<HashMap<String, Vec<u8>>>,
    /// Resource name of Cloud KMS crypto key to decrypt the encrypted value. In format: projects/*/locations/*/keyRings/*/cryptoKeys/*
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
}

impl client::Part for InlineSecret {}


/// RPC response object returned by ListBitbucketServerConfigs RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations bitbucket server configs list projects](ProjectLocationBitbucketServerConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBitbucketServerConfigsResponse {
    /// A list of BitbucketServerConfigs
    #[serde(rename="bitbucketServerConfigs")]
    
    pub bitbucket_server_configs: Option<Vec<BitbucketServerConfig>>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBitbucketServerConfigsResponse {}


/// RPC response object returned by the ListBitbucketServerRepositories RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations bitbucket server configs repos list projects](ProjectLocationBitbucketServerConfigRepoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBitbucketServerRepositoriesResponse {
    /// List of Bitbucket Server repositories.
    #[serde(rename="bitbucketServerRepositories")]
    
    pub bitbucket_server_repositories: Option<Vec<BitbucketServerRepository>>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBitbucketServerRepositoriesResponse {}


/// Response containing existing `BuildTriggers`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations triggers list projects](ProjectLocationTriggerListCall) (response)
/// * [triggers list projects](ProjectTriggerListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBuildTriggersResponse {
    /// Token to receive the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// `BuildTriggers` for the project, sorted by `create_time` descending.
    
    pub triggers: Option<Vec<BuildTrigger>>,
}

impl client::ResponseResult for ListBuildTriggersResponse {}


/// Response including listed builds.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [builds list projects](ProjectBuildListCall) (response)
/// * [locations builds list projects](ProjectLocationBuildListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBuildsResponse {
    /// Builds will be sorted by `create_time`, descending.
    
    pub builds: Option<Vec<Build>>,
    /// Token to receive the next page of results. This will be absent if the end of the response list has been reached.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListBuildsResponse {}


/// RPC response object returned by ListGitLabConfigs RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations git lab configs list projects](ProjectLocationGitLabConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGitLabConfigsResponse {
    /// A list of GitLabConfigs
    #[serde(rename="gitlabConfigs")]
    
    pub gitlab_configs: Option<Vec<GitLabConfig>>,
    /// A token that can be sent as `page_token` to retrieve the next page If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGitLabConfigsResponse {}


/// RPC response object returned by the ListGitLabRepositories RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations git lab configs repos list projects](ProjectLocationGitLabConfigRepoListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGitLabRepositoriesResponse {
    /// List of GitLab repositories
    #[serde(rename="gitlabRepositories")]
    
    pub gitlab_repositories: Option<Vec<GitLabRepository>>,
    /// A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListGitLabRepositoriesResponse {}


/// RPC response object returned by ListGithubEnterpriseConfigs RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [github enterprise configs list projects](ProjectGithubEnterpriseConfigListCall) (response)
/// * [locations github enterprise configs list projects](ProjectLocationGithubEnterpriseConfigListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListGithubEnterpriseConfigsResponse {
    /// A list of GitHubEnterpriseConfigs
    
    pub configs: Option<Vec<GitHubEnterpriseConfig>>,
}

impl client::ResponseResult for ListGithubEnterpriseConfigsResponse {}


/// Response containing existing `WorkerPools`.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations worker pools list projects](ProjectLocationWorkerPoolListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWorkerPoolsResponse {
    /// Continuation token used to page through large result sets. Provide this value in a subsequent ListWorkerPoolsRequest to return the next page of results.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// `WorkerPools` for the specified project.
    #[serde(rename="workerPools")]
    
    pub worker_pools: Option<Vec<WorkerPool>>,
}

impl client::ResponseResult for ListWorkerPoolsResponse {}


/// A Maven artifact to upload to Artifact Registry upon successful completion of all build steps.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MavenArtifact {
    /// Maven `artifactId` value used when uploading the artifact to Artifact Registry.
    #[serde(rename="artifactId")]
    
    pub artifact_id: Option<String>,
    /// Maven `groupId` value used when uploading the artifact to Artifact Registry.
    #[serde(rename="groupId")]
    
    pub group_id: Option<String>,
    /// Path to an artifact in the build's workspace to be uploaded to Artifact Registry. This can be either an absolute path, e.g. /workspace/my-app/target/my-app-1.0.SNAPSHOT.jar or a relative path from /workspace, e.g. my-app/target/my-app-1.0.SNAPSHOT.jar.
    
    pub path: Option<String>,
    /// Artifact Registry repository, in the form "https://$REGION-maven.pkg.dev/$PROJECT/$REPOSITORY" Artifact in the workspace specified by path will be uploaded to Artifact Registry with this location as a prefix.
    
    pub repository: Option<String>,
    /// Maven `version` value used when uploading the artifact to Artifact Registry.
    
    pub version: Option<String>,
}

impl client::Part for MavenArtifact {}


/// Defines the network configuration for the pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Option to configure network egress for the workers.
    #[serde(rename="egressOption")]
    
    pub egress_option: Option<NetworkConfigEgressOptionEnum>,
    /// Required. Immutable. The network definition that the workers are peered to. If this section is left empty, the workers will be peered to `WorkerPool.project_id` on the service producer network. Must be in the format `projects/{project}/global/networks/{network}`, where `{project}` is a project number, such as `12345`, and `{network}` is the name of a VPC network in the project. See [Understanding network configuration options](https://cloud.google.com/build/docs/private-pools/set-up-private-pool-environment)
    #[serde(rename="peeredNetwork")]
    
    pub peered_network: Option<String>,
    /// Immutable. Subnet IP range within the peered network. This is specified in CIDR notation with a slash and the subnet prefix size. You can optionally specify an IP address before the subnet prefix value. e.g. `192.168.0.0/29` would specify an IP range starting at 192.168.0.0 with a prefix size of 29 bits. `/16` would specify a prefix size of 16 bits, with an automatically determined IP within the peered VPC. If unspecified, a value of `/24` will be used.
    #[serde(rename="peeredNetworkIpRange")]
    
    pub peered_network_ip_range: Option<String>,
}

impl client::Part for NetworkConfig {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [cancel operations](OperationCancelCall) (none)
/// * [get operations](OperationGetCall) (response)
/// * [builds approve projects](ProjectBuildApproveCall) (response)
/// * [builds create projects](ProjectBuildCreateCall) (response)
/// * [builds retry projects](ProjectBuildRetryCall) (response)
/// * [github enterprise configs create projects](ProjectGithubEnterpriseConfigCreateCall) (response)
/// * [github enterprise configs delete projects](ProjectGithubEnterpriseConfigDeleteCall) (response)
/// * [github enterprise configs patch projects](ProjectGithubEnterpriseConfigPatchCall) (response)
/// * [locations bitbucket server configs connected repositories batch create projects](ProjectLocationBitbucketServerConfigConnectedRepositoryBatchCreateCall) (response)
/// * [locations bitbucket server configs create projects](ProjectLocationBitbucketServerConfigCreateCall) (response)
/// * [locations bitbucket server configs delete projects](ProjectLocationBitbucketServerConfigDeleteCall) (response)
/// * [locations bitbucket server configs patch projects](ProjectLocationBitbucketServerConfigPatchCall) (response)
/// * [locations builds approve projects](ProjectLocationBuildApproveCall) (response)
/// * [locations builds create projects](ProjectLocationBuildCreateCall) (response)
/// * [locations builds retry projects](ProjectLocationBuildRetryCall) (response)
/// * [locations git lab configs connected repositories batch create projects](ProjectLocationGitLabConfigConnectedRepositoryBatchCreateCall) (response)
/// * [locations git lab configs create projects](ProjectLocationGitLabConfigCreateCall) (response)
/// * [locations git lab configs delete projects](ProjectLocationGitLabConfigDeleteCall) (response)
/// * [locations git lab configs patch projects](ProjectLocationGitLabConfigPatchCall) (response)
/// * [locations github enterprise configs create projects](ProjectLocationGithubEnterpriseConfigCreateCall) (response)
/// * [locations github enterprise configs delete projects](ProjectLocationGithubEnterpriseConfigDeleteCall) (response)
/// * [locations github enterprise configs patch projects](ProjectLocationGithubEnterpriseConfigPatchCall) (response)
/// * [locations operations get projects](ProjectLocationOperationGetCall) (response)
/// * [locations triggers run projects](ProjectLocationTriggerRunCall) (response)
/// * [locations worker pools create projects](ProjectLocationWorkerPoolCreateCall) (response)
/// * [locations worker pools delete projects](ProjectLocationWorkerPoolDeleteCall) (response)
/// * [locations worker pools patch projects](ProjectLocationWorkerPoolPatchCall) (response)
/// * [triggers run projects](ProjectTriggerRunCall) (response)
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


/// Details about how a build should be executed on a `WorkerPool`. See [running builds in a private pool](https://cloud.google.com/build/docs/private-pools/run-builds-in-private-pool) for more information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PoolOption {
    /// The `WorkerPool` resource to execute the build on. You must have `cloudbuild.workerpools.use` on the project hosting the WorkerPool. Format projects/{project}/locations/{location}/workerPools/{workerPoolId}
    
    pub name: Option<String>,
}

impl client::Part for PoolOption {}


/// Configuration for a V1 `PrivatePool`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PrivatePoolV1Config {
    /// Network configuration for the pool.
    #[serde(rename="networkConfig")]
    
    pub network_config: Option<NetworkConfig>,
    /// Machine configuration for the workers in the pool.
    #[serde(rename="workerConfig")]
    
    pub worker_config: Option<WorkerConfig>,
}

impl client::Part for PrivatePoolV1Config {}


/// PubsubConfig describes the configuration of a trigger that creates a build whenever a Pub/Sub message is published.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PubsubConfig {
    /// Service account that will make the push request.
    #[serde(rename="serviceAccountEmail")]
    
    pub service_account_email: Option<String>,
    /// Potential issues with the underlying Pub/Sub subscription configuration. Only populated on get requests.
    
    pub state: Option<PubsubConfigStateEnum>,
    /// Output only. Name of the subscription. Format is `projects/{project}/subscriptions/{subscription}`.
    
    pub subscription: Option<String>,
    /// The name of the topic from which this subscription is receiving messages. Format is `projects/{project}/topics/{topic}`.
    
    pub topic: Option<String>,
}

impl client::Part for PubsubConfig {}


/// PullRequestFilter contains filter properties for matching GitHub Pull Requests.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestFilter {
    /// Regex of branches to match. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    
    pub branch: Option<String>,
    /// Configure builds to run whether a repository owner or collaborator need to comment `/gcbrun`.
    #[serde(rename="commentControl")]
    
    pub comment_control: Option<PullRequestFilterCommentControlEnum>,
    /// If true, branches that do NOT match the git_ref will trigger a build.
    #[serde(rename="invertRegex")]
    
    pub invert_regex: Option<bool>,
}

impl client::Part for PullRequestFilter {}


/// Push contains filter properties for matching GitHub git pushes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PushFilter {
    /// Regexes matching branches to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    
    pub branch: Option<String>,
    /// When true, only trigger a build if the revision regex does NOT match the git_ref regex.
    #[serde(rename="invertRegex")]
    
    pub invert_regex: Option<bool>,
    /// Regexes matching tags to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    
    pub tag: Option<String>,
}

impl client::Part for PushFilter {}


/// Python package to upload to Artifact Registry upon successful completion of all build steps. A package can encapsulate multiple objects to be uploaded to a single repository.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PythonPackage {
    /// Path globs used to match files in the build's workspace. For Python/ Twine, this is usually `dist/*`, and sometimes additionally an `.asc` file.
    
    pub paths: Option<Vec<String>>,
    /// Artifact Registry repository, in the form "https://$REGION-python.pkg.dev/$PROJECT/$REPOSITORY" Files in the workspace matching any path pattern will be uploaded to Artifact Registry with this location as a prefix.
    
    pub repository: Option<String>,
}

impl client::Part for PythonPackage {}


/// ReceiveTriggerWebhookResponse \[Experimental\] is the response object for the ReceiveTriggerWebhook method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations triggers webhook projects](ProjectLocationTriggerWebhookCall) (response)
/// * [triggers webhook projects](ProjectTriggerWebhookCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReceiveTriggerWebhookResponse { _never_set: Option<bool> }

impl client::ResponseResult for ReceiveTriggerWebhookResponse {}


/// RPC request object accepted by RemoveBitbucketServerConnectedRepository RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations bitbucket server configs remove bitbucket server connected repository projects](ProjectLocationBitbucketServerConfigRemoveBitbucketServerConnectedRepositoryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveBitbucketServerConnectedRepositoryRequest {
    /// The connected repository to remove.
    #[serde(rename="connectedRepository")]
    
    pub connected_repository: Option<BitbucketServerRepositoryId>,
}

impl client::RequestValue for RemoveBitbucketServerConnectedRepositoryRequest {}


/// RPC request object accepted by RemoveGitLabConnectedRepository RPC method.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations git lab configs remove git lab connected repository projects](ProjectLocationGitLabConfigRemoveGitLabConnectedRepositoryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveGitLabConnectedRepositoryRequest {
    /// The connected repository to remove.
    #[serde(rename="connectedRepository")]
    
    pub connected_repository: Option<GitLabRepositoryId>,
}

impl client::RequestValue for RemoveGitLabConnectedRepositoryRequest {}


/// Location of the source in a Google Cloud Source Repository.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [triggers run projects](ProjectTriggerRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepoSource {
    /// Regex matching branches to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    #[serde(rename="branchName")]
    
    pub branch_name: Option<String>,
    /// Explicit commit SHA to build.
    #[serde(rename="commitSha")]
    
    pub commit_sha: Option<String>,
    /// Directory, relative to the source root, in which to run the build. This must be a relative path. If a step's `dir` is specified and is an absolute path, this value is ignored for that step's execution.
    
    pub dir: Option<String>,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[serde(rename="invertRegex")]
    
    pub invert_regex: Option<bool>,
    /// ID of the project that owns the Cloud Source Repository. If omitted, the project ID requesting the build is assumed.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Name of the Cloud Source Repository.
    #[serde(rename="repoName")]
    
    pub repo_name: Option<String>,
    /// Substitutions to use in a triggered build. Should only be used with RunBuildTrigger
    
    pub substitutions: Option<HashMap<String, String>>,
    /// Regex matching tags to build. The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    #[serde(rename="tagName")]
    
    pub tag_name: Option<String>,
}

impl client::RequestValue for RepoSource {}


/// The configuration of a trigger that creates a build whenever an event from Repo API is received.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepositoryEventConfig {
    /// Filter to match changes in pull requests.
    #[serde(rename="pullRequest")]
    
    pub pull_request: Option<PullRequestFilter>,
    /// Filter to match changes in refs like branches, tags.
    
    pub push: Option<PushFilter>,
    /// The resource name of the Repo API resource.
    
    pub repository: Option<String>,
    /// Output only. The type of the SCM vendor the repository points to.
    #[serde(rename="repositoryType")]
    
    pub repository_type: Option<RepositoryEventConfigRepositoryTypeEnum>,
}

impl client::Part for RepositoryEventConfig {}


/// Artifacts created by the build pipeline.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Results {
    /// Path to the artifact manifest for non-container artifacts uploaded to Cloud Storage. Only populated when artifacts are uploaded to Cloud Storage.
    #[serde(rename="artifactManifest")]
    
    pub artifact_manifest: Option<String>,
    /// Time to push all non-container artifacts to Cloud Storage.
    #[serde(rename="artifactTiming")]
    
    pub artifact_timing: Option<TimeSpan>,
    /// List of build step digests, in the order corresponding to build step indices.
    #[serde(rename="buildStepImages")]
    
    pub build_step_images: Option<Vec<String>>,
    /// List of build step outputs, produced by builder images, in the order corresponding to build step indices. [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders) can produce this output by writing to `$BUILDER_OUTPUT/output`. Only the first 4KB of data is stored.
    #[serde(rename="buildStepOutputs")]
    
    #[serde_as(as = "Option<Vec<::client::serde::urlsafe_base64::Wrapper>>")]
    pub build_step_outputs: Option<Vec<Vec<u8>>>,
    /// Container images that were built as a part of the build.
    
    pub images: Option<Vec<BuiltImage>>,
    /// Maven artifacts uploaded to Artifact Registry at the end of the build.
    #[serde(rename="mavenArtifacts")]
    
    pub maven_artifacts: Option<Vec<UploadedMavenArtifact>>,
    /// Number of non-container artifacts uploaded to Cloud Storage. Only populated when artifacts are uploaded to Cloud Storage.
    #[serde(rename="numArtifacts")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub num_artifacts: Option<i64>,
    /// Python artifacts uploaded to Artifact Registry at the end of the build.
    #[serde(rename="pythonPackages")]
    
    pub python_packages: Option<Vec<UploadedPythonPackage>>,
}

impl client::Part for Results {}


/// Specifies a build to retry.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [builds retry projects](ProjectBuildRetryCall) (request)
/// * [locations builds retry projects](ProjectLocationBuildRetryCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RetryBuildRequest {
    /// Required. Build ID of the original build.
    
    pub id: Option<String>,
    /// The name of the `Build` to retry. Format: `projects/{project}/locations/{location}/builds/{build}`
    
    pub name: Option<String>,
    /// Required. ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
}

impl client::RequestValue for RetryBuildRequest {}


/// Specifies a build trigger to run and the source to use.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations triggers run projects](ProjectLocationTriggerRunCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunBuildTriggerRequest {
    /// Required. ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Source to build against this trigger. Branch and tag names cannot consist of regular expressions.
    
    pub source: Option<RepoSource>,
    /// Required. ID of the trigger.
    #[serde(rename="triggerId")]
    
    pub trigger_id: Option<String>,
}

impl client::RequestValue for RunBuildTriggerRequest {}


/// Pairs a set of secret environment variables containing encrypted values with the Cloud KMS key to use to decrypt the value. Note: Use `kmsKeyName` with `available_secrets` instead of using `kmsKeyName` with `secret`. For instructions see: https://cloud.google.com/cloud-build/docs/securing-builds/use-encrypted-credentials.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secret {
    /// Cloud KMS key name to use to decrypt these envs.
    #[serde(rename="kmsKeyName")]
    
    pub kms_key_name: Option<String>,
    /// Map of environment variable name to its encrypted value. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step. Values can be at most 64 KB in size. There can be at most 100 secret values across all of a build's secrets.
    #[serde(rename="secretEnv")]
    
    #[serde_as(as = "Option<HashMap<_, ::client::serde::urlsafe_base64::Wrapper>>")]
    pub secret_env: Option<HashMap<String, Vec<u8>>>,
}

impl client::Part for Secret {}


/// Pairs a secret environment variable with a SecretVersion in Secret Manager.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SecretManagerSecret {
    /// Environment variable name to associate with the secret. Secret environment variables must be unique across all of a build's secrets, and must be used by at least one build step.
    
    pub env: Option<String>,
    /// Resource name of the SecretVersion. In format: projects/*/secrets/*/versions/*
    #[serde(rename="versionName")]
    
    pub version_name: Option<String>,
}

impl client::Part for SecretManagerSecret {}


/// Secrets and secret environment variables.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Secrets {
    /// Secrets encrypted with KMS key and the associated secret environment variable.
    
    pub inline: Option<Vec<InlineSecret>>,
    /// Secrets in Secret Manager and associated secret environment variable.
    #[serde(rename="secretManager")]
    
    pub secret_manager: Option<Vec<SecretManagerSecret>>,
}

impl client::Part for Secrets {}


/// ServiceDirectoryConfig represents Service Directory configuration for a SCM host connection.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ServiceDirectoryConfig {
    /// The Service Directory service name. Format: projects/{project}/locations/{location}/namespaces/{namespace}/services/{service}.
    
    pub service: Option<String>,
}

impl client::Part for ServiceDirectoryConfig {}


/// Location of the source in a supported storage service.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Source {
    /// If provided, get the source from this location in a Cloud Source Repository.
    #[serde(rename="repoSource")]
    
    pub repo_source: Option<RepoSource>,
    /// If provided, get the source from this location in Google Cloud Storage.
    #[serde(rename="storageSource")]
    
    pub storage_source: Option<StorageSource>,
    /// If provided, get the source from this manifest in Google Cloud Storage. This feature is in Preview; see description [here](https://github.com/GoogleCloudPlatform/cloud-builders/tree/master/gcs-fetcher).
    #[serde(rename="storageSourceManifest")]
    
    pub storage_source_manifest: Option<StorageSourceManifest>,
}

impl client::Part for Source {}


/// Provenance of the source. Ways to find the original source, or verify that some source was used for this build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceProvenance {
    /// Output only. Hash(es) of the build source, which can be used to verify that the original source integrity was maintained in the build. Note that `FileHashes` will only be populated if `BuildOptions` has requested a `SourceProvenanceHash`. The keys to this map are file paths used as build source and the values contain the hash values for those files. If the build source came in a single package such as a gzipped tarfile (`.tar.gz`), the `FileHash` will be for the single path to that file.
    #[serde(rename="fileHashes")]
    
    pub file_hashes: Option<HashMap<String, FileHashes>>,
    /// A copy of the build's `source.repo_source`, if exists, with any revisions resolved.
    #[serde(rename="resolvedRepoSource")]
    
    pub resolved_repo_source: Option<RepoSource>,
    /// A copy of the build's `source.storage_source`, if exists, with any generations resolved.
    #[serde(rename="resolvedStorageSource")]
    
    pub resolved_storage_source: Option<StorageSource>,
    /// A copy of the build's `source.storage_source_manifest`, if exists, with any revisions resolved. This feature is in Preview.
    #[serde(rename="resolvedStorageSourceManifest")]
    
    pub resolved_storage_source_manifest: Option<StorageSourceManifest>,
}

impl client::Part for SourceProvenance {}


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


/// Location of the source in an archive file in Google Cloud Storage.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StorageSource {
    /// Google Cloud Storage bucket containing the source (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    
    pub bucket: Option<String>,
    /// Google Cloud Storage generation for the object. If the generation is omitted, the latest generation will be used.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// Google Cloud Storage object containing the source. This object must be a zipped (`.zip`) or gzipped archive file (`.tar.gz`) containing source to build.
    
    pub object: Option<String>,
}

impl client::Part for StorageSource {}


/// Location of the source manifest in Google Cloud Storage. This feature is in Preview; see description [here](https://github.com/GoogleCloudPlatform/cloud-builders/tree/master/gcs-fetcher).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StorageSourceManifest {
    /// Google Cloud Storage bucket containing the source manifest (see [Bucket Name Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    
    pub bucket: Option<String>,
    /// Google Cloud Storage generation for the object. If the generation is omitted, the latest generation will be used.
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub generation: Option<i64>,
    /// Google Cloud Storage object containing the source manifest. This object must be a JSON file.
    
    pub object: Option<String>,
}

impl client::Part for StorageSourceManifest {}


/// Start and end times for a build execution phase.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TimeSpan {
    /// End of time span.
    #[serde(rename="endTime")]
    
    pub end_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Start of time span.
    #[serde(rename="startTime")]
    
    pub start_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::Part for TimeSpan {}


/// A Maven artifact uploaded using the MavenArtifact directive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadedMavenArtifact {
    /// Hash types and values of the Maven Artifact.
    #[serde(rename="fileHashes")]
    
    pub file_hashes: Option<FileHashes>,
    /// Output only. Stores timing information for pushing the specified artifact.
    #[serde(rename="pushTiming")]
    
    pub push_timing: Option<TimeSpan>,
    /// URI of the uploaded artifact.
    
    pub uri: Option<String>,
}

impl client::Part for UploadedMavenArtifact {}


/// Artifact uploaded using the PythonPackage directive.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UploadedPythonPackage {
    /// Hash types and values of the Python Artifact.
    #[serde(rename="fileHashes")]
    
    pub file_hashes: Option<FileHashes>,
    /// Output only. Stores timing information for pushing the specified artifact.
    #[serde(rename="pushTiming")]
    
    pub push_timing: Option<TimeSpan>,
    /// URI of the uploaded artifact.
    
    pub uri: Option<String>,
}

impl client::Part for UploadedPythonPackage {}


/// Volume describes a Docker container volume which is mounted into build steps in order to persist files across build step execution.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Volume {
    /// Name of the volume to mount. Volume names must be unique per build step and must be valid names for Docker volumes. Each named volume must be used by at least two build steps.
    
    pub name: Option<String>,
    /// Path at which to mount the volume. Paths must be absolute and cannot conflict with other volume paths on the same build step or with certain reserved volume paths.
    
    pub path: Option<String>,
}

impl client::Part for Volume {}


/// A non-fatal problem encountered during the execution of the build.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Warning {
    /// The priority for this warning.
    
    pub priority: Option<WarningPriorityEnum>,
    /// Explanation of the warning generated.
    
    pub text: Option<String>,
}

impl client::Part for Warning {}


/// WebhookConfig describes the configuration of a trigger that creates a build whenever a webhook is sent to a trigger's webhook URL.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Required. Resource name for the secret required as a URL parameter.
    
    pub secret: Option<String>,
    /// Potential issues with the underlying Pub/Sub subscription configuration. Only populated on get requests.
    
    pub state: Option<WebhookConfigStateEnum>,
}

impl client::Part for WebhookConfig {}


/// Defines the configuration to be used for creating workers in the pool.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// Size of the disk attached to the worker, in GB. See [Worker pool config file](https://cloud.google.com/build/docs/private-pools/worker-pool-config-file-schema). Specify a value of up to 2000. If `0` is specified, Cloud Build will use a standard disk size.
    #[serde(rename="diskSizeGb")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub disk_size_gb: Option<i64>,
    /// Machine type of a worker, such as `e2-medium`. See [Worker pool config file](https://cloud.google.com/build/docs/private-pools/worker-pool-config-file-schema). If left blank, Cloud Build will use a sensible default.
    #[serde(rename="machineType")]
    
    pub machine_type: Option<String>,
}

impl client::Part for WorkerConfig {}


/// Configuration for a `WorkerPool`. Cloud Build owns and maintains a pool of workers for general use and have no access to a project’s private network. By default, builds submitted to Cloud Build will use a worker from this pool. If your build needs access to resources on a private network, create and use a `WorkerPool` to run your builds. Private `WorkerPool`s give your builds access to any single VPC network that you administer, including any on-prem resources connected to that VPC network. For an overview of private pools, see [Private pools overview](https://cloud.google.com/build/docs/private-pools/private-pools-overview).
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [locations worker pools create projects](ProjectLocationWorkerPoolCreateCall) (request)
/// * [locations worker pools get projects](ProjectLocationWorkerPoolGetCall) (response)
/// * [locations worker pools patch projects](ProjectLocationWorkerPoolPatchCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WorkerPool {
    /// User specified annotations. See https://google.aip.dev/128#annotations for more details such as format and size limitations.
    
    pub annotations: Option<HashMap<String, String>>,
    /// Output only. Time at which the request to create the `WorkerPool` was received.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Output only. Time at which the request to delete the `WorkerPool` was received.
    #[serde(rename="deleteTime")]
    
    pub delete_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// A user-specified, human-readable name for the `WorkerPool`. If provided, this value must be 1-63 characters.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// Output only. Checksum computed by the server. May be sent on update and delete requests to ensure that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Output only. The resource name of the `WorkerPool`, with format `projects/{project}/locations/{location}/workerPools/{worker_pool}`. The value of `{worker_pool}` is provided by `worker_pool_id` in `CreateWorkerPool` request and the value of `{location}` is determined by the endpoint accessed.
    
    pub name: Option<String>,
    /// Legacy Private Pool configuration.
    #[serde(rename="privatePoolV1Config")]
    
    pub private_pool_v1_config: Option<PrivatePoolV1Config>,
    /// Output only. `WorkerPool` state.
    
    pub state: Option<WorkerPoolStateEnum>,
    /// Output only. A unique identifier for the `WorkerPool`.
    
    pub uid: Option<String>,
    /// Output only. Time at which the request to update the `WorkerPool` was received.
    #[serde(rename="updateTime")]
    
    pub update_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
}

impl client::RequestValue for WorkerPool {}
impl client::ResponseResult for WorkerPool {}


