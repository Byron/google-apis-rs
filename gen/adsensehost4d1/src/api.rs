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
    /// View and manage your AdSense host data and associated accounts
    Full,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Full => "https://www.googleapis.com/auth/adsensehost",
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Scope {
    fn default() -> Scope {
        Scope::Full
    }
}

// ########
// HUB ###
// ######

/// Central instance to access all AdSenseHost related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
/// use adsensehost4d1::{Result, Error};
/// # async fn dox() {
/// use adsensehost4d1::{AdSenseHost, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdSenseHost::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.associationsessions().start(&vec!["amet.".into()], "websiteUrl")
///              .website_locale("ipsum")
///              .user_locale("gubergren")
///              .callback_url("Lorem")
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
pub struct AdSenseHost<C> {
    pub client: common::Client<C>,
    pub auth: Box<dyn common::GetToken>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<C> common::Hub for AdSenseHost<C> {}

impl<'a, C> AdSenseHost<C> {
    pub fn new<A: 'static + common::GetToken>(
        client: common::Client<C>,
        auth: A,
    ) -> AdSenseHost<C> {
        AdSenseHost {
            client,
            auth: Box::new(auth),
            _user_agent: "google-api-rust-client/8.0.0".to_string(),
            _base_url: "https://adsensehost.googleapis.com/".to_string(),
            _root_url: "https://adsensehost.googleapis.com/".to_string(),
        }
    }

    pub fn associationsessions(&'a self) -> AssociationsessionMethods<'a, C> {
        AssociationsessionMethods { hub: self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/8.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        std::mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://adsensehost.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        std::mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://adsensehost.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        std::mem::replace(&mut self._root_url, new_root_url)
    }
}

// ############
// SCHEMAS ###
// ##########
/// There is no detailed description.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [start associationsessions](AssociationsessionStartCall) (response)
/// * [verify associationsessions](AssociationsessionVerifyCall) (response)
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde_with::serde_as]
#[derive(Default, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssociationSession {
    /// Hosted account id of the associated publisher after association. Present if status is ACCEPTED.
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// Unique identifier of this association session.
    pub id: Option<String>,
    /// Kind of resource this is, in this case adsensehost#associationSession.
    pub kind: Option<String>,
    /// The products to associate with the user. Options: AFC, AFG, AFV, AFS (deprecated), AFMC (deprecated)
    #[serde(rename = "productCodes")]
    pub product_codes: Option<Vec<String>>,
    /// Redirect URL of this association session. Used to redirect users into the AdSense association flow.
    #[serde(rename = "redirectUrl")]
    pub redirect_url: Option<String>,
    /// Status of the completed association, available once the association callback token has been verified. One of ACCEPTED, REJECTED, or ERROR.
    pub status: Option<String>,
    /// The preferred locale of the user themselves when going through the AdSense association flow.
    #[serde(rename = "userLocale")]
    pub user_locale: Option<String>,
    /// The locale of the user's hosted website.
    #[serde(rename = "websiteLocale")]
    pub website_locale: Option<String>,
    /// The URL of the user's hosted website.
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
}

impl common::Resource for AssociationSession {}
impl common::ResponseResult for AssociationSession {}

// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *associationsession* resources.
/// It is not used directly, but through the [`AdSenseHost`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_adsensehost4d1 as adsensehost4d1;
///
/// # async fn dox() {
/// use adsensehost4d1::{AdSenseHost, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// let mut hub = AdSenseHost::new(client, auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `start(...)` and `verify(...)`
/// // to build up your call.
/// let rb = hub.associationsessions();
/// # }
/// ```
pub struct AssociationsessionMethods<'a, C>
where
    C: 'a,
{
    hub: &'a AdSenseHost<C>,
}

impl<'a, C> common::MethodsBuilder for AssociationsessionMethods<'a, C> {}

impl<'a, C> AssociationsessionMethods<'a, C> {
    /// Create a builder to help you perform the following task:
    ///
    /// Create an association session for initiating an association with an AdSense user.
    ///
    /// # Arguments
    ///
    /// * `productCode` - Products to associate with the user.
    /// * `websiteUrl` - The URL of the user's hosted website.
    pub fn start(
        &self,
        product_code: &Vec<String>,
        website_url: &str,
    ) -> AssociationsessionStartCall<'a, C> {
        AssociationsessionStartCall {
            hub: self.hub,
            _product_code: product_code.clone(),
            _website_url: website_url.to_string(),
            _website_locale: Default::default(),
            _user_locale: Default::default(),
            _callback_url: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }

    /// Create a builder to help you perform the following task:
    ///
    /// Verify an association session after the association callback returns from AdSense signup.
    ///
    /// # Arguments
    ///
    /// * `token` - The token returned to the association callback URL.
    pub fn verify(&self, token: &str) -> AssociationsessionVerifyCall<'a, C> {
        AssociationsessionVerifyCall {
            hub: self.hub,
            _token: token.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}

// ###################
// CallBuilders   ###
// #################

/// Create an association session for initiating an association with an AdSense user.
///
/// A builder for the *start* method supported by a *associationsession* resource.
/// It is not used directly, but through a [`AssociationsessionMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adsensehost4d1 as adsensehost4d1;
/// # async fn dox() {
/// # use adsensehost4d1::{AdSenseHost, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdSenseHost::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.associationsessions().start(&vec!["gubergren".into()], "websiteUrl")
///              .website_locale("dolor")
///              .user_locale("ea")
///              .callback_url("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct AssociationsessionStartCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdSenseHost<C>,
    _product_code: Vec<String>,
    _website_url: String,
    _website_locale: Option<String>,
    _user_locale: Option<String>,
    _callback_url: Option<String>,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AssociationsessionStartCall<'a, C> {}

impl<'a, C> AssociationsessionStartCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, AssociationSession)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adsensehost.associationsessions.start",
            http_method: hyper::Method::GET,
        });

        for &field in [
            "alt",
            "productCode",
            "websiteUrl",
            "websiteLocale",
            "userLocale",
            "callbackUrl",
        ]
        .iter()
        {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(7 + self._additional_params.len());
        if !self._product_code.is_empty() {
            for f in self._product_code.iter() {
                params.push("productCode", f);
            }
        }
        params.push("websiteUrl", self._website_url);
        if let Some(value) = self._website_locale.as_ref() {
            params.push("websiteLocale", value);
        }
        if let Some(value) = self._user_locale.as_ref() {
            params.push("userLocale", value);
        }
        if let Some(value) = self._callback_url.as_ref() {
            params.push("callbackUrl", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "adsensehost/v4.1/associationsessions/start";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
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

    /// Products to associate with the user.
    ///
    /// Append the given value to the *product code* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn add_product_code(mut self, new_value: &str) -> AssociationsessionStartCall<'a, C> {
        self._product_code.push(new_value.to_string());
        self
    }
    /// The URL of the user's hosted website.
    ///
    /// Sets the *website url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn website_url(mut self, new_value: &str) -> AssociationsessionStartCall<'a, C> {
        self._website_url = new_value.to_string();
        self
    }
    /// The locale of the user's hosted website.
    ///
    /// Sets the *website locale* query property to the given value.
    pub fn website_locale(mut self, new_value: &str) -> AssociationsessionStartCall<'a, C> {
        self._website_locale = Some(new_value.to_string());
        self
    }
    /// The preferred locale of the user.
    ///
    /// Sets the *user locale* query property to the given value.
    pub fn user_locale(mut self, new_value: &str) -> AssociationsessionStartCall<'a, C> {
        self._user_locale = Some(new_value.to_string());
        self
    }
    /// The URL to redirect the user to once association is completed. It receives a token parameter that can then be used to retrieve the associated account.
    ///
    /// Sets the *callback url* query property to the given value.
    pub fn callback_url(mut self, new_value: &str) -> AssociationsessionStartCall<'a, C> {
        self._callback_url = Some(new_value.to_string());
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
    ) -> AssociationsessionStartCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AssociationsessionStartCall<'a, C>
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
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AssociationsessionStartCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AssociationsessionStartCall<'a, C>
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
    pub fn clear_scopes(mut self) -> AssociationsessionStartCall<'a, C> {
        self._scopes.clear();
        self
    }
}

/// Verify an association session after the association callback returns from AdSense signup.
///
/// A builder for the *verify* method supported by a *associationsession* resource.
/// It is not used directly, but through a [`AssociationsessionMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_adsensehost4d1 as adsensehost4d1;
/// # async fn dox() {
/// # use adsensehost4d1::{AdSenseHost, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
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
/// # let mut hub = AdSenseHost::new(client, auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.associationsessions().verify("token")
///              .doit().await;
/// # }
/// ```
pub struct AssociationsessionVerifyCall<'a, C>
where
    C: 'a,
{
    hub: &'a AdSenseHost<C>,
    _token: String,
    _delegate: Option<&'a mut dyn common::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeSet<String>,
}

impl<'a, C> common::CallBuilder for AssociationsessionVerifyCall<'a, C> {}

impl<'a, C> AssociationsessionVerifyCall<'a, C>
where
    C: common::Connector,
{
    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> common::Result<(common::Response, AssociationSession)> {
        use std::borrow::Cow;
        use std::io::{Read, Seek};

        use common::{url::Params, ToParts};
        use hyper::header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, LOCATION, USER_AGENT};

        let mut dd = common::DefaultDelegate;
        let mut dlg: &mut dyn common::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(common::MethodInfo {
            id: "adsensehost.associationsessions.verify",
            http_method: hyper::Method::GET,
        });

        for &field in ["alt", "token"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(common::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());
        params.push("token", self._token);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "adsensehost/v4.1/associationsessions/verify";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Full.as_ref().to_string());
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

    /// The token returned to the association callback URL.
    ///
    /// Sets the *token* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn token(mut self, new_value: &str) -> AssociationsessionVerifyCall<'a, C> {
        self._token = new_value.to_string();
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
    ) -> AssociationsessionVerifyCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AssociationsessionVerifyCall<'a, C>
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
    /// [`Scope::Full`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AssociationsessionVerifyCall<'a, C>
    where
        St: AsRef<str>,
    {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AssociationsessionVerifyCall<'a, C>
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
    pub fn clear_scopes(mut self) -> AssociationsessionVerifyCall<'a, C> {
        self._scopes.clear();
        self
    }
}
