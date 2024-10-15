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
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,

    /// View your data across Google Cloud services and see the email address of your Google Account
    CloudPlatformReadOnly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            Scope::CloudPlatformReadOnly => {
                "https://www.googleapis.com/auth/cloud-platform.read-only"
            }
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all Testing related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
/// use testing1::api::DeviceSession;
/// use testing1::{Result, Error};
/// # async fn dox() {
/// use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// // Get an ApplicationSecret instance by some means. It contains the `client_id` and
/// // `client_secret`, among other things.
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// // Instantiate the authenticator. It will choose a suitable authentication flow for you,
/// // unless you replace  `None` with the desired Flow.
/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
/// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
/// // retrieve them from storage.
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DeviceSession::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_patch(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
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
pub struct Testing<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Testing<C> {}

impl<'a, C> Testing<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> Testing<C> {
        Testing {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://testing.googleapis.com/".to_string(),
            _root_url: "https://testing.googleapis.com/".to_string(),
        }
    }

    pub fn application_detail_service(&'a self) -> ApplicationDetailServiceMethods<'a, C> {
        ApplicationDetailServiceMethods { hub: self }
    }
    pub fn projects(&'a self) -> ProjectMethods<'a, C> {
        ProjectMethods { hub: self }
    }
    pub fn test_environment_catalog(&'a self) -> TestEnvironmentCatalogMethods<'a, C> {
        TestEnvironmentCatalogMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://testing.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// Identifies an account and how to log into it.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// An automatic google login account.
    #[serde(rename = "googleAuto")]
    pub google_auto: Option<GoogleAuto>,
}

impl common::Part for Account {}

/// A single Android device.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidDevice {
    /// Required. The id of the Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "androidModelId")]
    pub android_model_id: Option<String>,
    /// Required. The id of the Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "androidVersionId")]
    pub android_version_id: Option<String>,
    /// Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
}

impl common::Part for AndroidDevice {}

/// The currently supported Android devices.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidDeviceCatalog {
    /// The set of supported Android device models.
    pub models: Option<Vec<AndroidModel>>,
    /// The set of supported runtime configurations.
    #[serde(rename = "runtimeConfiguration")]
    pub runtime_configuration: Option<AndroidRuntimeConfiguration>,
    /// The set of supported Android OS versions.
    pub versions: Option<Vec<AndroidVersion>>,
}

impl common::Part for AndroidDeviceCatalog {}

/// A list of Android device configurations in which the test is to be executed.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidDeviceList {
    /// Required. A list of Android devices.
    #[serde(rename = "androidDevices")]
    pub android_devices: Option<Vec<AndroidDevice>>,
}

impl common::Part for AndroidDeviceList {}

/// A test of an Android application that can control an Android component independently of its normal lifecycle. Android instrumentation tests run an application APK and test APK inside the same process on a virtual or physical AndroidDevice. They also specify a test runner class, such as com.google.GoogleTestRunner, which can vary on the specific instrumentation framework chosen. See for more information on types of Android tests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidInstrumentationTest {
    /// The APK for the application under test.
    #[serde(rename = "appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename = "appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The java package for the application under test. The default value is determined by examining the application's manifest.
    #[serde(rename = "appPackageId")]
    pub app_package_id: Option<String>,
    /// The option of whether running each test within its own invocation of instrumentation with Android Test Orchestrator or not. ** Orchestrator is only compatible with AndroidJUnitRunner version 1.1 or higher! ** Orchestrator offers the following benefits: - No shared state - Crashes are isolated - Logs are scoped per test See for more information about Android Test Orchestrator. If not set, the test will be run without the orchestrator.
    #[serde(rename = "orchestratorOption")]
    pub orchestrator_option: Option<String>,
    /// The option to run tests in multiple shards in parallel.
    #[serde(rename = "shardingOption")]
    pub sharding_option: Option<ShardingOption>,
    /// Required. The APK containing the test code to be executed.
    #[serde(rename = "testApk")]
    pub test_apk: Option<FileReference>,
    /// The java package for the test to be executed. The default value is determined by examining the application's manifest.
    #[serde(rename = "testPackageId")]
    pub test_package_id: Option<String>,
    /// The InstrumentationTestRunner class. The default value is determined by examining the application's manifest.
    #[serde(rename = "testRunnerClass")]
    pub test_runner_class: Option<String>,
    /// Each target must be fully qualified with the package name or class name, in one of these formats: - "package package_name" - "class package_name.class_name" - "class package_name.class_name#method_name" If empty, all targets in the module will be run.
    #[serde(rename = "testTargets")]
    pub test_targets: Option<Vec<String>>,
}

impl common::Part for AndroidInstrumentationTest {}

/// A set of Android device configuration permutations is defined by the the cross-product of the given axes. Internally, the given AndroidMatrix will be expanded into a set of AndroidDevices. Only supported permutations will be instantiated. Invalid permutations (e.g., incompatible models/versions) are ignored.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidMatrix {
    /// Required. The ids of the set of Android device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "androidModelIds")]
    pub android_model_ids: Option<Vec<String>>,
    /// Required. The ids of the set of Android OS version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "androidVersionIds")]
    pub android_version_ids: Option<Vec<String>>,
    /// Required. The set of locales the test device will enable for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locales: Option<Vec<String>>,
    /// Required. The set of orientations to test with. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientations: Option<Vec<String>>,
}

impl common::Part for AndroidMatrix {}

/// A description of an Android device tests may be run on.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidModel {
    /// The company that this device is branded with. Example: "Google", "Samsung".
    pub brand: Option<String>,
    /// The name of the industrial design. This corresponds to android.os.Build.DEVICE.
    pub codename: Option<String>,
    /// Whether this device is virtual or physical.
    pub form: Option<String>,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename = "formFactor")]
    pub form_factor: Option<String>,
    /// The unique opaque id for this model. Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// True if and only if tests with this model are recorded by stitching together screenshots. See use_low_spec_video_recording in device config.
    #[serde(rename = "lowFpsVideoRecording")]
    pub low_fps_video_recording: Option<bool>,
    /// The manufacturer of this device.
    pub manufacturer: Option<String>,
    /// The human-readable marketing name for this device model. Examples: "Nexus 5", "Galaxy S5".
    pub name: Option<String>,
    /// Version-specific information of an Android model.
    #[serde(rename = "perVersionInfo")]
    pub per_version_info: Option<Vec<PerAndroidVersionInfo>>,
    /// Screen density in DPI. This corresponds to ro.sf.lcd_density
    #[serde(rename = "screenDensity")]
    pub screen_density: Option<i32>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[serde(rename = "screenX")]
    pub screen_x: Option<i32>,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[serde(rename = "screenY")]
    pub screen_y: Option<i32>,
    /// The list of supported ABIs for this device. This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2. The most preferred ABI is the first element in the list. Elements are optionally prefixed by "version_id:" (where version_id is the id of an AndroidVersion), denoting an ABI that is supported only on a particular version.
    #[serde(rename = "supportedAbis")]
    pub supported_abis: Option<Vec<String>>,
    /// The set of Android versions this device supports.
    #[serde(rename = "supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// URL of a thumbnail image (photo) of the device.
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
}

impl common::Part for AndroidModel {}

/// A test of an android application that explores the application on a virtual or physical Android Device, finding culprits and crashes as it goes.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidRoboTest {
    /// The APK for the application under test.
    #[serde(rename = "appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename = "appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The initial activity that should be used to start the app.
    #[serde(rename = "appInitialActivity")]
    pub app_initial_activity: Option<String>,
    /// The java package for the application under test. The default value is determined by examining the application's manifest.
    #[serde(rename = "appPackageId")]
    pub app_package_id: Option<String>,
    /// The max depth of the traversal stack Robo can explore. Needs to be at least 2 to make Robo explore the app beyond the first activity. Default is 50.
    #[serde(rename = "maxDepth")]
    pub max_depth: Option<i32>,
    /// The max number of steps Robo can execute. Default is no limit.
    #[serde(rename = "maxSteps")]
    pub max_steps: Option<i32>,
    /// A set of directives Robo should apply during the crawl. This allows users to customize the crawl. For example, the username and password for a test account can be provided.
    #[serde(rename = "roboDirectives")]
    pub robo_directives: Option<Vec<RoboDirective>>,
    /// The mode in which Robo should run. Most clients should allow the server to populate this field automatically.
    #[serde(rename = "roboMode")]
    pub robo_mode: Option<String>,
    /// A JSON file with a sequence of actions Robo should perform as a prologue for the crawl.
    #[serde(rename = "roboScript")]
    pub robo_script: Option<FileReference>,
    /// The intents used to launch the app for the crawl. If none are provided, then the main launcher activity is launched. If some are provided, then only those provided are launched (the main launcher activity must be provided explicitly).
    #[serde(rename = "startingIntents")]
    pub starting_intents: Option<Vec<RoboStartingIntent>>,
}

impl common::Part for AndroidRoboTest {}

/// Android configuration that can be selected at the time a test is run.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidRuntimeConfiguration {
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
}

impl common::Part for AndroidRuntimeConfiguration {}

