#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// Read, compose, send, and permanently delete all your email from Gmail
    Scrip,

    /// See, edit, share, and permanently delete all the calendars you can access using Google Calendar
    CalendarFeeds,

    /// See, edit, download, and permanently delete your contacts
    M8Feeds,

    /// View and manage the provisioning of groups on your domain
    AdminDirectoryGroup,

    /// View and manage the provisioning of users on your domain
    AdminDirectoryUser,

    /// See, edit, create, and delete all your Google Docs documents
    Document,

    /// See, edit, create, and delete all of your Google Drive files
    Drive,

    /// View and manage your forms in Google Drive
    Form,

    /// View and manage forms that this application has been installed in
    FormCurrentonly,

    /// View and manage your Google Groups
    Group,

    /// Create and update Google Apps Script deployments
    Deployment,

    /// View Google Apps Script deployments
    DeploymentReadonly,

    /// View Google Apps Script project's metrics
    Metric,

    /// View Google Apps Script processes
    Process,

    /// Create and update Google Apps Script projects
    Project,

    /// View Google Apps Script projects
    ProjectReadonly,

    /// See, edit, create, and delete all your Google Sheets spreadsheets
    Spreadsheet,

    /// See your primary Google Account email address
    UserinfoEmail,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Scrip => "https://mail.google.com/",
            Scope::CalendarFeeds => "https://www.google.com/calendar/feeds",
            Scope::M8Feeds => "https://www.google.com/m8/feeds",
            Scope::AdminDirectoryGroup => "https://www.googleapis.com/auth/admin.directory.group",
            Scope::AdminDirectoryUser => "https://www.googleapis.com/auth/admin.directory.user",
            Scope::Document => "https://www.googleapis.com/auth/documents",
            Scope::Drive => "https://www.googleapis.com/auth/drive",
            Scope::Form => "https://www.googleapis.com/auth/forms",
            Scope::FormCurrentonly => "https://www.googleapis.com/auth/forms.currentonly",
            Scope::Group => "https://www.googleapis.com/auth/groups",
            Scope::Deployment => "https://www.googleapis.com/auth/script.deployments",
            Scope::DeploymentReadonly => {
                "https://www.googleapis.com/auth/script.deployments.readonly"
            }
            Scope::Metric => "https://www.googleapis.com/auth/script.metrics",
            Scope::Process => "https://www.googleapis.com/auth/script.processes",
            Scope::Project => "https://www.googleapis.com/auth/script.projects",
            Scope::ProjectReadonly => "https://www.googleapis.com/auth/script.projects.readonly",
            Scope::Spreadsheet => "https://www.googleapis.com/auth/spreadsheets",
            Scope::UserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::DeploymentReadonly
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all Script related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_script1 as script1;
/// use script1::{Result, Error};
/// # async fn dox() {
/// use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_list("scriptId")
///              .page_token("sed")
///              .page_size(-2)
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
pub struct Script<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Script<C> {}

impl<'a, C> Script<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> Script<C> {
        Script {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/7.0.0".to_string(),
            _base_url: "https://script.googleapis.com/".to_string(),
            _root_url: "https://script.googleapis.com/".to_string(),
        }
    }

    pub fn processes(&'a self) -> ProcessMethods<'a, C> {
        ProcessMethods { hub: self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C> {
        ProjectMethods { hub: self }
    }
    pub fn scripts(&'a self) -> ScriptMethods<'a, C> {
        ScriptMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/7.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://script.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://script.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// The Content resource.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get content projects](ProjectGetContentCall) (response)
/// * [update content projects](ProjectUpdateContentCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Content {
    /// The list of script project files. One of the files is a script manifest; it must be named "appsscript", must have type of JSON, and include the manifest configurations for the project.
    pub files: Option<Vec<File>>,
    /// The script project's Drive ID.
    #[serde(rename = "scriptId")]
    pub script_id: Option<String>,
}

impl common::RequestValue for Content {}
impl common::ResponseResult for Content {}

/// Request to create a script project.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [create projects](ProjectCreateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateProjectRequest {
    /// The Drive ID of a parent file that the created script project is bound to. This is usually the ID of a Google Doc, Google Sheet, Google Form, or Google Slides file. If not set, a standalone script project is created.
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    /// The title for the project.
    pub title: Option<String>,
}

impl common::RequestValue for CreateProjectRequest {}

/// Representation of a single script deployment.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments create projects](ProjectDeploymentCreateCall) (response)
/// * [deployments get projects](ProjectDeploymentGetCall) (response)
/// * [deployments update projects](ProjectDeploymentUpdateCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Deployment {
    /// The deployment configuration.
    #[serde(rename = "deploymentConfig")]
    pub deployment_config: Option<DeploymentConfig>,
    /// The deployment ID for this deployment.
    #[serde(rename = "deploymentId")]
    pub deployment_id: Option<String>,
    /// The deployment's entry points.
    #[serde(rename = "entryPoints")]
    pub entry_points: Option<Vec<EntryPoint>>,
    /// Last modified date time stamp.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::ResponseResult for Deployment {}

/// Metadata the defines how a deployment is configured.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments create projects](ProjectDeploymentCreateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeploymentConfig {
    /// The description for this deployment.
    pub description: Option<String>,
    /// The manifest file name for this deployment.
    #[serde(rename = "manifestFileName")]
    pub manifest_file_name: Option<String>,
    /// The script project's Drive ID.
    #[serde(rename = "scriptId")]
    pub script_id: Option<String>,
    /// The version number on which this deployment is based.
    #[serde(rename = "versionNumber")]
    pub version_number: Option<i32>,
}

impl common::RequestValue for DeploymentConfig {}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments delete projects](ProjectDeploymentDeleteCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Empty {
    _never_set: Option<bool>,
}

impl common::ResponseResult for Empty {}

/// A configuration that defines how a deployment is accessed externally.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EntryPoint {
    /// Add-on properties.
    #[serde(rename = "addOn")]
    pub add_on: Option<GoogleAppsScriptTypeAddOnEntryPoint>,
    /// The type of the entry point.
    #[serde(rename = "entryPointType")]
    pub entry_point_type: Option<String>,
    /// An entry point specification for Apps Script API execution calls.
    #[serde(rename = "executionApi")]
    pub execution_api: Option<GoogleAppsScriptTypeExecutionApiEntryPoint>,
    /// An entry point specification for web apps.
    #[serde(rename = "webApp")]
    pub web_app: Option<GoogleAppsScriptTypeWebAppEntryPoint>,
}

impl common::Part for EntryPoint {}

/// A request to run the function in a script. The script is identified by the specified `script_id`. Executing a function on a script returns results based on the implementation of the script.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [run scripts](ScriptRunCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ExecutionRequest {
    /// If `true` and the user is an owner of the script, the script runs at the most recently saved version rather than the version deployed for use with the Apps Script API. Optional; default is `false`.
    #[serde(rename = "devMode")]
    pub dev_mode: Option<bool>,
    /// The name of the function to execute in the given script. The name does not include parentheses or parameters. It can reference a function in an included library such as `Library.libFunction1`.
    pub function: Option<String>,
    /// The parameters to be passed to the function being executed. The object type for each parameter should match the expected type in Apps Script. Parameters cannot be Apps Script-specific object types (such as a `Document` or a `Calendar`); they can only be primitive types such as `string`, `number`, `array`, `object`, or `boolean`. Optional.
    pub parameters: Option<Vec<serde_json::Value>>,
    /// *Deprecated*. For use with Android add-ons only. An ID that represents the user's current session in the Android app for Google Docs or Sheets, included as extra data in the [Intent](https://developer.android.com/guide/components/intents-filters.html) that launches the add-on. When an Android add-on is run with a session state, it gains the privileges of a [bound](https://developers.google.com/apps-script/guides/bound) script—that is, it can access information like the user's current cursor position (in Docs) or selected cell (in Sheets). To retrieve the state, call `Intent.getStringExtra("com.google.android.apps.docs.addons.SessionState")`. Optional.
    #[serde(rename = "sessionState")]
    pub session_state: Option<String>,
}

