use super::*;
/// An alias to a repo revision.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AliasContext {
    /// The alias kind.
    
    pub kind: Option<AliasContextKindEnum>,
    /// The alias name.
    
    pub name: Option<String>,
}

impl client::Part for AliasContext {}


/// —————————————————————————— ## Breakpoint (the resource) Represents the breakpoint specification, status and results.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints set debugger](DebuggerDebuggeeBreakpointSetCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    /// Action that the agent should perform when the code at the breakpoint location is hit.
    
    pub action: Option<BreakpointActionEnum>,
    /// The deadline for the breakpoint to stay in CANARY_ACTIVE state. The value is meaningless when the breakpoint is not in CANARY_ACTIVE state.
    #[serde(rename="canaryExpireTime")]
    
    pub canary_expire_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Condition that triggers the breakpoint. The condition is a compound boolean expression composed using expressions in a programming language at the source location.
    
    pub condition: Option<String>,
    /// Time this breakpoint was created by the server in seconds resolution.
    #[serde(rename="createTime")]
    
    pub create_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Values of evaluated expressions at breakpoint time. The evaluated expressions appear in exactly the same order they are listed in the `expressions` field. The `name` field holds the original expression text, the `value` or `members` field holds the result of the evaluated expression. If the expression cannot be evaluated, the `status` inside the `Variable` will indicate an error and contain the error text.
    #[serde(rename="evaluatedExpressions")]
    
    pub evaluated_expressions: Option<Vec<Variable>>,
    /// List of read-only expressions to evaluate at the breakpoint location. The expressions are composed using expressions in the programming language at the source location. If the breakpoint action is `LOG`, the evaluated expressions are included in log statements.
    
    pub expressions: Option<Vec<String>>,
    /// Time this breakpoint was finalized as seen by the server in seconds resolution.
    #[serde(rename="finalTime")]
    
    pub final_time: Option<client::chrono::DateTime<client::chrono::offset::Utc>>,
    /// Breakpoint identifier, unique in the scope of the debuggee.
    
    pub id: Option<String>,
    /// When true, indicates that this is a final result and the breakpoint state will not change from here on.
    #[serde(rename="isFinalState")]
    
    pub is_final_state: Option<bool>,
    /// A set of custom breakpoint properties, populated by the agent, to be displayed to the user.
    
    pub labels: Option<HashMap<String, String>>,
    /// Breakpoint source location.
    
    pub location: Option<SourceLocation>,
    /// Indicates the severity of the log. Only relevant when action is `LOG`.
    #[serde(rename="logLevel")]
    
    pub log_level: Option<BreakpointLogLevelEnum>,
    /// Only relevant when action is `LOG`. Defines the message to log when the breakpoint hits. The message may include parameter placeholders `$0`, `$1`, etc. These placeholders are replaced with the evaluated value of the appropriate expression. Expressions not referenced in `log_message_format` are not logged. Example: `Message received, id = $0, count = $1` with `expressions` = `[ message.id, message.count ]`.
    #[serde(rename="logMessageFormat")]
    
    pub log_message_format: Option<String>,
    /// The stack at breakpoint time, where stack_frames[0] represents the most recently entered function.
    #[serde(rename="stackFrames")]
    
    pub stack_frames: Option<Vec<StackFrame>>,
    /// The current state of the breakpoint.
    
    pub state: Option<BreakpointStateEnum>,
    /// Breakpoint status. The status includes an error flag and a human readable message. This field is usually unset. The message can be either informational or an error message. Regardless, clients should always display the text message back to the user. Error status indicates complete failure of the breakpoint. Example (non-final state): `Still loading symbols...` Examples (final state): * `Invalid line number` referring to location * `Field f not found in class C` referring to condition
    
    pub status: Option<StatusMessage>,
    /// E-mail address of the user that created this breakpoint
    #[serde(rename="userEmail")]
    
    pub user_email: Option<String>,
    /// The `variable_table` exists to aid with computation, memory and network traffic optimization. It enables storing a variable once and reference it from multiple variables, including variables stored in the `variable_table` itself. For example, the same `this` object, which may appear at many levels of the stack, can have all of its data stored once in this table. The stack frame variables then would hold only a reference to it. The variable `var_table_index` field is an index into this repeated field. The stored objects are nameless and get their name from the referencing variable. The effective variable is a merge of the referencing variable and the referenced variable.
    #[serde(rename="variableTable")]
    
    pub variable_table: Option<Vec<Variable>>,
}

