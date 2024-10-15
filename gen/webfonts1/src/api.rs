#![allow(clippy::ptr_arg)]

use std::collections::{BTreeSet, HashMap};

use tokio::time::sleep;

// ##############
// UTILITIES ###
// ############

// ########
// HUB ###
// ######

/// Central instance to access all Webfonts related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_webfonts1 as webfonts1;
/// use webfonts1::{Result, Error};
/// # async fn dox() {
/// use webfonts1::{Webfonts, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = Webfonts::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.webfonts().list()
///              .subset("amet.")
///              .sort("takimata")
///              .add_family("amet.")
///              .add_capability("duo")
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
pub struct Webfonts<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for Webfonts<C> {}

impl<'a, C> Webfonts<C> {
    pub fn new<A: 'static + common::GetToken>(client: common::Client<C>, auth: A) -> Webfonts<C> {
        Webfonts {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/6.0.0".to_string(),
            _base_url: "https://webfonts.googleapis.com/".to_string(),
            _root_url: "https://webfonts.googleapis.com/".to_string(),
        }
    }

    pub fn webfonts(&'a self) -> WebfontMethods<'a, C> {
        WebfontMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/6.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://webfonts.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://webfonts.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// Metadata for a variable font axis.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Axis {
    /// maximum value
    pub end: Option<f32>,
    /// minimum value
    pub start: Option<f32>,
    /// tag name.
    pub tag: Option<String>,
}

impl common::Part for Axis {}

/// Metadata describing a family of fonts.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list webfonts](WebfontListCall) (none)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Webfont {
    /// Axis for variable fonts.
    pub axes: Option<Vec<Axis>>,
    /// The category of the font.
    pub category: Option<String>,
    /// The color format(s) available for this family.
    #[serde(rename = "colorCapabilities")]
    pub color_capabilities: Option<Vec<String>>,
    /// The name of the font.
    pub family: Option<String>,
    /// The font files (with all supported scripts) for each one of the available variants, as a key : value map.
    pub files: Option<HashMap<String, String>>,
    /// This kind represents a webfont object in the webfonts service.
    pub kind: Option<String>,
    /// The date (format "yyyy-MM-dd") the font was modified for the last time.
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    /// Font URL for menu subset, a subset of the font that is enough to display the font name
    pub menu: Option<String>,
    /// The scripts supported by the font.
    pub subsets: Option<Vec<String>>,
    /// The available variants for the font.
    pub variants: Option<Vec<String>>,
    /// The font version.
    pub version: Option<String>,
}

impl common::Resource for Webfont {}

/// Response containing the list of fonts currently served by the Google Fonts API.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [list webfonts](WebfontListCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WebfontList {
    /// The list of fonts currently served by the Google Fonts API.
    pub items: Option<Vec<Webfont>>,
    /// This kind represents a list of webfont objects in the webfonts service.
    pub kind: Option<String>,
}

impl common::ResponseResult for WebfontList {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *webfont* resources.
/// It is not used directly, but through the [`Webfonts`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_webfonts1 as webfonts1;
///
/// # async fn dox() {
/// use webfonts1::{Webfonts, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = Webfonts::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)`
/// // to build up your call.
/// let rb = hub.webfonts();
/// # }
/// ```
pub struct WebfontMethods<'a, C>
where
    C: 'a,
{
    hub: &'a Webfonts<C>,
}

impl<'a, C> common::MethodsBuilder for WebfontMethods<'a, C> {}

impl<'a, C> WebfontMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves the list of fonts currently served by the Google Fonts Developer API.
    pub fn list(&self) -> WebfontListCall<'a, C> {
        WebfontListCall {
            hub: self.hub,
            _subset: Default::default(),
            _sort: Default::default(),
            _family: Default::default(),
            _capability: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Retrieves the list of fonts currently served by the Google Fonts Developer API.
///
/// A builder for the *list* method supported by a *webfont* resource.
/// It is not used directly, but through a [`WebfontMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_webfonts1 as webfonts1;
/// # async fn dox() {
/// # use webfonts1::{Webfonts, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = Webfonts::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.webfonts().list()
///              .subset("ipsum")
///              .sort("gubergren")
///              .add_family("Lorem")
///              .add_capability("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct WebfontListCall<'a, C>
where
    C: 'a,
{
    hub: &'a Webfonts<C>,
    _subset: Option<String>,
    _sort: Option<String>,
    _family: Vec<String>,
    _capability: Vec<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a, C> common::CallBuilder for WebfontListCall<'a, C> {}

impl<'a, C> WebfontListCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, WebfontList)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "webfonts.webfonts.list",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "subset", "sort", "family", "capability"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if let Some(value) = self._subset.as_ref() {
            params.push("subset", value);
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if !self._family.is_empty() {
            for f in self._family.iter() {
                params.push("family", f);
            }
        }
        if !self._capability.is_empty() {
            for f in self._capability.iter() {
                params.push("capability", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/webfonts";

        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(common::Error::MissingAPIKey);
            }
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

    /// Filters by Webfont.subset, if subset is found in Webfont.subsets. If not set, returns all families.
    ///
    /// Sets the *subset* query property to the given value.
    pub fn subset(mut self, new_value: &str) -> WebfontListCall<'a, C> {
        self._subset = Some(new_value.to_string());
        self
    }
    /// Enables sorting of the list.
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> WebfontListCall<'a, C> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Filters by Webfont.family, using literal match. If not set, returns all families
    ///
    /// Append the given value to the *family* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_family(mut self, new_value: &str) -> WebfontListCall<'a, C> {
        self._family.push(new_value.to_string());
        self
    }
    /// Controls the font urls in `Webfont.files`, by default, static ttf fonts are sent.
    ///
    /// Append the given value to the *capability* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_capability(mut self, new_value: &str) -> WebfontListCall<'a, C> {
        self._capability.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn common::Delegate) -> WebfontListCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> WebfontListCall<'a, C>
    where
        T: AsRef<str>,
    {
        self._additional_params
            .insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }
}