/// A test of an Android Application with a Test Loop. The intent \ will be implicitly added, since Games is the only user of this api, for the time being.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidTestLoop {
    /// The APK for the application under test.
    #[serde(rename = "appApk")]
    pub app_apk: Option<FileReference>,
    /// A multi-apk app bundle for the application under test.
    #[serde(rename = "appBundle")]
    pub app_bundle: Option<AppBundle>,
    /// The java package for the application under test. The default is determined by examining the application's manifest.
    #[serde(rename = "appPackageId")]
    pub app_package_id: Option<String>,
    /// The list of scenario labels that should be run during the test. The scenario labels should map to labels defined in the application's manifest. For example, player_experience and com.google.test.loops.player_experience add all of the loops labeled in the manifest with the com.google.test.loops.player_experience name to the execution. Scenarios can also be specified in the scenarios field.
    #[serde(rename = "scenarioLabels")]
    pub scenario_labels: Option<Vec<String>>,
    /// The list of scenarios that should be run during the test. The default is all test loops, derived from the application's manifest.
    pub scenarios: Option<Vec<i32>>,
}

impl common::Part for AndroidTestLoop {}

/// A version of the Android OS.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AndroidVersion {
    /// The API level for this Android version. Examples: 18, 19.
    #[serde(rename = "apiLevel")]
    pub api_level: Option<i32>,
    /// The code name for this Android version. Examples: "JellyBean", "KitKat".
    #[serde(rename = "codeName")]
    pub code_name: Option<String>,
    /// Market share for this version.
    pub distribution: Option<Distribution>,
    /// An opaque id for this Android version. Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
    /// The date this Android version became available in the market.
    #[serde(rename = "releaseDate")]
    pub release_date: Option<Date>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
    /// A string representing this version of the Android OS. Examples: "4.3", "4.4".
    #[serde(rename = "versionString")]
    pub version_string: Option<String>,
}

impl common::Part for AndroidVersion {}

/// An Android package file to install.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Apk {
    /// The path to an APK to be installed on the device before the test begins.
    pub location: Option<FileReference>,
    /// The java package for the APK to be installed. Value is determined by examining the application's manifest.
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
}

impl common::Part for Apk {}

/// Android application details based on application manifest and archive contents.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApkDetail {
    /// no description provided
    #[serde(rename = "apkManifest")]
    pub apk_manifest: Option<ApkManifest>,
}

impl common::Part for ApkDetail {}

/// An Android app manifest. See http://developer.android.com/guide/topics/manifest/manifest-intro.html
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApkManifest {
    /// User-readable name for the application.
    #[serde(rename = "applicationLabel")]
    pub application_label: Option<String>,
    /// no description provided
    #[serde(rename = "intentFilters")]
    pub intent_filters: Option<Vec<IntentFilter>>,
    /// Maximum API level on which the application is designed to run.
    #[serde(rename = "maxSdkVersion")]
    pub max_sdk_version: Option<i32>,
    /// Meta-data tags defined in the manifest.
    pub metadata: Option<Vec<Metadata>>,
    /// Minimum API level required for the application to run.
    #[serde(rename = "minSdkVersion")]
    pub min_sdk_version: Option<i32>,
    /// Full Java-style package name for this application, e.g. "com.example.foo".
    #[serde(rename = "packageName")]
    pub package_name: Option<String>,
    /// Services contained in the tag.
    pub services: Option<Vec<Service>>,
    /// Specifies the API Level on which the application is designed to run.
    #[serde(rename = "targetSdkVersion")]
    pub target_sdk_version: Option<i32>,
    /// Feature usage tags defined in the manifest.
    #[serde(rename = "usesFeature")]
    pub uses_feature: Option<Vec<UsesFeature>>,
    /// Permissions declared to be used by the application
    #[serde(rename = "usesPermission")]
    pub uses_permission: Option<Vec<String>>,
    /// Version number used internally by the app.
    #[serde(rename = "versionCode")]
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
    /// Version number shown to users.
    #[serde(rename = "versionName")]
    pub version_name: Option<String>,
}

impl common::Part for ApkManifest {}

/// An Android App Bundle file format, containing a BundleConfig.pb file, a base module directory, zero or more dynamic feature module directories. See https://developer.android.com/guide/app-bundle/build for guidance on building App Bundles.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AppBundle {
    /// .aab file representing the app bundle under test.
    #[serde(rename = "bundleLocation")]
    pub bundle_location: Option<FileReference>,
}

impl common::Part for AppBundle {}

/// The request object for cancelling a Device Session.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [device sessions cancel projects](ProjectDeviceSessionCancelCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CancelDeviceSessionRequest {
    _never_set: Option<bool>,
}

impl common::RequestValue for CancelDeviceSessionRequest {}

/// Response containing the current state of the specified test matrix.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [test matrices cancel projects](ProjectTestMatriceCancelCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CancelTestMatrixResponse {
    /// The current rolled-up state of the test matrix. If this state is already final, then the cancelation request will have no effect.
    #[serde(rename = "testState")]
    pub test_state: Option<String>,
}

impl common::ResponseResult for CancelTestMatrixResponse {}

/// Information about the client which invoked the test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientInfo {
    /// The list of detailed information about client.
    #[serde(rename = "clientInfoDetails")]
    pub client_info_details: Option<Vec<ClientInfoDetail>>,
    /// Required. Client name, such as gcloud.
    pub name: Option<String>,
}

impl common::Part for ClientInfo {}

/// Key-value pair of detailed information about the client which invoked the test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientInfoDetail {
    /// Required. The key of detailed client information.
    pub key: Option<String>,
    /// Required. The value of detailed client information.
    pub value: Option<String>,
}

impl common::Part for ClientInfoDetail {}

/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Date {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    pub year: Option<i32>,
}

impl common::Part for Date {}

/// A single device file description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceFile {
    /// A reference to an opaque binary blob file.
    #[serde(rename = "obbFile")]
    pub obb_file: Option<ObbFile>,
    /// A reference to a regular file.
    #[serde(rename = "regularFile")]
    pub regular_file: Option<RegularFile>,
}

impl common::Part for DeviceFile {}

/// A single device IP block
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceIpBlock {
    /// The date this block was added to Firebase Test Lab
    #[serde(rename = "addedDate")]
    pub added_date: Option<Date>,
    /// An IP address block in CIDR notation eg: 34.68.194.64/29
    pub block: Option<String>,
    /// Whether this block is used by physical or virtual devices
    pub form: Option<String>,
}

impl common::Part for DeviceIpBlock {}

/// List of IP blocks used by the Firebase Test Lab
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceIpBlockCatalog {
    /// The device IP blocks used by Firebase Test Lab
    #[serde(rename = "ipBlocks")]
    pub ip_blocks: Option<Vec<DeviceIpBlock>>,
}

impl common::Part for DeviceIpBlockCatalog {}

/// Protobuf message describing the device message, used from several RPCs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [device sessions create projects](ProjectDeviceSessionCreateCall) (request|response)
/// * [device sessions get projects](ProjectDeviceSessionGetCall) (response)
/// * [device sessions patch projects](ProjectDeviceSessionPatchCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeviceSession {
    /// Output only. The timestamp that the session first became ACTIVE.
    #[serde(rename = "activeStartTime")]
    pub active_start_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Required. The requested device
    #[serde(rename = "androidDevice")]
    pub android_device: Option<AndroidDevice>,
    /// Output only. The time that the Session was created.
    #[serde(rename = "createTime")]
    pub create_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Output only. The title of the DeviceSession to be presented in the UI.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// Optional. If the device is still in use at this time, any connections will be ended and the SessionState will transition from ACTIVE to FINISHED.
    #[serde(rename = "expireTime")]
    pub expire_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Output only. The interval of time that this device must be interacted with before it transitions from ACTIVE to TIMEOUT_INACTIVITY.
    #[serde(rename = "inactivityTimeout")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub inactivity_timeout: Option<chrono::Duration>,
    /// Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub name: Option<String>,
    /// Output only. Current state of the DeviceSession.
    pub state: Option<String>,
    /// Output only. The historical state transitions of the session_state message including the current session state.
    #[serde(rename = "stateHistories")]
    pub state_histories: Option<Vec<SessionStateEvent>>,
    /// Optional. The amount of time that a device will be initially allocated for. This can eventually be extended with the UpdateDeviceSession RPC. Default: 15 minutes.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub ttl: Option<chrono::Duration>,
}

impl common::RequestValue for DeviceSession {}
impl common::ResponseResult for DeviceSession {}

/// Denotes whether Direct Access is supported, and by which client versions. DirectAccessService is currently available as a preview to select developers. You can register today on behalf of you and your team at https://developer.android.com/studio/preview/android-device-streaming
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DirectAccessVersionInfo {
    /// Whether direct access is supported at all. Clients are expected to filter down the device list to only android models and versions which support Direct Access when that is the user intent.
    #[serde(rename = "directAccessSupported")]
    pub direct_access_supported: Option<bool>,
    /// Output only. Indicates client-device compatibility, where a device is known to work only with certain workarounds implemented in the Android Studio client. Expected format "major.minor.micro.patch", e.g. "5921.22.2211.8881706".
    #[serde(rename = "minimumAndroidStudioVersion")]
    pub minimum_android_studio_version: Option<String>,
}

impl common::Part for DirectAccessVersionInfo {}

/// Data about the relative number of devices running a given configuration of the Android platform.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Distribution {
    /// Output only. The estimated fraction (0-1) of the total market with this configuration.
    #[serde(rename = "marketShare")]
    pub market_share: Option<f64>,
    /// Output only. The time this distribution was measured.
    #[serde(rename = "measurementTime")]
    pub measurement_time: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::Part for Distribution {}