impl common::RequestValue for ExecutionRequest {}

/// An individual file within a script project. A file is a third-party source code created by one or more developers. It can be a server-side JS code, HTML, or a configuration file. Each script project can contain multiple files.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct File {
    /// Creation date timestamp.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The defined set of functions in the script file, if any.
    #[serde(rename = "functionSet")]
    pub function_set: Option<GoogleAppsScriptTypeFunctionSet>,
    /// The user who modified the file most recently. The details visible in this object are controlled by the profile visibility settings of the last modifying user.
    #[serde(rename = "lastModifyUser")]
    pub last_modify_user: Option<GoogleAppsScriptTypeUser>,
    /// The name of the file. The file extension is not part of the file name, which can be identified from the type field.
    pub name: Option<String>,
    /// The file content.
    pub source: Option<String>,
    /// The type of the file.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Last modified date timestamp.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for File {}

/// An add-on entry point.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeAddOnEntryPoint {
    /// The add-on's required list of supported container types.
    #[serde(rename = "addOnType")]
    pub add_on_type: Option<String>,
    /// The add-on's optional description.
    pub description: Option<String>,
    /// The add-on's optional help URL.
    #[serde(rename = "helpUrl")]
    pub help_url: Option<String>,
    /// The add-on's required post install tip URL.
    #[serde(rename = "postInstallTipUrl")]
    pub post_install_tip_url: Option<String>,
    /// The add-on's optional report issue URL.
    #[serde(rename = "reportIssueUrl")]
    pub report_issue_url: Option<String>,
    /// The add-on's required title.
    pub title: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeAddOnEntryPoint {}

/// API executable entry point configuration.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeExecutionApiConfig {
    /// Who has permission to run the API executable.
    pub access: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeExecutionApiConfig {}

/// An API executable entry point.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeExecutionApiEntryPoint {
    /// The entry point's configuration.
    #[serde(rename = "entryPointConfig")]
    pub entry_point_config: Option<GoogleAppsScriptTypeExecutionApiConfig>,
}

impl common::Part for GoogleAppsScriptTypeExecutionApiEntryPoint {}

/// Represents a function in a script project.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeFunction {
    /// The function name in the script project.
    pub name: Option<String>,
    /// The ordered list of parameter names of the function in the script project.
    pub parameters: Option<Vec<String>>,
}

impl common::Part for GoogleAppsScriptTypeFunction {}

/// A set of functions. No duplicates are permitted.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeFunctionSet {
    /// A list of functions composing the set.
    pub values: Option<Vec<GoogleAppsScriptTypeFunction>>,
}

impl common::Part for GoogleAppsScriptTypeFunctionSet {}

/// Representation of a single script process execution that was started from the script editor, a trigger, an application, or using the Apps Script API. This is distinct from the `Operation` resource, which only represents executions started via the Apps Script API.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeProcess {
    /// Duration the execution spent executing.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub duration: Option<chrono::Duration>,
    /// Name of the function the started the execution.
    #[serde(rename = "functionName")]
    pub function_name: Option<String>,
    /// The executions status.
    #[serde(rename = "processStatus")]
    pub process_status: Option<String>,
    /// The executions type.
    #[serde(rename = "processType")]
    pub process_type: Option<String>,
    /// Name of the script being executed.
    #[serde(rename = "projectName")]
    pub project_name: Option<String>,
    /// Which version of maestro to use to execute the script.
    #[serde(rename = "runtimeVersion")]
    pub runtime_version: Option<String>,
    /// Time the execution started.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The executing users access level to the script.
    #[serde(rename = "userAccessLevel")]
    pub user_access_level: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeProcess {}

/// A simple user profile resource.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeUser {
    /// The user's domain.
    pub domain: Option<String>,
    /// The user's identifying email address.
    pub email: Option<String>,
    /// The user's display name.
    pub name: Option<String>,
    /// The user's photo.
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeUser {}

/// Web app entry point configuration.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeWebAppConfig {
    /// Who has permission to run the web app.
    pub access: Option<String>,
    /// Who to execute the web app as.
    #[serde(rename = "executeAs")]
    pub execute_as: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeWebAppConfig {}

/// A web application entry point.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAppsScriptTypeWebAppEntryPoint {
    /// The entry point's configuration.
    #[serde(rename = "entryPointConfig")]
    pub entry_point_config: Option<GoogleAppsScriptTypeWebAppConfig>,
    /// The URL for the web application.
    pub url: Option<String>,
}

impl common::Part for GoogleAppsScriptTypeWebAppEntryPoint {}

/// Response with the list of deployments for the specified Apps Script project.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments list projects](ProjectDeploymentListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListDeploymentsResponse {
    /// The list of deployments.
    pub deployments: Option<Vec<Deployment>>,
    /// The token that can be used in the next call to get the next page of results.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for ListDeploymentsResponse {}

/// Response with the list of Process resources.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list script processes processes](ProcessListScriptProcessCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListScriptProcessesResponse {
    /// Token for the next page of results. If empty, there are no more pages remaining.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// List of processes matching request parameters.
    pub processes: Option<Vec<GoogleAppsScriptTypeProcess>>,
}