impl client::RequestValue for Breakpoint {}


/// A CloudRepoSourceContext denotes a particular revision in a cloud repo (a repo hosted by the Google Cloud Platform).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudRepoSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The name of an alias (branch, tag, etc.).
    #[serde(rename="aliasName")]
    
    pub alias_name: Option<String>,
    /// The ID of the repo.
    #[serde(rename="repoId")]
    
    pub repo_id: Option<RepoId>,
    /// A revision ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for CloudRepoSourceContext {}


/// A CloudWorkspaceId is a unique identifier for a cloud workspace. A cloud workspace is a place associated with a repo where modified files can be stored before they are committed.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudWorkspaceId {
    /// The unique name of the workspace within the repo. This is the name chosen by the client in the Source API's CreateWorkspace method.
    
    pub name: Option<String>,
    /// The ID of the repo containing the workspace.
    #[serde(rename="repoId")]
    
    pub repo_id: Option<RepoId>,
}

impl client::Part for CloudWorkspaceId {}


/// A CloudWorkspaceSourceContext denotes a workspace at a particular snapshot.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CloudWorkspaceSourceContext {
    /// The ID of the snapshot. An empty snapshot_id refers to the most recent snapshot.
    #[serde(rename="snapshotId")]
    
    pub snapshot_id: Option<String>,
    /// The ID of the workspace.
    #[serde(rename="workspaceId")]
    
    pub workspace_id: Option<CloudWorkspaceId>,
}

impl client::Part for CloudWorkspaceSourceContext {}


/// Represents the debugged application. The application may include one or more replicated processes executing the same code. Each of these processes is attached with a debugger agent, carrying out the debugging commands. Agents attached to the same debuggee identify themselves as such by using exactly the same Debuggee message value when registering.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Debuggee {
    /// Version ID of the agent. Schema: `domain/language-platform/vmajor.minor` (for example `google.com/java-gcp/v1.1`).
    #[serde(rename="agentVersion")]
    
    pub agent_version: Option<String>,
    /// Used when setting breakpoint canary for this debuggee.
    #[serde(rename="canaryMode")]
    
    pub canary_mode: Option<DebuggeeCanaryModeEnum>,
    /// Human readable description of the debuggee. Including a human-readable project name, environment name and version information is recommended.
    
    pub description: Option<String>,
    /// References to the locations and revisions of the source code used in the deployed application.
    #[serde(rename="extSourceContexts")]
    
    pub ext_source_contexts: Option<Vec<ExtendedSourceContext>>,
    /// Unique identifier for the debuggee generated by the controller service.
    
    pub id: Option<String>,
    /// If set to `true`, indicates that the agent should disable itself and detach from the debuggee.
    #[serde(rename="isDisabled")]
    
    pub is_disabled: Option<bool>,
    /// If set to `true`, indicates that Controller service does not detect any activity from the debuggee agents and the application is possibly stopped.
    #[serde(rename="isInactive")]
    
    pub is_inactive: Option<bool>,
    /// A set of custom debuggee properties, populated by the agent, to be displayed to the user.
    
    pub labels: Option<HashMap<String, String>>,
    /// Project the debuggee is associated with. Use project number or id when registering a Google Cloud Platform project.
    
    pub project: Option<String>,
    /// References to the locations and revisions of the source code used in the deployed application.
    #[serde(rename="sourceContexts")]
    
    pub source_contexts: Option<Vec<SourceContext>>,
    /// Human readable message to be displayed to the user about this debuggee. Absence of this field indicates no status. The message can be either informational or an error status.
    
    pub status: Option<StatusMessage>,
    /// Uniquifier to further distinguish the application. It is possible that different applications might have identical values in the debuggee message, thus, incorrectly identified as a single application by the Controller service. This field adds salt to further distinguish the application. Agents should consider seeding this field with value that identifies the code, binary, configuration and environment.
    
    pub uniquifier: Option<String>,
}

impl client::Part for Debuggee {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints delete debugger](DebuggerDebuggeeBreakpointDeleteCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// An ExtendedSourceContext is a SourceContext combined with additional details describing the context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ExtendedSourceContext {
    /// Any source context.
    
    pub context: Option<SourceContext>,
    /// Labels with user defined metadata.
    
    pub labels: Option<HashMap<String, String>>,
}