/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [device sessions cancel projects](ProjectDeviceSessionCancelCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Empty {
    _never_set: Option<bool>,
}

impl common::ResponseResult for Empty {}

/// The environment in which the test is run.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Environment {
    /// An Android device which must be used with an Android test.
    #[serde(rename = "androidDevice")]
    pub android_device: Option<AndroidDevice>,
    /// An iOS device which must be used with an iOS test.
    #[serde(rename = "iosDevice")]
    pub ios_device: Option<IosDevice>,
}

impl common::Part for Environment {}

/// The matrix of environments in which the test is to be executed.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentMatrix {
    /// A list of Android devices; the test will be run only on the specified devices.
    #[serde(rename = "androidDeviceList")]
    pub android_device_list: Option<AndroidDeviceList>,
    /// A matrix of Android devices.
    #[serde(rename = "androidMatrix")]
    pub android_matrix: Option<AndroidMatrix>,
    /// A list of iOS devices.
    #[serde(rename = "iosDeviceList")]
    pub ios_device_list: Option<IosDeviceList>,
}

impl common::Part for EnvironmentMatrix {}

/// A key-value pair passed as an environment variable to the test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentVariable {
    /// Key for the environment variable.
    pub key: Option<String>,
    /// Value for the environment variable.
    pub value: Option<String>,
}

impl common::Part for EnvironmentVariable {}

/// A reference to a file, used for user inputs.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get apk details application detail service](ApplicationDetailServiceGetApkDetailCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FileReference {
    /// A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding)
    #[serde(rename = "gcsPath")]
    pub gcs_path: Option<String>,
}

impl common::RequestValue for FileReference {}

/// Response containing the details of the specified Android application.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get apk details application detail service](ApplicationDetailServiceGetApkDetailCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetApkDetailsResponse {
    /// Details of the Android App.
    #[serde(rename = "apkDetail")]
    pub apk_detail: Option<ApkDetail>,
}

impl common::ResponseResult for GetApkDetailsResponse {}

/// Enables automatic Google account login. If set, the service automatically generates a Google test account and adds it to the device, before executing the test. Note that test accounts might be reused. Many applications show their full set of functionalities when an account is present on the device. Logging into the device with these generated accounts allows testing more functionalities.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleAuto {
    _never_set: Option<bool>,
}

impl common::Part for GoogleAuto {}

/// A storage location within Google cloud storage (GCS).
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GoogleCloudStorage {
    /// Required. The path to a directory in GCS that will eventually contain the results for this test. The requesting user must have write access on the bucket in the supplied path.
    #[serde(rename = "gcsPath")]
    pub gcs_path: Option<String>,
}

impl common::Part for GoogleCloudStorage {}

/// The section of an tag. https://developer.android.com/guide/topics/manifest/intent-filter-element.html
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IntentFilter {
    /// The android:name value of the tag.
    #[serde(rename = "actionNames")]
    pub action_names: Option<Vec<String>>,
    /// The android:name value of the tag.
    #[serde(rename = "categoryNames")]
    pub category_names: Option<Vec<String>>,
    /// The android:mimeType value of the tag.
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
}

impl common::Part for IntentFilter {}

/// A single iOS device.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosDevice {
    /// Required. The id of the iOS device to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "iosModelId")]
    pub ios_model_id: Option<String>,
    /// Required. The id of the iOS major software version to be used. Use the TestEnvironmentDiscoveryService to get supported options.
    #[serde(rename = "iosVersionId")]
    pub ios_version_id: Option<String>,
    /// Required. The locale the test device used for testing. Use the TestEnvironmentDiscoveryService to get supported options.
    pub locale: Option<String>,
    /// Required. How the device is oriented during the test. Use the TestEnvironmentDiscoveryService to get supported options.
    pub orientation: Option<String>,
}

impl common::Part for IosDevice {}

/// The currently supported iOS devices.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosDeviceCatalog {
    /// The set of supported iOS device models.
    pub models: Option<Vec<IosModel>>,
    /// The set of supported runtime configurations.
    #[serde(rename = "runtimeConfiguration")]
    pub runtime_configuration: Option<IosRuntimeConfiguration>,
    /// The set of supported iOS software versions.
    pub versions: Option<Vec<IosVersion>>,
    /// The set of supported Xcode versions.
    #[serde(rename = "xcodeVersions")]
    pub xcode_versions: Option<Vec<XcodeVersion>>,
}

impl common::Part for IosDeviceCatalog {}

/// A file or directory to install on the device before the test starts.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosDeviceFile {
    /// The bundle id of the app where this file lives. iOS apps sandbox their own filesystem, so app files must specify which app installed on the device.
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<String>,
    /// The source file
    pub content: Option<FileReference>,
    /// Location of the file on the device, inside the app's sandboxed filesystem
    #[serde(rename = "devicePath")]
    pub device_path: Option<String>,
}

impl common::Part for IosDeviceFile {}

/// A list of iOS device configurations in which the test is to be executed.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosDeviceList {
    /// Required. A list of iOS devices.
    #[serde(rename = "iosDevices")]
    pub ios_devices: Option<Vec<IosDevice>>,
}

impl common::Part for IosDeviceList {}

/// A description of an iOS device tests may be run on.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosModel {
    /// Device capabilities. Copied from https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html
    #[serde(rename = "deviceCapabilities")]
    pub device_capabilities: Option<Vec<String>>,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[serde(rename = "formFactor")]
    pub form_factor: Option<String>,
    /// The unique opaque id for this model. Use this for invoking the TestExecutionService.
    pub id: Option<String>,
    /// The human-readable name for this device model. Examples: "iPhone 4s", "iPad Mini 2".
    pub name: Option<String>,
    /// Version-specific information of an iOS model.
    #[serde(rename = "perVersionInfo")]
    pub per_version_info: Option<Vec<PerIosVersionInfo>>,
    /// Screen density in DPI.
    #[serde(rename = "screenDensity")]
    pub screen_density: Option<i32>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[serde(rename = "screenX")]
    pub screen_x: Option<i32>,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[serde(rename = "screenY")]
    pub screen_y: Option<i32>,
    /// The set of iOS major software versions this device supports.
    #[serde(rename = "supportedVersionIds")]
    pub supported_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
}

impl common::Part for IosModel {}

/// A test that explores an iOS application on an iOS device.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosRoboTest {
    /// The bundle ID for the app-under-test. This is determined by examining the application's "Info.plist" file.
    #[serde(rename = "appBundleId")]
    pub app_bundle_id: Option<String>,
    /// Required. The ipa stored at this file should be used to run the test.
    #[serde(rename = "appIpa")]
    pub app_ipa: Option<FileReference>,
    /// An optional Roboscript to customize the crawl. See https://firebase.google.com/docs/test-lab/android/robo-scripts-reference for more information about Roboscripts.
    #[serde(rename = "roboScript")]
    pub robo_script: Option<FileReference>,
}

impl common::Part for IosRoboTest {}

/// iOS configuration that can be selected at the time a test is run.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosRuntimeConfiguration {
    /// The set of available locales.
    pub locales: Option<Vec<Locale>>,
    /// The set of available orientations.
    pub orientations: Option<Vec<Orientation>>,
}

impl common::Part for IosRuntimeConfiguration {}

/// A test of an iOS application that implements one or more game loop scenarios. This test type accepts an archived application (.ipa file) and a list of integer scenarios that will be executed on the app sequentially.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosTestLoop {
    /// Output only. The bundle id for the application under test.
    #[serde(rename = "appBundleId")]
    pub app_bundle_id: Option<String>,
    /// Required. The .ipa of the application to test.
    #[serde(rename = "appIpa")]
    pub app_ipa: Option<FileReference>,
    /// The list of scenarios that should be run during the test. Defaults to the single scenario 0 if unspecified.
    pub scenarios: Option<Vec<i32>>,
}

impl common::Part for IosTestLoop {}

/// A description of how to set up an iOS device prior to running the test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosTestSetup {
    /// iOS apps to install in addition to those being directly tested.
    #[serde(rename = "additionalIpas")]
    pub additional_ipas: Option<Vec<FileReference>>,
    /// The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename = "networkProfile")]
    pub network_profile: Option<String>,
    /// List of directories on the device to upload to Cloud Storage at the end of the test. Directories should either be in a shared directory (such as /private/var/mobile/Media) or within an accessible directory inside the app's filesystem (such as /Documents) by specifying the bundle ID.
    #[serde(rename = "pullDirectories")]
    pub pull_directories: Option<Vec<IosDeviceFile>>,
    /// List of files to push to the device before starting the test.
    #[serde(rename = "pushFiles")]
    pub push_files: Option<Vec<IosDeviceFile>>,
}

impl common::Part for IosTestSetup {}

/// An iOS version.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosVersion {
    /// An opaque id for this iOS version. Use this id to invoke the TestExecutionService.
    pub id: Option<String>,
    /// An integer representing the major iOS version. Examples: "8", "9".
    #[serde(rename = "majorVersion")]
    pub major_version: Option<i32>,
    /// An integer representing the minor iOS version. Examples: "1", "2".
    #[serde(rename = "minorVersion")]
    pub minor_version: Option<i32>,
    /// The available Xcode versions for this version.
    #[serde(rename = "supportedXcodeVersionIds")]
    pub supported_xcode_version_ids: Option<Vec<String>>,
    /// Tags for this dimension. Examples: "default", "preview", "deprecated".
    pub tags: Option<Vec<String>>,
}

