#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

// ########
// HUB ###
// ######

/// Central instance to access all MyBusinessNotificationSettings related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessnotifications1 as mybusinessnotifications1;
/// use mybusinessnotifications1::api::NotificationSetting;
/// use mybusinessnotifications1::{Result, Error};
/// # async fn dox() {
/// use mybusinessnotifications1::{MyBusinessNotificationSettings, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = MyBusinessNotificationSettings::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = NotificationSetting::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().update_notification_setting(req, "name")
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
pub struct MyBusinessNotificationSettings<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for MyBusinessNotificationSettings<C> {}

impl<'a, C> MyBusinessNotificationSettings<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> MyBusinessNotificationSettings<C> {
        MyBusinessNotificationSettings {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://mybusinessnotifications.googleapis.com/".to_string(),
            _root_url: "https://mybusinessnotifications.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C> {
        AccountMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://mybusinessnotifications.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://mybusinessnotifications.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// A Google Pub/Sub topic where notifications can be published when a location is updated or has a new review. There will be only one notification setting resource per-account.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [get notification setting accounts](AccountGetNotificationSettingCall) (response)
/// * [update notification setting accounts](AccountUpdateNotificationSettingCall) (request|response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NotificationSetting {
    /// Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`.
    pub name: Option<String>,
    /// The types of notifications that will be sent to the Pub/Sub topic. To stop receiving notifications entirely, use NotificationSettings.UpdateNotificationSetting with an empty notification_types or set the pubsub_topic to an empty string.
    #[serde(rename = "notificationTypes")]
    pub notification_types: Option<Vec<String>>,
    /// Optional. The Google Pub/Sub topic that will receive notifications when locations managed by this account are updated. If unset, no notifications will be posted. The account mybusiness-api-pubsub@system.gserviceaccount.com must have at least Publish permissions on the Pub/Sub topic.
    #[serde(rename = "pubsubTopic")]
    pub pubsub_topic: Option<String>,
}

impl common::RequestValue for NotificationSetting {}
impl common::ResponseResult for NotificationSetting {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the [`MyBusinessNotificationSettings`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_mybusinessnotifications1 as mybusinessnotifications1;
///
/// # async fn dox() {
/// use mybusinessnotifications1::{MyBusinessNotificationSettings, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = MyBusinessNotificationSettings::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_notification_setting(...)` and `update_notification_setting(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C>
where
    C: 'a,
{
    hub: &'a MyBusinessNotificationSettings<C>,
}

impl<'a, C> common::MethodsBuilder for AccountMethods<'a, C> {}

impl<'a, C> AccountMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Returns the pubsub notification settings for the account.
    ///
    /// # Arguments
    ///
    /// * `name` - Required. The resource name of the notification setting we are trying to fetch.
    pub fn get_notification_setting(&self, name: &str) -> AccountGetNotificationSettingCall<'a, C> {
        AccountGetNotificationSettingCall {
            hub: self.hub,
            _name: name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Sets the pubsub notification setting for the account informing Google which topic to send pubsub notifications for. Use the notification_types field within notification_setting to manipulate the events an account wants to subscribe to. An account will only have one notification setting resource, and only one pubsub topic can be set. To delete the setting, update with an empty notification_types
    ///
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `name` - Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`.
    pub fn update_notification_setting(
        &self,
        request: NotificationSetting,
        name: &str,
    ) -> AccountUpdateNotificationSettingCall<'a, C> {
        AccountUpdateNotificationSettingCall {
            hub: self.hub,
            _request: request,
            _name: name.to_string(),
            _update_mask: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Returns the pubsub notification settings for the account.
///
/// A builder for the *getNotificationSetting* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_mybusinessnotifications1 as mybusinessnotifications1;
/// # async fn dox() {
/// # use mybusinessnotifications1::{MyBusinessNotificationSettings, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = MyBusinessNotificationSettings::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().get_notification_setting("name")
///              .doit().await;
/// # }
/// ```
pub struct AccountGetNotificationSettingCall<'a, C>
where
    C: 'a,
{
    hub: &'a MyBusinessNotificationSettings<C>,
    _name: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for AccountGetNotificationSettingCall<'a, C> {}

impl<'a, C> AccountGetNotificationSettingCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, NotificationSetting)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "mybusinessnotifications.accounts.getNotificationSetting",
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

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
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
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::GET)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());

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

    /// Required. The resource name of the notification setting we are trying to fetch.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountGetNotificationSettingCall<'a, C> {
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
    ) -> AccountGetNotificationSettingCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountGetNotificationSettingCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}

/// Sets the pubsub notification setting for the account informing Google which topic to send pubsub notifications for. Use the notification_types field within notification_setting to manipulate the events an account wants to subscribe to. An account will only have one notification setting resource, and only one pubsub topic can be set. To delete the setting, update with an empty notification_types
///
/// A builder for the *updateNotificationSetting* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_mybusinessnotifications1 as mybusinessnotifications1;
/// use mybusinessnotifications1::api::NotificationSetting;
/// # async fn dox() {
/// # use mybusinessnotifications1::{MyBusinessNotificationSettings, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = MyBusinessNotificationSettings::new(client, auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = NotificationSetting::default();
///
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().update_notification_setting(req, "name")
///              .update_mask(FieldMask::new::<&str>(&[]))
///              .doit().await;
/// # }
/// ```
pub struct AccountUpdateNotificationSettingCall<'a, C>
where
    C: 'a,
{
    hub: &'a MyBusinessNotificationSettings<C>,
    _request: NotificationSetting,
    _name: String,
    _update_mask: Option<common::FieldMask>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for AccountUpdateNotificationSettingCall<'a, C> {}

impl<'a, C> AccountUpdateNotificationSettingCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, NotificationSetting)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "mybusinessnotifications.accounts.updateNotificationSetting",
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

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
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
        new_value: NotificationSetting,
    ) -> AccountUpdateNotificationSettingCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Required. The resource name this setting is for. This is of the form `accounts/{account_id}/notificationSetting`.
    ///
    /// Sets the *name* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn name(mut self, new_value: &str) -> AccountUpdateNotificationSettingCall<'a, C> {
        self._name = new_value.to_string();
        self
    }
    /// Required. The specific fields that should be updated. The only editable field is notification_setting.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(
        mut self,
        new_value: common::FieldMask,
    ) -> AccountUpdateNotificationSettingCall<'a, C> {
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
    ) -> AccountUpdateNotificationSettingCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountUpdateNotificationSettingCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