impl common::ResponseResult for ListScriptProcessesResponse {}

/// Response with the list of Process resources.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list processes](ProcessListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListUserProcessesResponse {
    /// Token for the next page of results. If empty, there are no more pages remaining.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// List of processes matching request parameters.
    pub processes: Option<Vec<GoogleAppsScriptTypeProcess>>,
}

impl common::ResponseResult for ListUserProcessesResponse {}

/// Response with the list of the versions for the specified script project.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [versions list projects](ProjectVersionListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListVersionsResponse {
    /// The token use to fetch the next page of records. if not exist in the response, that means no more versions to list.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// The list of versions.
    pub versions: Option<Vec<Version>>,
}

impl common::ResponseResult for ListVersionsResponse {}

/// Resource containing usage stats for a given script, based on the supplied filter and mask present in the request.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get metrics projects](ProjectGetMetricCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Metrics {
    /// Number of active users.
    #[serde(rename = "activeUsers")]
    pub active_users: Option<Vec<MetricsValue>>,
    /// Number of failed executions.
    #[serde(rename = "failedExecutions")]
    pub failed_executions: Option<Vec<MetricsValue>>,
    /// Number of total executions.
    #[serde(rename = "totalExecutions")]
    pub total_executions: Option<Vec<MetricsValue>>,
}

impl common::ResponseResult for Metrics {}

/// Metrics value that holds number of executions counted.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricsValue {
    /// Required field indicating the end time of the interval.
    #[serde(rename = "endTime")]
    pub end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required field indicating the start time of the interval.
    #[serde(rename = "startTime")]
    pub start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Indicates the number of executions counted.
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub value: Option<u64>,
}

impl common::Part for MetricsValue {}

/// A representation of an execution of an Apps Script function started with run. The execution response does not arrive until the function finishes executing. The maximum execution runtime is listed in the [Apps Script quotas guide](https://developers.google.com/apps-script/guides/services/quotas#current_limitations). After execution has started, it can have one of four outcomes: - If the script function returns successfully, the response field contains an ExecutionResponse object with the function’s return value in the object’s `result` field. - If the script function (or Apps Script itself) throws an exception, the error field contains a Status object. The `Status` object’s `details` field contains an array with a single ExecutionError object that provides information about the nature of the error. - If the execution has not yet completed, the done field is `false` and the neither the `response` nor `error` fields are present. - If the `run` call itself fails (for example, because of a malformed request or an authorization error), the method returns an HTTP response code in the 4XX range with a different format for the response body. Client libraries automatically convert a 4XX response into an exception class.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [run scripts](ScriptRunCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Operation {
    /// This field indicates whether the script execution has completed. A completed execution has a populated `response` field containing the ExecutionResponse from function that was executed.
    pub done: Option<bool>,
    /// If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, this field contains a Status object. The `Status` object's `details` field contains an array with a single ExecutionError object that provides information about the nature of the error.
    pub error: Option<Status>,
    /// If the script function returns successfully, this field contains an ExecutionResponse object with the function's return value.
    pub response: Option<HashMap<String, serde_json::Value>>,
}

impl common::ResponseResult for Operation {}

/// The script project resource.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments create projects](ProjectDeploymentCreateCall) (none)
/// * [deployments delete projects](ProjectDeploymentDeleteCall) (none)
/// * [deployments get projects](ProjectDeploymentGetCall) (none)
/// * [deployments list projects](ProjectDeploymentListCall) (none)
/// * [deployments update projects](ProjectDeploymentUpdateCall) (none)
/// * [versions create projects](ProjectVersionCreateCall) (none)
/// * [versions get projects](ProjectVersionGetCall) (none)
/// * [versions list projects](ProjectVersionListCall) (none)
/// * [create projects](ProjectCreateCall) (response)
/// * [get projects](ProjectGetCall) (response)
/// * [get content projects](ProjectGetContentCall) (none)
/// * [get metrics projects](ProjectGetMetricCall) (none)
/// * [update content projects](ProjectUpdateContentCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    /// When the script was created.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// User who originally created the script.
    pub creator: Option<GoogleAppsScriptTypeUser>,
    /// User who last modified the script.
    #[serde(rename = "lastModifyUser")]
    pub last_modify_user: Option<GoogleAppsScriptTypeUser>,
    /// The parent's Drive ID that the script will be attached to. This is usually the ID of a Google Document or Google Sheet. This field is optional, and if not set, a stand-alone script will be created.
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    /// The script project's Drive ID.
    #[serde(rename = "scriptId")]
    pub script_id: Option<String>,
    /// The title for the project.
    pub title: Option<String>,
    /// When the script was last updated.
    #[serde(rename = "updateTime")]
    pub update_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Resource for Project {}
impl common::ResponseResult for Project {}

/// If a `run` call succeeds but the script function (or Apps Script itself) throws an exception, the response body's error field contains this `Status` object.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Status {
    /// The status code. For this API, this value either: - 10, indicating a `SCRIPT_TIMEOUT` error, - 3, indicating an `INVALID_ARGUMENT` error, or - 1, indicating a `CANCELLED` execution.
    pub code: Option<i32>,
    /// An array that contains a single ExecutionError object that provides information about the nature of the error.
    pub details: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// A developer-facing error message, which is in English. Any user-facing error message is localized and sent in the details field, or localized by the client.
    pub message: Option<String>,
}

impl common::Part for Status {}

/// Request with deployment information to update an existing deployment.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [deployments update projects](ProjectDeploymentUpdateCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateDeploymentRequest {
    /// The deployment configuration.
    #[serde(rename = "deploymentConfig")]
    pub deployment_config: Option<DeploymentConfig>,
}

impl common::RequestValue for UpdateDeploymentRequest {}

/// A resource representing a script project version. A version is a “snapshot” of a script project and is similar to a read-only branched release. When creating deployments, the version to use must be specified.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [versions create projects](ProjectVersionCreateCall) (request|response)
/// * [versions get projects](ProjectVersionGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Version {
    /// When the version was created.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The description for this version.
    pub description: Option<String>,
    /// The script project's Drive ID.
    #[serde(rename = "scriptId")]
    pub script_id: Option<String>,
    /// The incremental ID that is created by Apps Script when a version is created. This is system assigned number and is immutable once created.
    #[serde(rename = "versionNumber")]
    pub version_number: Option<i32>,
}

