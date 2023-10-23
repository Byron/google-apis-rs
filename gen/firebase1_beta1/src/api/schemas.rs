use super::*;
/// All fields are required.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add firebase projects](ProjectAddFirebaseCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddFirebaseRequest {
    /// Deprecated. Instead, to set a Project’s default GCP resource location, call [`FinalizeDefaultLocation`](https://firebase.google.com/../projects.defaultLocation/finalize) after you add Firebase resources to the GCP `Project`. The ID of the Project’s default GCP resource location. The location must be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations).
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::RequestValue for AddFirebaseRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [add google analytics projects](ProjectAddGoogleAnalyticCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AddGoogleAnalyticsRequest {
    /// The ID for the existing [Google Analytics account](http://www.google.com/analytics/) that you want to link with the `FirebaseProject`. Specifying this field will provision a new Google Analytics property in your Google Analytics account and associate the new property with the `FirebaseProject`.
    #[serde(rename="analyticsAccountId")]
    
    pub analytics_account_id: Option<String>,
    /// The ID for the existing Google Analytics property that you want to associate with the `FirebaseProject`.
    #[serde(rename="analyticsPropertyId")]
    
    pub analytics_property_id: Option<String>,
}

impl client::RequestValue for AddGoogleAnalyticsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get admin sdk config projects](ProjectGetAdminSdkConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AdminSdkConfig {
    /// The default Firebase Realtime Database URL.
    #[serde(rename="databaseURL")]
    
    pub database_url: Option<String>,
    /// The ID of the Project’s default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project’s default GCP resource location, call [`FinalizeDefaultLocation`](https://firebase.google.com/../projects.defaultLocation/finalize) after you add Firebase resources to the Project.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Immutable. A user-assigned unique identifier for the `FirebaseProject`. This identifier may appear in URLs or names for some Firebase resources associated with the Project, but it should generally be treated as a convenience alias to reference the Project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The default Cloud Storage for Firebase storage bucket name.
    #[serde(rename="storageBucket")]
    
    pub storage_bucket: Option<String>,
}

impl client::ResponseResult for AdminSdkConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get analytics details projects](ProjectGetAnalyticsDetailCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsDetails {
    /// The Analytics Property object associated with the specified `FirebaseProject`. This object contains the details of the Google Analytics property associated with the Project.
    #[serde(rename="analyticsProperty")]
    
    pub analytics_property: Option<AnalyticsProperty>,
    ///  - For `AndroidApps` and `IosApps`: a map of `app` to `streamId` for each Firebase App in the specified `FirebaseProject`. Each `app` and `streamId` appears only once. - For `WebApps`: a map of `app` to `streamId` and `measurementId` for each `WebApp` in the specified `FirebaseProject`. Each `app`, `streamId`, and `measurementId` appears only once.
    #[serde(rename="streamMappings")]
    
    pub stream_mappings: Option<Vec<StreamMapping>>,
}

impl client::ResponseResult for AnalyticsDetails {}


/// Details of a Google Analytics property
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AnalyticsProperty {
    /// Output only. The ID of the [Google Analytics account](https://www.google.com/analytics/) for the Google Analytics property associated with the specified FirebaseProject.
    #[serde(rename="analyticsAccountId")]
    
    pub analytics_account_id: Option<String>,
    /// The display name of the Google Analytics property associated with the specified `FirebaseProject`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The globally unique, Google-assigned identifier of the Google Analytics property associated with the specified `FirebaseProject`. If you called [`AddGoogleAnalytics`](https://firebase.google.com/../../v1beta1/projects/addGoogleAnalytics) to link the `FirebaseProject` with a Google Analytics account, the value in this `id` field is the same as the ID of the property either specified or provisioned with that call to `AddGoogleAnalytics`.
    
    pub id: Option<String>,
}

impl client::Part for AnalyticsProperty {}


/// Details of a Firebase App for Android.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps create projects](ProjectAndroidAppCreateCall) (request)
/// * [android apps get projects](ProjectAndroidAppGetCall) (response)
/// * [android apps patch projects](ProjectAndroidAppPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidApp {
    /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `AndroidApp`. Be aware that this value is the UID of the API key, *not* the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App’s [configuration artifact](https://firebase.google.com/../../rest/v1beta1/projects.androidApps/getConfig). If `api_key_id` is not set in requests to [`androidApps.Create`](https://firebase.google.com/../../rest/v1beta1/projects.androidApps/create), then Firebase automatically associates an `api_key_id` with the `AndroidApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `AndroidApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys.
    #[serde(rename="apiKeyId")]
    
    pub api_key_id: Option<String>,
    /// Output only. Immutable. The globally unique, Firebase-assigned identifier for the `AndroidApp`. This identifier should be treated as an opaque token, as the data format is not specified.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The user-assigned display name for the `AndroidApp`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated.
    
    pub etag: Option<String>,
    /// The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.app_id)).
    
    pub name: Option<String>,
    /// Immutable. The canonical package name of the Android app as would appear in the Google Play Developer Console.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `AndroidApp`.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The SHA1 certificate hashes for the AndroidApp.
    #[serde(rename="sha1Hashes")]
    
    pub sha1_hashes: Option<Vec<String>>,
    /// The SHA256 certificate hashes for the AndroidApp.
    #[serde(rename="sha256Hashes")]
    
    pub sha256_hashes: Option<Vec<String>>,
    /// Output only. The lifecycle state of the App.
    
    pub state: Option<AndroidAppStateEnum>,
}

impl client::RequestValue for AndroidApp {}
impl client::ResponseResult for AndroidApp {}


/// Configuration metadata of a single Firebase App for Android.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps get config projects](ProjectAndroidAppGetConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AndroidAppConfig {
    /// The contents of the JSON configuration file.
    #[serde(rename="configFileContents")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub config_file_contents: Option<Vec<u8>>,
    /// The filename that the configuration artifact for the `AndroidApp` is typically saved as. For example: `google-services.json`
    #[serde(rename="configFilename")]
    
    pub config_filename: Option<String>,
}