impl client::Part for ExtendedSourceContext {}


/// Represents a message with parameters.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FormatMessage {
    /// Format template for the message. The `format` uses placeholders `$0`, `$1`, etc. to reference parameters. `$$` can be used to denote the `$` character. Examples: * `Failed to load '$0' which helps debug $1 the first time it is loaded. Again, $0 is very important.` * `Please pay $$10 to use $0 instead of $1.`
    
    pub format: Option<String>,
    /// Optional parameters to be embedded into the message.
    
    pub parameters: Option<Vec<String>>,
}

impl client::Part for FormatMessage {}


/// A SourceContext referring to a Gerrit project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GerritSourceContext {
    /// An alias, which may be a branch or tag.
    #[serde(rename="aliasContext")]
    
    pub alias_context: Option<AliasContext>,
    /// The name of an alias (branch, tag, etc.).
    #[serde(rename="aliasName")]
    
    pub alias_name: Option<String>,
    /// The full project name within the host. Projects may be nested, so "project/subproject" is a valid project name. The "repo name" is hostURI/project.
    #[serde(rename="gerritProject")]
    
    pub gerrit_project: Option<String>,
    /// The URI of a running Gerrit instance.
    #[serde(rename="hostUri")]
    
    pub host_uri: Option<String>,
    /// A revision (commit) ID.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
}

impl client::Part for GerritSourceContext {}


/// Response for getting breakpoint information.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints get debugger](DebuggerDebuggeeBreakpointGetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GetBreakpointResponse {
    /// Complete breakpoint state. The fields `id` and `location` are guaranteed to be set.
    
    pub breakpoint: Option<Breakpoint>,
}

impl client::ResponseResult for GetBreakpointResponse {}


/// A GitSourceContext denotes a particular revision in a third party Git repository (e.g. GitHub).
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GitSourceContext {
    /// Git commit hash. required.
    #[serde(rename="revisionId")]
    
    pub revision_id: Option<String>,
    /// Git repository URL.
    
    pub url: Option<String>,
}

impl client::Part for GitSourceContext {}


/// Response for listing active breakpoints.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints list controller](ControllerDebuggeeBreakpointListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListActiveBreakpointsResponse {
    /// List of all active breakpoints. The fields `id` and `location` are guaranteed to be set on each breakpoint.
    
    pub breakpoints: Option<Vec<Breakpoint>>,
    /// A token that can be used in the next method call to block until the list of breakpoints changes.
    #[serde(rename="nextWaitToken")]
    
    pub next_wait_token: Option<String>,
    /// If set to `true`, indicates that there is no change to the list of active breakpoints and the server-selected timeout has expired. The `breakpoints` field would be empty and should be ignored.
    #[serde(rename="waitExpired")]
    
    pub wait_expired: Option<bool>,
}

impl client::ResponseResult for ListActiveBreakpointsResponse {}


/// Response for listing breakpoints.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints list debugger](DebuggerDebuggeeBreakpointListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListBreakpointsResponse {
    /// List of breakpoints matching the request. The fields `id` and `location` are guaranteed to be set on each breakpoint. The fields: `stack_frames`, `evaluated_expressions` and `variable_table` are cleared on each breakpoint regardless of its status.
    
    pub breakpoints: Option<Vec<Breakpoint>>,
    /// A wait token that can be used in the next call to `list` (REST) or `ListBreakpoints` (RPC) to block until the list of breakpoints has changes.
    #[serde(rename="nextWaitToken")]
    
    pub next_wait_token: Option<String>,
}

impl client::ResponseResult for ListBreakpointsResponse {}


/// Response for listing debuggees.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees list debugger](DebuggerDebuggeeListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListDebuggeesResponse {
    /// List of debuggees accessible to the calling user. The fields `debuggee.id` and `description` are guaranteed to be set. The `description` field is a human readable field provided by agents and can be displayed to users.
    
    pub debuggees: Option<Vec<Debuggee>>,
}

impl client::ResponseResult for ListDebuggeesResponse {}


/// Selects a repo using a Google Cloud Platform project ID (e.g. winged-cargo-31) and a repo name within that project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The name of the repo. Leave empty for the default repo.
    #[serde(rename="repoName")]
    
    pub repo_name: Option<String>,
}

