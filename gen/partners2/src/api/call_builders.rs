use super::*;
/// Logs a user event.
///
/// A builder for the *log* method supported by a *userEvent* resource.
/// It is not used directly, but through a [`UserEventMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::LogUserEventRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogUserEventRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_events().log(req)
///              .doit().await;
/// # }
/// ```
pub struct UserEventLogCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: LogUserEventRequest,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserEventLogCall<'a, S> {}

impl<'a, S> UserEventLogCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, LogUserEventResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.userEvents.log",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/userEvents:log";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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
    pub fn request(mut self, new_value: LogUserEventRequest) -> UserEventLogCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserEventLogCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserEventLogCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Logs a generic message from the client, such as
/// `Failed to render component`, `Profile page is running slow`,
/// `More than 500 users have accessed this result.`, etc.
///
/// A builder for the *log* method supported by a *clientMessage* resource.
/// It is not used directly, but through a [`ClientMessageMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::LogMessageRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = LogMessageRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.client_messages().log(req)
///              .doit().await;
/// # }
/// ```
pub struct ClientMessageLogCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: LogMessageRequest,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for ClientMessageLogCall<'a, S> {}

impl<'a, S> ClientMessageLogCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, LogMessageResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.clientMessages.log",
                               http_method: hyper::Method::POST });

        for &field in ["alt"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(3 + self._additional_params.len());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/clientMessages:log";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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
    pub fn request(mut self, new_value: LogMessageRequest) -> ClientMessageLogCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ClientMessageLogCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> ClientMessageLogCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists advertiser leads for a user's associated company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *list* method supported by a *lead* resource.
/// It is not used directly, but through a [`LeadMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.leads().list()
///              .request_metadata_user_overrides_user_id("sed")
///              .request_metadata_user_overrides_ip_address("ut")
///              .request_metadata_traffic_source_traffic_sub_id("gubergren")
///              .request_metadata_traffic_source_traffic_source_id("rebum.")
///              .request_metadata_partners_session_id("est")
///              .request_metadata_locale("ipsum")
///              .add_request_metadata_experiment_ids("ipsum")
///              .page_token("est")
///              .page_size(-62)
///              .order_by("ea")
///              .doit().await;
/// # }
/// ```
pub struct LeadListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _order_by: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for LeadListCall<'a, S> {}

impl<'a, S> LeadListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListLeadsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.leads.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/leads";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListLeadsResponse.next_page_token`
    /// returned from the previous call to
    /// ListLeads.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer leads than requested.
    /// If unspecified, server picks an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> LeadListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// How to order Leads. Currently, only `create_time`
    /// and `create_time desc` are supported
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> LeadListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LeadListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> LeadListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists the Historical Offers for the current user (or user's entire company)
///
/// A builder for the *history.list* method supported by a *offer* resource.
/// It is not used directly, but through a [`OfferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.offers().history_list()
///              .request_metadata_user_overrides_user_id("dolor")
///              .request_metadata_user_overrides_ip_address("Lorem")
///              .request_metadata_traffic_source_traffic_sub_id("eos")
///              .request_metadata_traffic_source_traffic_source_id("labore")
///              .request_metadata_partners_session_id("sed")
///              .request_metadata_locale("duo")
///              .add_request_metadata_experiment_ids("sed")
///              .page_token("no")
///              .page_size(-15)
///              .order_by("kasd")
///              .entire_company(true)
///              .doit().await;
/// # }
/// ```
pub struct OfferHistoryListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _order_by: Option<String>,
   pub(super) _entire_company: Option<bool>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for OfferHistoryListCall<'a, S> {}

impl<'a, S> OfferHistoryListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListOffersHistoryResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.offers.history.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy", "entireCompany"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._entire_company.as_ref() {
            params.push("entireCompany", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/offers/history";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// Token to retrieve a specific page.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Maximum number of rows to return per page.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> OfferHistoryListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Comma-separated list of fields to order by, e.g.: “foo,bar,baz”.
    /// Use “foo desc” to sort descending.
    /// List of valid field names is: name, offer_code, expiration_time, status,
    /// last_modified_time, sender_name, creation_time, country_code,
    /// offer_type.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> OfferHistoryListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// if true, show history for the entire company.  Requires user to be admin.
    ///
    /// Sets the *entire company* query property to the given value.
    pub fn entire_company(mut self, new_value: bool) -> OfferHistoryListCall<'a, S> {
        self._entire_company = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OfferHistoryListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> OfferHistoryListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists the Offers available for the current user
///
/// A builder for the *list* method supported by a *offer* resource.
/// It is not used directly, but through a [`OfferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.offers().list()
///              .request_metadata_user_overrides_user_id("et")
///              .request_metadata_user_overrides_ip_address("et")
///              .request_metadata_traffic_source_traffic_sub_id("vero")
///              .request_metadata_traffic_source_traffic_source_id("erat")
///              .request_metadata_partners_session_id("sed")
///              .request_metadata_locale("duo")
///              .add_request_metadata_experiment_ids("dolore")
///              .doit().await;
/// # }
/// ```
pub struct OfferListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for OfferListCall<'a, S> {}

impl<'a, S> OfferListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListOffersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.offers.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/offers";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> OfferListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> OfferListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> OfferListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists analytics data for a user's associated company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *list* method supported by a *analytic* resource.
/// It is not used directly, but through a [`AnalyticMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.analytics().list()
///              .request_metadata_user_overrides_user_id("et")
///              .request_metadata_user_overrides_ip_address("voluptua.")
///              .request_metadata_traffic_source_traffic_sub_id("amet.")
///              .request_metadata_traffic_source_traffic_source_id("consetetur")
///              .request_metadata_partners_session_id("diam")
///              .request_metadata_locale("dolor")
///              .add_request_metadata_experiment_ids("et")
///              .page_token("et")
///              .page_size(-95)
///              .doit().await;
/// # }
/// ```
pub struct AnalyticListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for AnalyticListCall<'a, S> {}

impl<'a, S> AnalyticListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListAnalyticsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.analytics.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/analytics";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListAnalyticsResponse.next_page_token`
    /// returned from the previous call to
    /// ListAnalytics.
    /// Will be a date string in `YYYY-MM-DD` format representing the end date
    /// of the date range of results to return.
    /// If unspecified or set to "", default value is the current date.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AnalyticListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer analytics than requested.
    /// If unspecified or set to 0, default value is 30.
    /// Specifies the number of days in the date range when querying analytics.
    /// The `page_token` represents the end date of the date range
    /// and the start date is calculated using the `page_size` as the number
    /// of days BEFORE the end date.
    /// Must be a non-negative integer.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AnalyticListCall<'a, S> {
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AnalyticListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> AnalyticListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists states for current user.
///
/// A builder for the *list* method supported by a *userState* resource.
/// It is not used directly, but through a [`UserStateMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.user_states().list()
///              .request_metadata_user_overrides_user_id("Stet")
///              .request_metadata_user_overrides_ip_address("dolor")
///              .request_metadata_traffic_source_traffic_sub_id("duo")
///              .request_metadata_traffic_source_traffic_source_id("vero")
///              .request_metadata_partners_session_id("vero")
///              .request_metadata_locale("invidunt")
///              .add_request_metadata_experiment_ids("Stet")
///              .doit().await;
/// # }
/// ```
pub struct UserStateListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserStateListCall<'a, S> {}

impl<'a, S> UserStateListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListUserStatesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.userStates.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/userStates";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserStateListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserStateListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserStateListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Updates the specified lead.
///
/// A builder for the *updateLeads* method.
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::Lead;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Lead::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().update_leads(req)
///              .update_mask(&Default::default())
///              .request_metadata_user_overrides_user_id("vero")
///              .request_metadata_user_overrides_ip_address("elitr")
///              .request_metadata_traffic_source_traffic_sub_id("Lorem")
///              .request_metadata_traffic_source_traffic_source_id("diam")
///              .request_metadata_partners_session_id("no")
///              .request_metadata_locale("ipsum")
///              .add_request_metadata_experiment_ids("accusam")
///              .doit().await;
/// # }
/// ```
pub struct MethodUpdateLeadCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: Lead,
   pub(super) _update_mask: Option<client::FieldMask>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for MethodUpdateLeadCall<'a, S> {}

impl<'a, S> MethodUpdateLeadCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Lead)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.updateLeads",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "updateMask", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/leads";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
    pub fn request(mut self, new_value: Lead) -> MethodUpdateLeadCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Standard field mask for the set of fields to be updated.
    /// Required with at least 1 value in FieldMask's paths.
    /// Only `state` and `adwords_customer_id` are currently supported.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: client::FieldMask) -> MethodUpdateLeadCall<'a, S> {
        self._update_mask = Some(new_value);
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodUpdateLeadCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodUpdateLeadCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> MethodUpdateLeadCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Update company.
/// Should only be called within the context of an authorized logged in user.
///
/// A builder for the *updateCompanies* method.
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::Company;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Company::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().update_companies(req)
///              .update_mask(&Default::default())
///              .request_metadata_user_overrides_user_id("takimata")
///              .request_metadata_user_overrides_ip_address("consetetur")
///              .request_metadata_traffic_source_traffic_sub_id("voluptua.")
///              .request_metadata_traffic_source_traffic_source_id("et")
///              .request_metadata_partners_session_id("erat")
///              .request_metadata_locale("consetetur")
///              .add_request_metadata_experiment_ids("amet.")
///              .doit().await;
/// # }
/// ```
pub struct MethodUpdateCompanyCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: Company,
   pub(super) _update_mask: Option<client::FieldMask>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for MethodUpdateCompanyCall<'a, S> {}

impl<'a, S> MethodUpdateCompanyCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Company)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.updateCompanies",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "updateMask", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._update_mask.as_ref() {
            params.push("updateMask", value.to_string());
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/companies";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
    pub fn request(mut self, new_value: Company) -> MethodUpdateCompanyCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Standard field mask for the set of fields to be updated.
    /// Required with at least 1 value in FieldMask's paths.
    ///
    /// Sets the *update mask* query property to the given value.
    pub fn update_mask(mut self, new_value: client::FieldMask) -> MethodUpdateCompanyCall<'a, S> {
        self._update_mask = Some(new_value);
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodUpdateCompanyCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodUpdateCompanyCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> MethodUpdateCompanyCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets Partners Status of the logged in user's agency.
/// Should only be called if the logged in user is the admin of the agency.
///
/// A builder for the *getPartnersstatus* method.
/// It is not used directly, but through a [`MethodMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.methods().get_partnersstatus()
///              .request_metadata_user_overrides_user_id("sed")
///              .request_metadata_user_overrides_ip_address("takimata")
///              .request_metadata_traffic_source_traffic_sub_id("dolores")
///              .request_metadata_traffic_source_traffic_source_id("gubergren")
///              .request_metadata_partners_session_id("et")
///              .request_metadata_locale("accusam")
///              .add_request_metadata_experiment_ids("voluptua.")
///              .doit().await;
/// # }
/// ```
pub struct MethodGetPartnersstatuCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for MethodGetPartnersstatuCall<'a, S> {}

impl<'a, S> MethodGetPartnersstatuCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetPartnersStatusResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.getPartnersstatus",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/partnersstatus";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> MethodGetPartnersstatuCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> MethodGetPartnersstatuCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> MethodGetPartnersstatuCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates an advertiser lead for the given company ID.
///
/// A builder for the *leads.create* method supported by a *company* resource.
/// It is not used directly, but through a [`CompanyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::CreateLeadRequest;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CreateLeadRequest::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().leads_create(req, "companyId")
///              .doit().await;
/// # }
/// ```
pub struct CompanyLeadCreateCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: CreateLeadRequest,
   pub(super) _company_id: String,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CompanyLeadCreateCall<'a, S> {}

impl<'a, S> CompanyLeadCreateCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CreateLeadResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.companies.leads.create",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "companyId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("companyId", self._company_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/companies/{companyId}/leads";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["companyId"];
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::POST)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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
    pub fn request(mut self, new_value: CreateLeadRequest) -> CompanyLeadCreateCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the company to contact.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyLeadCreateCall<'a, S> {
        self._company_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CompanyLeadCreateCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> CompanyLeadCreateCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a company.
///
/// A builder for the *get* method supported by a *company* resource.
/// It is not used directly, but through a [`CompanyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().get("companyId")
///              .view("dolore")
///              .request_metadata_user_overrides_user_id("voluptua.")
///              .request_metadata_user_overrides_ip_address("amet.")
///              .request_metadata_traffic_source_traffic_sub_id("ea")
///              .request_metadata_traffic_source_traffic_source_id("sadipscing")
///              .request_metadata_partners_session_id("Lorem")
///              .request_metadata_locale("invidunt")
///              .add_request_metadata_experiment_ids("no")
///              .order_by("est")
///              .currency_code("At")
///              .address("sed")
///              .doit().await;
/// # }
/// ```
pub struct CompanyGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _company_id: String,
   pub(super) _view: Option<String>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _order_by: Option<String>,
   pub(super) _currency_code: Option<String>,
   pub(super) _address: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CompanyGetCall<'a, S> {}

impl<'a, S> CompanyGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GetCompanyResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.companies.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "companyId", "view", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "orderBy", "currencyCode", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(14 + self._additional_params.len());
        params.push("companyId", self._company_id);
        if let Some(value) = self._view.as_ref() {
            params.push("view", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._currency_code.as_ref() {
            params.push("currencyCode", value);
        }
        if let Some(value) = self._address.as_ref() {
            params.push("address", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/companies/{companyId}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{companyId}", "companyId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["companyId"];
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
                        .body(hyper::body::Body::empty());

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


    /// The ID of the company to retrieve.
    ///
    /// Sets the *company id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn company_id(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._company_id = new_value.to_string();
        self
    }
    /// The view of `Company` resource to be returned. This must not be
    /// `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._view = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// How to order addresses within the returned company. Currently, only
    /// `address` and `address desc` is supported which will sorted by closest to
    /// farthest in distance from given address and farthest to closest distance
    /// from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// If the company's budget is in a different currency code than this one, then
    /// the converted budget is converted to this currency code.
    ///
    /// Sets the *currency code* query property to the given value.
    pub fn currency_code(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._currency_code = Some(new_value.to_string());
        self
    }
    /// The address to use for sorting the company's addresses by proximity.
    /// If not given, the geo-located address of the request is used.
    /// Used when order_by is set.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyGetCall<'a, S> {
        self._address = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CompanyGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> CompanyGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Lists companies.
///
/// A builder for the *list* method supported by a *company* resource.
/// It is not used directly, but through a [`CompanyMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.companies().list()
///              .website_url("sit")
///              .view("et")
///              .add_specializations("tempor")
///              .add_services("aliquyam")
///              .request_metadata_user_overrides_user_id("ipsum")
///              .request_metadata_user_overrides_ip_address("et")
///              .request_metadata_traffic_source_traffic_sub_id("sanctus")
///              .request_metadata_traffic_source_traffic_source_id("Lorem")
///              .request_metadata_partners_session_id("est")
///              .request_metadata_locale("sed")
///              .add_request_metadata_experiment_ids("diam")
///              .page_token("dolores")
///              .page_size(-69)
///              .order_by("et")
///              .min_monthly_budget_units(-93)
///              .min_monthly_budget_nanos(-11)
///              .min_monthly_budget_currency_code("et")
///              .max_monthly_budget_units(-94)
///              .max_monthly_budget_nanos(-80)
///              .max_monthly_budget_currency_code("no")
///              .add_language_codes("nonumy")
///              .add_industries("At")
///              .add_gps_motivations("sadipscing")
///              .company_name("aliquyam")
///              .address("dolores")
///              .doit().await;
/// # }
/// ```
pub struct CompanyListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _website_url: Option<String>,
   pub(super) _view: Option<String>,
   pub(super) _specializations: Vec<String>,
   pub(super) _services: Vec<String>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _order_by: Option<String>,
   pub(super) _min_monthly_budget_units: Option<i64>,
   pub(super) _min_monthly_budget_nanos: Option<i32>,
   pub(super) _min_monthly_budget_currency_code: Option<String>,
   pub(super) _max_monthly_budget_units: Option<i64>,
   pub(super) _max_monthly_budget_nanos: Option<i32>,
   pub(super) _max_monthly_budget_currency_code: Option<String>,
   pub(super) _language_codes: Vec<String>,
   pub(super) _industries: Vec<String>,
   pub(super) _gps_motivations: Vec<String>,
   pub(super) _company_name: Option<String>,
   pub(super) _address: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CompanyListCall<'a, S> {}

impl<'a, S> CompanyListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListCompaniesResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.companies.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "websiteUrl", "view", "specializations", "services", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds", "pageToken", "pageSize", "orderBy", "minMonthlyBudget.units", "minMonthlyBudget.nanos", "minMonthlyBudget.currencyCode", "maxMonthlyBudget.units", "maxMonthlyBudget.nanos", "maxMonthlyBudget.currencyCode", "languageCodes", "industries", "gpsMotivations", "companyName", "address"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(27 + self._additional_params.len());
        if let Some(value) = self._website_url.as_ref() {
            params.push("websiteUrl", value);
        }
        if let Some(value) = self._view.as_ref() {
            params.push("view", value);
        }
        if self._specializations.len() > 0 {
            for f in self._specializations.iter() {
                params.push("specializations", f);
            }
        }
        if self._services.len() > 0 {
            for f in self._services.iter() {
                params.push("services", f);
            }
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._order_by.as_ref() {
            params.push("orderBy", value);
        }
        if let Some(value) = self._min_monthly_budget_units.as_ref() {
            params.push("minMonthlyBudget.units", value.to_string());
        }
        if let Some(value) = self._min_monthly_budget_nanos.as_ref() {
            params.push("minMonthlyBudget.nanos", value.to_string());
        }
        if let Some(value) = self._min_monthly_budget_currency_code.as_ref() {
            params.push("minMonthlyBudget.currencyCode", value);
        }
        if let Some(value) = self._max_monthly_budget_units.as_ref() {
            params.push("maxMonthlyBudget.units", value.to_string());
        }
        if let Some(value) = self._max_monthly_budget_nanos.as_ref() {
            params.push("maxMonthlyBudget.nanos", value.to_string());
        }
        if let Some(value) = self._max_monthly_budget_currency_code.as_ref() {
            params.push("maxMonthlyBudget.currencyCode", value);
        }
        if self._language_codes.len() > 0 {
            for f in self._language_codes.iter() {
                params.push("languageCodes", f);
            }
        }
        if self._industries.len() > 0 {
            for f in self._industries.iter() {
                params.push("industries", f);
            }
        }
        if self._gps_motivations.len() > 0 {
            for f in self._gps_motivations.iter() {
                params.push("gpsMotivations", f);
            }
        }
        if let Some(value) = self._company_name.as_ref() {
            params.push("companyName", value);
        }
        if let Some(value) = self._address.as_ref() {
            params.push("address", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/companies";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
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
                        .body(hyper::body::Body::empty());

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


    /// Website URL that will help to find a better matched company.
    /// .
    ///
    /// Sets the *website url* query property to the given value.
    pub fn website_url(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._website_url = Some(new_value.to_string());
        self
    }
    /// The view of the `Company` resource to be returned. This must not be
    /// `COMPANY_VIEW_UNSPECIFIED`.
    ///
    /// Sets the *view* query property to the given value.
    pub fn view(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._view = Some(new_value.to_string());
        self
    }
    /// List of specializations that the returned agencies should provide. If this
    /// is not empty, any returned agency must have at least one of these
    /// specializations, or one of the services in the "services" field.
    ///
    /// Append the given value to the *specializations* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_specializations(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._specializations.push(new_value.to_string());
        self
    }
    /// List of services that the returned agencies should provide. If this is
    /// not empty, any returned agency must have at least one of these services,
    /// or one of the specializations in the "specializations" field.
    ///
    /// Append the given value to the *services* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_services(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._services.push(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
        self
    }
    /// A token identifying a page of results that the server returns.
    /// Typically, this is the value of `ListCompaniesResponse.next_page_token`
    /// returned from the previous call to
    /// ListCompanies.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Requested page size. Server may return fewer companies than requested.
    /// If unspecified, server picks an appropriate default.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> CompanyListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// How to order addresses within the returned companies. Currently, only
    /// `address` and `address desc` is supported which will sorted by closest to
    /// farthest in distance from given address and farthest to closest distance
    /// from given address respectively.
    ///
    /// Sets the *order by* query property to the given value.
    pub fn order_by(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._order_by = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *min monthly budget.units* query property to the given value.
    pub fn min_monthly_budget_units(mut self, new_value: i64) -> CompanyListCall<'a, S> {
        self._min_monthly_budget_units = Some(new_value);
        self
    }
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *min monthly budget.nanos* query property to the given value.
    pub fn min_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, S> {
        self._min_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *min monthly budget.currency code* query property to the given value.
    pub fn min_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._min_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// The whole units of the amount.
    /// For example if `currencyCode` is `"USD"`, then 1 unit is one US dollar.
    ///
    /// Sets the *max monthly budget.units* query property to the given value.
    pub fn max_monthly_budget_units(mut self, new_value: i64) -> CompanyListCall<'a, S> {
        self._max_monthly_budget_units = Some(new_value);
        self
    }
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999 inclusive.
    /// If `units` is positive, `nanos` must be positive or zero.
    /// If `units` is zero, `nanos` can be positive, zero, or negative.
    /// If `units` is negative, `nanos` must be negative or zero.
    /// For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000.
    ///
    /// Sets the *max monthly budget.nanos* query property to the given value.
    pub fn max_monthly_budget_nanos(mut self, new_value: i32) -> CompanyListCall<'a, S> {
        self._max_monthly_budget_nanos = Some(new_value);
        self
    }
    /// The 3-letter currency code defined in ISO 4217.
    ///
    /// Sets the *max monthly budget.currency code* query property to the given value.
    pub fn max_monthly_budget_currency_code(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._max_monthly_budget_currency_code = Some(new_value.to_string());
        self
    }
    /// List of language codes that company can support. Only primary language
    /// subtags are accepted as defined by
    /// <a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
    /// (IETF BCP 47, "Tags for Identifying Languages").
    ///
    /// Append the given value to the *language codes* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_language_codes(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._language_codes.push(new_value.to_string());
        self
    }
    /// List of industries the company can help with.
    ///
    /// Append the given value to the *industries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_industries(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._industries.push(new_value.to_string());
        self
    }
    /// List of reasons for using Google Partner Search to get companies.
    ///
    /// Append the given value to the *gps motivations* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_gps_motivations(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._gps_motivations.push(new_value.to_string());
        self
    }
    /// Company name to search for.
    ///
    /// Sets the *company name* query property to the given value.
    pub fn company_name(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._company_name = Some(new_value.to_string());
        self
    }
    /// The address to use when searching for companies.
    /// If not given, the geo-located address of the request is used.
    ///
    /// Sets the *address* query property to the given value.
    pub fn address(mut self, new_value: &str) -> CompanyListCall<'a, S> {
        self._address = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CompanyListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> CompanyListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Updates a user's profile. A user can only update their own profile and
/// should only be called within the context of a logged in user.
///
/// A builder for the *updateProfile* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::UserProfile;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = UserProfile::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().update_profile(req)
///              .request_metadata_user_overrides_user_id("sadipscing")
///              .request_metadata_user_overrides_ip_address("erat")
///              .request_metadata_traffic_source_traffic_sub_id("aliquyam")
///              .request_metadata_traffic_source_traffic_source_id("amet")
///              .request_metadata_partners_session_id("est")
///              .request_metadata_locale("et")
///              .add_request_metadata_experiment_ids("sea")
///              .doit().await;
/// # }
/// ```
pub struct UserUpdateProfileCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: UserProfile,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserUpdateProfileCall<'a, S> {}

impl<'a, S> UserUpdateProfileCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, UserProfile)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.users.updateProfile",
                               http_method: hyper::Method::PATCH });

        for &field in ["alt", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/users/profile";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
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
    pub fn request(mut self, new_value: UserProfile) -> UserUpdateProfileCall<'a, S> {
        self._request = new_value;
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserUpdateProfileCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserUpdateProfileCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserUpdateProfileCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Creates a user's company relation. Affiliates the user to a company.
///
/// A builder for the *createCompanyRelation* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// use partners2::api::CompanyRelation;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = CompanyRelation::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().create_company_relation(req, "userId")
///              .request_metadata_user_overrides_user_id("consetetur")
///              .request_metadata_user_overrides_ip_address("Stet")
///              .request_metadata_traffic_source_traffic_sub_id("est")
///              .request_metadata_traffic_source_traffic_source_id("aliquyam")
///              .request_metadata_partners_session_id("elitr")
///              .request_metadata_locale("duo")
///              .add_request_metadata_experiment_ids("diam")
///              .doit().await;
/// # }
/// ```
pub struct UserCreateCompanyRelationCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _request: CompanyRelation,
   pub(super) _user_id: String,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserCreateCompanyRelationCall<'a, S> {}

impl<'a, S> UserCreateCompanyRelationCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CompanyRelation)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.users.createCompanyRelation",
                               http_method: hyper::Method::PUT });

        for &field in ["alt", "userId", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/users/{userId}/companyRelation";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
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
            request_value_reader.seek(io::SeekFrom::Start(0)).unwrap();
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::PUT)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



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
    pub fn request(mut self, new_value: CompanyRelation) -> UserCreateCompanyRelationCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The ID of the user. Can be set to <code>me</code> to mean
    /// the currently authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserCreateCompanyRelationCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserCreateCompanyRelationCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserCreateCompanyRelationCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Deletes a user's company relation. Unaffiliaites the user from a company.