impl client::ResponseResult for AndroidAppConfig {}


/// The default resources associated with the Project.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DefaultResources {
    /// Output only. The default Firebase Hosting site name, in the format: PROJECT_ID Though rare, your `projectId` might already be used as the name for an existing Hosting site in another project (learn more about creating non-default, [additional sites](https://firebase.google.com/docs/hosting/multisites)). In these cases, your `projectId` is appended with a hyphen then five alphanumeric characters to create your default Hosting site name. For example, if your `projectId` is `myproject123`, your default Hosting site name might be: `myproject123-a5c16`
    #[serde(rename="hostingSite")]
    
    pub hosting_site: Option<String>,
    /// Output only. The ID of the Project’s default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project’s default GCP resource location, call [`FinalizeDefaultLocation`](https://firebase.google.com/../projects.defaultLocation/finalize) after you add Firebase resources to the Project.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Output only. The default Firebase Realtime Database instance name, in the format: PROJECT_ID Though rare, your `projectId` might already be used as the name for an existing Realtime Database instance in another project (learn more about [database sharding](https://firebase.google.com/docs/database/usage/sharding)). In these cases, your `projectId` is appended with a hyphen then five alphanumeric characters to create your default Realtime Database instance name. For example, if your `projectId` is `myproject123`, your default database instance name might be: `myproject123-a5c16`
    #[serde(rename="realtimeDatabaseInstance")]
    
    pub realtime_database_instance: Option<String>,
    /// Output only. The default Cloud Storage for Firebase storage bucket, in the format: PROJECT_ID.appspot.com
    #[serde(rename="storageBucket")]
    
    pub storage_bucket: Option<String>,
}

impl client::Part for DefaultResources {}


/// A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); }
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps sha delete projects](ProjectAndroidAppShaDeleteCall) (response)
/// * [remove analytics projects](ProjectRemoveAnalyticCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl client::ResponseResult for Empty {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [default location finalize projects](ProjectDefaultLocationFinalizeCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FinalizeDefaultLocationRequest {
    /// The ID of the Project's default GCP resource location. The location must be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations).
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
}

impl client::RequestValue for FinalizeDefaultLocationRequest {}


