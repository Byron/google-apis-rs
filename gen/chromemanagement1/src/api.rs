use std::collections::HashMap;
use std::cell::RefCell;
use std::borrow::BorrowMut;
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
    /// See reports about devices and Chrome browsers managed within your organization
    ChromeManagementReportReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::ChromeManagementReportReadonly => "https://www.googleapis.com/auth/chrome.management.reports.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::ChromeManagementReportReadonly
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all ChromeManagement related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// use chromemanagement1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use chromemanagement1::ChromeManagement;
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
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_versions("customer")
///              .page_token("duo")
///              .page_size(-55)
///              .org_unit_id("gubergren")
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
pub struct ChromeManagement<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for ChromeManagement<C> {}

impl<'a, C> ChromeManagement<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> ChromeManagement<C> {
        ChromeManagement {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://chromemanagement.googleapis.com/".to_string(),
            _root_url: "https://chromemanagement.googleapis.com/".to_string(),
        }
    }

    pub fn customers(&'a self) -> CustomerMethods<'a, C> {
        CustomerMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://chromemanagement.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Describes a browser version and its install count.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1BrowserVersion {
    /// Output only. The release channel of the installed browser.
    pub channel: Option<String>,
    /// Output only. Count grouped by device_system and major version
    pub count: Option<String>,
    /// Output only. Version of the system-specified operating system.
    #[serde(rename="deviceOsVersion")]
    pub device_os_version: Option<String>,
    /// Output only. The device operating system.
    pub system: Option<String>,
    /// Output only. The full version of the installed browser.
    pub version: Option<String>,
}

impl client::Part for GoogleChromeManagementV1BrowserVersion {}


/// Response containing requested browser versions details and counts.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count chrome versions customers](CustomerReportCountChromeVersionCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
    /// List of all browser versions and their install counts.
    #[serde(rename="browserVersions")]
    pub browser_versions: Option<Vec<GoogleChromeManagementV1BrowserVersion>>,
    /// Token to specify the next page in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Total number browser versions matching request.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountChromeVersionsResponse {}


/// Response containing details of queried installed apps.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports count installed apps customers](CustomerReportCountInstalledAppCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
    /// List of installed apps matching request.
    #[serde(rename="installedApps")]
    pub installed_apps: Option<Vec<GoogleChromeManagementV1InstalledApp>>,
    /// Token to specify next page in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Total number of installed apps matching request.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1CountInstalledAppsResponse {}


/// Describes a device reporting Chrome browser information.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1Device {
    /// Output only. The ID of the device that reported this Chrome browser information.
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
    /// Output only. The name of the machine within its local network.
    pub machine: Option<String>,
}

impl client::Part for GoogleChromeManagementV1Device {}


/// Response containing a list of devices with queried app installed.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [reports find installed app devices customers](CustomerReportFindInstalledAppDeviceCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
    /// A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field.
    pub devices: Option<Vec<GoogleChromeManagementV1Device>>,
    /// Token to specify the next page in the list.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
    /// Total number of devices matching request.
    #[serde(rename="totalSize")]
    pub total_size: Option<i32>,
}

impl client::ResponseResult for GoogleChromeManagementV1FindInstalledAppDevicesResponse {}


/// Describes an installed app.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleChromeManagementV1InstalledApp {
    /// Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    #[serde(rename="appId")]
    pub app_id: Option<String>,
    /// Output only. How the app was installed.
    #[serde(rename="appInstallType")]
    pub app_install_type: Option<String>,
    /// Output only. Source of the installed app.
    #[serde(rename="appSource")]
    pub app_source: Option<String>,
    /// Output only. Type of the app.
    #[serde(rename="appType")]
    pub app_type: Option<String>,
    /// Output only. Count of browser devices with this app installed.
    #[serde(rename="browserDeviceCount")]
    pub browser_device_count: Option<String>,
    /// Output only. Description of the installed app.
    pub description: Option<String>,
    /// Output only. Whether the app is disabled.
    pub disabled: Option<bool>,
    /// Output only. Name of the installed app.
    #[serde(rename="displayName")]
    pub display_name: Option<String>,
    /// Output only. Homepage uri of the installed app.
    #[serde(rename="homepageUri")]
    pub homepage_uri: Option<String>,
    /// Output only. Count of ChromeOS users with this app installed.
    #[serde(rename="osUserCount")]
    pub os_user_count: Option<String>,
    /// Output only. Permissions of the installed app.
    pub permissions: Option<Vec<String>>,
}

