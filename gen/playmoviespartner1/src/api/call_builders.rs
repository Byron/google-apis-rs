use super::*;
/// List Orders owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *orders.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_list("accountId")
///              .add_video_ids("sed")
///              .add_studio_names("amet.")
///              .add_status("takimata")
///              .add_pph_names("amet.")
///              .page_token("duo")
///              .page_size(-55)
///              .name("gubergren")
///              .custom_id("Lorem")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _video_ids: Vec<String>,
   pub(super) _studio_names: Vec<String>,
   pub(super) _status: Vec<String>,
   pub(super) _pph_names: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _name: Option<String>,
   pub(super) _custom_id: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountOrderListCall<'a, S> {}

impl<'a, S> AccountOrderListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListOrdersResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.orders.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "videoIds", "studioNames", "status", "pphNames", "pageToken", "pageSize", "name", "customId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if self._status.len() > 0 {
            for f in self._status.iter() {
                params.push("status", f);
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }
        if let Some(value) = self._custom_id.as_ref() {
            params.push("customId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Orders that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter Orders that match one of the given status.
    ///
    /// Append the given value to the *status* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_status(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._status.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountOrderListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches Orders with a `name`, `show`, `season` or `episode`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter Orders that match a case-insensitive, partner-specific custom id.
    ///
    /// Sets the *custom id* query property to the given value.
    pub fn custom_id(mut self, new_value: &str) -> AccountOrderListCall<'a, S> {
        self._custom_id = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountOrderListCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountOrderListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountOrderListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountOrderListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get an Order given its id.
/// 
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *orders.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().orders_get("accountId", "orderId")
///              .doit().await;
/// # }
/// ```
pub struct AccountOrderGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _order_id: String,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountOrderGetCall<'a, S> {}

impl<'a, S> AccountOrderGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Order)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.orders.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "orderId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("orderId", self._order_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/orders/{orderId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{orderId}", "orderId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["orderId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Order ID.
    ///
    /// Sets the *order id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn order_id(mut self, new_value: &str) -> AccountOrderGetCall<'a, S> {
        self._order_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountOrderGetCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountOrderGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountOrderGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountOrderGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountOrderGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List Avails owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *avails.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_list("accountId")
///              .add_video_ids("ea")
///              .title("ipsum")
///              .add_territories("invidunt")
///              .add_studio_names("amet")
///              .add_pph_names("duo")
///              .page_token("ipsum")
///              .page_size(-93)
///              .add_alt_ids("ut")
///              .alt_id("gubergren")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _video_ids: Vec<String>,
   pub(super) _title: Option<String>,
   pub(super) _territories: Vec<String>,
   pub(super) _studio_names: Vec<String>,
   pub(super) _pph_names: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _alt_ids: Vec<String>,
   pub(super) _alt_id: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountAvailListCall<'a, S> {}

impl<'a, S> AccountAvailListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListAvailsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.avails.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "videoIds", "title", "territories", "studioNames", "pphNames", "pageToken", "pageSize", "altIds", "altId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(12 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if let Some(value) = self._title.as_ref() {
            params.push("title", value);
        }
        if self._territories.len() > 0 {
            for f in self._territories.iter() {
                params.push("territories", f);
            }
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if self._alt_ids.len() > 0 {
            for f in self._alt_ids.iter() {
                params.push("altIds", f);
            }
        }
        if let Some(value) = self._alt_id.as_ref() {
            params.push("altId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter Avails that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter that matches Avails with a `title_internal_alias`,
    /// `series_title_internal_alias`, `season_title_internal_alias`,
    /// or `episode_title_internal_alias` that contains the given
    /// case-insensitive title.
    ///
    /// Sets the *title* query property to the given value.
    pub fn title(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._title = Some(new_value.to_string());
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given country codes,
    /// using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *territories* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_territories(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._territories.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountAvailListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter Avails that match (case-insensitive) any of the given partner-specific custom ids.
    ///
    /// Append the given value to the *alt ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_alt_ids(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._alt_ids.push(new_value.to_string());
        self
    }
    /// Filter Avails that match a case-insensitive, partner-specific custom id.
    /// NOTE: this field is deprecated and will be removed on V2; `alt_ids`
    /// should be used instead.
    ///
    /// Sets the *alt id* query property to the given value.
    pub fn alt_id(mut self, new_value: &str) -> AccountAvailListCall<'a, S> {
        self._alt_id = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountAvailListCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountAvailListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountAvailListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountAvailListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get an Avail given its avail group id and avail id.
///
/// A builder for the *avails.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().avails_get("accountId", "availId")
///              .doit().await;
/// # }
/// ```
pub struct AccountAvailGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _avail_id: String,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountAvailGetCall<'a, S> {}

impl<'a, S> AccountAvailGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Avail)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.avails.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "availId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(4 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("availId", self._avail_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/avails/{availId}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{availId}", "availId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["availId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Avail ID.
    ///
    /// Sets the *avail id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn avail_id(mut self, new_value: &str) -> AccountAvailGetCall<'a, S> {
        self._avail_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountAvailGetCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountAvailGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountAvailGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountAvailGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountAvailGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get a StoreInfo given its video id and country.
/// 
/// See _Authentication and Authorization rules_ and
/// _Get methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.country.get* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_country_get("accountId", "videoId", "country")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoCountryGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _video_id: String,
   pub(super) _country: String,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountStoreInfoCountryGetCall<'a, S> {}

impl<'a, S> AccountStoreInfoCountryGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, StoreInfo)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.storeInfos.country.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "videoId", "country"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("accountId", self._account_id);
        params.push("videoId", self._video_id);
        params.push("country", self._country);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/storeInfos/{videoId}/country/{country}";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId"), ("{videoId}", "videoId"), ("{country}", "country")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["country", "videoId", "accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// REQUIRED. Video ID.
    ///
    /// Sets the *video id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, S> {
        self._video_id = new_value.to_string();
        self
    }
    /// REQUIRED. Edit country.
    ///
    /// Sets the *country* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn country(mut self, new_value: &str) -> AccountStoreInfoCountryGetCall<'a, S> {
        self._country = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountStoreInfoCountryGetCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoCountryGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountStoreInfoCountryGetCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountStoreInfoCountryGetCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountStoreInfoCountryGetCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// List StoreInfos owned or managed by the partner.
/// 
/// See _Authentication and Authorization rules_ and
/// _List methods rules_ for more information about this method.
///
/// A builder for the *storeInfos.list* method supported by a *account* resource.
/// It is not used directly, but through a [`AccountMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_playmoviespartner1 as playmoviespartner1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use playmoviespartner1::{PlayMovies, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PlayMovies::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.accounts().store_infos_list("accountId")
///              .add_video_ids("ea")
///              .video_id("dolor")
///              .add_studio_names("Lorem")
///              .add_season_ids("eos")
///              .add_pph_names("labore")
///              .page_token("sed")
///              .page_size(-70)
///              .name("sed")
///              .add_mids("no")
///              .add_countries("Stet")
///              .doit().await;
/// # }
/// ```
pub struct AccountStoreInfoListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PlayMovies<S>,
   pub(super) _account_id: String,
   pub(super) _video_ids: Vec<String>,
   pub(super) _video_id: Option<String>,
   pub(super) _studio_names: Vec<String>,
   pub(super) _season_ids: Vec<String>,
   pub(super) _pph_names: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _name: Option<String>,
   pub(super) _mids: Vec<String>,
   pub(super) _countries: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountStoreInfoListCall<'a, S> {}

impl<'a, S> AccountStoreInfoListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, ListStoreInfosResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "playmoviespartner.accounts.storeInfos.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "accountId", "videoIds", "videoId", "studioNames", "seasonIds", "pphNames", "pageToken", "pageSize", "name", "mids", "countries"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(13 + self._additional_params.len());
        params.push("accountId", self._account_id);
        if self._video_ids.len() > 0 {
            for f in self._video_ids.iter() {
                params.push("videoIds", f);
            }
        }
        if let Some(value) = self._video_id.as_ref() {
            params.push("videoId", value);
        }
        if self._studio_names.len() > 0 {
            for f in self._studio_names.iter() {
                params.push("studioNames", f);
            }
        }
        if self._season_ids.len() > 0 {
            for f in self._season_ids.iter() {
                params.push("seasonIds", f);
            }
        }
        if self._pph_names.len() > 0 {
            for f in self._pph_names.iter() {
                params.push("pphNames", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._name.as_ref() {
            params.push("name", value);
        }
        if self._mids.len() > 0 {
            for f in self._mids.iter() {
                params.push("mids", f);
            }
        }
        if self._countries.len() > 0 {
            for f in self._countries.iter() {
                params.push("countries", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accounts/{accountId}/storeInfos";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::PlaymovyPartnerReadonly.as_ref().to_string());
        }

        for &(find_this, param_name) in [("{accountId}", "accountId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["accountId"];
            params.remove_params(&to_remove);
        }

        let url = params.parse_with_url(&url);



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


    /// REQUIRED. See _General rules_ for more information about this field.
    ///
    /// Sets the *account id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn account_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._account_id = new_value.to_string();
        self
    }
    /// Filter StoreInfos that match any of the given `video_id`s.
    ///
    /// Append the given value to the *video ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_video_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._video_ids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match a given `video_id`.
    /// NOTE: this field is deprecated and will be removed on V2; `video_ids`
    /// should be used instead.
    ///
    /// Sets the *video id* query property to the given value.
    pub fn video_id(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._video_id = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *studio names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_studio_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._studio_names.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `season_id`s.
    ///
    /// Append the given value to the *season ids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_season_ids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._season_ids.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Append the given value to the *pph names* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_pph_names(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._pph_names.push(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// See _List methods rules_ for info about this field.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountStoreInfoListCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Filter that matches StoreInfos with a `name` or `show_name`
    /// that contains the given case-insensitive name.
    ///
    /// Sets the *name* query property to the given value.
    pub fn name(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._name = Some(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match any of the given `mid`s.
    ///
    /// Append the given value to the *mids* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_mids(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._mids.push(new_value.to_string());
        self
    }
    /// Filter StoreInfos that match (case-insensitive) any of the given country
    /// codes, using the "ISO 3166-1 alpha-2" format (examples: "US", "us", "Us").
    ///
    /// Append the given value to the *countries* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_countries(mut self, new_value: &str) -> AccountStoreInfoListCall<'a, S> {
        self._countries.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountStoreInfoListCall<'a, S> {
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
    /// * *upload_protocol* (query-string) - Upload protocol for media (e.g. "raw", "multipart").
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *uploadType* (query-string) - Legacy upload protocol for media (e.g. "media", "multipart").
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *callback* (query-string) - JSONP
    /// * *$.xgafv* (query-string) - V1 error format.
    /// * *alt* (query-string) - Data format for response.
    /// * *access_token* (query-string) - OAuth access token.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    /// * *pp* (query-boolean) - Pretty-print response.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *bearer_token* (query-string) - OAuth bearer token.
    pub fn param<T>(mut self, name: T, value: T) -> AccountStoreInfoListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::PlaymovyPartnerReadonly`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountStoreInfoListCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountStoreInfoListCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountStoreInfoListCall<'a, S> {
        self._scopes.clear();
        self
    }
}