impl common::Part for IosVersion {}

/// A test of an iOS application that uses the XCTest framework. Xcode supports the option to "build for testing", which generates an .xctestrun file that contains a test specification (arguments, test methods, etc). This test type accepts a zip file containing the .xctestrun file and the corresponding contents of the Build/Products directory that contains all the binaries needed to run the tests.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IosXcTest {
    /// Output only. The bundle id for the application under test.
    #[serde(rename = "appBundleId")]
    pub app_bundle_id: Option<String>,
    /// The option to test special app entitlements. Setting this would re-sign the app having special entitlements with an explicit application-identifier. Currently supports testing aps-environment entitlement.
    #[serde(rename = "testSpecialEntitlements")]
    pub test_special_entitlements: Option<bool>,
    /// Required. The .zip containing the .xctestrun file and the contents of the DerivedData/Build/Products directory. The .xctestrun file in this zip is ignored if the xctestrun field is specified.
    #[serde(rename = "testsZip")]
    pub tests_zip: Option<FileReference>,
    /// The Xcode version that should be used for the test. Use the TestEnvironmentDiscoveryService to get supported options. Defaults to the latest Xcode version Firebase Test Lab supports.
    #[serde(rename = "xcodeVersion")]
    pub xcode_version: Option<String>,
    /// An .xctestrun file that will override the .xctestrun file in the tests zip. Because the .xctestrun file contains environment variables along with test methods to run and/or ignore, this can be useful for sharding tests. Default is taken from the tests zip.
    pub xctestrun: Option<FileReference>,
}

impl common::Part for IosXcTest {}

/// Specifies an intent that starts the main launcher activity.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LauncherActivityIntent {
    _never_set: Option<bool>,
}

impl common::Part for LauncherActivityIntent {}

/// A list of device sessions.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [device sessions list projects](ProjectDeviceSessionListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListDeviceSessionsResponse {
    /// The sessions matching the specified filter in the given cloud project.
    #[serde(rename = "deviceSessions")]
    pub device_sessions: Option<Vec<DeviceSession>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

impl common::ResponseResult for ListDeviceSessionsResponse {}

/// A location/region designation for language.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Locale {
    /// The id for this locale. Example: "en_US".
    pub id: Option<String>,
    /// A human-friendly name for this language/locale. Example: "English".
    pub name: Option<String>,
    /// A human-friendly string representing the region for this locale. Example: "United States". Not present for every locale.
    pub region: Option<String>,
    /// Tags for this dimension. Example: "default".
    pub tags: Option<Vec<String>>,
}

impl common::Part for Locale {}

/// Shards test cases into the specified groups of packages, classes, and/or methods. With manual sharding enabled, specifying test targets via environment_variables or in InstrumentationTest is invalid.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ManualSharding {
    /// Required. Group of packages, classes, and/or test methods to be run for each manually-created shard. You must specify at least one shard if this field is present. When you select one or more physical devices, the number of repeated test_targets_for_shard must be <= 50. When you select one or more ARM virtual devices, it must be <= 200. When you select only x86 virtual devices, it must be <= 500.
    #[serde(rename = "testTargetsForShard")]
    pub test_targets_for_shard: Option<Vec<TestTargetsForShard>>,
}

impl common::Part for ManualSharding {}

/// Describes a single error or issue with a matrix.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MatrixErrorDetail {
    /// Output only. A human-readable message about how the error in the TestMatrix. Expands on the `reason` field with additional details and possible options to fix the issue.
    pub message: Option<String>,
    /// Output only. The reason for the error. This is a constant value in UPPER_SNAKE_CASE that identifies the cause of the error.
    pub reason: Option<String>,
}

impl common::Part for MatrixErrorDetail {}

/// A tag within a manifest. https://developer.android.com/guide/topics/manifest/meta-data-element.html
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    /// The android:name value
    pub name: Option<String>,
    /// The android:value value
    pub value: Option<String>,
}

impl common::Part for Metadata {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfiguration {
    /// The emulation rule applying to the download traffic.
    #[serde(rename = "downRule")]
    pub down_rule: Option<TrafficRule>,
    /// The unique opaque id for this network traffic configuration.
    pub id: Option<String>,
    /// The emulation rule applying to the upload traffic.
    #[serde(rename = "upRule")]
    pub up_rule: Option<TrafficRule>,
}

impl common::Part for NetworkConfiguration {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfigurationCatalog {
    /// no description provided
    pub configurations: Option<Vec<NetworkConfiguration>>,
}

impl common::Part for NetworkConfigurationCatalog {}

/// Skips the starting activity
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NoActivityIntent {
    _never_set: Option<bool>,
}

impl common::Part for NoActivityIntent {}

/// An opaque binary blob file to install on the device before the test starts.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObbFile {
    /// Required. Opaque Binary Blob (OBB) file(s) to install on the device.
    pub obb: Option<FileReference>,
    /// Required. OBB file name which must conform to the format as specified by Android e.g. [main|patch].0300110.com.example.android.obb which will be installed into \/Android/obb/\/ on the device.
    #[serde(rename = "obbFileName")]
    pub obb_file_name: Option<String>,
}

impl common::Part for ObbFile {}

/// Screen orientation of the device.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Orientation {
    /// The id for this orientation. Example: "portrait".
    pub id: Option<String>,
    /// A human-friendly name for this orientation. Example: "portrait".
    pub name: Option<String>,
    /// Tags for this dimension. Example: "default".
    pub tags: Option<Vec<String>>,
}

impl common::Part for Orientation {}

/// A version-specific information of an Android model.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PerAndroidVersionInfo {
    /// The number of online devices for an Android version.
    #[serde(rename = "deviceCapacity")]
    pub device_capacity: Option<String>,
    /// Output only. Identifies supported clients for DirectAccess for this Android version.
    #[serde(rename = "directAccessVersionInfo")]
    pub direct_access_version_info: Option<DirectAccessVersionInfo>,
    /// Output only. The estimated wait time for a single interactive device session using Direct Access.
    #[serde(rename = "interactiveDeviceAvailabilityEstimate")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub interactive_device_availability_estimate: Option<chrono::Duration>,
    /// An Android version.
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

impl common::Part for PerAndroidVersionInfo {}

/// A version-specific information of an iOS model.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PerIosVersionInfo {
    /// The number of online devices for an iOS version.
    #[serde(rename = "deviceCapacity")]
    pub device_capacity: Option<String>,
    /// An iOS version.
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

impl common::Part for PerIosVersionInfo {}

/// The currently provided software environment on the devices under test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ProvidedSoftwareCatalog {
    /// A string representing the current version of AndroidX Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#androidx.test:orchestrator.
    #[serde(rename = "androidxOrchestratorVersion")]
    pub androidx_orchestrator_version: Option<String>,
    /// Deprecated: Use AndroidX Test Orchestrator going forward. A string representing the current version of Android Test Orchestrator that is used in the environment. The package is available at https://maven.google.com/web/index.html#com.android.support.test:orchestrator.
    #[serde(rename = "orchestratorVersion")]
    pub orchestrator_version: Option<String>,
}

impl common::Part for ProvidedSoftwareCatalog {}

/// A file or directory to install on the device before the test starts.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RegularFile {
    /// Required. The source file.
    pub content: Option<FileReference>,
    /// Required. Where to put the content on the device. Must be an absolute, allowlisted path. If the file exists, it will be replaced. The following device-side directories and any of their subdirectories are allowlisted: ${EXTERNAL_STORAGE}, /sdcard, or /storage ${ANDROID_DATA}/local/tmp, or /data/local/tmp Specifying a path outside of these directory trees is invalid. The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device and copy the file there. It is strongly advised to use the Environment API in app and test code to access files on the device in a portable way.
    #[serde(rename = "devicePath")]
    pub device_path: Option<String>,
}

impl common::Part for RegularFile {}

/// Locations where the results of running the test are stored.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResultStorage {
    /// Required.
    #[serde(rename = "googleCloudStorage")]
    pub google_cloud_storage: Option<GoogleCloudStorage>,
    /// Output only. URL to the results in the Firebase Web Console.
    #[serde(rename = "resultsUrl")]
    pub results_url: Option<String>,
    /// Output only. The tool results execution that results are written to.
    #[serde(rename = "toolResultsExecution")]
    pub tool_results_execution: Option<ToolResultsExecution>,
    /// The tool results history that contains the tool results execution that results are written to. If not provided, the service will choose an appropriate value.
    #[serde(rename = "toolResultsHistory")]
    pub tool_results_history: Option<ToolResultsHistory>,
}

impl common::Part for ResultStorage {}

/// Directs Robo to interact with a specific UI element if it is encountered during the crawl. Currently, Robo can perform text entry or element click.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoboDirective {
    /// Required. The type of action that Robo should perform on the specified element.
    #[serde(rename = "actionType")]
    pub action_type: Option<String>,
    /// The text that Robo is directed to set. If left empty, the directive will be treated as a CLICK on the element matching the resource_name.
    #[serde(rename = "inputText")]
    pub input_text: Option<String>,
    /// Required. The android resource name of the target UI element. For example, in Java: R.string.foo in xml: @string/foo Only the "foo" part is needed. Reference doc: https://developer.android.com/guide/topics/resources/accessing-resources.html
    #[serde(rename = "resourceName")]
    pub resource_name: Option<String>,
}

