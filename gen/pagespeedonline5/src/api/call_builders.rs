use super::*;
/// Runs PageSpeed analysis on the page at the specified URL, and returns PageSpeed scores, a list of suggestions to make that page faster, and other information.
///
/// A builder for the *runpagespeed* method supported by a *pagespeedapi* resource.
/// It is not used directly, but through a [`PagespeedapiMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_pagespeedonline5 as pagespeedonline5;
/// # async fn dox() {
/// # use std::default::Default;
/// # use pagespeedonline5::{PagespeedInsights, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = PagespeedInsights::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.pagespeedapi().runpagespeed("url")
///              .utm_source("eos")
///              .utm_campaign("dolor")
///              .strategy(&Default::default())
///              .locale("ea")
///              .add_category(&Default::default())
///              .captcha_token("ipsum")
///              .doit().await;
/// # }
/// ```
pub struct PagespeedapiRunpagespeedCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a PagespeedInsights<S>,
   pub(super) _url: String,
   pub(super) _utm_source: Option<String>,
   pub(super) _utm_campaign: Option<String>,
   pub(super) _strategy: Option<PagespeedapiStrategyEnum>,
   pub(super) _locale: Option<String>,
   pub(super) _category: Option<PagespeedapiCategoryEnum>,
   pub(super) _captcha_token: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for PagespeedapiRunpagespeedCall<'a, S> {}

impl<'a, S> PagespeedapiRunpagespeedCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, PagespeedApiPagespeedResponseV5)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "pagespeedonline.pagespeedapi.runpagespeed",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "url", "utm_source", "utm_campaign", "strategy", "locale", "category", "captchaToken"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(9 + self._additional_params.len());
        params.push("url", &self._url);
        if let Some(value) = self._utm_source.as_ref() {
            params.push("utm_source", value);
        }
        if let Some(value) = self._utm_campaign.as_ref() {
            params.push("utm_campaign", value);
        }
        if let Some(value) = self._strategy.as_ref() {
            params.push("strategy", value);
        }
        if let Some(value) = self._locale.as_ref() {
            params.push("locale", value);
        }
        if self._category.len() > 0 {
            for f in self._category.iter() {
                params.push("category", f);
            }
        }
        if let Some(value) = self._captcha_token.as_ref() {
            params.push("captchaToken", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "pagespeedonline/v5/runPagespeed";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Openid.as_ref().to_string());
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


    /// Required. The URL to fetch and analyze
    ///
    /// Sets the *url* query property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn url(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._url = new_value.to_string();
        self
    }
    /// Campaign source for analytics.
    ///
    /// Sets the *utm_source* query property to the given value.
    pub fn utm_source(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._utm_source = Some(new_value.to_string());
        self
    }
    /// Campaign name for analytics.
    ///
    /// Sets the *utm_campaign* query property to the given value.
    pub fn utm_campaign(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._utm_campaign = Some(new_value.to_string());
        self
    }
    /// The analysis strategy (desktop or mobile) to use, and desktop is the default
    ///
    /// Sets the *strategy* query property to the given value.
    pub fn strategy(mut self, new_value: &PagespeedapiStrategyEnum) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._strategy = Some(new_value.clone());
        self
    }
    /// The locale used to localize formatted results
    ///
    /// Sets the *locale* query property to the given value.
    pub fn locale(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._locale = Some(new_value.to_string());
        self
    }
    /// A Lighthouse category to run; if none are given, only Performance category will be run
    ///
    /// Append the given value to the *category* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_category(mut self, new_value: &PagespeedapiCategoryEnum) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._category.push(new_value.clone());
        self
    }
    /// The captcha token passed when filling out a captcha.
    ///
    /// Sets the *captcha token* query property to the given value.
    pub fn captcha_token(mut self, new_value: &str) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._captcha_token = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PagespeedapiRunpagespeedCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> PagespeedapiRunpagespeedCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Openid`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> PagespeedapiRunpagespeedCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> PagespeedapiRunpagespeedCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> PagespeedapiRunpagespeedCall<'a, S> {
        self._scopes.clear();
        self
    }
}