impl common::RequestValue for Version {}
impl common::ResponseResult for Version {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *process* resources.
/// It is not used directly, but through the [`Script`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_script1 as script1;
///
/// # async fn dox() {
/// use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = Script::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `list_script_processes(...)`
/// // to build up your call.
/// let rb = hub.processes();
/// # }
/// ```
pub struct ProcessMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
}

impl<'a, C> common::MethodsBuilder for ProcessMethods<'a, C> {}

impl<'a, C> ProcessMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// List information about processes made by or on behalf of a user, such as process type and current status.
    pub fn list(&self) -> ProcessListCall<'a, C> {
        ProcessListCall {
            hub: self.hub,
            _user_process_filter_user_access_levels: Default::default(),
            _user_process_filter_types: Default::default(),
            _user_process_filter_statuses: Default::default(),
            _user_process_filter_start_time: Default::default(),
            _user_process_filter_script_id: Default::default(),
            _user_process_filter_project_name: Default::default(),
            _user_process_filter_function_name: Default::default(),
            _user_process_filter_end_time: Default::default(),
            _user_process_filter_deployment_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// List information about a script's executed processes, such as process type and current status.
    pub fn list_script_processes(&self) -> ProcessListScriptProcessCall<'a, C> {
        ProcessListScriptProcessCall {
            hub: self.hub,
            _script_process_filter_user_access_levels: Default::default(),
            _script_process_filter_types: Default::default(),
            _script_process_filter_statuses: Default::default(),
            _script_process_filter_start_time: Default::default(),
            _script_process_filter_function_name: Default::default(),
            _script_process_filter_end_time: Default::default(),
            _script_process_filter_deployment_id: Default::default(),
            _script_id: Default::default(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Script`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_script1 as script1;
///
/// # async fn dox() {
/// use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = Script::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `create(...)`, `deployments_create(...)`, `deployments_delete(...)`, `deployments_get(...)`, `deployments_list(...)`, `deployments_update(...)`, `get(...)`, `get_content(...)`, `get_metrics(...)`, `update_content(...)`, `versions_create(...)`, `versions_get(...)` and `versions_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
}

impl<'a, C> common::MethodsBuilder for ProjectMethods<'a, C> {}

impl<'a, C> ProjectMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a deployment of an Apps Script project.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The script project's Drive ID.
    pub fn deployments_create(
        &self,
        request: DeploymentConfig,
        script_id: &str,
    ) -> ProjectDeploymentCreateCall<'a, C> {
        ProjectDeploymentCreateCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a deployment of an Apps Script project.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    /// * `deploymentId` - The deployment ID to be undeployed.
    pub fn deployments_delete(
        &self,
        script_id: &str,
        deployment_id: &str,
    ) -> ProjectDeploymentDeleteCall<'a, C> {
        ProjectDeploymentDeleteCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _deployment_id: deployment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets a deployment of an Apps Script project.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    /// * `deploymentId` - The deployment ID.
    pub fn deployments_get(
        &self,
        script_id: &str,
        deployment_id: &str,
    ) -> ProjectDeploymentGetCall<'a, C> {
        ProjectDeploymentGetCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _deployment_id: deployment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Lists the deployments of an Apps Script project.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    pub fn deployments_list(&self, script_id: &str) -> ProjectDeploymentListCall<'a, C> {
        ProjectDeploymentListCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates a deployment of an Apps Script project.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The script project's Drive ID.
    /// * `deploymentId` - The deployment ID for this deployment.
    pub fn deployments_update(
        &self,
        request: UpdateDeploymentRequest,
        script_id: &str,
        deployment_id: &str,
    ) -> ProjectDeploymentUpdateCall<'a, C> {
        ProjectDeploymentUpdateCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _deployment_id: deployment_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new immutable version using the current code, with a unique version number.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The script project's Drive ID.
    pub fn versions_create(
        &self,
        request: Version,
        script_id: &str,
    ) -> ProjectVersionCreateCall<'a, C> {
        ProjectVersionCreateCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets a version of a script project.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    /// * `versionNumber` - The version number.
    pub fn versions_get(
        &self,
        script_id: &str,
        version_number: i32,
    ) -> ProjectVersionGetCall<'a, C> {
        ProjectVersionGetCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _version_number: version_number,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// List the versions of a script project.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    pub fn versions_list(&self, script_id: &str) -> ProjectVersionListCall<'a, C> {
        ProjectVersionListCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new, empty script project with no script files and a base manifest file.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn create(&self, request: CreateProjectRequest) -> ProjectCreateCall<'a, C> {
        ProjectCreateCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets a script project's metadata.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    pub fn get(&self, script_id: &str) -> ProjectGetCall<'a, C> {
        ProjectGetCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Gets the content of the script project, including the code source and metadata for each script file.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - The script project's Drive ID.
    pub fn get_content(&self, script_id: &str) -> ProjectGetContentCall<'a, C> {
        ProjectGetContentCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _version_number: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Get metrics data for scripts, such as number of executions and active users.
    ///
    /// # Arguments
    ///
    /// * `scriptId` - Required field indicating the script to get metrics for.
    pub fn get_metrics(&self, script_id: &str) -> ProjectGetMetricCall<'a, C> {
        ProjectGetMetricCall {
            hub: self.hub,
            _script_id: script_id.to_string(),
            _metrics_granularity: Default::default(),
            _metrics_filter_deployment_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Updates the content of the specified script project. This content is stored as the HEAD version, and is used when the script is executed as a trigger, in the script editor, in add-on preview mode, or as a web app or Apps Script API in development mode. This clears all the existing files in the project.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The script project's Drive ID.
    pub fn update_content(
        &self,
        request: Content,
        script_id: &str,
    ) -> ProjectUpdateContentCall<'a, C> {
        ProjectUpdateContentCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *script* resources.
/// It is not used directly, but through the [`Script`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_script1 as script1;
///
/// # async fn dox() {
/// use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
/// ).build().await.unwrap();
///
/// let client = hyper_util::client::legacy::Client::builder(
///     hyper_util::rt::TokioExecutor::new()
/// )
/// .build(
///     hyper_rustls::HttpsConnectorBuilder::new()
///         .with_native_roots()
///         .unwrap()
///         .https_or_http()
///         .enable_http2()
///         .build()
/// );
/// let mut hub = Script::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `run(...)`
/// // to build up your call.
/// let rb = hub.scripts();
/// # }
/// ```
pub struct ScriptMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
}

impl<'a, C> common::MethodsBuilder for ScriptMethods<'a, C> {}

impl<'a, C> ScriptMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    ///
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `scriptId` - The script ID of the script to be executed. Find the script ID on the **Project settings** page under "IDs." As multiple executable APIs can be deployed in new IDE for same script, this field should be populated with DeploymentID generated while deploying in new IDE instead of script ID.
    pub fn run(&self, request: ExecutionRequest, script_id: &str) -> ScriptRunCall<'a, C> {
        ScriptRunCall {
            hub: self.hub,
            _request: request,
            _script_id: script_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// List information about processes made by or on behalf of a user, such as process type and current status.
///
/// A builder for the *list* method supported by a *process* resource.
/// It is not used directly, but through a [`ProcessMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.processes().list()
///              .add_user_process_filter_user_access_levels("takimata")
///              .add_user_process_filter_types("amet.")
///              .add_user_process_filter_statuses("duo")
///              .user_process_filter_start_time(chrono::Utc::now())
///              .user_process_filter_script_id("ipsum")
///              .user_process_filter_project_name("gubergren")
///              .user_process_filter_function_name("Lorem")
///              .user_process_filter_end_time(chrono::Utc::now())
///              .user_process_filter_deployment_id("gubergren")
///              .page_token("eos")
///              .page_size(-4)
///              .doit().await;
/// # }
/// ```
pub struct ProcessListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _user_process_filter_user_access_levels: Vec<String>,
    _user_process_filter_types: Vec<String>,
    _user_process_filter_statuses: Vec<String>,
    _user_process_filter_start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    _user_process_filter_script_id: Option<String>,
    _user_process_filter_project_name: Option<String>,
    _user_process_filter_function_name: Option<String>,
    _user_process_filter_end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    _user_process_filter_deployment_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProcessListCall<'a, C> {}

impl<'a, C> ProcessListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListUserProcessesResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.processes.list",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "userProcessFilter.userAccessLevels",
            "userProcessFilter.types",
            "userProcessFilter.statuses",
            "userProcessFilter.startTime",
            "userProcessFilter.scriptId",
            "userProcessFilter.projectName",
            "userProcessFilter.functionName",
            "userProcessFilter.endTime",
            "userProcessFilter.deploymentId",
            "pageToken",
            "pageSize",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        if !self._user_process_filter_user_access_levels.is_empty() {
            for f in self._user_process_filter_user_access_levels.iter() {
                params.push("userProcessFilter.userAccessLevels", f);
            }
        }
        if !self._user_process_filter_types.is_empty() {
            for f in self._user_process_filter_types.iter() {
                params.push("userProcessFilter.types", f);
            }
        }
        if !self._user_process_filter_statuses.is_empty() {
            for f in self._user_process_filter_statuses.iter() {
                params.push("userProcessFilter.statuses", f);
            }
        }
        if let Some(value) = self._user_process_filter_start_time.as_ref() {
            params.push(
                "userProcessFilter.startTime",
                common::serde::datetime_to_string(&value),
            );
        }
        if let Some(value) = self._user_process_filter_script_id.as_ref() {
            params.push("userProcessFilter.scriptId", value);
        }
        if let Some(value) = self._user_process_filter_project_name.as_ref() {
            params.push("userProcessFilter.projectName", value);
        }
        if let Some(value) = self._user_process_filter_function_name.as_ref() {
            params.push("userProcessFilter.functionName", value);
        }
        if let Some(value) = self._user_process_filter_end_time.as_ref() {
            params.push(
                "userProcessFilter.endTime",
                common::serde::datetime_to_string(&value),
            );
        }
        if let Some(value) = self._user_process_filter_deployment_id.as_ref() {
            params.push("userProcessFilter.deploymentId", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/processes";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Process.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Optional field used to limit returned processes to those having one of the specified user access levels.
    ///
    /// Append the given value to the *user process filter.user access levels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_user_process_filter_user_access_levels(
        mut self,
        new_value: &str,
    ) -> ProcessListCall<'a, C> {
        self._user_process_filter_user_access_levels
            .push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those having one of the specified process types.
    ///
    /// Append the given value to the *user process filter.types* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_user_process_filter_types(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_types.push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those having one of the specified process statuses.
    ///
    /// Append the given value to the *user process filter.statuses* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_user_process_filter_statuses(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_statuses
            .push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those that were started on or after the given timestamp.
    ///
    /// Sets the *user process filter.start time* query property to the given value.
    pub fn user_process_filter_start_time(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> ProcessListCall<'a, C> {
        self._user_process_filter_start_time = Some(new_value);
        self
    }
    /// Optional field used to limit returned processes to those originating from projects with a specific script ID.
    ///
    /// Sets the *user process filter.script id* query property to the given value.
    pub fn user_process_filter_script_id(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_script_id = Some(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those originating from projects with project names containing a specific string.
    ///
    /// Sets the *user process filter.project name* query property to the given value.
    pub fn user_process_filter_project_name(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_project_name = Some(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those originating from a script function with the given function name.
    ///
    /// Sets the *user process filter.function name* query property to the given value.
    pub fn user_process_filter_function_name(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_function_name = Some(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those that completed on or before the given timestamp.
    ///
    /// Sets the *user process filter.end time* query property to the given value.
    pub fn user_process_filter_end_time(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> ProcessListCall<'a, C> {
        self._user_process_filter_end_time = Some(new_value);
        self
    }
    /// Optional field used to limit returned processes to those originating from projects with a specific deployment ID.
    ///
    /// Sets the *user process filter.deployment id* query property to the given value.
    pub fn user_process_filter_deployment_id(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._user_process_filter_deployment_id = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProcessListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of returned processes per page of results. Defaults to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProcessListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ProcessListCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProcessListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Process`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProcessListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProcessListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProcessListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// List information about a script's executed processes, such as process type and current status.
///
/// A builder for the *listScriptProcesses* method supported by a *process* resource.
/// It is not used directly, but through a [`ProcessMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.processes().list_script_processes()
///              .add_script_process_filter_user_access_levels("ea")
///              .add_script_process_filter_types("ipsum")
///              .add_script_process_filter_statuses("invidunt")
///              .script_process_filter_start_time(chrono::Utc::now())
///              .script_process_filter_function_name("amet")
///              .script_process_filter_end_time(chrono::Utc::now())
///              .script_process_filter_deployment_id("duo")
///              .script_id("ipsum")
///              .page_token("sed")
///              .page_size(-37)
///              .doit().await;
/// # }
/// ```
pub struct ProcessListScriptProcessCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_process_filter_user_access_levels: Vec<String>,
    _script_process_filter_types: Vec<String>,
    _script_process_filter_statuses: Vec<String>,
    _script_process_filter_start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    _script_process_filter_function_name: Option<String>,
    _script_process_filter_end_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    _script_process_filter_deployment_id: Option<String>,
    _script_id: Option<String>,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProcessListScriptProcessCall<'a, C> {}

impl<'a, C> ProcessListScriptProcessCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListScriptProcessesResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.processes.listScriptProcesses",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "scriptProcessFilter.userAccessLevels",
            "scriptProcessFilter.types",
            "scriptProcessFilter.statuses",
            "scriptProcessFilter.startTime",
            "scriptProcessFilter.functionName",
            "scriptProcessFilter.endTime",
            "scriptProcessFilter.deploymentId",
            "scriptId",
            "pageToken",
            "pageSize",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        if !self._script_process_filter_user_access_levels.is_empty() {
            for f in self._script_process_filter_user_access_levels.iter() {
                params.push("scriptProcessFilter.userAccessLevels", f);
            }
        }
        if !self._script_process_filter_types.is_empty() {
            for f in self._script_process_filter_types.iter() {
                params.push("scriptProcessFilter.types", f);
            }
        }
        if !self._script_process_filter_statuses.is_empty() {
            for f in self._script_process_filter_statuses.iter() {
                params.push("scriptProcessFilter.statuses", f);
            }
        }
        if let Some(value) = self._script_process_filter_start_time.as_ref() {
            params.push(
                "scriptProcessFilter.startTime",
                common::serde::datetime_to_string(&value),
            );
        }
        if let Some(value) = self._script_process_filter_function_name.as_ref() {
            params.push("scriptProcessFilter.functionName", value);
        }
        if let Some(value) = self._script_process_filter_end_time.as_ref() {
            params.push(
                "scriptProcessFilter.endTime",
                common::serde::datetime_to_string(&value),
            );
        }
        if let Some(value) = self._script_process_filter_deployment_id.as_ref() {
            params.push("scriptProcessFilter.deploymentId", value);
        }
        if let Some(value) = self._script_id.as_ref() {
            params.push("scriptId", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/processes:listScriptProcesses";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Process.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Optional field used to limit returned processes to those having one of the specified user access levels.
    ///
    /// Append the given value to the *script process filter.user access levels* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_script_process_filter_user_access_levels(
        mut self,
        new_value: &str,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_user_access_levels
            .push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those having one of the specified process types.
    ///
    /// Append the given value to the *script process filter.types* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_script_process_filter_types(
        mut self,
        new_value: &str,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_types
            .push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those having one of the specified process statuses.
    ///
    /// Append the given value to the *script process filter.statuses* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_script_process_filter_statuses(
        mut self,
        new_value: &str,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_statuses
            .push(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those that were started on or after the given timestamp.
    ///
    /// Sets the *script process filter.start time* query property to the given value.
    pub fn script_process_filter_start_time(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_start_time = Some(new_value);
        self
    }
    /// Optional field used to limit returned processes to those originating from a script function with the given function name.
    ///
    /// Sets the *script process filter.function name* query property to the given value.
    pub fn script_process_filter_function_name(
        mut self,
        new_value: &str,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_function_name = Some(new_value.to_string());
        self
    }
    /// Optional field used to limit returned processes to those that completed on or before the given timestamp.
    ///
    /// Sets the *script process filter.end time* query property to the given value.
    pub fn script_process_filter_end_time(
        mut self,
        new_value: chrono::DateTime<chrono::offset::Utc>,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_end_time = Some(new_value);
        self
    }
    /// Optional field used to limit returned processes to those originating from projects with a specific deployment ID.
    ///
    /// Sets the *script process filter.deployment id* query property to the given value.
    pub fn script_process_filter_deployment_id(
        mut self,
        new_value: &str,
    ) -> ProcessListScriptProcessCall<'a, C> {
        self._script_process_filter_deployment_id = Some(new_value.to_string());
        self
    }
    /// The script ID of the project whose processes are listed.
    ///
    /// Sets the *script id* query property to the given value.
    pub fn script_id(mut self, new_value: &str) -> ProcessListScriptProcessCall<'a, C> {
        self._script_id = Some(new_value.to_string());
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProcessListScriptProcessCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of returned processes per page of results. Defaults to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProcessListScriptProcessCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProcessListScriptProcessCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProcessListScriptProcessCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Process`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProcessListScriptProcessCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProcessListScriptProcessCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProcessListScriptProcessCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates a deployment of an Apps Script project.
///
/// A builder for the *deployments.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::DeploymentConfig;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DeploymentConfig::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_create(req, "scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeploymentCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: DeploymentConfig,
    _script_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeploymentCreateCall<'a, C> {}

impl<'a, C> ProjectDeploymentCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Deployment)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.deployments.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/deployments";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Deployment.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: DeploymentConfig) -> ProjectDeploymentCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectDeploymentCreateCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectDeploymentCreateCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeploymentCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Deployment`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeploymentCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeploymentCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDeploymentCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Deletes a deployment of an Apps Script project.
///
/// A builder for the *deployments.delete* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_delete("scriptId", "deploymentId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeploymentDeleteCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _deployment_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeploymentDeleteCall<'a, C> {}

impl<'a, C> ProjectDeploymentDeleteCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Empty)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.deployments.delete",
            http_method: hyper::Method::DELETE,
        });

        for &field in ["alt", "scriptId", "deploymentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        params.push("deploymentId", self._deployment_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v1/projects/{scriptId}/deployments/{deploymentId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Deployment.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{scriptId}", "scriptId"),
            ("{deploymentId}", "deploymentId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["deploymentId", "scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectDeploymentDeleteCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The deployment ID to be undeployed.
    ///
    /// Sets the *deployment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn deployment_id(mut self, new_value: &str) -> ProjectDeploymentDeleteCall<'a, C> {
        self._deployment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectDeploymentDeleteCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeploymentDeleteCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Deployment`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeploymentDeleteCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeploymentDeleteCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDeploymentDeleteCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets a deployment of an Apps Script project.
///
/// A builder for the *deployments.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_get("scriptId", "deploymentId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeploymentGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _deployment_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeploymentGetCall<'a, C> {}

impl<'a, C> ProjectDeploymentGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Deployment)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.deployments.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId", "deploymentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        params.push("deploymentId", self._deployment_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v1/projects/{scriptId}/deployments/{deploymentId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::DeploymentReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{scriptId}", "scriptId"),
            ("{deploymentId}", "deploymentId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["deploymentId", "scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectDeploymentGetCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The deployment ID.
    ///
    /// Sets the *deployment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn deployment_id(mut self, new_value: &str) -> ProjectDeploymentGetCall<'a, C> {
        self._deployment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectDeploymentGetCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeploymentGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DeploymentReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeploymentGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeploymentGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDeploymentGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Lists the deployments of an Apps Script project.
///
/// A builder for the *deployments.list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_list("scriptId")
///              .page_token("gubergren")
///              .page_size(-17)
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeploymentListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeploymentListCall<'a, C> {}

impl<'a, C> ProjectDeploymentListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListDeploymentsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.deployments.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/deployments";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::DeploymentReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectDeploymentListCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectDeploymentListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of deployments on each returned page. Defaults to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectDeploymentListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectDeploymentListCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeploymentListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::DeploymentReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeploymentListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeploymentListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDeploymentListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates a deployment of an Apps Script project.
///
/// A builder for the *deployments.update* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::UpdateDeploymentRequest;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UpdateDeploymentRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().deployments_update(req, "scriptId", "deploymentId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeploymentUpdateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: UpdateDeploymentRequest,
    _script_id: String,
    _deployment_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeploymentUpdateCall<'a, C> {}

impl<'a, C> ProjectDeploymentUpdateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Deployment)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.deployments.update",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "scriptId", "deploymentId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        params.push("deploymentId", self._deployment_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v1/projects/{scriptId}/deployments/{deploymentId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Deployment.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{scriptId}", "scriptId"),
            ("{deploymentId}", "deploymentId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["deploymentId", "scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(
        mut self,
        new_value: UpdateDeploymentRequest,
    ) -> ProjectDeploymentUpdateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectDeploymentUpdateCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The deployment ID for this deployment.
    ///
    /// Sets the *deployment id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn deployment_id(mut self, new_value: &str) -> ProjectDeploymentUpdateCall<'a, C> {
        self._deployment_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectDeploymentUpdateCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeploymentUpdateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Deployment`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeploymentUpdateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeploymentUpdateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectDeploymentUpdateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates a new immutable version using the current code, with a unique version number.
///
/// A builder for the *versions.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::Version;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Version::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().versions_create(req, "scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectVersionCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: Version,
    _script_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectVersionCreateCall<'a, C> {}

impl<'a, C> ProjectVersionCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Version)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.versions.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/versions";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Project.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Version) -> ProjectVersionCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectVersionCreateCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectVersionCreateCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectVersionCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Project`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectVersionCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectVersionCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectVersionCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets a version of a script project.
///
/// A builder for the *versions.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().versions_get("scriptId", -43)
///              .doit().await;
/// # }
/// ```
pub struct ProjectVersionGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _version_number: i32,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectVersionGetCall<'a, C> {}

impl<'a, C> ProjectVersionGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Version)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.versions.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId", "versionNumber"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        params.push("versionNumber", self._version_number.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v1/projects/{scriptId}/versions/{versionNumber}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::ProjectReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{scriptId}", "scriptId"),
            ("{versionNumber}", "versionNumber"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["versionNumber", "scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectVersionGetCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The version number.
    ///
    /// Sets the *version number* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn version_number(mut self, new_value: i32) -> ProjectVersionGetCall<'a, C> {
        self._version_number = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectVersionGetCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectVersionGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ProjectReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectVersionGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectVersionGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectVersionGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// List the versions of a script project.
///
/// A builder for the *versions.list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().versions_list("scriptId")
///              .page_token("sed")
///              .page_size(-61)
///              .doit().await;
/// # }
/// ```
pub struct ProjectVersionListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectVersionListCall<'a, C> {}

impl<'a, C> ProjectVersionListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListVersionsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.versions.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/versions";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::ProjectReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectVersionListCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from a previous response.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectVersionListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of versions on each returned page. Defaults to 50.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectVersionListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectVersionListCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectVersionListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ProjectReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectVersionListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectVersionListCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectVersionListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates a new, empty script project with no script files and a base manifest file.
///
/// A builder for the *create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::CreateProjectRequest;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateProjectRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().create(req)
///              .doit().await;
/// # }
/// ```
pub struct ProjectCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: CreateProjectRequest,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectCreateCall<'a, C> {}

impl<'a, C> ProjectCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Project)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Project.as_ref().to_string());
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CreateProjectRequest) -> ProjectCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ProjectCreateCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectCreateCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Project`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectCreateCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets a script project's metadata.
///
/// A builder for the *get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get("scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectGetCall<'a, C> {}

impl<'a, C> ProjectGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Project)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("scriptId", self._script_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::ProjectReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectGetCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ProjectGetCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ProjectReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectGetCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets the content of the script project, including the code source and metadata for each script file.
///
/// A builder for the *getContent* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_content("scriptId")
///              .version_number(-24)
///              .doit().await;
/// # }
/// ```
pub struct ProjectGetContentCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _version_number: Option<i32>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectGetContentCall<'a, C> {}

impl<'a, C> ProjectGetContentCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Content)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.getContent",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "scriptId", "versionNumber"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        if let Some(value) = self._version_number.as_ref() {
            params.push("versionNumber", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/content";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::ProjectReadonly.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectGetContentCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The version number of the project to retrieve. If not provided, the project's HEAD version is returned.
    ///
    /// Sets the *version number* query property to the given value.
    pub fn version_number(mut self, new_value: i32) -> ProjectGetContentCall<'a, C> {
        self._version_number = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectGetContentCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetContentCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::ProjectReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectGetContentCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectGetContentCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectGetContentCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Get metrics data for scripts, such as number of executions and active users.
///
/// A builder for the *getMetrics* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().get_metrics("scriptId")
///              .metrics_granularity("et")
///              .metrics_filter_deployment_id("et")
///              .doit().await;
/// # }
/// ```
pub struct ProjectGetMetricCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _script_id: String,
    _metrics_granularity: Option<String>,
    _metrics_filter_deployment_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectGetMetricCall<'a, C> {}

impl<'a, C> ProjectGetMetricCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Metrics)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.getMetrics",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "scriptId",
            "metricsGranularity",
            "metricsFilter.deploymentId",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("scriptId", self._script_id);
        if let Some(value) = self._metrics_granularity.as_ref() {
            params.push("metricsGranularity", value);
        }
        if let Some(value) = self._metrics_filter_deployment_id.as_ref() {
            params.push("metricsFilter.deploymentId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/metrics";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Metric.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_LENGTH, 0_u64)
                    .body(common::to_body::<String>(None));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    /// Required field indicating the script to get metrics for.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectGetMetricCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// Required field indicating what granularity of metrics are returned.
    ///
    /// Sets the *metrics granularity* query property to the given value.
    pub fn metrics_granularity(mut self, new_value: &str) -> ProjectGetMetricCall<'a, C> {
        self._metrics_granularity = Some(new_value.to_string());
        self
    }
    /// Optional field indicating a specific deployment to retrieve metrics from.
    ///
    /// Sets the *metrics filter.deployment id* query property to the given value.
    pub fn metrics_filter_deployment_id(mut self, new_value: &str) -> ProjectGetMetricCall<'a, C> {
        self._metrics_filter_deployment_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectGetMetricCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectGetMetricCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Metric`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectGetMetricCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectGetMetricCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectGetMetricCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Updates the content of the specified script project. This content is stored as the HEAD version, and is used when the script is executed as a trigger, in the script editor, in add-on preview mode, or as a web app or Apps Script API in development mode. This clears all the existing files in the project.
///
/// A builder for the *updateContent* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::Content;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Content::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().update_content(req, "scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectUpdateContentCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: Content,
    _script_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectUpdateContentCall<'a, C> {}

impl<'a, C> ProjectUpdateContentCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Content)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.projects.updateContent",
            http_method: hyper::Method::PUT,
        });

        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{scriptId}/content";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Project.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: Content) -> ProjectUpdateContentCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The script project's Drive ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ProjectUpdateContentCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(
        mut self,
        new_value: &'a mut dyn common::Delegate,
    ) -> ProjectUpdateContentCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ProjectUpdateContentCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Project`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectUpdateContentCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectUpdateContentCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ProjectUpdateContentCall<'a, C> {
        self._scopes.clear();
        self
    }
}

///
///
/// A builder for the *run* method supported by a *script* resource.
/// It is not used directly, but through a [`ScriptMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_script1 as script1;
/// use script1::api::ExecutionRequest;
/// # async fn dox() {
/// # use script1::{Script, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
/// # ).build().await.unwrap();
///
/// # let client = hyper_util::client::legacy::Client::builder(
/// #     hyper_util::rt::TokioExecutor::new()
/// # )
/// # .build(
/// #     hyper_rustls::HttpsConnectorBuilder::new()
/// #         .with_native_roots()
/// #         .unwrap()
/// #         .https_or_http()
/// #         .enable_http2()
/// #         .build()
/// # );
/// # let mut hub = Script::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = ExecutionRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.scripts().run(req, "scriptId")
///              .doit().await;
/// # }
/// ```
pub struct ScriptRunCall<'a, C>
where
    C: 'a,
{
    hub: &'a Script<C>,
    _request: ExecutionRequest,
    _script_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ScriptRunCall<'a, C> {}

impl<'a, C> ScriptRunCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, Operation)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "script.scripts.run",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "scriptId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("scriptId", self._script_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/scripts/{scriptId}:run";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Scrip.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{scriptId}", "scriptId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["scriptId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
        let mut request_value_reader = {
            let mut value = serde_json::value::to_value(&self._request).expect("serde to work");
            common::remove_json_null_values(&mut value);
            let mut dst = std::io::Cursor::new(Vec::with_capacity(128));
            serde_json::to_writer(&mut dst, &value).unwrap();
            dst
        };
        let request_size = request_value_reader
            .seek(std::io::SeekFrom::End(0))
            .unwrap();
        request_value_reader
            .seek(std::io::SeekFrom::Start(0))
            .unwrap();

        loop {
            let token = match self
                .hub
                .auth
                .get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..])
                .await
            {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(common::Error::MissingToken(e));
                    }
                },
            };
            request_value_reader
                .seek(std::io::SeekFrom::Start(0))
                .unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

                if let Some(token) = token.as_ref() {
                    req_builder = req_builder.header(AUTHORIZATION, format!("Bearer {}", token));
                }

                let request = req_builder
                    .header(CONTENT_TYPE, json_mime_type.to_string())
                    .header(CONTENT_LENGTH, request_size as u64)
                    .body(common::to_body(
                        request_value_reader.get_ref().clone().into(),
                    ));

                client.request(request.unwrap()).await
            };

            match req_result {
                Err(err) => {
                    if let common::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
                        continue;
                    }
                    dlg.finished(false);
                    return Err(common::Error::HttpError(err));
                }
                Ok(res) => {
                    let (mut parts, body) = res.into_parts();
                    let mut body = common::Body::new(body);
                    if !parts.status.is_success() {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let error = serde_json::from_str(&common::to_string(&bytes));
                        let response = common::to_response(parts, bytes.into());

                        if let common::Retry::After(d) =
                            dlg.http_failure(&response, error.as_ref().ok())
                        {
                            sleep(d).await;
                            continue;
                        }

                        dlg.finished(false);

                        return Err(match error {
                            Ok(value) => common::Error::BadRequest(value),
                            _ => common::Error::Failure(response),
                        });
                    }
                    let response = {
                        let bytes = common::to_bytes(body).await.unwrap_or_default();
                        let encoded = common::to_string(&bytes);
                        match serde_json::from_str(&encoded) {
                            Ok(decoded) => (common::to_response(parts, bytes.into()), decoded),
                            Err(error) => {
                                dlg.response_json_decode_error(&encoded, &error);
                                return Err(common::Error::JsonDecodeError(
                                    encoded.to_string(),
                                    error,
                                ));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(response);
                }
            }
        }
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: ExecutionRequest) -> ScriptRunCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The script ID of the script to be executed. Find the script ID on the **Project settings** page under "IDs." As multiple executable APIs can be deployed in new IDE for same script, this field should be populated with DeploymentID generated while deploying in new IDE instead of script ID.
    ///
    /// Sets the *script id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn script_id(mut self, new_value: &str) -> ScriptRunCall<'a, C> {
        self._script_id = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    ///
    /// ````text
    ///                   It should be used to handle progress information, and to implement a certain level of resilience.
    /// ````
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> ScriptRunCall<'a, C> {
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
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *alt* (query-string) - Data format for response.
    /// * *callback* (query-string) - JSONP
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    pub fn param<T>(mut self, name: T, value: T) -> ScriptRunCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Scrip`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ScriptRunCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ScriptRunCall<'a, C>
    where
        I: IntoIterator<Item = St>,
        St: AsRef<str>,
    {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> ScriptRunCall<'a, C> {
        self._scopes.clear();
        self
    }
}