impl common::Part for RoboDirective {}

/// Message for specifying the start activities to crawl.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoboStartingIntent {
    /// An intent that starts the main launcher activity.
    #[serde(rename = "launcherActivity")]
    pub launcher_activity: Option<LauncherActivityIntent>,
    /// Skips the starting activity
    #[serde(rename = "noActivity")]
    pub no_activity: Option<NoActivityIntent>,
    /// An intent that starts an activity with specific details.
    #[serde(rename = "startActivity")]
    pub start_activity: Option<StartActivityIntent>,
    /// Timeout in seconds for each intent.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub timeout: Option<chrono::Duration>,
}

impl common::Part for RoboStartingIntent {}

/// The section of an tag. https://developer.android.com/guide/topics/manifest/service-element
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Service {
    /// Intent filters in the service
    #[serde(rename = "intentFilter")]
    pub intent_filter: Option<Vec<IntentFilter>>,
    /// The android:name value
    pub name: Option<String>,
}

impl common::Part for Service {}

/// A message encapsulating a series of Session states and the time that the DeviceSession first entered those states.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionStateEvent {
    /// Output only. The time that the session_state first encountered that state.
    #[serde(rename = "eventTime")]
    pub event_time: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Output only. The session_state tracked by this event
    #[serde(rename = "sessionState")]
    pub session_state: Option<String>,
    /// Output only. A human-readable message to explain the state.
    #[serde(rename = "stateMessage")]
    pub state_message: Option<String>,
}

impl common::Part for SessionStateEvent {}

/// Output only. Details about the shard.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Shard {
    /// Output only. The estimated shard duration based on previous test case timing records, if available.
    #[serde(rename = "estimatedShardDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub estimated_shard_duration: Option<chrono::Duration>,
    /// Output only. The total number of shards.
    #[serde(rename = "numShards")]
    pub num_shards: Option<i32>,
    /// Output only. The index of the shard among all the shards.
    #[serde(rename = "shardIndex")]
    pub shard_index: Option<i32>,
    /// Output only. Test targets for each shard. Only set for manual sharding.
    #[serde(rename = "testTargetsForShard")]
    pub test_targets_for_shard: Option<TestTargetsForShard>,
}

impl common::Part for Shard {}

/// Options for enabling sharding.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ShardingOption {
    /// Shards test cases into the specified groups of packages, classes, and/or methods.
    #[serde(rename = "manualSharding")]
    pub manual_sharding: Option<ManualSharding>,
    /// Shards test based on previous test case timing records.
    #[serde(rename = "smartSharding")]
    pub smart_sharding: Option<SmartSharding>,
    /// Uniformly shards test cases given a total number of shards.
    #[serde(rename = "uniformSharding")]
    pub uniform_sharding: Option<UniformSharding>,
}

impl common::Part for ShardingOption {}

/// Shards test based on previous test case timing records.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SmartSharding {
    /// The amount of time tests within a shard should take. Default: 300 seconds (5 minutes). The minimum allowed: 120 seconds (2 minutes). The shard count is dynamically set based on time, up to the maximum shard limit (described below). To guarantee at least one test case for each shard, the number of shards will not exceed the number of test cases. Shard duration will be exceeded if: - The maximum shard limit is reached and there is more calculated test time remaining to allocate into shards. - Any individual test is estimated to be longer than the targeted shard duration. Shard duration is not guaranteed because smart sharding uses test case history and default durations which may not be accurate. The rules for finding the test case timing records are: - If the service has processed a test case in the last 30 days, the record of the latest successful test case will be used. - For new test cases, the average duration of other known test cases will be used. - If there are no previous test case timing records available, the default test case duration is 15 seconds. Because the actual shard duration can exceed the targeted shard duration, we recommend that you set the targeted value at least 5 minutes less than the maximum allowed test timeout (45 minutes for physical devices and 60 minutes for virtual), or that you use the custom test timeout value that you set. This approach avoids cancelling the shard before all tests can finish. Note that there is a limit for maximum number of shards. When you select one or more physical devices, the number of shards must be <= 50. When you select one or more ARM virtual devices, it must be <= 200. When you select only x86 virtual devices, it must be <= 500. To guarantee at least one test case for per shard, the number of shards will not exceed the number of test cases. Each shard created counts toward daily test quota.
    #[serde(rename = "targetedShardDuration")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub targeted_shard_duration: Option<chrono::Duration>,
}

impl common::Part for SmartSharding {}

/// A starting intent specified by an action, uri, and categories.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StartActivityIntent {
    /// Action name. Required for START_ACTIVITY.
    pub action: Option<String>,
    /// Intent categories to set on the intent.
    pub categories: Option<Vec<String>>,
    /// URI for the action.
    pub uri: Option<String>,
}

impl common::Part for StartActivityIntent {}

/// There is no detailed description.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SystraceSetup {
    /// Systrace duration in seconds. Should be between 1 and 30 seconds. 0 disables systrace.
    #[serde(rename = "durationSeconds")]
    pub duration_seconds: Option<i32>,
}

impl common::Part for SystraceSetup {}

/// Additional details about the progress of the running test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestDetails {
    /// Output only. If the TestState is ERROR, then this string will contain human-readable details about the error.
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    /// Output only. Human-readable, detailed descriptions of the test's progress. For example: "Provisioning a device", "Starting Test". During the course of execution new data may be appended to the end of progress_messages.
    #[serde(rename = "progressMessages")]
    pub progress_messages: Option<Vec<String>>,
}

impl common::Part for TestDetails {}

/// A description of a test environment.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get test environment catalog](TestEnvironmentCatalogGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestEnvironmentCatalog {
    /// Supported Android devices.
    #[serde(rename = "androidDeviceCatalog")]
    pub android_device_catalog: Option<AndroidDeviceCatalog>,
    /// The IP blocks used by devices in the test environment.
    #[serde(rename = "deviceIpBlockCatalog")]
    pub device_ip_block_catalog: Option<DeviceIpBlockCatalog>,
    /// Supported iOS devices.
    #[serde(rename = "iosDeviceCatalog")]
    pub ios_device_catalog: Option<IosDeviceCatalog>,
    /// Supported network configurations.
    #[serde(rename = "networkConfigurationCatalog")]
    pub network_configuration_catalog: Option<NetworkConfigurationCatalog>,
    /// The software test environment provided by TestExecutionService.
    #[serde(rename = "softwareCatalog")]
    pub software_catalog: Option<ProvidedSoftwareCatalog>,
}

impl common::ResponseResult for TestEnvironmentCatalog {}

/// A single test executed in a single environment.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestExecution {
    /// Output only. How the host machine(s) are configured.
    pub environment: Option<Environment>,
    /// Output only. Unique id set by the service.
    pub id: Option<String>,
    /// Output only. Id of the containing TestMatrix.
    #[serde(rename = "matrixId")]
    pub matrix_id: Option<String>,
    /// Output only. The cloud project that owns the test execution.
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    /// Output only. Details about the shard.
    pub shard: Option<Shard>,
    /// Output only. Indicates the current progress of the test execution (e.g., FINISHED).
    pub state: Option<String>,
    /// Output only. Additional details about the running test.
    #[serde(rename = "testDetails")]
    pub test_details: Option<TestDetails>,
    /// Output only. How to run the test.
    #[serde(rename = "testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. The time this test execution was initially created.
    pub timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// Output only. Where the results for this execution are written.
    #[serde(rename = "toolResultsStep")]
    pub tool_results_step: Option<ToolResultsStep>,
}

impl common::Part for TestExecution {}

/// TestMatrix captures all details about a test. It contains the environment configuration, test specification, test executions and overall state and outcome.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [test matrices create projects](ProjectTestMatriceCreateCall) (request|response)
/// * [test matrices get projects](ProjectTestMatriceGetCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestMatrix {
    /// Information about the client which invoked the test.
    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,
    /// Required. The devices the tests are being executed on.
    #[serde(rename = "environmentMatrix")]
    pub environment_matrix: Option<EnvironmentMatrix>,
    /// Output only. Details about why a matrix was deemed invalid. If multiple checks can be safely performed, they will be reported but no assumptions should be made about the length of this list.
    #[serde(rename = "extendedInvalidMatrixDetails")]
    pub extended_invalid_matrix_details: Option<Vec<MatrixErrorDetail>>,
    /// If true, only a single attempt at most will be made to run each execution/shard in the matrix. Flaky test attempts are not affected. Normally, 2 or more attempts are made if a potential infrastructure issue is detected. This feature is for latency sensitive workloads. The incidence of execution failures may be significantly greater for fail-fast matrices and support is more limited because of that expectation.
    #[serde(rename = "failFast")]
    pub fail_fast: Option<bool>,
    /// The number of times a TestExecution should be re-attempted if one or more of its test cases fail for any reason. The maximum number of reruns allowed is 10. Default is 0, which implies no reruns.
    #[serde(rename = "flakyTestAttempts")]
    pub flaky_test_attempts: Option<i32>,
    /// Output only. Describes why the matrix is considered invalid. Only useful for matrices in the INVALID state.
    #[serde(rename = "invalidMatrixDetails")]
    pub invalid_matrix_details: Option<String>,
    /// Output Only. The overall outcome of the test. Only set when the test matrix state is FINISHED.
    #[serde(rename = "outcomeSummary")]
    pub outcome_summary: Option<String>,
    /// The cloud project that owns the test matrix.
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    /// Required. Where the results for the matrix are written.
    #[serde(rename = "resultStorage")]
    pub result_storage: Option<ResultStorage>,
    /// Output only. Indicates the current progress of the test matrix.
    pub state: Option<String>,
    /// Output only. The list of test executions that the service creates for this matrix.
    #[serde(rename = "testExecutions")]
    pub test_executions: Option<Vec<TestExecution>>,
    /// Output only. Unique id set by the service.
    #[serde(rename = "testMatrixId")]
    pub test_matrix_id: Option<String>,
    /// Required. How to run the test.
    #[serde(rename = "testSpecification")]
    pub test_specification: Option<TestSpecification>,
    /// Output only. The time this test matrix was initially created.
    pub timestamp: Option<chrono::DateTime<chrono::offset::Utc>>,
}