///
/// A builder for the *deleteCompanyRelation* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().delete_company_relation("userId")
///              .request_metadata_user_overrides_user_id("sit")
///              .request_metadata_user_overrides_ip_address("sed")
///              .request_metadata_traffic_source_traffic_sub_id("eos")
///              .request_metadata_traffic_source_traffic_source_id("Lorem")
///              .request_metadata_partners_session_id("ea")
///              .request_metadata_locale("Stet")
///              .add_request_metadata_experiment_ids("dolores")
///              .doit().await;
/// # }
/// ```
pub struct UserDeleteCompanyRelationCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _user_id: String,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserDeleteCompanyRelationCall<'a, S> {}

impl<'a, S> UserDeleteCompanyRelationCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Empty)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.users.deleteCompanyRelation",
                               http_method: hyper::Method::DELETE });

        for &field in ["alt", "userId", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(10 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/users/{userId}/companyRelation";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



        loop {
            let mut req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = hyper::Request::builder()
                    .method(hyper::Method::DELETE)
                    .uri(url.as_str())
                    .header(USER_AGENT, self.hub._user_agent.clone());



                        let request = req_builder
                        .body(hyper::body::Body::empty());

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


    /// The ID of the user. Can be set to <code>me</code> to mean
    /// the currently authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserDeleteCompanyRelationCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserDeleteCompanyRelationCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserDeleteCompanyRelationCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Gets a user.
///
/// A builder for the *get* method supported by a *user* resource.
/// It is not used directly, but through a [`UserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_partners2 as partners2;
/// # async fn dox() {
/// # use std::default::Default;
/// # use partners2::{Partners, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Partners::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.users().get("userId")
///              .user_view("et")
///              .request_metadata_user_overrides_user_id("sea")
///              .request_metadata_user_overrides_ip_address("et")
///              .request_metadata_traffic_source_traffic_sub_id("At")
///              .request_metadata_traffic_source_traffic_source_id("dolore")
///              .request_metadata_partners_session_id("eirmod")
///              .request_metadata_locale("Lorem")
///              .add_request_metadata_experiment_ids("accusam")
///              .doit().await;
/// # }
/// ```
pub struct UserGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Partners<S>,
   pub(super) _user_id: String,
   pub(super) _user_view: Option<String>,
   pub(super) _request_metadata_user_overrides_user_id: Option<String>,
   pub(super) _request_metadata_user_overrides_ip_address: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_sub_id: Option<String>,
   pub(super) _request_metadata_traffic_source_traffic_source_id: Option<String>,
   pub(super) _request_metadata_partners_session_id: Option<String>,
   pub(super) _request_metadata_locale: Option<String>,
   pub(super) _request_metadata_experiment_ids: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for UserGetCall<'a, S> {}

impl<'a, S> UserGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, User)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "partners.users.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "userId", "userView", "requestMetadata.userOverrides.userId", "requestMetadata.userOverrides.ipAddress", "requestMetadata.trafficSource.trafficSubId", "requestMetadata.trafficSource.trafficSourceId", "requestMetadata.partnersSessionId", "requestMetadata.locale", "requestMetadata.experimentIds"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("userId", self._user_id);
        if let Some(value) = self._user_view.as_ref() {
            params.push("userView", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_user_id.as_ref() {
            params.push("requestMetadata.userOverrides.userId", value);
        }
        if let Some(value) = self._request_metadata_user_overrides_ip_address.as_ref() {
            params.push("requestMetadata.userOverrides.ipAddress", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_sub_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSubId", value);
        }
        if let Some(value) = self._request_metadata_traffic_source_traffic_source_id.as_ref() {
            params.push("requestMetadata.trafficSource.trafficSourceId", value);
        }
        if let Some(value) = self._request_metadata_partners_session_id.as_ref() {
            params.push("requestMetadata.partnersSessionId", value);
        }
        if let Some(value) = self._request_metadata_locale.as_ref() {
            params.push("requestMetadata.locale", value);
        }
        if self._request_metadata_experiment_ids.len() > 0 {
            for f in self._request_metadata_experiment_ids.iter() {
                params.push("requestMetadata.experimentIds", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v2/users/{userId}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{userId}", "userId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["userId"];
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
                        .body(hyper::body::Body::empty());

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


    /// Identifier of the user. Can be set to <code>me</code> to mean the currently
    /// authenticated user.
    ///
    /// Sets the *user id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn user_id(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._user_id = new_value.to_string();
        self
    }
    /// Specifies what parts of the user information to return.
    ///
    /// Sets the *user view* query property to the given value.
    pub fn user_view(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._user_view = Some(new_value.to_string());
        self
    }
    /// Logged-in user ID to impersonate instead of the user's ID.
    ///
    /// Sets the *request metadata.user overrides.user id* query property to the given value.
    pub fn request_metadata_user_overrides_user_id(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_user_overrides_user_id = Some(new_value.to_string());
        self
    }
    /// IP address to use instead of the user's geo-located IP address.
    ///
    /// Sets the *request metadata.user overrides.ip address* query property to the given value.
    pub fn request_metadata_user_overrides_ip_address(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_user_overrides_ip_address = Some(new_value.to_string());
        self
    }
    /// Second level identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic sub id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_sub_id(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_traffic_source_traffic_sub_id = Some(new_value.to_string());
        self
    }
    /// Identifier to indicate where the traffic comes from.
    /// An identifier has multiple letters created by a team which redirected the
    /// traffic to us.
    ///
    /// Sets the *request metadata.traffic source.traffic source id* query property to the given value.
    pub fn request_metadata_traffic_source_traffic_source_id(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_traffic_source_traffic_source_id = Some(new_value.to_string());
        self
    }
    /// Google Partners session ID.
    ///
    /// Sets the *request metadata.partners session id* query property to the given value.
    pub fn request_metadata_partners_session_id(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_partners_session_id = Some(new_value.to_string());
        self
    }
    /// Locale to use for the current request.
    ///
    /// Sets the *request metadata.locale* query property to the given value.
    pub fn request_metadata_locale(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_locale = Some(new_value.to_string());
        self
    }
    /// Experiment IDs the current request belongs to.
    ///
    /// Append the given value to the *request metadata.experiment ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_request_metadata_experiment_ids(mut self, new_value: &str) -> UserGetCall<'a, S> {
        self._request_metadata_experiment_ids.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> UserGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *callback* (query-string) - JSONP
    pub fn param<T>(mut self, name: T, value: T) -> UserGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


