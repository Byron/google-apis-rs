use std::collections::HashMap;
use std::cell::RefCell;
use std::default::Default;
use std::collections::BTreeSet;
use std::error::Error as StdError;
use serde_json as json;
use std::io;
use std::fs;
use std::mem;

use hyper::client::connect;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::time::sleep;
use tower_service;
use serde::{Serialize, Deserialize};

use crate::{client, client::GetToken, client::serde_with};

// ##############
// UTILITIES ###
// ############

/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
pub enum Scope {
    /// Private Service: https://www.googleapis.com/auth/playintegrity
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/playintegrity",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all PlayIntegrity related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playintegrity1 as playintegrity1;
/// use playintegrity1::api::DecodeIntegrityTokenRequest;
/// use playintegrity1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use playintegrity1::{PlayIntegrity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
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
/// let mut hub = PlayIntegrity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DecodeIntegrityTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().decode_integrity_token(req, "packageName")
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
pub struct PlayIntegrity<S> {
    pub client: hyper::Client<S, hyper::body::Body>,
    pub auth: Box<dyn client::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, S> client::Hub for PlayIntegrity<S> {}

impl<'a, S> PlayIntegrity<S> {

    pub fn new<A: 'static + client::GetToken>(client: hyper::Client<S, hyper::body::Body>, auth: A) -> PlayIntegrity<S> {
        PlayIntegrity {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/5.0.5".to_string(),
            _base_url: "https://playintegrity.googleapis.com/".to_string(),
            _root_url: "https://playintegrity.googleapis.com/".to_string(),
        }
    }

