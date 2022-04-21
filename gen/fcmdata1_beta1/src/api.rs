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
    /// See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account.
    CloudPlatform,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::CloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::CloudPlatform
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Fcmdata related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fcmdata1_beta1 as fcmdata1_beta1;
/// use fcmdata1_beta1::{Result, Error};
/// # async fn dox() {
/// use std::default::Default;
/// use fcmdata1_beta1::{Fcmdata, oauth2, hyper, hyper_rustls};
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
/// let mut hub = Fcmdata::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().android_apps_delivery_data_list("parent")
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
pub struct Fcmdata<> {
    pub client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>,
    pub auth: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, > client::Hub for Fcmdata<> {}

impl<'a, > Fcmdata<> {

    pub fn new(client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Fcmdata<> {
        Fcmdata {
            client,
            auth: authenticator,
            _user_agent: "google-api-rust-client/3.0.2".to_string(),
            _base_url: "https://fcmdata.googleapis.com/".to_string(),
            _root_url: "https://fcmdata.googleapis.com/".to_string(),
        }
    }

    pub fn projects(&'a self) -> ProjectMethods<'a> {
        ProjectMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/3.0.2`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://fcmdata.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://fcmdata.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// Message delivery data for a given date, app, and analytics label combination.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1AndroidDeliveryData {
    /// The analytics label associated with the messages sent. All messages sent without an analytics label will be grouped together in a single entry.
    #[serde(rename="analyticsLabel")]
    pub analytics_label: Option<String>,
    /// The app ID to which the messages were sent.
    #[serde(rename="appId")]
    pub app_id: Option<String>,
    /// The data for the specified appId, date, and analyticsLabel.
    pub data: Option<GoogleFirebaseFcmDataV1beta1Data>,
    /// The date represented by this entry.
    pub date: Option<GoogleTypeDate>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1AndroidDeliveryData {}


/// Data detailing messaging delivery
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1Data {
    /// Count of messages accepted by FCM intended to Android devices. The targeted device must have opted in to the collection of usage and diagnostic information.
    #[serde(rename="countMessagesAccepted")]
    pub count_messages_accepted: Option<String>,
    /// Additional information about delivery performance for messages that were successfully delivered.
    #[serde(rename="deliveryPerformancePercents")]
    pub delivery_performance_percents: Option<GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents>,
    /// Additional general insights about message delivery.
    #[serde(rename="messageInsightPercents")]
    pub message_insight_percents: Option<GoogleFirebaseFcmDataV1beta1MessageInsightPercents>,
    /// Mutually exclusive breakdown of message delivery outcomes.
    #[serde(rename="messageOutcomePercents")]
    pub message_outcome_percents: Option<GoogleFirebaseFcmDataV1beta1MessageOutcomePercents>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1Data {}


/// Overview of delivery performance for messages that were successfully delivered. All percentages are calculated with countMessagesAccepted as the denominator. These categories are not mutually exclusive; a message can be delayed for multiple reasons.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents {
    /// The percentage of accepted messages that were delayed because the device was in doze mode. Only [normal priority messages](https://firebase.google.com/docs/cloud-messaging/concept-options#setting-the-priority-of-a-message) should be delayed due to doze mode.
    #[serde(rename="delayedDeviceDoze")]
    pub delayed_device_doze: Option<f32>,
    /// The percentage of accepted messages that were delayed because the target device was not connected at the time of sending. These messages were eventually delivered when the device reconnected.
    #[serde(rename="delayedDeviceOffline")]
    pub delayed_device_offline: Option<f32>,
    /// The percentage of accepted messages that were delayed due to message throttling, such as [collapsible message throttling](https://firebase.google.com/docs/cloud-messaging/concept-options#collapsible_throttling) or [maximum message rate throttling](https://firebase.google.com/docs/cloud-messaging/concept-options#device_throttling).
    #[serde(rename="delayedMessageThrottled")]
    pub delayed_message_throttled: Option<f32>,
    /// The percentage of accepted messages that were delayed because the intended device user-profile was [stopped](https://firebase.google.com/docs/cloud-messaging/android/receive#handling_messages) on the target device at the time of the send. The messages were eventually delivered when the user-profile was started again.
    #[serde(rename="delayedUserStopped")]
    pub delayed_user_stopped: Option<f32>,
    /// The percentage of accepted messages that were delivered to the device without delay from the FCM system.
    #[serde(rename="deliveredNoDelay")]
    pub delivered_no_delay: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1DeliveryPerformancePercents {}


/// Response message for ListAndroidDeliveryData.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [android apps delivery data list projects](ProjectAndroidAppDeliveryDataListCall) (response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1ListAndroidDeliveryDataResponse {
    /// The delivery data for the provided app. There will be one entry per combination of app, date, and analytics label.
    #[serde(rename="androidDeliveryData")]
    pub android_delivery_data: Option<Vec<GoogleFirebaseFcmDataV1beta1AndroidDeliveryData>>,
    /// A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages.
    #[serde(rename="nextPageToken")]
    pub next_page_token: Option<String>,
}

impl client::ResponseResult for GoogleFirebaseFcmDataV1beta1ListAndroidDeliveryDataResponse {}


/// Additional information about message delivery. All percentages are calculated with countMessagesAccepted as the denominator.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1MessageInsightPercents {
    /// The percentage of accepted messages that had their priority lowered from high to normal due to [app standby buckets](https://firebase.google.com/docs/cloud-messaging/concept-options#setting-the-priority-of-a-message).
    #[serde(rename="priorityLowered")]
    pub priority_lowered: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1MessageInsightPercents {}


/// Percentage breakdown of message delivery outcomes. These categories are mutually exclusive. All percentages are calculated with countMessagesAccepted as the denominator. These categories may not account for all message outcomes.
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleFirebaseFcmDataV1beta1MessageOutcomePercents {
    /// The percentage of all accepted messages that were successfully delivered to the device.
    pub delivered: Option<f32>,
    /// The percentage of accepted messages that were dropped because the application was force stopped on the device at the time of delivery and retries were unsuccessful.
    #[serde(rename="droppedAppForceStopped")]
    pub dropped_app_force_stopped: Option<f32>,
    /// The percentage of accepted messages that were dropped because the target device is inactive. FCM will drop messages if the target device is deemed inactive by our servers. If a device does reconnect, we call [OnDeletedMessages()](https://firebase.google.com/docs/cloud-messaging/android/receive#override-ondeletedmessages) in our SDK instead of delivering the messages.
    #[serde(rename="droppedDeviceInactive")]
    pub dropped_device_inactive: Option<f32>,
    /// The percentage of accepted messages that were dropped due to [too many undelivered non-collapsible messages](https://firebase.google.com/docs/cloud-messaging/concept-options#collapsible_and_non-collapsible_messages). Specifically, each app instance can only have 100 pending messages stored on our servers for a device which is disconnected. When that device reconnects, those messages are delivered. When there are more than the maximum pending messages, we call [OnDeletedMessages()](https://firebase.google.com/docs/cloud-messaging/android/receive#override-ondeletedmessages) in our SDK instead of delivering the messages.
    #[serde(rename="droppedTooManyPendingMessages")]
    pub dropped_too_many_pending_messages: Option<f32>,
    /// The percentage of messages accepted on this day that were not dropped and not delivered, due to the device being disconnected (as of the end of the America/Los_Angeles day when the message was sent to FCM). A portion of these messages will be delivered the next day when the device connects but others may be destined to devices that ultimately never reconnect.
    pub pending: Option<f32>,
}

impl client::Part for GoogleFirebaseFcmDataV1beta1MessageOutcomePercents {}


/// Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values. * A month and day, with a zero year (for example, an anniversary). * A year on its own, with a zero month and a zero day. * A year and month, with a zero day (for example, a credit card expiration date). Related types: * google.type.TimeOfDay * google.type.DateTime * google.protobuf.Timestamp
/// 
/// This type is not used in any activity, and only used as *part* of another schema.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct GoogleTypeDate {
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    pub day: Option<i32>,
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    pub month: Option<i32>,
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    pub year: Option<i32>,
}

impl client::Part for GoogleTypeDate {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *project* resources.
/// It is not used directly, but through the `Fcmdata` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_fcmdata1_beta1 as fcmdata1_beta1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use fcmdata1_beta1::{Fcmdata, oauth2, hyper, hyper_rustls};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Fcmdata::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `android_apps_delivery_data_list(...)`
/// // to build up your call.
/// let rb = hub.projects();
/// # }
/// ```
pub struct ProjectMethods<'a>
    where  {

    hub: &'a Fcmdata<>,
}

impl<'a> client::MethodsBuilder for ProjectMethods<'a> {}

impl<'a> ProjectMethods<'a> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// List aggregate delivery data for the given Android application.
    /// 
    /// # Arguments
    ///
    /// * `parent` - Required. The application for which to list delivery data. Format: `projects/{project_id}/androidApps/{app_id}`
    pub fn android_apps_delivery_data_list(&self, parent: &str) -> ProjectAndroidAppDeliveryDataListCall<'a> {
        ProjectAndroidAppDeliveryDataListCall {
            hub: self.hub,
            _parent: parent.to_string(),
            _page_token: Default::default(),
            _page_size: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// List aggregate delivery data for the given Android application.
///
/// A builder for the *androidApps.deliveryData.list* method supported by a *project* resource.
/// It is not used directly, but through a `ProjectMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_fcmdata1_beta1 as fcmdata1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use fcmdata1_beta1::{Fcmdata, oauth2, hyper, hyper_rustls};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Fcmdata::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots().https_or_http().enable_http1().enable_http2().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.projects().android_apps_delivery_data_list("parent")
///              .page_token("amet.")
///              .page_size(-20)
///              .doit().await;
/// # }
/// ```
pub struct ProjectAndroidAppDeliveryDataListCall<'a>
    where  {

    hub: &'a Fcmdata<>,
    _parent: String,
    _page_token: Option<String>,
    _page_size: Option<i32>,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a> client::CallBuilder for ProjectAndroidAppDeliveryDataListCall<'a> {}

impl<'a> ProjectAndroidAppDeliveryDataListCall<'a> {


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleFirebaseFcmDataV1beta1ListAndroidDeliveryDataResponse)> {
        use url::percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "fcmdata.projects.androidApps.deliveryData.list",
                               http_method: hyper::Method::GET });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(5 + self._additional_params.len());
        params.push(("parent", self._parent.to_string()));
        if let Some(value) = self._page_token {
            params.push(("pageToken", value.to_string()));
        }
        if let Some(value) = self._page_size {
            params.push(("pageSize", value.to_string()));
        }
        for &field in ["alt", "parent", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let mut url = self.hub._base_url.clone() + "v1beta1/{+parent}/deliveryData";
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::CloudPlatform.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{+parent}", "parent")].iter() {
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
            for param_name in ["parent"].iter() {
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


    /// Required. The application for which to list delivery data. Format: `projects/{project_id}/androidApps/{app_id}`
    ///
    /// Sets the *parent* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn parent(mut self, new_value: &str) -> ProjectAndroidAppDeliveryDataListCall<'a> {
        self._parent = new_value.to_string();
        self
    }
    /// A page token, received from a previous `ListAndroidDeliveryDataRequest` call. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `ListAndroidDeliveryDataRequest` must match the call that provided the page token.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> ProjectAndroidAppDeliveryDataListCall<'a> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of entries to return. The service may return fewer than this value. If unspecified, at most 1,000 entries will be returned. The maximum value is 10,000; values above 10,000 will be capped to 10,000. This default may change over time.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> ProjectAndroidAppDeliveryDataListCall<'a> {
        self._page_size = Some(new_value);
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ProjectAndroidAppDeliveryDataListCall<'a> {
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
    pub fn param<T>(mut self, name: T, value: T) -> ProjectAndroidAppDeliveryDataListCall<'a>
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
    pub fn add_scope<T, S>(mut self, scope: T) -> ProjectAndroidAppDeliveryDataListCall<'a>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


