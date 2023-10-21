use super::*;
/// Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
///
/// A builder for the *search* method supported by a *accountReport* resource.
/// It is not used directly, but through a [`AccountReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_localservices1 as localservices1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.account_reports().search()
///              .start_date_year(-16)
///              .start_date_month(-57)
///              .start_date_day(-50)
///              .query("ipsum")
///              .page_token("est")
///              .page_size(-62)
///              .end_date_year(-17)
///              .end_date_month(-99)
///              .end_date_day(-56)
///              .doit().await;
/// # }
/// ```
pub struct AccountReportSearchCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Localservices<S>,
   pub(super) _start_date_year: Option<i32>,
   pub(super) _start_date_month: Option<i32>,
   pub(super) _start_date_day: Option<i32>,
   pub(super) _query: Option<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _end_date_year: Option<i32>,
   pub(super) _end_date_month: Option<i32>,
   pub(super) _end_date_day: Option<i32>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for AccountReportSearchCall<'a, S> {}

impl<'a, S> AccountReportSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleAdsHomeservicesLocalservicesV1SearchAccountReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "localservices.accountReports.search",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "startDate.year", "startDate.month", "startDate.day", "query", "pageToken", "pageSize", "endDate.year", "endDate.month", "endDate.day"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._start_date_year.as_ref() {
            params.push("startDate.year", value.to_string());
        }
        if let Some(value) = self._start_date_month.as_ref() {
            params.push("startDate.month", value.to_string());
        }
        if let Some(value) = self._start_date_day.as_ref() {
            params.push("startDate.day", value.to_string());
        }
        if let Some(value) = self._query.as_ref() {
            params.push("query", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._end_date_year.as_ref() {
            params.push("endDate.year", value.to_string());
        }
        if let Some(value) = self._end_date_month.as_ref() {
            params.push("endDate.month", value.to_string());
        }
        if let Some(value) = self._end_date_day.as_ref() {
            params.push("endDate.day", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/accountReports:search";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Adword.as_ref().to_string());
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


    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *start date.year* query property to the given value.
    pub fn start_date_year(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *start date.month* query property to the given value.
    pub fn start_date_month(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *start date.day* query property to the given value.
    pub fn start_date_day(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._start_date_day = Some(new_value);
        self
    }
    /// A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Account Report for Manager with id 123. | Required.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> AccountReportSearchCall<'a, S> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The `next_page_token` value returned from a previous request to SearchAccountReports that indicates where listing should continue. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AccountReportSearchCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *end date.year* query property to the given value.
    pub fn end_date_year(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *end date.month* query property to the given value.
    pub fn end_date_month(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *end date.day* query property to the given value.
    pub fn end_date_day(mut self, new_value: i32) -> AccountReportSearchCall<'a, S> {
        self._end_date_day = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AccountReportSearchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> AccountReportSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Adword`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> AccountReportSearchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> AccountReportSearchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> AccountReportSearchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


/// Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.
///
/// A builder for the *search* method supported by a *detailedLeadReport* resource.
/// It is not used directly, but through a [`DetailedLeadReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_localservices1 as localservices1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use localservices1::{Localservices, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Localservices::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.detailed_lead_reports().search()
///              .start_date_year(-25)
///              .start_date_month(-86)
///              .start_date_day(-43)
///              .query("duo")
///              .page_token("sed")
///              .page_size(-61)
///              .end_date_year(-15)
///              .end_date_month(-13)
///              .end_date_day(-24)
///              .doit().await;
/// # }
/// ```
pub struct DetailedLeadReportSearchCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Localservices<S>,
   pub(super) _start_date_year: Option<i32>,
   pub(super) _start_date_month: Option<i32>,
   pub(super) _start_date_day: Option<i32>,
   pub(super) _query: Option<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _page_size: Option<i32>,
   pub(super) _end_date_year: Option<i32>,
   pub(super) _end_date_month: Option<i32>,
   pub(super) _end_date_day: Option<i32>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
   pub(super) _scopes: BTreeSet<String>
}

impl<'a, S> client::CallBuilder for DetailedLeadReportSearchCall<'a, S> {}

impl<'a, S> DetailedLeadReportSearchCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, GoogleAdsHomeservicesLocalservicesV1SearchDetailedLeadReportsResponse)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "localservices.detailedLeadReports.search",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "startDate.year", "startDate.month", "startDate.day", "query", "pageToken", "pageSize", "endDate.year", "endDate.month", "endDate.day"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        if let Some(value) = self._start_date_year.as_ref() {
            params.push("startDate.year", value.to_string());
        }
        if let Some(value) = self._start_date_month.as_ref() {
            params.push("startDate.month", value.to_string());
        }
        if let Some(value) = self._start_date_day.as_ref() {
            params.push("startDate.day", value.to_string());
        }
        if let Some(value) = self._query.as_ref() {
            params.push("query", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._page_size.as_ref() {
            params.push("pageSize", value.to_string());
        }
        if let Some(value) = self._end_date_year.as_ref() {
            params.push("endDate.year", value.to_string());
        }
        if let Some(value) = self._end_date_month.as_ref() {
            params.push("endDate.month", value.to_string());
        }
        if let Some(value) = self._end_date_day.as_ref() {
            params.push("endDate.day", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "v1/detailedLeadReports:search";
        if self._scopes.is_empty() {
            self._scopes.insert(Scope::Adword.as_ref().to_string());
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


    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *start date.year* query property to the given value.
    pub fn start_date_year(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *start date.month* query property to the given value.
    pub fn start_date_month(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *start date.day* query property to the given value.
    pub fn start_date_day(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._start_date_day = Some(new_value);
        self
    }
    /// A query string for searching for account reports. Caller must provide a customer id of their MCC account with an associated Gaia Mint that allows read permission on their linked accounts. Search expressions are case insensitive. Example query: | Query | Description | |-------------------------|-----------------------------------------------| | manager_customer_id:123 | Get Detailed Lead Report for Manager with id | | | 123. | Required.
    ///
    /// Sets the *query* query property to the given value.
    pub fn query(mut self, new_value: &str) -> DetailedLeadReportSearchCall<'a, S> {
        self._query = Some(new_value.to_string());
        self
    }
    /// The `next_page_token` value returned from a previous request to SearchDetailedLeadReports that indicates where listing should continue. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> DetailedLeadReportSearchCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// The maximum number of accounts to return. If the page size is unset, page size will default to 1000. Maximum page_size is 10000. Optional.
    ///
    /// Sets the *page size* query property to the given value.
    pub fn page_size(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._page_size = Some(new_value);
        self
    }
    /// Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year.
    ///
    /// Sets the *end date.year* query property to the given value.
    pub fn end_date_year(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_year = Some(new_value);
        self
    }
    /// Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day.
    ///
    /// Sets the *end date.month* query property to the given value.
    pub fn end_date_month(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_month = Some(new_value);
        self
    }
    /// Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant.
    ///
    /// Sets the *end date.day* query property to the given value.
    pub fn end_date_day(mut self, new_value: i32) -> DetailedLeadReportSearchCall<'a, S> {
        self._end_date_day = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> DetailedLeadReportSearchCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> DetailedLeadReportSearchCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    /// Identifies the authorization scope for the method you are building.
    ///
    /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Adword`].
    ///
    /// The `scope` will be added to a set of scopes. This is important as one can maintain access
    /// tokens for more than one scope.
    ///
    /// Usually there is more than one suitable scope to authorize an operation, some of which may
    /// encompass more rights than others. For example, for listing resources, a *read-only* scope will be
    /// sufficient, a read-write scope will do as well.
    pub fn add_scope<St>(mut self, scope: St) -> DetailedLeadReportSearchCall<'a, S>
                                                        where St: AsRef<str> {
        self._scopes.insert(String::from(scope.as_ref()));
        self
    }
    /// Identifies the authorization scope(s) for the method you are building.
    ///
    /// See [`Self::add_scope()`] for details.
    pub fn add_scopes<I, St>(mut self, scopes: I) -> DetailedLeadReportSearchCall<'a, S>
                                                        where I: IntoIterator<Item = St>,
                                                         St: AsRef<str> {
        self._scopes
            .extend(scopes.into_iter().map(|s| String::from(s.as_ref())));
        self
    }

    /// Removes all scopes, and no default scope will be used either.
    /// In this case, you have to specify your API-key using the `key` parameter (see [`Self::param()`]
    /// for details).
    pub fn clear_scopes(mut self) -> DetailedLeadReportSearchCall<'a, S> {
        self._scopes.clear();
        self
    }
}


