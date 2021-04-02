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
    /// View and manage your Google Play Developer account
    Androidpublisher,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Androidpublisher => "https://www.googleapis.com/auth/androidpublisher",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::Androidpublisher
    }
}



// ########
// HUB ###
// ######

/// Central instance to access all Playcustomapp related resource activities
///
/// # Examples
///
/// Instantiate a new hub
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playcustomapp1 as playcustomapp1;
/// use playcustomapp1::api::CustomApp;
/// use playcustomapp1::{Result, Error};
/// use std::fs;
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use playcustomapp1::Playcustomapp;
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
/// let mut hub = Playcustomapp::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CustomApp::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().custom_apps_create(req, "account")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
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
pub struct Playcustomapp<C> {
    client: RefCell<C>,
    auth: RefCell<oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>>,
    _user_agent: String,
    _base_url: String,
    _root_url: String,
}

impl<'a, C> client::Hub for Playcustomapp<C> {}

impl<'a, C> Playcustomapp<C>
    where  C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {

    pub fn new(client: C, authenticator: oauth2::authenticator::Authenticator<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>) -> Playcustomapp<C> {
        Playcustomapp {
            client: RefCell::new(client),
            auth: RefCell::new(authenticator),
            _user_agent: "google-api-rust-client/2.0.0".to_string(),
            _base_url: "https://playcustomapp.googleapis.com/".to_string(),
            _root_url: "https://playcustomapp.googleapis.com/".to_string(),
        }
    }

    pub fn accounts(&'a self) -> AccountMethods<'a, C> {
        AccountMethods { hub: &self }
    }

    /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/2.0.0`.
    ///
    /// Returns the previously set user-agent.
    pub fn user_agent(&mut self, agent_name: String) -> String {
        mem::replace(&mut self._user_agent, agent_name)
    }

    /// Set the base url to use in all requests to the server.
    /// It defaults to `https://playcustomapp.googleapis.com/`.
    ///
    /// Returns the previously set base url.
    pub fn base_url(&mut self, new_base_url: String) -> String {
        mem::replace(&mut self._base_url, new_base_url)
    }

    /// Set the root url to use in all requests to the server.
    /// It defaults to `https://playcustomapp.googleapis.com/`.
    ///
    /// Returns the previously set root url.
    pub fn root_url(&mut self, new_root_url: String) -> String {
        mem::replace(&mut self._root_url, new_root_url)
    }
}


// ############
// SCHEMAS ###
// ##########
/// This resource represents a custom app.
/// 
/// # Activities
/// 
/// This type is used in activities, which are methods you may call on this type or where this type is involved in. 
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
/// 
/// * [custom apps create accounts](AccountCustomAppCreateCall) (request|response)
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CustomApp {
    /// Default listing language in BCP 47 format.
    #[serde(rename="languageCode")]
    pub language_code: Option<String>,
    /// Output only. Package name of the created Android app. Only present in the API response.
    #[serde(rename="packageName")]
    pub package_name: Option<String>,
    /// Title for the Android app.
    pub title: Option<String>,
}

impl client::RequestValue for CustomApp {}
impl client::ResponseResult for CustomApp {}



// ###################
// MethodBuilders ###
// #################

/// A builder providing access to all methods supported on *account* resources.
/// It is not used directly, but through the `Playcustomapp` hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate yup_oauth2 as oauth2;
/// extern crate google_playcustomapp1 as playcustomapp1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use oauth2;
/// use playcustomapp1::Playcustomapp;
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = Playcustomapp::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `custom_apps_create(...)`
/// // to build up your call.
/// let rb = hub.accounts();
/// # }
/// ```
pub struct AccountMethods<'a, C>
    where C: 'a {

    hub: &'a Playcustomapp<C>,
}

impl<'a, C> client::MethodsBuilder for AccountMethods<'a, C> {}

impl<'a, C> AccountMethods<'a, C> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Creates a new custom app.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `account` - Developer account ID.
    pub fn custom_apps_create(&self, request: CustomApp, account: &str) -> AccountCustomAppCreateCall<'a, C> {
        AccountCustomAppCreateCall {
            hub: self.hub,
            _request: request,
            _account: account.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}





// ###################
// CallBuilders   ###
// #################

/// Creates a new custom app.
///
/// A builder for the *customApps.create* method supported by a *account* resource.
/// It is not used directly, but through a `AccountMethods` instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate yup_oauth2 as oauth2;
/// # extern crate google_playcustomapp1 as playcustomapp1;
/// use playcustomapp1::api::CustomApp;
/// use std::fs;
/// # async fn dox() {
/// # use std::default::Default;
/// # use oauth2;
/// # use playcustomapp1::Playcustomapp;
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Playcustomapp::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CustomApp::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `upload(...)`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().custom_apps_create(req, "account")
///              .upload(fs::File::open("file.ext").unwrap(), "application/octet-stream".parse().unwrap()).await;
/// # }
/// ```
pub struct AccountCustomAppCreateCall<'a, C>
    where C: 'a {

    hub: &'a Playcustomapp<C>,
    _request: CustomApp,
    _account: String,
    _delegate: Option<&'a mut dyn client::Delegate>,
    _additional_params: HashMap<String, String>,
    _scopes: BTreeMap<String, ()>
}