impl common::RequestValue for TestMatrix {}
impl common::ResponseResult for TestMatrix {}

/// A description of how to set up the Android device prior to running the test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestSetup {
    /// The device will be logged in on this account for the duration of the test.
    pub account: Option<Account>,
    /// APKs to install in addition to those being directly tested. These will be installed after the app under test. Currently capped at 100.
    #[serde(rename = "additionalApks")]
    pub additional_apks: Option<Vec<Apk>>,
    /// List of directories on the device to upload to GCS at the end of the test; they must be absolute paths under /sdcard, /storage or /data/local/tmp. Path names are restricted to characters a-z A-Z 0-9 _ - . + and / Note: The paths /sdcard and /data will be made available and treated as implicit path substitutions. E.g. if /sdcard on a particular device does not map to external storage, the system will replace it with the external storage path prefix for that device.
    #[serde(rename = "directoriesToPull")]
    pub directories_to_pull: Option<Vec<String>>,
    /// Whether to prevent all runtime permissions to be granted at app install
    #[serde(rename = "dontAutograntPermissions")]
    pub dont_autogrant_permissions: Option<bool>,
    /// Environment variables to set for the test (only applicable for instrumentation tests).
    #[serde(rename = "environmentVariables")]
    pub environment_variables: Option<Vec<EnvironmentVariable>>,
    /// List of files to push to the device before starting the test.
    #[serde(rename = "filesToPush")]
    pub files_to_push: Option<Vec<DeviceFile>>,
    /// Optional. Initial setup APKs to install before the app under test is installed. Currently capped at 100.
    #[serde(rename = "initialSetupApks")]
    pub initial_setup_apks: Option<Vec<Apk>>,
    /// The network traffic profile used for running the test. Available network profiles can be queried by using the NETWORK_CONFIGURATION environment type when calling TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[serde(rename = "networkProfile")]
    pub network_profile: Option<String>,
    /// Systrace configuration for the run. Deprecated: Systrace used Python 2 which was sunsetted on 2020-01-01. Systrace is no longer supported in the Cloud Testing API, and no Systrace file will be provided in the results.
    pub systrace: Option<SystraceSetup>,
}

impl common::Part for TestSetup {}

/// A description of how to run the test.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestSpecification {
    /// An Android instrumentation test.
    #[serde(rename = "androidInstrumentationTest")]
    pub android_instrumentation_test: Option<AndroidInstrumentationTest>,
    /// An Android robo test.
    #[serde(rename = "androidRoboTest")]
    pub android_robo_test: Option<AndroidRoboTest>,
    /// An Android Application with a Test Loop.
    #[serde(rename = "androidTestLoop")]
    pub android_test_loop: Option<AndroidTestLoop>,
    /// Disables performance metrics recording. May reduce test latency.
    #[serde(rename = "disablePerformanceMetrics")]
    pub disable_performance_metrics: Option<bool>,
    /// Disables video recording. May reduce test latency.
    #[serde(rename = "disableVideoRecording")]
    pub disable_video_recording: Option<bool>,
    /// An iOS Robo test.
    #[serde(rename = "iosRoboTest")]
    pub ios_robo_test: Option<IosRoboTest>,
    /// An iOS application with a test loop.
    #[serde(rename = "iosTestLoop")]
    pub ios_test_loop: Option<IosTestLoop>,
    /// Test setup requirements for iOS.
    #[serde(rename = "iosTestSetup")]
    pub ios_test_setup: Option<IosTestSetup>,
    /// An iOS XCTest, via an .xctestrun file.
    #[serde(rename = "iosXcTest")]
    pub ios_xc_test: Option<IosXcTest>,
    /// Test setup requirements for Android e.g. files to install, bootstrap scripts.
    #[serde(rename = "testSetup")]
    pub test_setup: Option<TestSetup>,
    /// Max time a test execution is allowed to run before it is automatically cancelled. The default value is 5 min.
    #[serde(rename = "testTimeout")]
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub test_timeout: Option<chrono::Duration>,
}

impl common::Part for TestSpecification {}

/// Test targets for a shard.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TestTargetsForShard {
    /// Group of packages, classes, and/or test methods to be run for each shard. The targets need to be specified in AndroidJUnitRunner argument format. For example, "package com.my.packages" "class com.my.package.MyClass". The number of test_targets must be greater than 0.
    #[serde(rename = "testTargets")]
    pub test_targets: Option<Vec<String>>,
}

impl common::Part for TestTargetsForShard {}

/// Represents a tool results execution resource. This has the results of a TestMatrix.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolResultsExecution {
    /// Output only. A tool results execution ID.
    #[serde(rename = "executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename = "historyId")]
    pub history_id: Option<String>,
    /// Output only. The cloud project that owns the tool results execution.
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
}

impl common::Part for ToolResultsExecution {}

/// Represents a tool results history resource.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolResultsHistory {
    /// Required. A tool results history ID.
    #[serde(rename = "historyId")]
    pub history_id: Option<String>,
    /// Required. The cloud project that owns the tool results history.
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
}

impl common::Part for ToolResultsHistory {}

/// Represents a tool results step resource. This has the results of a TestExecution.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolResultsStep {
    /// Output only. A tool results execution ID.
    #[serde(rename = "executionId")]
    pub execution_id: Option<String>,
    /// Output only. A tool results history ID.
    #[serde(rename = "historyId")]
    pub history_id: Option<String>,
    /// Output only. The cloud project that owns the tool results step.
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    /// Output only. A tool results step ID.
    #[serde(rename = "stepId")]
    pub step_id: Option<String>,
}

impl common::Part for ToolResultsStep {}

/// Network emulation parameters.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrafficRule {
    /// Bandwidth in kbits/second.
    pub bandwidth: Option<f32>,
    /// Burst size in kbits.
    pub burst: Option<f32>,
    /// Packet delay, must be >= 0.
    #[serde_as(as = "Option<common::serde::duration::Wrapper>")]
    pub delay: Option<chrono::Duration>,
    /// Packet duplication ratio (0.0 - 1.0).
    #[serde(rename = "packetDuplicationRatio")]
    pub packet_duplication_ratio: Option<f32>,
    /// Packet loss ratio (0.0 - 1.0).
    #[serde(rename = "packetLossRatio")]
    pub packet_loss_ratio: Option<f32>,
}

impl common::Part for TrafficRule {}

/// Uniformly shards test cases given a total number of shards. For instrumentation tests, it will be translated to "-e numShard" and "-e shardIndex" AndroidJUnitRunner arguments. With uniform sharding enabled, specifying either of these sharding arguments via `environment_variables` is invalid. Based on the sharding mechanism AndroidJUnitRunner uses, there is no guarantee that test cases will be distributed uniformly across all shards.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UniformSharding {
    /// Required. The total number of shards to create. This must always be a positive number that is no greater than the total number of test cases. When you select one or more physical devices, the number of shards must be <= 50. When you select one or more ARM virtual devices, it must be <= 200. When you select only x86 virtual devices, it must be <= 500.
    #[serde(rename = "numShards")]
    pub num_shards: Option<i32>,
}

impl common::Part for UniformSharding {}

/// A tag within a manifest. https://developer.android.com/guide/topics/manifest/uses-feature-element.html
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsesFeature {
    /// The android:required value
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    /// The android:name value
    pub name: Option<String>,
}

impl common::Part for UsesFeature {}

/// An Xcode version that an iOS version is compatible with.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct XcodeVersion {
    /// Tags for this Xcode version. Example: "default".
    pub tags: Option<Vec<String>>,
    /// The id for this version. Example: "9.2".
    pub version: Option<String>,
}

impl common::Part for XcodeVersion {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *applicationDetailService* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
///
/// # async fn dox() {
/// use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Testing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_apk_details(...)`
/// // to build up your call.
/// let rb = hub.application_detail_service();
/// # }
/// ```
pub struct ApplicationDetailServiceMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
}

impl<'a, C> common::MethodsBuilder for ApplicationDetailServiceMethods<'a, C> {}

impl<'a, C> ApplicationDetailServiceMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the details of an Android application APK.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn get_apk_details(
        &self,
        request: FileReference,
    ) -> ApplicationDetailServiceGetApkDetailCall<'a, C> {
        ApplicationDetailServiceGetApkDetailCall {
            hub: self.hub,
            _request: request,
            _bundle_location_gcs_path: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
///
/// # async fn dox() {
/// use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Testing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `device_sessions_cancel(...)`, `device_sessions_create(...)`, `device_sessions_get(...)`, `device_sessions_list(...)`, `device_sessions_patch(...)`, `test_matrices_cancel(...)`, `test_matrices_create(...)` and `test_matrices_get(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
}