impl client::Part for ProjectRepoId {}


/// Request to register a debuggee.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees register controller](ControllerDebuggeeRegisterCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterDebuggeeRequest {
    /// Required. Debuggee information to register. The fields `project`, `uniquifier`, `description` and `agent_version` of the debuggee must be set.
    
    pub debuggee: Option<Debuggee>,
}

impl client::RequestValue for RegisterDebuggeeRequest {}


/// Response for registering a debuggee.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees register controller](ControllerDebuggeeRegisterCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RegisterDebuggeeResponse {
    /// A unique ID generated for the agent. Each RegisterDebuggee request will generate a new agent ID.
    #[serde(rename="agentId")]
    
    pub agent_id: Option<String>,
    /// Debuggee resource. The field `id` is guaranteed to be set (in addition to the echoed fields). If the field `is_disabled` is set to `true`, the agent should disable itself by removing all breakpoints and detaching from the application. It should however continue to poll `RegisterDebuggee` until reenabled.
    
    pub debuggee: Option<Debuggee>,
}

impl client::ResponseResult for RegisterDebuggeeResponse {}


/// A unique identifier for a cloud repo.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RepoId {
    /// A combination of a project ID and a repo name.
    #[serde(rename="projectRepoId")]
    
    pub project_repo_id: Option<ProjectRepoId>,
    /// A server-assigned, globally unique identifier.
    
    pub uid: Option<String>,
}

impl client::Part for RepoId {}


/// Response for setting a breakpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints set debugger](DebuggerDebuggeeBreakpointSetCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SetBreakpointResponse {
    /// Breakpoint resource. The field `id` is guaranteed to be set (in addition to the echoed fields).
    
    pub breakpoint: Option<Breakpoint>,
}

impl client::ResponseResult for SetBreakpointResponse {}


/// A SourceContext is a reference to a tree of files. A SourceContext together with a path point to a unique revision of a single file or directory.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceContext {
    /// A SourceContext referring to a revision in a cloud repo.
    #[serde(rename="cloudRepo")]
    
    pub cloud_repo: Option<CloudRepoSourceContext>,
    /// A SourceContext referring to a snapshot in a cloud workspace.
    #[serde(rename="cloudWorkspace")]
    
    pub cloud_workspace: Option<CloudWorkspaceSourceContext>,
    /// A SourceContext referring to a Gerrit project.
    
    pub gerrit: Option<GerritSourceContext>,
    /// A SourceContext referring to any third party Git repo (e.g. GitHub).
    
    pub git: Option<GitSourceContext>,
}

impl client::Part for SourceContext {}


/// Represents a location in the source code.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Column within a line. The first column in a line as the value `1`. Agents that do not support setting breakpoints on specific columns ignore this field.
    
    pub column: Option<i32>,
    /// Line inside the file. The first line in the file has the value `1`.
    
    pub line: Option<i32>,
    /// Path to the source file within the source context of the target binary.
    
    pub path: Option<String>,
}

impl client::Part for SourceLocation {}


/// Represents a stack frame context.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StackFrame {
    /// Set of arguments passed to this function. Note that this might not be populated for all stack frames.
    
    pub arguments: Option<Vec<Variable>>,
    /// Demangled function name at the call site.
    
    pub function: Option<String>,
    /// Set of local variables at the stack frame location. Note that this might not be populated for all stack frames.
    
    pub locals: Option<Vec<Variable>>,
    /// Source location of the call site.
    
    pub location: Option<SourceLocation>,
}

impl client::Part for StackFrame {}


/// Represents a contextual status message. The message can indicate an error or informational status, and refer to specific parts of the containing object. For example, the `Breakpoint.status` field can indicate an error referring to the `BREAKPOINT_SOURCE_LOCATION` with the message `Location not found`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StatusMessage {
    /// Status message text.
    
    pub description: Option<FormatMessage>,
    /// Distinguishes errors from informational messages.
    #[serde(rename="isError")]
    
    pub is_error: Option<bool>,
    /// Reference to which the message applies.
    #[serde(rename="refersTo")]
    
    pub refers_to: Option<StatusMessageRefersToEnum>,
}

impl client::Part for StatusMessage {}