    pub fn methods(&'a self) -> MethodMethods<'a, S> {
        MethodMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/5.0.5`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://playintegrity.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://playintegrity.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// (Restricted Access) Contains a signal helping apps differentiating between likely genuine and likely non-genuine user traffic.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountActivity {
    /// Required. Indicates the activity level of the account.
    #[serde(rename="activityLevel")]
    
    pub activity_level: Option<String>,
}

impl client::Part for AccountActivity {}


/// Contains the account information such as the licensing status for the user in the scope.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AccountDetails {
    /// (Restricted Access) Details about the account activity for the user in the scope.
    #[serde(rename="accountActivity")]
    
    pub account_activity: Option<AccountActivity>,
    /// Required. Details about the licensing status of the user for the app in the scope.
    #[serde(rename="appLicensingVerdict")]
    
    pub app_licensing_verdict: Option<String>,
}

impl client::Part for AccountDetails {}


/// Contains signals about others apps on the device which could be used to access or control the requesting app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppAccessRiskVerdict {
    /// List of detected app types signalled for App Access Risk.
    #[serde(rename="appsDetected")]
    
    pub apps_detected: Option<Vec<String>>,
    /// Deprecated: this field will be removed, please use apps_detected instead. App access risk verdict related to apps that are not installed by Google Play, and are not preloaded on the system image by the device manufacturer.
    #[serde(rename="otherApps")]
    
    pub other_apps: Option<String>,
    /// Deprecated: this field will be removed, please use apps_detected instead. App access risk verdict related to apps that are not installed by the Google Play Store, and are not preloaded on the system image by the device manufacturer.
    #[serde(rename="playOrSystemApps")]
    
    pub play_or_system_apps: Option<String>,
}

impl client::Part for AppAccessRiskVerdict {}


/// Contains the application integrity information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AppIntegrity {
    /// Required. Details about the app recognition verdict
    #[serde(rename="appRecognitionVerdict")]
    
    pub app_recognition_verdict: Option<String>,
    /// The SHA256 hash of the requesting app's signing certificates (base64 web-safe encoded). Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="certificateSha256Digest")]
    
    pub certificate_sha256_digest: Option<Vec<String>>,
    /// Package name of the application under attestation. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="packageName")]
    
    pub package_name: Option<String>,
    /// Version code of the application. Set iff app_recognition_verdict != UNEVALUATED.
    #[serde(rename="versionCode")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub version_code: Option<i64>,
}

impl client::Part for AppIntegrity {}


/// Request to decode the integrity token.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (request)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenRequest {
    /// Encoded integrity token.
    #[serde(rename="integrityToken")]
    
    pub integrity_token: Option<String>,
}

impl client::RequestValue for DecodeIntegrityTokenRequest {}


/// Response containing the decoded integrity payload.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [decode integrity token](MethodDecodeIntegrityTokenCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DecodeIntegrityTokenResponse {
    /// Plain token payload generated from the decoded integrity token.
    #[serde(rename="tokenPayloadExternal")]
    
    pub token_payload_external: Option<TokenPayloadExternal>,
}

impl client::ResponseResult for DecodeIntegrityTokenResponse {}


/// Contains the device attestation information. Next tag: 4
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct DeviceIntegrity {
    /// Details about the integrity of the device the app is running on.
    #[serde(rename="deviceRecognitionVerdict")]
    
    pub device_recognition_verdict: Option<Vec<String>>,
    /// Details about the device activity of the device the app is running on.
    #[serde(rename="recentDeviceActivity")]
    
    pub recent_device_activity: Option<RecentDeviceActivity>,
}

impl client::Part for DeviceIntegrity {}


/// Contains information about the environment Play Integrity API runs in, e.g. Play Protect verdict.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentDetails {
    /// The evaluation of the App Access Risk verdicts.
    #[serde(rename="appAccessRiskVerdict")]
    
    pub app_access_risk_verdict: Option<AppAccessRiskVerdict>,
    /// The evaluation of Play Protect verdict.
    #[serde(rename="playProtectVerdict")]
    
    pub play_protect_verdict: Option<String>,
}

impl client::Part for EnvironmentDetails {}


/// Recent device activity can help developers identify devices that have exhibited hyperactive attestation activity, which could be a sign of an attack or token farming.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RecentDeviceActivity {
    /// Required. Indicates the activity level of the device.
    #[serde(rename="deviceActivityLevel")]
    
    pub device_activity_level: Option<String>,
}

impl client::Part for RecentDeviceActivity {}


/// Contains the integrity request information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    /// Nonce that was provided in the request (which is base64 web-safe no-wrap).
    
    pub nonce: Option<String>,
    /// Request hash that was provided in the request.
    #[serde(rename="requestHash")]
    
    pub request_hash: Option<String>,
    /// Required. Application package name this attestation was requested for. Note: This field makes no guarantees or promises on the caller integrity. For details on application integrity, check application_integrity.
    #[serde(rename="requestPackageName")]
    
    pub request_package_name: Option<String>,
    /// Required. Timestamp, in milliseconds, of the integrity application request.
    #[serde(rename="timestampMillis")]
    
    #[serde_as(as = "Option<::client::serde_with::DisplayFromStr>")]
    pub timestamp_millis: Option<i64>,
}

impl client::Part for RequestDetails {}


/// Contains additional information generated for testing responses.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TestingDetails {
    /// Required. Indicates that the information contained in this payload is a testing response that is statically overridden for a tester.
    #[serde(rename="isTestingResponse")]
    
    pub is_testing_response: Option<bool>,
}

impl client::Part for TestingDetails {}


/// Contains basic app information and integrity signals like device attestation and licensing details.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as(crate = "::client::serde_with")]
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TokenPayloadExternal {
    /// Required. Details about the Play Store account.
    #[serde(rename="accountDetails")]
    
    pub account_details: Option<AccountDetails>,
    /// Required. Details about the application integrity.
    #[serde(rename="appIntegrity")]
    
    pub app_integrity: Option<AppIntegrity>,
    /// Required. Details about the device integrity.
    #[serde(rename="deviceIntegrity")]
    
    pub device_integrity: Option<DeviceIntegrity>,
    /// Details of the environment Play Integrity API runs in.
    #[serde(rename="environmentDetails")]
    
    pub environment_details: Option<EnvironmentDetails>,
    /// Required. Details about the integrity request.
    #[serde(rename="requestDetails")]
    
    pub request_details: Option<RequestDetails>,
    /// Indicates that this payload is generated for testing purposes and contains any additional data that is linked with testing status.
    #[serde(rename="testingDetails")]
    
    pub testing_details: Option<TestingDetails>,
}

impl client::Part for TokenPayloadExternal {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`PlayIntegrity`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_playintegrity1 as playintegrity1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use playintegrity1::{PlayIntegrity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = PlayIntegrity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `decode_integrity_token(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

    hub: &'a PlayIntegrity<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Decodes the integrity token and returns the token payload.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `packageName` -  Package name of the app the attached integrity token belongs to.
    pub fn decode_integrity_token(&self, request: DecodeIntegrityTokenRequest, package_name: &str) -> MethodDecodeIntegrityTokenCall<'a, S> {
        MethodDecodeIntegrityTokenCall {
            hub: self.hub,
            _request: request,
            _package_name: package_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Decodes the integrity token and returns the token payload.
///
/// A builder for the *decodeIntegrityToken* method.
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playintegrity1 as playintegrity1;
/// use playintegrity1::api::DecodeIntegrityTokenRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playintegrity1::{PlayIntegrity, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayIntegrity::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().unwrap().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = DecodeIntegrityTokenRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().decode_integrity_token(req, "packageName")
///              .doit().await;
/// # }
/// ```
pub struct MethodDecodeIntegrityTokenCall<'a, S>
    where S: 'a {

    hub: &'a PlayIntegrity<S>,
    _request: DecodeIntegrityTokenRequest,
    _package_name: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for MethodDecodeIntegrityTokenCall<'a, S> {}

impl<'a, S> MethodDecodeIntegrityTokenCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, DecodeIntegrityTokenResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playintegrity.decodeIntegrityToken",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "packageName"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("packageName", self._package_name);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/{+packageName}:decodeIntegrityToken";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{+packageName}", "packageName")].iter() {
            url = params.uri_replacement(url, param_name, find_this, true);
        }
        {
            let to_remove = ["packageName"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);

        let mut json_mime_type = mime::APPLICATION_JSON;
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
            let token = match self.hub.auth.get_token(&self._scopes.iter().map(String::as_str).collect::<Vec<_>>()[..]).await {
                Ok(token) => token,
                Err(e) => {
                    match dlg.token(e) {
                        Ok(token) => token,
                        Err(e) => {
                            dlg.finished(false);
                            return Err(client::Error::MissingToken(e));
                        }
                    }
                }
            };
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
                        .body(hyper::body::Body::from(request_value_reader.get_ref().clone()));

                client.request(request.unwrap()).await

            };

            match req_result {
                Err(err) => {
                    if let client::Retry::After(d) = dlg.http_error(&err) {
                        sleep(d).await;
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
                            sleep(d).await;
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
    pub fn request(mut self, new_value: DecodeIntegrityTokenRequest) -> MethodDecodeIntegrityTokenCall<'a, S> {
        self._request = new_value;
        self
    }
    ///  Package name of the app the attached integrity token belongs to.
    ///
    /// Sets the *package name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn package_name(mut self, new_value: &str) -> MethodDecodeIntegrityTokenCall<'a, S> {
        self._package_name = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodDecodeIntegrityTokenCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> MethodDecodeIntegrityTokenCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> MethodDecodeIntegrityTokenCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> MethodDecodeIntegrityTokenCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> MethodDecodeIntegrityTokenCall<'a, S> {
        self._scopes.clear();
        self
    }
}