/// A high-level summary of an App.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirebaseAppInfo {
    /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the App. Be aware that this value is the UID of the API key, *not* the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App’s configuration artifact ([`AndroidApp`](https://firebase.google.com/../../rest/v1beta1/projects.androidApps/getConfig) | [`IosApp`](https://firebase.google.com/../../rest/v1beta1/projects.iosApps/getConfig) | [`WebApp`](https://firebase.google.com/../../rest/v1beta1/projects.webApps/getConfig)). If `api_key_id` is not set in requests to create the App ([`AndroidApp`](https://firebase.google.com/../../rest/v1beta1/projects.androidApps/create) | [`IosApp`](https://firebase.google.com/../../rest/v1beta1/projects.iosApps/create) | [`WebApp`](https://firebase.google.com/../../rest/v1beta1/projects.webApps/create)), then Firebase automatically associates an `api_key_id` with the App. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned.
    #[serde(rename="apiKeyId")]
    
    pub api_key_id: Option<String>,
    /// Output only. Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The user-assigned display name of the Firebase App.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The resource name of the Firebase App, in the format: projects/PROJECT_ID /iosApps/APP_ID or projects/PROJECT_ID/androidApps/APP_ID or projects/ PROJECT_ID/webApps/APP_ID
    
    pub name: Option<String>,
    /// Output only. Immutable. The platform-specific identifier of the App. *Note:* For most use cases, use `appId`, which is the canonical, globally unique identifier for referencing an App. This string is derived from a native identifier for each platform: `packageName` for an `AndroidApp`, `bundleId` for an `IosApp`, and `webId` for a `WebApp`. Its contents should be treated as opaque, as the native identifier format may change as platforms evolve. This string is only unique within a `FirebaseProject` and its associated Apps.
    
    pub namespace: Option<String>,
    /// The platform of the Firebase App.
    
    pub platform: Option<FirebaseAppInfoPlatformEnum>,
    /// Output only. The lifecycle state of the App.
    
    pub state: Option<FirebaseAppInfoStateEnum>,
}

impl client::Part for FirebaseAppInfo {}


/// A `FirebaseProject` is the top-level Firebase entity. It is the container for Firebase Apps, Firebase Hosting sites, storage systems (Firebase Realtime Database, Cloud Firestore, Cloud Storage buckets), and other Firebase and Google Cloud Platform (GCP) resources. You create a `FirebaseProject` by calling AddFirebase and specifying an *existing* [GCP `Project`](https://cloud.google.com/resource-manager/reference/rest/v1/projects). This adds Firebase resources to the existing GCP `Project`. Since a FirebaseProject is actually also a GCP `Project`, a `FirebaseProject` has the same underlying GCP identifiers (`projectNumber` and `projectId`). This allows for easy interop with Google APIs.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get projects](ProjectGetCall) (response)
/// * [patch projects](ProjectPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FirebaseProject {
    /// A set of user-defined annotations for the FirebaseProject. Learn more about annotations in Google's [AIP-128 standard](https://google.aip.dev/128#annotations). These annotations are intended solely for developers and client-side tools. Firebase services will not mutate this annotations set.
    
    pub annotations: Option<HashMap<String, String>>,
    /// The user-assigned display name of the Project.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated.
    
    pub etag: Option<String>,
    /// The resource name of the Project, in the format: projects/PROJECT_IDENTIFIER PROJECT_IDENTIFIER: the Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`.
    
    pub name: Option<String>,
    /// Output only. Immutable. A user-assigned unique identifier for the Project. This identifier may appear in URLs or names for some Firebase resources associated with the Project, but it should generally be treated as a convenience alias to reference the Project.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. Immutable. The globally unique, Google-assigned canonical identifier for the Project. Use this identifier when configuring integrations and/or making API calls to Firebase or third-party services.
    #[serde(rename="projectNumber")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub project_number: Option<i64>,
    /// Output only. The default Firebase resources associated with the Project.
    
    pub resources: Option<DefaultResources>,
    /// Output only. The lifecycle state of the Project.
    
    pub state: Option<FirebaseProjectStateEnum>,
}

impl client::RequestValue for FirebaseProject {}
impl client::ResponseResult for FirebaseProject {}