/// Request to update an active breakpoint.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints update controller](ControllerDebuggeeBreakpointUpdateCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateActiveBreakpointRequest {
    /// Required. Updated breakpoint information. The field `id` must be set. The agent must echo all Breakpoint specification fields in the update.
    
    pub breakpoint: Option<Breakpoint>,
}

impl client::RequestValue for UpdateActiveBreakpointRequest {}


/// Response for updating an active breakpoint. The message is defined to allow future extensions.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [debuggees breakpoints update controller](ControllerDebuggeeBreakpointUpdateCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateActiveBreakpointResponse { _never_set: Option<bool> }

impl client::ResponseResult for UpdateActiveBreakpointResponse {}


/// Represents a variable or an argument possibly of a compound object type. Note how the following variables are represented: 1) A simple variable: int x = 5 { name: "x", value: "5", type: "int" } // Captured variable 2) A compound object: struct T { int m1; int m2; }; T x = { 3, 7 }; { // Captured variable name: "x", type: "T", members { name: "m1", value: "3", type: "int" }, members { name: "m2", value: "7", type: "int" } } 3) A pointer where the pointee was captured: T x = { 3, 7 }; T* p = &x; { // Captured variable name: "p", type: "T*", value: "0x00500500", members { name: "m1", value: "3", type: "int" }, members { name: "m2", value: "7", type: "int" } } 4) A pointer where the pointee was not captured: T* p = new T; { // Captured variable name: "p", type: "T*", value: "0x00400400" status { is_error: true, description { format: "unavailable" } } } The status should describe the reason for the missing value, such as ``, ``, ``. Note that a null pointer should not have members. 5) An unnamed value: int* p = new int(7); { // Captured variable name: "p", value: "0x00500500", type: "int*", members { value: "7", type: "int" } } 6) An unnamed pointer where the pointee was not captured: int* p = new int(7); int** pp = &p; { // Captured variable name: "pp", value: "0x00500500", type: "int**", members { value: "0x00400400", type: "int*" status { is_error: true, description: { format: "unavailable" } } } } } To optimize computation, memory and network traffic, variables that repeat in the output multiple times can be stored once in a shared variable table and be referenced using the `var_table_index` field. The variables stored in the shared table are nameless and are essentially a partition of the complete variable. To reconstruct the complete variable, merge the referencing variable with the referenced variable. When using the shared variable table, the following variables: T x = { 3, 7 }; T* p = &x; T& r = x; { name: "x", var_table_index: 3, type: "T" } // Captured variables { name: "p", value "0x00500500", type="T*", var_table_index: 3 } { name: "r", type="T&", var_table_index: 3 } { // Shared variable table entry #3: members { name: "m1", value: "3", type: "int" }, members { name: "m2", value: "7", type: "int" } } Note that the pointer address is stored with the referencing variable and not with the referenced variable. This allows the referenced variable to be shared between pointers and references. The type field is optional. The debugger agent may or may not support it.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    /// Members contained or pointed to by the variable.
    
    pub members: Option<Vec<Variable>>,
    /// Name of the variable, if any.
    
    pub name: Option<String>,
    /// Status associated with the variable. This field will usually stay unset. A status of a single variable only applies to that variable or expression. The rest of breakpoint data still remains valid. Variables might be reported in error state even when breakpoint is not in final state. The message may refer to variable name with `refers_to` set to `VARIABLE_NAME`. Alternatively `refers_to` will be set to `VARIABLE_VALUE`. In either case variable value and members will be unset. Example of error message applied to name: `Invalid expression syntax`. Example of information message applied to value: `Not captured`. Examples of error message applied to value: * `Malformed string`, * `Field f not found in class C` * `Null pointer dereference`
    
    pub status: Option<StatusMessage>,
    /// Variable type (e.g. `MyClass`). If the variable is split with `var_table_index`, `type` goes next to `value`. The interpretation of a type is agent specific. It is recommended to include the dynamic type rather than a static type of an object.
    #[serde(rename="type")]
    
    pub type_: Option<String>,
    /// Simple value of the variable.
    
    pub value: Option<String>,
    /// Reference to a variable in the shared variable table. More than one variable can reference the same variable in the table. The `var_table_index` field is an index into `variable_table` in Breakpoint.
    #[serde(rename="varTableIndex")]
    
    pub var_table_index: Option<i32>,
}

impl client::Part for Variable {}