impl<'a, C> client::CallBuilder for AccountCustomAppCreateCall<'a, C> {}

impl<'a, C> AccountCustomAppCreateCall<'a, C> where C: BorrowMut<hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>, hyper::body::Body>> {


    /// Perform the operation you have build so far.
    async fn doit<RS>(mut self, mut reader: RS, reader_mime_type: mime::Mime, protocol: &'static str) -> client::Result<(hyper::Response<hyper::body::Body>, CustomApp)>
		where RS: client::ReadSeek {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::ToParts;
        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = match self._delegate {
            Some(d) => d,
            None => &mut dd
        };
        dlg.begin(client::MethodInfo { id: "playcustomapp.accounts.customApps.create",
                               http_method: hyper::Method::POST });
        let mut params: Vec<(&str, String)> = Vec::with_capacity(4 + self._additional_params.len());
        params.push(("account", self._account.to_string()));
        for &field in ["alt", "account"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }
        for (name, value) in self._additional_params.iter() {
            params.push((&name, value.clone()));
        }

        params.push(("alt", "json".to_string()));

        let (mut url, upload_type) =
            if protocol == "resumable" {
                (self.hub._root_url.clone() + "resumable/upload/playcustomapp/v1/accounts/{account}/customApps", "resumable")
            } else if protocol == "simple" {
                (self.hub._root_url.clone() + "upload/playcustomapp/v1/accounts/{account}/customApps", "multipart")
            } else {
                unreachable!()
            };
        params.push(("uploadType", upload_type.to_string()));
        if self._scopes.len() == 0 {
            self._scopes.insert(Scope::Androidpublisher.as_ref().to_string(), ());
        }

        for &(find_this, param_name) in [("{account}", "account")].iter() {
            let mut replace_with: Option<&str> = None;
            for &(name, ref value) in params.iter() {
                if name == param_name {
                    replace_with = Some(value);
                    break;
                }
            }
            url = url.replace(find_this, replace_with.expect("to find substitution value in params"));
        }
        {
            let mut indices_for_removal: Vec<usize> = Vec::with_capacity(1);
            for param_name in ["account"].iter() {
                if let Some(index) = params.iter().position(|t| &t.0 == param_name) {
                    indices_for_removal.push(index);
                }
            }
            for &index in indices_for_removal.iter() {
                params.remove(index);
            }
        }

        let url = url::Url::parse_with_params(&url, params).unwrap();

        let mut json_mime_type: mime::Mime = "application/json".parse().unwrap();
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

        let mut should_ask_dlg_for_url = false;
        let mut upload_url_from_server;
        let mut upload_url: Option<String> = None;

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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                if should_ask_dlg_for_url && (upload_url = dlg.upload_url()) == () && upload_url.is_some() {
                    should_ask_dlg_for_url = false;
                    upload_url_from_server = false;
                    let url = upload_url.as_ref().and_then(|s| Some(url::Url::parse(s).unwrap())).unwrap();
                    Ok(hyper::Response::builder()
                        .status(hyper::StatusCode::OK)
                        .header("Localtion", upload_url.as_ref().unwrap().clone())
                        .body(hyper::body::Body::empty())
                        .unwrap())
                } else {
                    let mut mp_reader: client::MultiPartReader = Default::default();
                    let (mut body_reader, content_type) = match protocol {
                        "simple" => {
                            mp_reader.reserve_exact(2);
                            let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 10737418240 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 10737418240))
                        }
                            mp_reader.add_part(&mut request_value_reader, request_size, json_mime_type.clone())
                                     .add_part(&mut reader, size, reader_mime_type.clone());
                            let mime_type = mp_reader.mime_type();
                            (&mut mp_reader as &mut dyn io::Read, (CONTENT_TYPE, mime_type.to_string()))
                        },
                        _ => (&mut request_value_reader as &mut dyn io::Read, (CONTENT_TYPE, json_mime_type.to_string())),
                    };
                    let mut client = &mut *self.hub.client.borrow_mut();
                    dlg.pre_request();
                    let mut req_builder = hyper::Request::builder().method(hyper::Method::POST).uri(url.clone().into_string())
                            .header(USER_AGENT, self.hub._user_agent.clone())                            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()));
    
                    upload_url_from_server = true;
                    if protocol == "resumable" {
                        req_builder = req_builder.header("X-Upload-Content-Type", format!("{}", reader_mime_type));
                    }
    
                            let mut body_reader_bytes = vec![];
                            body_reader.read_to_end(&mut body_reader_bytes).unwrap();
                            let request = req_builder
                            .header(content_type.0, content_type.1.to_string())
                            .body(hyper::body::Body::from(body_reader_bytes));
    
                    client.borrow_mut().request(request.unwrap()).await
                    
                }
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

                        let json_server_error = json::from_str::<client::JsonServerError>(&res_body_string).ok();
                        let server_error = json::from_str::<client::ServerError>(&res_body_string)
                            .or_else(|_| json::from_str::<client::ErrorResponse>(&res_body_string).map(|r| r.error))
                            .ok();

                        if let client::Retry::After(d) = dlg.http_failure(&res,
                                                              json_server_error,
                                                              server_error) {
                            sleep(d);
                            continue;
                        }
                        dlg.finished(false);
                        return match json::from_str::<client::ErrorResponse>(&res_body_string){
                            Err(_) => Err(client::Error::Failure(res)),
                            Ok(serr) => Err(client::Error::BadRequest(serr))
                        }
                    }
                    if protocol == "resumable" {
                        let size = reader.seek(io::SeekFrom::End(0)).unwrap();
                        reader.seek(io::SeekFrom::Start(0)).unwrap();
                        if size > 10737418240 {
                        	return Err(client::Error::UploadSizeLimitExceeded(size, 10737418240))
                        }
                        let mut client = &mut *self.hub.client.borrow_mut();
                        let upload_result = {
                            let url_str = &res.headers().get("Location").expect("LOCATION header is part of protocol").to_str().unwrap();
                            if upload_url_from_server {
                                dlg.store_upload_url(Some(url_str));
                            }

                            client::ResumableUploadHelper {
                                client: &mut client.borrow_mut(),
                                delegate: dlg,
                                start_at: if upload_url_from_server { Some(0) } else { None },
                                auth: &mut *self.hub.auth.borrow_mut(),
                                user_agent: &self.hub._user_agent,
                                auth_header: format!("Bearer {}", token.as_str()),
                                url: url_str,
                                reader: &mut reader,
                                media_type: reader_mime_type.clone(),
                                content_length: size
                            }.upload().await
                        };
                        match upload_result {
                            None => {
                                dlg.finished(false);
                                return Err(client::Error::Cancelled)
                            }
                            Some(Err(err)) => {
                                dlg.finished(false);
                                return Err(client::Error::HttpError(err))
                            }
                            Some(Ok(upload_result)) => {
                                res = upload_result;
                                if !res.status().is_success() {
                                    dlg.store_upload_url(None);
                                    dlg.finished(false);
                                    return Err(client::Error::Failure(res))
                                }
                            }
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

    /// Upload media in a resumable fashion.
    /// Even if the upload fails or is interrupted, it can be resumed for a
    /// certain amount of time as the server maintains state temporarily.
    /// 
    /// The delegate will be asked for an `upload_url()`, and if not provided, will be asked to store an upload URL
    /// that was provided by the server, using `store_upload_url(...)`. The upload will be done in chunks, the delegate
    /// may specify the `chunk_size()` and may cancel the operation before each chunk is uploaded, using
    /// `cancel_chunk_upload(...)`.
    ///
    /// * *multipart*: yes
    /// * *max size*: 10737418240
    /// * *valid mime types*: '*/*'
    pub async fn upload_resumable<RS>(self, resumeable_stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, CustomApp)>
                where RS: client::ReadSeek {
        self.doit(resumeable_stream, mime_type, "resumable").await
    }
    /// Upload media all at once.
    /// If the upload fails for whichever reason, all progress is lost.
    ///
    /// * *multipart*: yes
    /// * *max size*: 10737418240
    /// * *valid mime types*: '*/*'
    pub async fn upload<RS>(self, stream: RS, mime_type: mime::Mime) -> client::Result<(hyper::Response<hyper::body::Body>, CustomApp)>
                where RS: client::ReadSeek {
        self.doit(stream, mime_type, "simple").await
    }

    ///
    /// Sets the *request* property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn request(mut self, new_value: CustomApp) -> AccountCustomAppCreateCall<'a, C> {
        self._request = new_value;
        self
    }
    /// Developer account ID.
    ///
    /// Sets the *account* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account(mut self, new_value: &str) -> AccountCustomAppCreateCall<'a, C> {
        self._account = new_value.to_string();
        self
    }
    /// The delegate implementation is consulted whenever there is an intermediate result, or if something goes wrong
    /// while executing the actual API request.
    /// 
    /// It should be used to handle progress information, and to implement a certain level of resilience.
    ///
    /// Sets the *delegate* property to the given value.
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountCustomAppCreateCall<'a, C> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountCustomAppCreateCall<'a, C>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead the default `Scope` variant
    /// `Scope::Androidpublisher`.
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
    pub fn add_scope<T, S>(mut self, scope: T) -> AccountCustomAppCreateCall<'a, C>
                                                        where T: Into<Option<S>>,
                                                              S: AsRef<str> {
        match scope.into() {
          Some(scope) => self._scopes.insert(scope.as_ref().to_string(), ()),
          None => None,
        };
        self
    }
}