/// Details of a Firebase App for iOS.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ios apps create projects](ProjectIosAppCreateCall) (request)
/// * [ios apps get projects](ProjectIosAppGetCall) (response)
/// * [ios apps patch projects](ProjectIosAppPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosApp {
    /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `IosApp`. Be aware that this value is the UID of the API key, *not* the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App’s [configuration artifact](https://firebase.google.com/../../rest/v1beta1/projects.iosApps/getConfig). If `api_key_id` is not set in requests to [`iosApps.Create`](https://firebase.google.com/../../rest/v1beta1/projects.iosApps/create), then Firebase automatically associates an `api_key_id` with the `IosApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `IosApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys.
    #[serde(rename="apiKeyId")]
    
    pub api_key_id: Option<String>,
    /// Output only. Immutable. The globally unique, Firebase-assigned identifier for the `IosApp`. This identifier should be treated as an opaque token, as the data format is not specified.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The automatically generated Apple ID assigned to the iOS app by Apple in the iOS App Store.
    #[serde(rename="appStoreId")]
    
    pub app_store_id: Option<String>,
    /// Immutable. The canonical bundle ID of the iOS app as it would appear in the iOS AppStore.
    #[serde(rename="bundleId")]
    
    pub bundle_id: Option<String>,
    /// The user-assigned display name for the `IosApp`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated.
    
    pub etag: Option<String>,
    /// The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.iosApps#IosApp.FIELDS.app_id)).
    
    pub name: Option<String>,
    /// Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `IosApp`.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. The lifecycle state of the App.
    
    pub state: Option<IosAppStateEnum>,
    /// The Apple Developer Team ID associated with the App in the App Store.
    #[serde(rename="teamId")]
    
    pub team_id: Option<String>,
}

impl client::RequestValue for IosApp {}
impl client::ResponseResult for IosApp {}


/// Configuration metadata of a single Firebase App for iOS.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ios apps get config projects](ProjectIosAppGetConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct IosAppConfig {
    /// The content of the XML configuration file.
    #[serde(rename="configFileContents")]
    
    #[serde_as(as = "Option<::client::serde::urlsafe_base64::Wrapper>")]
    pub config_file_contents: Option<Vec<u8>>,
    /// The filename that the configuration artifact for the `IosApp` is typically saved as. For example: `GoogleService-Info.plist`
    #[serde(rename="configFilename")]
    
    pub config_filename: Option<String>,
}