impl<'a, C> common::MethodsBuilder for ProjectMethods<'a, C> {}

impl<'a, C> ProjectMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// POST /v1/projects/{project_id}/deviceSessions/{device_session_id}:cancel Changes the DeviceSession to state FINISHED and terminates all connections. Canceled sessions are not deleted and can be retrieved or listed by the user until they expire based on the 28 day deletion policy.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_cancel(
        &self,
        request: CancelDeviceSessionRequest,
        name: &str,
    ) -> ProjectDeviceSessionCancelCall<'a, C> {
        ProjectDeviceSessionCancelCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// POST /v1/projects/{project_id}/deviceSessions
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `parent` - Required. The Compute Engine project under which this device will be allocated. "projects/{project_id}"
    pub fn device_sessions_create(
        &self,
        request: DeviceSession,
        parent: &str,
    ) -> ProjectDeviceSessionCreateCall<'a, C> {
        ProjectDeviceSessionCreateCall {
            hub: self.hub,
            _request: request,
            _parent: parent.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// GET /v1/projects/{project_id}/deviceSessions/{device_session_id} Return a DeviceSession, which documents the allocation status and whether the device is allocated. Clients making requests from this API must poll GetDeviceSession.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_get(&self, name: &str) -> ProjectDeviceSessionGetCall<'a, C> {
        ProjectDeviceSessionGetCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// GET /v1/projects/{project_id}/deviceSessions Lists device Sessions owned by the project user.
    ///
    /// # Arguments
    ///
    /// * `parent` - Required. The name of the parent to request, e.g. "projects/{project_id}"
    pub fn device_sessions_list(&self, parent: &str) -> ProjectDeviceSessionListCall<'a, C> {
        ProjectDeviceSessionListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// PATCH /v1/projects/{projectId}/deviceSessions/deviceSessionId}:updateDeviceSession Updates the current device session to the fields described by the update_mask.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    pub fn device_sessions_patch(
        &self,
        request: DeviceSession,
        name: &str,
    ) -> ProjectDeviceSessionPatchCall<'a, C> {
        ProjectDeviceSessionPatchCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    ///
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test.
    /// * `testMatrixId` - Test matrix that will be canceled.
    pub fn test_matrices_cancel(
        &self,
        project_id: &str,
        test_matrix_id: &str,
    ) -> ProjectTestMatriceCancelCall<'a, C> {
        ProjectTestMatriceCancelCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. The returned matrix will not yet contain the executions that will be created for this matrix. Execution creation happens later on and will require a call to GetTestMatrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `projectId` - The GCE project under which this job will run.
    pub fn test_matrices_create(
        &self,
        request: TestMatrix,
        project_id: &str,
    ) -> ProjectTestMatriceCreateCall<'a, C> {
        ProjectTestMatriceCreateCall {
            hub: self.hub,
            _request: request,
            _project_id: project_id.to_string(),
            _request_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Checks the status of a test matrix and the executions once they are created. The test matrix will contain the list of test executions to run if and only if the resultStorage.toolResultsExecution fields have been populated. Note: Flaky test executions may be added to the matrix at a later stage. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
    ///
    /// # Arguments
    ///
    /// * `projectId` - Cloud project that owns the test matrix.
    /// * `testMatrixId` - Unique test matrix id which was assigned by the service.
    pub fn test_matrices_get(
        &self,
        project_id: &str,
        test_matrix_id: &str,
    ) -> ProjectTestMatriceGetCall<'a, C> {
        ProjectTestMatriceGetCall {
            hub: self.hub,
            _project_id: project_id.to_string(),
            _test_matrix_id: test_matrix_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

/// A builder providing access to all methods supported on *testEnvironmentCatalog* resources.
/// It is not used directly, but through the [`Testing`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_testing1 as testing1;
///
/// # async fn dox() {
/// use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///     secret,
///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
///         .enable_http1()
///         .build()
/// );
/// let mut hub = Testing::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.test_environment_catalog();
/// # }
/// ```
pub struct TestEnvironmentCatalogMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
}

impl<'a, C> common::MethodsBuilder for TestEnvironmentCatalogMethods<'a, C> {}

impl<'a, C> TestEnvironmentCatalogMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
    ///
    /// # Arguments
    ///
    /// * `environmentType` - Required. The type of environment that should be listed.
    pub fn get(&self, environment_type: &str) -> TestEnvironmentCatalogGetCall<'a, C> {
        TestEnvironmentCatalogGetCall {
            hub: self.hub,
            _environment_type: environment_type.to_string(),
            _project_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Gets the details of an Android application APK.
///
/// A builder for the *getApkDetails* method supported by a *applicationDetailService* resource.
/// It is not used directly, but through a [`ApplicationDetailServiceMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::FileReference;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = FileReference::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.application_detail_service().get_apk_details(req)
///              .bundle_location_gcs_path("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct ApplicationDetailServiceGetApkDetailCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _request: FileReference,
    _bundle_location_gcs_path: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ApplicationDetailServiceGetApkDetailCall<'a, C> {}

impl<'a, C> ApplicationDetailServiceGetApkDetailCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, GetApkDetailsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.applicationDetailService.getApkDetails",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "bundleLocation.gcsPath"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        if let Some(value) = self._bundle_location_gcs_path.as_ref() {
            params.push("bundleLocation.gcsPath", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/applicationDetailService/getApkDetails";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
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
    pub fn request(
        mut self,
        new_value: FileReference,
    ) -> ApplicationDetailServiceGetApkDetailCall<'a, C> {
        self._request = new_value;
        self
    }
    /// A path to a file in Google Cloud Storage. Example: gs://build-app-1414623860166/app%40debug-unaligned.apk These paths are expected to be url encoded (percent encoding)
    ///
    /// Sets the *bundle location.gcs path* query property to the given value.
    pub fn bundle_location_gcs_path(
        mut self,
        new_value: &str,
    ) -> ApplicationDetailServiceGetApkDetailCall<'a, C> {
        self._bundle_location_gcs_path = Some(new_value.to_string());
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
    ) -> ApplicationDetailServiceGetApkDetailCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ApplicationDetailServiceGetApkDetailCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ApplicationDetailServiceGetApkDetailCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ApplicationDetailServiceGetApkDetailCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ApplicationDetailServiceGetApkDetailCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// POST /v1/projects/{project_id}/deviceSessions/{device_session_id}:cancel Changes the DeviceSession to state FINISHED and terminates all connections. Canceled sessions are not deleted and can be retrieved or listed by the user until they expire based on the 28 day deletion policy.
///
/// A builder for the *deviceSessions.cancel* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::CancelDeviceSessionRequest;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CancelDeviceSessionRequest::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_cancel(req, "name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeviceSessionCancelCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _request: CancelDeviceSessionRequest,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeviceSessionCancelCall<'a, C> {}

impl<'a, C> ProjectDeviceSessionCancelCall<'a, C>
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
            id: "testing.projects.deviceSessions.cancel",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}:cancel";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
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
    pub fn request(
        mut self,
        new_value: CancelDeviceSessionRequest,
    ) -> ProjectDeviceSessionCancelCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectDeviceSessionCancelCall<'a, C> {
        self._name = new_value.to_string();
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
    ) -> ProjectDeviceSessionCancelCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeviceSessionCancelCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeviceSessionCancelCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeviceSessionCancelCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectDeviceSessionCancelCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// POST /v1/projects/{project_id}/deviceSessions
///
/// A builder for the *deviceSessions.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::DeviceSession;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DeviceSession::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_create(req, "parent")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeviceSessionCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _request: DeviceSession,
    _parent: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeviceSessionCreateCall<'a, C> {}

impl<'a, C> ProjectDeviceSessionCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DeviceSession)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.deviceSessions.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "parent"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("parent", self._parent);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+parent}/deviceSessions";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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
    pub fn request(mut self, new_value: DeviceSession) -> ProjectDeviceSessionCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The Compute Engine project under which this device will be allocated. "projects/{project_id}"
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDeviceSessionCreateCall<'a, C> {
        self._parent = new_value.to_string();
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
    ) -> ProjectDeviceSessionCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeviceSessionCreateCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeviceSessionCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeviceSessionCreateCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectDeviceSessionCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// GET /v1/projects/{project_id}/deviceSessions/{device_session_id} Return a DeviceSession, which documents the allocation status and whether the device is allocated. Clients making requests from this API must poll GetDeviceSession.
///
/// A builder for the *deviceSessions.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_get("name")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeviceSessionGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeviceSessionGetCall<'a, C> {}

impl<'a, C> ProjectDeviceSessionGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DeviceSession)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.deviceSessions.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "name"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("name", self._name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
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

    /// Required. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectDeviceSessionGetCall<'a, C> {
        self._name = new_value.to_string();
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
    ) -> ProjectDeviceSessionGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeviceSessionGetCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeviceSessionGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeviceSessionGetCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectDeviceSessionGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// GET /v1/projects/{project_id}/deviceSessions Lists device Sessions owned by the project user.
///
/// A builder for the *deviceSessions.list* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_list("parent")
///              .page_token("amet.")
///              .page_size(-59)
///              .filter("amet.")
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeviceSessionListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeviceSessionListCall<'a, C> {}

impl<'a, C> ProjectDeviceSessionListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, ListDeviceSessionsResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.deviceSessions.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "parent", "pageToken", "pageSize", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        params.push("parent", self._parent);
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+parent}/deviceSessions";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["parent"];
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

    /// Required. The name of the parent to request, e.g. "projects/{project_id}"
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectDeviceSessionListCall<'a, C> {
        self._parent = new_value.to_string();
        self
    }
    /// Optional. A continuation token for paging.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectDeviceSessionListCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Optional. The maximum number of DeviceSessions to return.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectDeviceSessionListCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// Optional. If specified, responses will be filtered by the given filter. Allowed fields are: session_state.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> ProjectDeviceSessionListCall<'a, C> {
        self._filter = Some(new_value.to_string());
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
    ) -> ProjectDeviceSessionListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeviceSessionListCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeviceSessionListCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeviceSessionListCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectDeviceSessionListCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// PATCH /v1/projects/{projectId}/deviceSessions/deviceSessionId}:updateDeviceSession Updates the current device session to the fields described by the update_mask.
///
/// A builder for the *deviceSessions.patch* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::DeviceSession;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DeviceSession::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().device_sessions_patch(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct ProjectDeviceSessionPatchCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _request: DeviceSession,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectDeviceSessionPatchCall<'a, C> {}

impl<'a, C> ProjectDeviceSessionPatchCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, DeviceSession)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.deviceSessions.patch",
            http_method: hyper::Method::PATCH,
        });

        for &field in ["alt", "name", "updateMask"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("name", self._name);
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+name}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{+name}", "name")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["name"];
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
                    .method(hyper::Method::PATCH)
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
    pub fn request(mut self, new_value: DeviceSession) -> ProjectDeviceSessionPatchCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Optional. Name of the DeviceSession, e.g. "projects/{project_id}/deviceSessions/{session_id}"
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> ProjectDeviceSessionPatchCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// Required. The list of fields to update.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(
        mut self,
        new_value: common::FieldMask,
    ) -> ProjectDeviceSessionPatchCall<'a, C> {
        self._update_mask = Some(new_value);
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
    ) -> ProjectDeviceSessionPatchCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectDeviceSessionPatchCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectDeviceSessionPatchCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectDeviceSessionPatchCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectDeviceSessionPatchCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Cancels unfinished test executions in a test matrix. This call returns immediately and cancellation proceeds asynchronously. If the matrix is already final, this operation will have no effect. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.cancel* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_cancel("projectId", "testMatrixId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceCancelCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectTestMatriceCancelCall<'a, C> {}

impl<'a, C> ProjectTestMatriceCancelCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, CancelTestMatrixResponse)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.testMatrices.cancel",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("testMatrixId", self._test_matrix_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone()
            + "v1/projects/{projectId}/testMatrices/{testMatrixId}:cancel";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{projectId}", "projectId"),
            ("{testMatrixId}", "testMatrixId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["testMatrixId", "projectId"];
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
                    .method(hyper::Method::POST)
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

    /// Cloud project that owns the test.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, C> {
        self._project_id = new_value.to_string();
        self
    }
    /// Test matrix that will be canceled.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceCancelCall<'a, C> {
        self._test_matrix_id = new_value.to_string();
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
    ) -> ProjectTestMatriceCancelCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCancelCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectTestMatriceCancelCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectTestMatriceCancelCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectTestMatriceCancelCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Creates and runs a matrix of tests according to the given specifications. Unsupported environments will be returned in the state UNSUPPORTED. A test matrix is limited to use at most 2000 devices in parallel. The returned matrix will not yet contain the executions that will be created for this matrix. Execution creation happens later on and will require a call to GetTestMatrix. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to write to project - INVALID_ARGUMENT - if the request is malformed or if the matrix tries to use too many simultaneous devices.
///
/// A builder for the *testMatrices.create* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// use testing1::api::TestMatrix;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = TestMatrix::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_create(req, "projectId")
///              .request_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceCreateCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _request: TestMatrix,
    _project_id: String,
    _request_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectTestMatriceCreateCall<'a, C> {}

impl<'a, C> ProjectTestMatriceCreateCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, TestMatrix)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.testMatrices.create",
            http_method: hyper::Method::POST,
        });

        for &field in ["alt", "projectId", "requestId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("projectId", self._project_id);
        if let Some(value) = self._request_id.as_ref() {
            params.push("requestId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{projectId}", "projectId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["projectId"];
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
    pub fn request(mut self, new_value: TestMatrix) -> ProjectTestMatriceCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// The GCE project under which this job will run.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, C> {
        self._project_id = new_value.to_string();
        self
    }
    /// A string id used to detect duplicated requests. Ids are automatically scoped to a project, so users should ensure the ID is unique per-project. A UUID is recommended. Optional, but strongly recommended.
    ///
    /// Sets the *request id* query property to the given value.
    pub fn request_id(mut self, new_value: &str) -> ProjectTestMatriceCreateCall<'a, C> {
        self._request_id = Some(new_value.to_string());
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
    ) -> ProjectTestMatriceCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceCreateCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectTestMatriceCreateCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectTestMatriceCreateCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectTestMatriceCreateCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Checks the status of a test matrix and the executions once they are created. The test matrix will contain the list of test executions to run if and only if the resultStorage.toolResultsExecution fields have been populated. Note: Flaky test executions may be added to the matrix at a later stage. May return any of the following canonical error codes: - PERMISSION_DENIED - if the user is not authorized to read project - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the Test Matrix does not exist
///
/// A builder for the *testMatrices.get* method supported by a *project* resource.
/// It is not used directly, but through a [`ProjectMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().test_matrices_get("projectId", "testMatrixId")
///              .doit().await;
/// # }
/// ```
pub struct ProjectTestMatriceGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _project_id: String,
    _test_matrix_id: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for ProjectTestMatriceGetCall<'a, C> {}

impl<'a, C> ProjectTestMatriceGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, TestMatrix)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.projects.testMatrices.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "projectId", "testMatrixId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("projectId", self._project_id);
        params.push("testMatrixId", self._test_matrix_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url =
            self.hub._base_url.clone() + "v1/projects/{projectId}/testMatrices/{testMatrixId}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [
            ("{projectId}", "projectId"),
            ("{testMatrixId}", "testMatrixId"),
        ]
        .iter()
        {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["testMatrixId", "projectId"];
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

    /// Cloud project that owns the test matrix.
    ///
    /// Sets the *project id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn project_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, C> {
        self._project_id = new_value.to_string();
        self
    }
    /// Unique test matrix id which was assigned by the service.
    ///
    /// Sets the *test matrix id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn test_matrix_id(mut self, new_value: &str) -> ProjectTestMatriceGetCall<'a, C> {
        self._test_matrix_id = new_value.to_string();
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
    ) -> ProjectTestMatriceGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectTestMatriceGetCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> ProjectTestMatriceGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> ProjectTestMatriceGetCall<'a, C>
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
    pub fn clear_scopes(mut self) -> ProjectTestMatriceGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Gets the catalog of supported test environments. May return any of the following canonical error codes: - INVALID_ARGUMENT - if the request is malformed - NOT_FOUND - if the environment type does not exist - INTERNAL - if an internal error occurred
///
/// A builder for the *get* method supported by a *testEnvironmentCatalog* resource.
/// It is not used directly, but through a [`TestEnvironmentCatalogMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_testing1 as testing1;
/// # async fn dox() {
/// # use testing1::{Testing, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
///
/// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #     secret,
/// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
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
/// #         .enable_http1()
/// #         .build()
/// # );
/// # let mut hub = Testing::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.test_environment_catalog().get("environmentType")
///              .project_id("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct TestEnvironmentCatalogGetCall<'a, C>
where
    C: 'a,
{
    hub: &'a Testing<C>,
    _environment_type: String,
    _project_id: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for TestEnvironmentCatalogGetCall<'a, C> {}

impl<'a, C> TestEnvironmentCatalogGetCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, TestEnvironmentCatalog)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "testing.testEnvironmentCatalog.get",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "environmentType", "projectId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("environmentType", self._environment_type);
        if let Some(value) = self._project_id.as_ref() {
            params.push("projectId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/testEnvironmentCatalog/{environmentType}";
        if self._scopes.is_empty() {
            self._scopes
                .insert(Scope::CloudPlatform.as_ref().to_string());
        }

        #[allow(clippy::single_element_loop)]
        for &(find_this, param_name) in [("{environmentType}", "environmentType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["environmentType"];
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

    /// Required. The type of environment that should be listed.
    ///
    /// Sets the *environment type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn environment_type(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, C> {
        self._environment_type = new_value.to_string();
        self
    }
    /// For authorization, the cloud project requesting the TestEnvironmentCatalog.
    ///
    /// Sets the *project id* query property to the given value.
    pub fn project_id(mut self, new_value: &str) -> TestEnvironmentCatalogGetCall<'a, C> {
        self._project_id = Some(new_value.to_string());
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
    ) -> TestEnvironmentCatalogGetCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> TestEnvironmentCatalogGetCall<'a, C>
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
    /// [`Scope::CloudPlatform`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> TestEnvironmentCatalogGetCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> TestEnvironmentCatalogGetCall<'a, C>
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
    pub fn clear_scopes(mut self) -> TestEnvironmentCatalogGetCall<'a, C> {
        self._scopes.clear();
        self
    }
}