impl client::Part for GoogleChromeManagementV1InstalledApp {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *customer* resources.
/// It is not used directly, but through the `ChromeManagement` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_chromemanagement1 as chromemanagement1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use chromemanagement1::ChromeManagement;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `reports_count_chrome_versions(...)`, `reports_count_installed_apps(...)` and `reports_find_installed_app_devices(...)`
/// // to build up your call.
/// let rb = hub.customers();
/// # }
/// ```
pub struct CustomerMethods<'a, C>
    where C: 'a {

    hub: &'a ChromeManagement<C>,
}

impl<'a, C> client::MethodsBuilder for CustomerMethods<'a, C> {}

impl<'a, C> CustomerMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of installed Chrome versions.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_chrome_versions(&self, customer: &str) -> CustomerReportCountChromeVersionCall<'a, C> {
        CustomerReportCountChromeVersionCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of app installations.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_count_installed_apps(&self, customer: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        CustomerReportCountInstalledAppCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Generate report of devices that have a specified app installed.
    /// 
    /// # Arguments
    ///
    /// * `customer` - Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    pub fn reports_find_installed_app_devices(&self, customer: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        CustomerReportFindInstalledAppDeviceCall {
            hub: self.hub,
            _customer: customer.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _org_unit_id: Default::default(),
            _order_by: Default::default(),
            _filter: Default::default(),
            _app_type: Default::default(),
            _app_id: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Generate report of installed Chrome versions.
///
/// A builder for the *reports.countChromeVersions* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use chromemanagement1::ChromeManagement;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_chrome_versions("customer")
///              .page_token("eos")
///              .page_size(-4)
///              .org_unit_id("ea")
///              .filter("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountChromeVersionCall<'a, C>
    where C: 'a {

    hub: &'a ChromeManagement<C>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for CustomerReportCountChromeVersionCall<'a, C> {}

impl<'a, C> CustomerReportCountChromeVersionCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountChromeVersionsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countChromeVersions",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(7 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countChromeVersions";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
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
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
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


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountChromeVersionCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, C> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountChromeVersionCall<'a, C> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountChromeVersionCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountChromeVersionCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportCountChromeVersionCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate report of app installations.
///
/// A builder for the *reports.countInstalledApps* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use chromemanagement1::ChromeManagement;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_count_installed_apps("customer")
///              .page_token("amet")
///              .page_size(-20)
///              .org_unit_id("ipsum")
///              .order_by("sed")
///              .filter("ut")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportCountInstalledAppCall<'a, C>
    where C: 'a {

    hub: &'a ChromeManagement<C>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for CustomerReportCountInstalledAppCall<'a, C> {}

impl<'a, C> CustomerReportCountInstalledAppCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1CountInstalledAppsResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.countInstalledApps",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(8 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:countInstalledApps";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
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
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
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


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportCountInstalledAppCall<'a, C> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportCountInstalledAppCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportCountInstalledAppCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportCountInstalledAppCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


/// Generate report of devices that have a specified app installed.
///
/// A builder for the *reports.findInstalledAppDevices* method supported by a *customer* resource.
/// It is not used directly, but through a `CustomerMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_chromemanagement1 as chromemanagement1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use chromemanagement1::ChromeManagement;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = ChromeManagement::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.customers().reports_find_installed_app_devices("customer")
///              .page_token("rebum.")
///              .page_size(-57)
///              .org_unit_id("ipsum")
///              .order_by("ipsum")
///              .filter("est")
///              .app_type("gubergren")
///              .app_id("ea")
///              .doit().await;
/// # }
/// ```
pub struct CustomerReportFindInstalledAppDeviceCall<'a, C>
    where C: 'a {

    hub: &'a ChromeManagement<C>,
    _customer: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _org_unit_id: Option<String>,
    _order_by: Option<String>,
    _filter: Option<String>,
    _app_type: Option<String>,
    _app_id: Option<String>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for CustomerReportFindInstalledAppDeviceCall<'a, C> {}

impl<'a, C> CustomerReportFindInstalledAppDeviceCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleChromeManagementV1FindInstalledAppDevicesResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "chromemanagement.customers.reports.findInstalledAppDevices",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(10 + self._additional_params.len());
        params.push(("customer", self._customer.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        if let Some(value) = self._org_unit_id {
            params.push(("orgUnitId", value.to_string()));
        }
        if let Some(value) = self._order_by {
            params.push(("orderBy", value.to_string()));
        }
        if let Some(value) = self._filter {
            params.push(("filter", value.to_string()));
        }
        if let Some(value) = self._app_type {
            params.push(("appType", value.to_string()));
        }
        if let Some(value) = self._app_id {
            params.push(("appId", value.to_string()));
        }
        for &field in ["alt", "customer", "pageToken", "pageSize", "orgUnitId", "orderBy", "filter", "appType", "appId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1/{+customer}/reports:findInstalledAppDevices";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::ChromeManagementReportReadonly.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+customer}", "customer")].iter() {
            let mut replace_with = String::new();
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = value.to_string();
                    break;
                }
            }
            if find_this.as_bytes()[1] == '+' as u8 {
                replace_with = percent_encode(replace_with.as_bytes(), DEFAULT_ENCODE_SET).to_string();
            }
            url = url.replace(find_this, &replace_with);
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["customer"].iter() {
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
            let authenticator = self.hub.auth.borrow_mut();
            let token = match authenticator.token(&self._scopes.keys().collect::<Vec<_>>()[..]).await {
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
                let mut client = &mut *self.hub.client.borrow_mut();
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder().method(hyper::Method::GET).uri(url.clone().into_string())
                        .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));


                        let request = req_builder
                        .body(hyper::body::Body::empty());

                client.borrow_mut().request(request.unwrap()).await
                
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
                    let (res_parts, res_body) = res.into_parts();
                    let res_body_string: String = String::from_utf8(
                        hyper::body::to_bytes(res_body)
                            .await
                            .unwrap()
                            .into_iter()
                            .collect(),
                    )
                    .unwrap();
                    let reconstructed_result =
                        hyper::Response::from_parts(res_parts, res_body_string.clone().into());

                    if !reconstructed_result.status().is_success() {
                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&reconstructed_result,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(reconstructed_result)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    let result_value = {
                        match json::from_str(&res_body_string) {
                            Ok(decoded) => (reconstructed_result, decoded),
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


    /// Required. Customer id or "my_customer" to use the customer associated to the account making the request.
    ///
    /// Sets the *customer* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn customer(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._customer = new_value.to_string();
        self
    }
    /// Token to specify the next page in the list.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of results to return. Maximum and default are 100.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._page_size = Some(new_value);
        self
    }
    /// The ID of the organizational unit.
    ///
    /// Sets the *org unit id* query property to the given value.
    pub fn org_unit_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._org_unit_id = Some(new_value.to_string());
        self
    }
    /// Field used to order results. Supported order by fields: * machine * device_id
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Type of the app.
    ///
    /// Sets the *app type* query property to the given value.
    pub fn app_type(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._app_type = Some(new_value.to_string());
        self
    }
    /// Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote).
    ///
    /// Sets the *app id* query property to the given value.
    pub fn app_id(mut self, new_value: &str) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
        self._app_id = Some(new_value.to_string());
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CustomerReportFindInstalledAppDeviceCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CustomerReportFindInstalledAppDeviceCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::ChromeManagementReportReadonly`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> CustomerReportFindInstalledAppDeviceCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