impl client::ResponseResult for IosAppConfig {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps list projects](ProjectAndroidAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAndroidAppsResponse {
    /// List of each `AndroidApp` associated with the specified `FirebaseProject`.
    
    pub apps: Option<Vec<AndroidApp>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListAndroidApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAndroidAppsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [available locations list projects](ProjectAvailableLocationListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailableLocationsResponse {
    /// One page of results from a call to `ListAvailableLocations`.
    
    pub locations: Option<Vec<Location>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results and all available locations have been listed. This token can be used in a subsequent call to `ListAvailableLocations` to find more locations. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListAvailableLocationsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list available projects](AvailableProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListAvailableProjectsResponse {
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent calls to `ListAvailableProjects` to find the next group of Projects. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// The list of GCP `Projects` which can have Firebase resources added to them.
    #[serde(rename="projectInfo")]
    
    pub project_info: Option<Vec<ProjectInfo>>,
}

impl client::ResponseResult for ListAvailableProjectsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [list projects](ProjectListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListFirebaseProjectsResponse {
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent calls to `ListFirebaseProjects` to find the next group of Projects. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
    /// One page of the list of Projects that are accessible to the caller.
    
    pub results: Option<Vec<FirebaseProject>>,
}

impl client::ResponseResult for ListFirebaseProjectsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ios apps list projects](ProjectIosAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListIosAppsResponse {
    /// List of each `IosApp` associated with the specified `FirebaseProject`.
    
    pub apps: Option<Vec<IosApp>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListIosApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListIosAppsResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps sha list projects](ProjectAndroidAppShaListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListShaCertificatesResponse {
    /// The list of each `ShaCertificate` associated with the `AndroidApp`.
    
    pub certificates: Option<Vec<ShaCertificate>>,
}

impl client::ResponseResult for ListShaCertificatesResponse {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps list projects](ProjectWebAppListCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ListWebAppsResponse {
    /// List of each `WebApp` associated with the specified `FirebaseProject`.
    
    pub apps: Option<Vec<WebApp>>,
    /// If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent call to `ListWebApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for ListWebAppsResponse {}


/// A GCP resource location that can be selected for a FirebaseProject.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Products and services that are available in the GCP resource location.
    
    pub features: Option<Vec<LocationFeaturesEnum>>,
    /// The ID of the GCP resource location. It will be one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations#types).
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// Indicates whether the GCP resource location is a [regional or multi-regional location](https://firebase.google.com/docs/projects/locations#types) for data replication.
    #[serde(rename="type")]
    
    pub type_: Option<LocationTypeEnum>,
}

impl client::Part for Location {}


/// This resource represents a long-running operation that is the result of a network API call.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [get operations](OperationGetCall) (response)
/// * [android apps create projects](ProjectAndroidAppCreateCall) (response)
/// * [android apps remove projects](ProjectAndroidAppRemoveCall) (response)
/// * [android apps undelete projects](ProjectAndroidAppUndeleteCall) (response)
/// * [default location finalize projects](ProjectDefaultLocationFinalizeCall) (response)
/// * [ios apps create projects](ProjectIosAppCreateCall) (response)
/// * [ios apps remove projects](ProjectIosAppRemoveCall) (response)
/// * [ios apps undelete projects](ProjectIosAppUndeleteCall) (response)
/// * [web apps create projects](ProjectWebAppCreateCall) (response)
/// * [web apps remove projects](ProjectWebAppRemoveCall) (response)
/// * [web apps undelete projects](ProjectWebAppUndeleteCall) (response)
/// * [add firebase projects](ProjectAddFirebaseCall) (response)
/// * [add google analytics projects](ProjectAddGoogleAnalyticCall) (response)
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


/// A reference to a Google Cloud Platform (GCP) `Project`.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// The user-assigned display name of the GCP `Project`, for example: `My App`
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// The ID of the Project’s default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). Not all Projects will have this field populated. If it is not populated, it means that the Project does not yet have a default GCP resource location. To set a Project’s default GCP resource location, call [`FinalizeDefaultLocation`](https://firebase.google.com/../projects.defaultLocation/finalize) after you add Firebase resources to the Project.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The resource name of the GCP `Project` to which Firebase resources can be added, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    
    pub project: Option<String>,
}

impl client::Part for ProjectInfo {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [remove analytics projects](ProjectRemoveAnalyticCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveAnalyticsRequest {
    /// Optional. The ID of the Google Analytics property associated with the specified `FirebaseProject`. - If not set, then the Google Analytics property that is currently associated with the specified `FirebaseProject` is removed. - If set, and the specified `FirebaseProject` is currently associated with a *different* Google Analytics property, then the response is a `412 Precondition Failed` error. 
    #[serde(rename="analyticsPropertyId")]
    
    pub analytics_property_id: Option<String>,
}

impl client::RequestValue for RemoveAnalyticsRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps remove projects](ProjectAndroidAppRemoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveAndroidAppRequest {
    /// If set to true, and the App is not found, the request will succeed but no action will be taken on the server.
    #[serde(rename="allowMissing")]
    
    pub allow_missing: Option<bool>,
    /// Checksum provided in the AndroidApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Determines whether to _immediately_ delete the AndroidApp. If set to true, the App is immediately deleted from the Project and cannot be restored to the Project. If not set, defaults to false, which means the App will be set to expire in 30 days. Within the 30 days, the App may be restored to the Project using UndeleteAndroidApp.
    
    pub immediate: Option<bool>,
    /// If set to true, the request is only validated. The App will _not_ be removed.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for RemoveAndroidAppRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ios apps remove projects](ProjectIosAppRemoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveIosAppRequest {
    /// If set to true, and the App is not found, the request will succeed but no action will be taken on the server.
    #[serde(rename="allowMissing")]
    
    pub allow_missing: Option<bool>,
    /// Checksum provided in the IosApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Determines whether to _immediately_ delete the IosApp. If set to true, the App is immediately deleted from the Project and cannot be restored to the Project. If not set, defaults to false, which means the App will be set to expire in 30 days. Within the 30 days, the App may be restored to the Project using UndeleteIosApp
    
    pub immediate: Option<bool>,
    /// If set to true, the request is only validated. The App will _not_ be removed.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for RemoveIosAppRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps remove projects](ProjectWebAppRemoveCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RemoveWebAppRequest {
    /// If set to true, and the App is not found, the request will succeed but no action will be taken on the server.
    #[serde(rename="allowMissing")]
    
    pub allow_missing: Option<bool>,
    /// Checksum provided in the WebApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// Determines whether to _immediately_ delete the WebApp. If set to true, the App is immediately deleted from the Project and cannot be restored to the Project. If not set, defaults to false, which means the App will be set to expire in 30 days. Within the 30 days, the App may be restored to the Project using UndeleteWebApp
    
    pub immediate: Option<bool>,
    /// If set to true, the request is only validated. The App will _not_ be removed.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for RemoveWebAppRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [search apps projects](ProjectSearchAppCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SearchFirebaseAppsResponse {
    /// One page of results from a call to `SearchFirebaseApps`.
    
    pub apps: Option<Vec<FirebaseAppInfo>>,
    /// If the result list is too large to fit in a single response, then a token is returned. This token can be used in a subsequent calls to `SearchFirebaseApps` to find the next group of Apps. Page tokens are short-lived and should not be persisted.
    #[serde(rename="nextPageToken")]
    
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for SearchFirebaseAppsResponse {}


/// A SHA-1 or SHA-256 certificate associated with the AndroidApp.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps sha create projects](ProjectAndroidAppShaCreateCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ShaCertificate {
    /// The type of SHA certificate encoded in the hash.
    #[serde(rename="certType")]
    
    pub cert_type: Option<ShaCertificateCertTypeEnum>,
    /// The resource name of the ShaCertificate for the AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID/sha/SHA_HASH * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.androidApps#AndroidApp.FIELDS.app_id)). * SHA_HASH: the certificate hash for the App (see [`shaHash`](https://firebase.google.com/../projects.androidApps.sha#ShaCertificate.FIELDS.sha_hash)).
    
    pub name: Option<String>,
    /// The certificate hash for the `AndroidApp`.
    #[serde(rename="shaHash")]
    
    pub sha_hash: Option<String>,
}

impl client::RequestValue for ShaCertificate {}
impl client::ResponseResult for ShaCertificate {}


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


/// A mapping of a Firebase App to a Google Analytics data stream
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StreamMapping {
    /// The resource name of the Firebase App associated with the Google Analytics data stream, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID or projects/PROJECT_IDENTIFIER/iosApps/APP_ID or projects/PROJECT_IDENTIFIER /webApps/APP_ID Refer to the `FirebaseProject` [`name`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
    
    pub app: Option<String>,
    /// Applicable for Firebase Web Apps only. The unique Google-assigned identifier of the Google Analytics web stream associated with the Firebase Web App. Firebase SDKs use this ID to interact with Google Analytics APIs. Learn more about this ID and Google Analytics web streams in the [Analytics documentation](https://support.google.com/analytics/answer/9304153).
    #[serde(rename="measurementId")]
    
    pub measurement_id: Option<String>,
    /// The unique Google-assigned identifier of the Google Analytics data stream associated with the Firebase App. Learn more about Google Analytics data streams in the [Analytics documentation](https://support.google.com/analytics/answer/9303323).
    #[serde(rename="streamId")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub stream_id: Option<i64>,
}

impl client::Part for StreamMapping {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps undelete projects](ProjectAndroidAppUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteAndroidAppRequest {
    /// Checksum provided in the AndroidApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// If set to true, the request is only validated. The App will _not_ be undeleted.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for UndeleteAndroidAppRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [ios apps undelete projects](ProjectIosAppUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteIosAppRequest {
    /// Checksum provided in the IosApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// If set to true, the request is only validated. The App will _not_ be undeleted.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for UndeleteIosAppRequest {}


/// There is no detailed description.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps undelete projects](ProjectWebAppUndeleteCall) (request)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UndeleteWebAppRequest {
    /// Checksum provided in the WebApp resource. If provided, this checksum ensures that the client has an up-to-date value before proceeding.
    
    pub etag: Option<String>,
    /// If set to true, the request is only validated. The App will _not_ be undeleted.
    #[serde(rename="validateOnly")]
    
    pub validate_only: Option<bool>,
}

impl client::RequestValue for UndeleteWebAppRequest {}


/// Details of a Firebase App for the web.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps create projects](ProjectWebAppCreateCall) (request)
/// * [web apps get projects](ProjectWebAppGetCall) (response)
/// * [web apps patch projects](ProjectWebAppPatchCall) (request|response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebApp {
    /// The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `WebApp`. Be aware that this value is the UID of the API key, *not* the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App’s [configuration artifact](https://firebase.google.com/../../rest/v1beta1/projects.webApps/getConfig). If `api_key_id` is not set in requests to [`webApps.Create`](https://firebase.google.com/../../rest/v1beta1/projects.webApps/create), then Firebase automatically associates an `api_key_id` with the `WebApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `WebApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys.
    #[serde(rename="apiKeyId")]
    
    pub api_key_id: Option<String>,
    /// Output only. Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The URLs where the `WebApp` is hosted.
    #[serde(rename="appUrls")]
    
    pub app_urls: Option<Vec<String>>,
    /// The user-assigned display name for the `WebApp`.
    #[serde(rename="displayName")]
    
    pub display_name: Option<String>,
    /// This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated.
    
    pub etag: Option<String>,
    /// The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID * PROJECT_IDENTIFIER: the parent Project’s [`ProjectNumber`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google’s [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.app_id)).
    
    pub name: Option<String>,
    /// Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `WebApp`.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// Output only. The lifecycle state of the App.
    
    pub state: Option<WebAppStateEnum>,
    /// Output only. Immutable. A unique, Firebase-assigned identifier for the `WebApp`. This identifier is only used to populate the `namespace` value for the `WebApp`. For most use cases, use `appId` to identify or reference the App. The `webId` value is only unique within a `FirebaseProject` and its associated Apps.
    #[serde(rename="webId")]
    
    pub web_id: Option<String>,
}

impl client::RequestValue for WebApp {}
impl client::ResponseResult for WebApp {}


/// Configuration metadata of a single Firebase App for the web.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [web apps get config projects](ProjectWebAppGetConfigCall) (response)
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WebAppConfig {
    /// The [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key associated with the `WebApp`. Note that this value is *not* the [`apiKeyId`](https://firebase.google.com/../projects.webApps#WebApp.FIELDS.api_key_id) (the UID) of the API key associated with the `WebApp`.
    #[serde(rename="apiKey")]
    
    pub api_key: Option<String>,
    /// Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`.
    #[serde(rename="appId")]
    
    pub app_id: Option<String>,
    /// The domain Firebase Auth configures for OAuth redirects, in the format: PROJECT_ID.firebaseapp.com
    #[serde(rename="authDomain")]
    
    pub auth_domain: Option<String>,
    /// The default Firebase Realtime Database URL.
    #[serde(rename="databaseURL")]
    
    pub database_url: Option<String>,
    /// The ID of the Project’s default GCP resource location. The location is one of the available [GCP resource locations](https://firebase.google.com/docs/projects/locations). This field is omitted if the default GCP resource location has not been finalized yet. To set a Project’s default GCP resource location, call [`FinalizeDefaultLocation`](https://firebase.google.com/../projects.defaultLocation/finalize) after you add Firebase resources to the Project.
    #[serde(rename="locationId")]
    
    pub location_id: Option<String>,
    /// The unique Google-assigned identifier of the Google Analytics web stream associated with the `WebApp`. Firebase SDKs use this ID to interact with Google Analytics APIs. This field is only present if the `WebApp` is linked to a web stream in a Google Analytics App + Web property. Learn more about this ID and Google Analytics web streams in the [Analytics documentation](https://support.google.com/analytics/answer/9304153). To generate a `measurementId` and link the `WebApp` with a Google Analytics web stream, call [`AddGoogleAnalytics`](https://firebase.google.com/../../v1beta1/projects/addGoogleAnalytics). For apps using the Firebase JavaScript SDK v7.20.0 and later, Firebase dynamically fetches the `measurementId` when your app initializes Analytics. Having this ID in your config object is optional, but it does serve as a fallback in the rare case that the dynamic fetch fails.
    #[serde(rename="measurementId")]
    
    pub measurement_id: Option<String>,
    /// The sender ID for use with Firebase Cloud Messaging.
    #[serde(rename="messagingSenderId")]
    
    pub messaging_sender_id: Option<String>,
    /// Immutable. A user-assigned unique identifier for the `FirebaseProject`.
    #[serde(rename="projectId")]
    
    pub project_id: Option<String>,
    /// The default Cloud Storage for Firebase storage bucket name.
    #[serde(rename="storageBucket")]
    
    pub storage_bucket: Option<String>,
}

impl client::ResponseResult for WebAppConfig {}


