use super::*;
/// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only publishers can lookup advertisers. Advertisers can request information about themselves by omitting the advertiserId query parameter.
///
/// A builder for the *get* method supported by a *advertiser* resource.
/// It is not used directly, but through a [`AdvertiserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.advertisers().get("role", "roleId")
///              .advertiser_id("et")
///              .doit().await;
/// # }
/// ```
pub struct AdvertiserGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _advertiser_id: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for AdvertiserGetCall<'a, S> {}

impl<'a, S> AdvertiserGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Advertiser)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.advertisers.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._advertiser_id.as_ref() {
            params.push("advertiserId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/advertiser";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> AdvertiserGetCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> AdvertiserGetCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the advertiser to look up. Optional.
    ///
    /// Sets the *advertiser id* query property to the given value.
    pub fn advertiser_id(mut self, new_value: &str) -> AdvertiserGetCall<'a, S> {
        self._advertiser_id = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AdvertiserGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> AdvertiserGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about all advertisers that the requesting advertiser/publisher has access to.
///
/// A builder for the *list* method supported by a *advertiser* resource.
/// It is not used directly, but through a [`AdvertiserMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.advertisers().list("role", "roleId")
///              .relationship_status("sed")
///              .page_token("duo")
///              .min_seven_day_epc(0.5254434270373415)
///              .min_payout_rank(-22)
///              .min_ninety_day_epc(0.9697780726648698)
///              .max_results(99)
///              .advertiser_category("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct AdvertiserListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _relationship_status: Option<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _min_seven_day_epc: Option<f64>,
   pub(super) _min_payout_rank: Option<i32>,
   pub(super) _min_ninety_day_epc: Option<f64>,
   pub(super) _max_results: Option<u32>,
   pub(super) _advertiser_category: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for AdvertiserListCall<'a, S> {}

impl<'a, S> AdvertiserListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Advertisers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.advertisers.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "relationshipStatus", "pageToken", "minSevenDayEpc", "minPayoutRank", "minNinetyDayEpc", "maxResults", "advertiserCategory"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._relationship_status.as_ref() {
            params.push("relationshipStatus", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._min_seven_day_epc.as_ref() {
            params.push("minSevenDayEpc", value.to_string());
        }
        if let Some(value) = self._min_payout_rank.as_ref() {
            params.push("minPayoutRank", value.to_string());
        }
        if let Some(value) = self._min_ninety_day_epc.as_ref() {
            params.push("minNinetyDayEpc", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._advertiser_category.as_ref() {
            params.push("advertiserCategory", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/advertisers";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> AdvertiserListCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> AdvertiserListCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all advertisers for which do not have the given relationship status with the requesting publisher.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> AdvertiserListCall<'a, S> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> AdvertiserListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Filters out all advertisers that have a seven day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min seven day epc* query property to the given value.
    pub fn min_seven_day_epc(mut self, new_value: f64) -> AdvertiserListCall<'a, S> {
        self._min_seven_day_epc = Some(new_value);
        self
    }
    /// A value between 1 and 4, where 1 represents the quartile of advertisers with the lowest ranks and 4 represents the quartile of advertisers with the highest ranks. Filters out all advertisers with a lower rank than the given quartile. For example if a 2 was given only advertisers with a payout rank of 25 or higher would be included. Optional.
    ///
    /// Sets the *min payout rank* query property to the given value.
    pub fn min_payout_rank(mut self, new_value: i32) -> AdvertiserListCall<'a, S> {
        self._min_payout_rank = Some(new_value);
        self
    }
    /// Filters out all advertisers that have a ninety day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min ninety day epc* query property to the given value.
    pub fn min_ninety_day_epc(mut self, new_value: f64) -> AdvertiserListCall<'a, S> {
        self._min_ninety_day_epc = Some(new_value);
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> AdvertiserListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Caret(^) delimted list of advertiser categories. Valid categories are defined here: http://www.google.com/support/affiliatenetwork/advertiser/bin/answer.py?hl=en&answer=107581. Filters out all advertisers not in one of the given advertiser categories. Optional.
    ///
    /// Sets the *advertiser category* query property to the given value.
    pub fn advertiser_category(mut self, new_value: &str) -> AdvertiserListCall<'a, S> {
        self._advertiser_category = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> AdvertiserListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> AdvertiserListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves credit card offers for the given publisher.
///
/// A builder for the *list* method supported by a *ccOffer* resource.
/// It is not used directly, but through a [`CcOfferMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cc_offers().list("publisher")
///              .projection("dolor")
///              .add_advertiser("et")
///              .doit().await;
/// # }
/// ```
pub struct CcOfferListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _publisher: String,
   pub(super) _projection: Option<String>,
   pub(super) _advertiser: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CcOfferListCall<'a, S> {}

impl<'a, S> CcOfferListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, CcOffers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.ccOffers.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "publisher", "projection", "advertiser"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("publisher", self._publisher);
        if let Some(value) = self._projection.as_ref() {
            params.push("projection", value);
        }
        if self._advertiser.len() > 0 {
            for f in self._advertiser.iter() {
                params.push("advertiser", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "publishers/{publisher}/ccOffers";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{publisher}", "publisher")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["publisher"];
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


    /// The ID of the publisher in question.
    ///
    /// Sets the *publisher* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn publisher(mut self, new_value: &str) -> CcOfferListCall<'a, S> {
        self._publisher = new_value.to_string();
        self
    }
    /// The set of fields to return.
    ///
    /// Sets the *projection* query property to the given value.
    pub fn projection(mut self, new_value: &str) -> CcOfferListCall<'a, S> {
        self._projection = Some(new_value.to_string());
        self
    }
    /// The advertiser ID of a card issuer whose offers to include. Optional, may be repeated.
    ///
    /// Append the given value to the *advertiser* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser(mut self, new_value: &str) -> CcOfferListCall<'a, S> {
        self._advertiser.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CcOfferListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> CcOfferListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves event data for a given advertiser/publisher.
///
/// A builder for the *list* method supported by a *event* resource.
/// It is not used directly, but through a [`EventMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.events().list("role", "roleId")
///              .type_("Stet")
///              .status("dolor")
///              .sku("duo")
///              .publisher_id("vero")
///              .product_category("vero")
///              .page_token("invidunt")
///              .order_id("Stet")
///              .modify_date_min("vero")
///              .modify_date_max("elitr")
///              .member_id("Lorem")
///              .max_results(72)
///              .link_id("no")
///              .event_date_min("ipsum")
///              .event_date_max("accusam")
///              .charge_type("takimata")
///              .advertiser_id("consetetur")
///              .doit().await;
/// # }
/// ```
pub struct EventListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _type_: Option<String>,
   pub(super) _status: Option<String>,
   pub(super) _sku: Option<String>,
   pub(super) _publisher_id: Option<String>,
   pub(super) _product_category: Option<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _order_id: Option<String>,
   pub(super) _modify_date_min: Option<String>,
   pub(super) _modify_date_max: Option<String>,
   pub(super) _member_id: Option<String>,
   pub(super) _max_results: Option<u32>,
   pub(super) _link_id: Option<String>,
   pub(super) _event_date_min: Option<String>,
   pub(super) _event_date_max: Option<String>,
   pub(super) _charge_type: Option<String>,
   pub(super) _advertiser_id: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for EventListCall<'a, S> {}

impl<'a, S> EventListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Events)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.events.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "type", "status", "sku", "publisherId", "productCategory", "pageToken", "orderId", "modifyDateMin", "modifyDateMax", "memberId", "maxResults", "linkId", "eventDateMin", "eventDateMax", "chargeType", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(20 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._type_.as_ref() {
            params.push("type", value);
        }
        if let Some(value) = self._status.as_ref() {
            params.push("status", value);
        }
        if let Some(value) = self._sku.as_ref() {
            params.push("sku", value);
        }
        if let Some(value) = self._publisher_id.as_ref() {
            params.push("publisherId", value);
        }
        if let Some(value) = self._product_category.as_ref() {
            params.push("productCategory", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._order_id.as_ref() {
            params.push("orderId", value);
        }
        if let Some(value) = self._modify_date_min.as_ref() {
            params.push("modifyDateMin", value);
        }
        if let Some(value) = self._modify_date_max.as_ref() {
            params.push("modifyDateMax", value);
        }
        if let Some(value) = self._member_id.as_ref() {
            params.push("memberId", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._link_id.as_ref() {
            params.push("linkId", value);
        }
        if let Some(value) = self._event_date_min.as_ref() {
            params.push("eventDateMin", value);
        }
        if let Some(value) = self._event_date_max.as_ref() {
            params.push("eventDateMax", value);
        }
        if let Some(value) = self._charge_type.as_ref() {
            params.push("chargeType", value);
        }
        if let Some(value) = self._advertiser_id.as_ref() {
            params.push("advertiserId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/events";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', 'charge'. Optional.
    ///
    /// Sets the *type* query property to the given value.
    pub fn type_(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._type_ = Some(new_value.to_string());
        self
    }
    /// Filters out all events that do not have the given status. Valid values: 'active', 'canceled'. Optional.
    ///
    /// Sets the *status* query property to the given value.
    pub fn status(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._status = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of SKUs. Filters out all events that do not reference one of the given SKU. Optional.
    ///
    /// Sets the *sku* query property to the given value.
    pub fn sku(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._sku = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of publisher IDs. Filters out all events that do not reference one of the given publishers IDs. Only used when under advertiser role. Optional.
    ///
    /// Sets the *publisher id* query property to the given value.
    pub fn publisher_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._publisher_id = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of product categories. Filters out all events that do not reference a product in one of the given product categories. Optional.
    ///
    /// Sets the *product category* query property to the given value.
    pub fn product_category(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._product_category = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of order IDs. Filters out all events that do not reference one of the given order IDs. Optional.
    ///
    /// Sets the *order id* query property to the given value.
    pub fn order_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._order_id = Some(new_value.to_string());
        self
    }
    /// Filters out all events modified earlier than given date. Optional. Defaults to 24 hours before the current modifyDateMax, if modifyDateMax is explicitly set.
    ///
    /// Sets the *modify date min* query property to the given value.
    pub fn modify_date_min(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._modify_date_min = Some(new_value.to_string());
        self
    }
    /// Filters out all events modified later than given date. Optional. Defaults to 24 hours after modifyDateMin, if modifyDateMin is explicitly set.
    ///
    /// Sets the *modify date max* query property to the given value.
    pub fn modify_date_max(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._modify_date_max = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of member IDs. Filters out all events that do not reference one of the given member IDs. Optional.
    ///
    /// Sets the *member id* query property to the given value.
    pub fn member_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._member_id = Some(new_value.to_string());
        self
    }
    /// Max number of offers to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> EventListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Caret(^) delimited list of link IDs. Filters out all events that do not reference one of the given link IDs. Optional.
    ///
    /// Sets the *link id* query property to the given value.
    pub fn link_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._link_id = Some(new_value.to_string());
        self
    }
    /// Filters out all events earlier than given date. Optional. Defaults to 24 hours from current date/time.
    ///
    /// Sets the *event date min* query property to the given value.
    pub fn event_date_min(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._event_date_min = Some(new_value.to_string());
        self
    }
    /// Filters out all events later than given date. Optional. Defaults to 24 hours after eventMin.
    ///
    /// Sets the *event date max* query property to the given value.
    pub fn event_date_max(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._event_date_max = Some(new_value.to_string());
        self
    }
    /// Filters out all charge events that are not of the given charge type. Valid values: 'other', 'slotting_fee', 'monthly_minimum', 'tier_bonus', 'credit', 'debit'. Optional.
    ///
    /// Sets the *charge type* query property to the given value.
    pub fn charge_type(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._charge_type = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimited list of advertiser IDs. Filters out all events that do not reference one of the given advertiser IDs. Only used when under publishers role. Optional.
    ///
    /// Sets the *advertiser id* query property to the given value.
    pub fn advertiser_id(mut self, new_value: &str) -> EventListCall<'a, S> {
        self._advertiser_id = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> EventListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> EventListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about a single link if the requesting advertiser/publisher has access to it. Advertisers can look up their own links. Publishers can look up visible links or links belonging to advertisers they are in a relationship with.
///
/// A builder for the *get* method supported by a *link* resource.
/// It is not used directly, but through a [`LinkMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().get("role", "roleId", -31)
///              .doit().await;
/// # }
/// ```
pub struct LinkGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _link_id: i64,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for LinkGetCall<'a, S> {}

impl<'a, S> LinkGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Link)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.links.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "linkId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        params.push("linkId", self._link_id.to_string());

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/link/{linkId}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId"), ("{linkId}", "linkId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["linkId", "roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkGetCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkGetCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the link to look up.
    ///
    /// Sets the *link id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn link_id(mut self, new_value: i64) -> LinkGetCall<'a, S> {
        self._link_id = new_value;
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LinkGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> LinkGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Inserts a new link.
///
/// A builder for the *insert* method supported by a *link* resource.
/// It is not used directly, but through a [`LinkMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// use gan1_beta1::api::Link;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // As the method needs a request, you would usually fill it with the desired information
/// // into the respective structure. Some of the parts shown here might not be applicable !
/// // Values shown here are possibly random and not representative !
/// let mut req = Link::default();
/// 
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().insert(req, "role", "roleId")
///              .doit().await;
/// # }
/// ```
pub struct LinkInsertCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _request: Link,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for LinkInsertCall<'a, S> {}

impl<'a, S> LinkInsertCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Link)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.links.insert",
                               http_method: hyper::Method::POST });

        for &field in ["alt", "role", "roleId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/link";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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
    pub fn request(mut self, new_value: Link) -> LinkInsertCall<'a, S> {
        self._request = new_value;
        self
    }
    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkInsertCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkInsertCall<'a, S> {
        self._role_id = new_value.to_string();
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LinkInsertCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> LinkInsertCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves all links that match the query parameters.
///
/// A builder for the *list* method supported by a *link* resource.
/// It is not used directly, but through a [`LinkMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.links().list("role", "roleId")
///              .start_date_min("dolores")
///              .start_date_max("gubergren")
///              .search_text("et")
///              .relationship_status("accusam")
///              .add_promotion_type("voluptua.")
///              .page_token("dolore")
///              .max_results(67)
///              .link_type("dolore")
///              .create_date_min("voluptua.")
///              .create_date_max("amet.")
///              .authorship("ea")
///              .add_asset_size("sadipscing")
///              .add_advertiser_id(-6)
///              .doit().await;
/// # }
/// ```
pub struct LinkListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _start_date_min: Option<String>,
   pub(super) _start_date_max: Option<String>,
   pub(super) _search_text: Option<String>,
   pub(super) _relationship_status: Option<String>,
   pub(super) _promotion_type: Vec<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _max_results: Option<u32>,
   pub(super) _link_type: Option<String>,
   pub(super) _create_date_min: Option<String>,
   pub(super) _create_date_max: Option<String>,
   pub(super) _authorship: Option<String>,
   pub(super) _asset_size: Vec<String>,
   pub(super) _advertiser_id: Vec<i64>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for LinkListCall<'a, S> {}

impl<'a, S> LinkListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Links)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.links.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "startDateMin", "startDateMax", "searchText", "relationshipStatus", "promotionType", "pageToken", "maxResults", "linkType", "createDateMin", "createDateMax", "authorship", "assetSize", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(17 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._start_date_min.as_ref() {
            params.push("startDateMin", value);
        }
        if let Some(value) = self._start_date_max.as_ref() {
            params.push("startDateMax", value);
        }
        if let Some(value) = self._search_text.as_ref() {
            params.push("searchText", value);
        }
        if let Some(value) = self._relationship_status.as_ref() {
            params.push("relationshipStatus", value);
        }
        if self._promotion_type.len() > 0 {
            for f in self._promotion_type.iter() {
                params.push("promotionType", f);
            }
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if let Some(value) = self._link_type.as_ref() {
            params.push("linkType", value);
        }
        if let Some(value) = self._create_date_min.as_ref() {
            params.push("createDateMin", value);
        }
        if let Some(value) = self._create_date_max.as_ref() {
            params.push("createDateMax", value);
        }
        if let Some(value) = self._authorship.as_ref() {
            params.push("authorship", value);
        }
        if self._asset_size.len() > 0 {
            for f in self._asset_size.iter() {
                params.push("assetSize", f);
            }
        }
        if self._advertiser_id.len() > 0 {
            for f in self._advertiser_id.iter() {
                params.push("advertiserId", f.to_string());
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/links";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// The beginning of the start date range.
    ///
    /// Sets the *start date min* query property to the given value.
    pub fn start_date_min(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._start_date_min = Some(new_value.to_string());
        self
    }
    /// The end of the start date range.
    ///
    /// Sets the *start date max* query property to the given value.
    pub fn start_date_max(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._start_date_max = Some(new_value.to_string());
        self
    }
    /// Field for full text search across title and merchandising text, supports link id search.
    ///
    /// Sets the *search text* query property to the given value.
    pub fn search_text(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._search_text = Some(new_value.to_string());
        self
    }
    /// The status of the relationship.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// The promotion type.
    ///
    /// Append the given value to the *promotion type* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_promotion_type(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._promotion_type.push(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> LinkListCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// The type of the link.
    ///
    /// Sets the *link type* query property to the given value.
    pub fn link_type(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._link_type = Some(new_value.to_string());
        self
    }
    /// The beginning of the create date range.
    ///
    /// Sets the *create date min* query property to the given value.
    pub fn create_date_min(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._create_date_min = Some(new_value.to_string());
        self
    }
    /// The end of the create date range.
    ///
    /// Sets the *create date max* query property to the given value.
    pub fn create_date_max(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._create_date_max = Some(new_value.to_string());
        self
    }
    /// The role of the author of the link.
    ///
    /// Sets the *authorship* query property to the given value.
    pub fn authorship(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._authorship = Some(new_value.to_string());
        self
    }
    /// The size of the given asset.
    ///
    /// Append the given value to the *asset size* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_asset_size(mut self, new_value: &str) -> LinkListCall<'a, S> {
        self._asset_size.push(new_value.to_string());
        self
    }
    /// Limits the resulting links to the ones belonging to the listed advertisers.
    ///
    /// Append the given value to the *advertiser id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser_id(mut self, new_value: i64) -> LinkListCall<'a, S> {
        self._advertiser_id.push(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> LinkListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> LinkListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about a single advertiser if that the requesting advertiser/publisher has access to it. Only advertisers can look up publishers. Publishers can request information about themselves by omitting the publisherId query parameter.
///
/// A builder for the *get* method supported by a *publisher* resource.
/// It is not used directly, but through a [`PublisherMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.publishers().get("role", "roleId")
///              .publisher_id("est")
///              .doit().await;
/// # }
/// ```
pub struct PublisherGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _publisher_id: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for PublisherGetCall<'a, S> {}

impl<'a, S> PublisherGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Publisher)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.publishers.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "publisherId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(5 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._publisher_id.as_ref() {
            params.push("publisherId", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/publisher";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> PublisherGetCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> PublisherGetCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// The ID of the publisher to look up. Optional.
    ///
    /// Sets the *publisher id* query property to the given value.
    pub fn publisher_id(mut self, new_value: &str) -> PublisherGetCall<'a, S> {
        self._publisher_id = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PublisherGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> PublisherGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves data about all publishers that the requesting advertiser/publisher has access to.
///
/// A builder for the *list* method supported by a *publisher* resource.
/// It is not used directly, but through a [`PublisherMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.publishers().list("role", "roleId")
///              .relationship_status("sit")
///              .publisher_category("et")
///              .page_token("tempor")
///              .min_seven_day_epc(0.5423272308124675)
///              .min_payout_rank(-18)
///              .min_ninety_day_epc(0.728870793677753)
///              .max_results(45)
///              .doit().await;
/// # }
/// ```
pub struct PublisherListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _relationship_status: Option<String>,
   pub(super) _publisher_category: Option<String>,
   pub(super) _page_token: Option<String>,
   pub(super) _min_seven_day_epc: Option<f64>,
   pub(super) _min_payout_rank: Option<i32>,
   pub(super) _min_ninety_day_epc: Option<f64>,
   pub(super) _max_results: Option<u32>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for PublisherListCall<'a, S> {}

impl<'a, S> PublisherListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Publishers)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.publishers.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "relationshipStatus", "publisherCategory", "pageToken", "minSevenDayEpc", "minPayoutRank", "minNinetyDayEpc", "maxResults"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(11 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        if let Some(value) = self._relationship_status.as_ref() {
            params.push("relationshipStatus", value);
        }
        if let Some(value) = self._publisher_category.as_ref() {
            params.push("publisherCategory", value);
        }
        if let Some(value) = self._page_token.as_ref() {
            params.push("pageToken", value);
        }
        if let Some(value) = self._min_seven_day_epc.as_ref() {
            params.push("minSevenDayEpc", value.to_string());
        }
        if let Some(value) = self._min_payout_rank.as_ref() {
            params.push("minPayoutRank", value.to_string());
        }
        if let Some(value) = self._min_ninety_day_epc.as_ref() {
            params.push("minNinetyDayEpc", value.to_string());
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/publishers";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> PublisherListCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> PublisherListCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// Filters out all publishers for which do not have the given relationship status with the requesting publisher.
    ///
    /// Sets the *relationship status* query property to the given value.
    pub fn relationship_status(mut self, new_value: &str) -> PublisherListCall<'a, S> {
        self._relationship_status = Some(new_value.to_string());
        self
    }
    /// Caret(^) delimted list of publisher categories. Valid categories: (unclassified|community_and_content|shopping_and_promotion|loyalty_and_rewards|network|search_specialist|comparison_shopping|email). Filters out all publishers not in one of the given advertiser categories. Optional.
    ///
    /// Sets the *publisher category* query property to the given value.
    pub fn publisher_category(mut self, new_value: &str) -> PublisherListCall<'a, S> {
        self._publisher_category = Some(new_value.to_string());
        self
    }
    /// The value of 'nextPageToken' from the previous page. Optional.
    ///
    /// Sets the *page token* query property to the given value.
    pub fn page_token(mut self, new_value: &str) -> PublisherListCall<'a, S> {
        self._page_token = Some(new_value.to_string());
        self
    }
    /// Filters out all publishers that have a seven day EPC average lower than the given value (inclusive). Min value 0.0. Optional.
    ///
    /// Sets the *min seven day epc* query property to the given value.
    pub fn min_seven_day_epc(mut self, new_value: f64) -> PublisherListCall<'a, S> {
        self._min_seven_day_epc = Some(new_value);
        self
    }
    /// A value between 1 and 4, where 1 represents the quartile of publishers with the lowest ranks and 4 represents the quartile of publishers with the highest ranks. Filters out all publishers with a lower rank than the given quartile. For example if a 2 was given only publishers with a payout rank of 25 or higher would be included. Optional.
    ///
    /// Sets the *min payout rank* query property to the given value.
    pub fn min_payout_rank(mut self, new_value: i32) -> PublisherListCall<'a, S> {
        self._min_payout_rank = Some(new_value);
        self
    }
    /// Filters out all publishers that have a ninety day EPC average lower than the given value (inclusive). Min value: 0.0. Optional.
    ///
    /// Sets the *min ninety day epc* query property to the given value.
    pub fn min_ninety_day_epc(mut self, new_value: f64) -> PublisherListCall<'a, S> {
        self._min_ninety_day_epc = Some(new_value);
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to 20.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> PublisherListCall<'a, S> {
        self._max_results = Some(new_value);
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> PublisherListCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> PublisherListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Retrieves a report of the specified type.
///
/// A builder for the *get* method supported by a *report* resource.
/// It is not used directly, but through a [`ReportMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_gan1_beta1 as gan1_beta1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use gan1_beta1::{Gan, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = Gan::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.reports().get("role", "roleId", "reportType")
///              .status("dolores")
///              .start_index(32)
///              .start_date("et")
///              .add_publisher_id("sed")
///              .add_order_id("no")
///              .max_results(16)
///              .add_link_id("elitr")
///              .event_type("sed")
///              .end_date("no")
///              .calculate_totals(false)
///              .add_advertiser_id("At")
///              .doit().await;
/// # }
/// ```
pub struct ReportGetCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a Gan<S>,
   pub(super) _role: String,
   pub(super) _role_id: String,
   pub(super) _report_type: String,
   pub(super) _status: Option<String>,
   pub(super) _start_index: Option<u32>,
   pub(super) _start_date: Option<String>,
   pub(super) _publisher_id: Vec<String>,
   pub(super) _order_id: Vec<String>,
   pub(super) _max_results: Option<u32>,
   pub(super) _link_id: Vec<String>,
   pub(super) _event_type: Option<String>,
   pub(super) _end_date: Option<String>,
   pub(super) _calculate_totals: Option<bool>,
   pub(super) _advertiser_id: Vec<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for ReportGetCall<'a, S> {}

impl<'a, S> ReportGetCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Report)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "gan.reports.get",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "role", "roleId", "reportType", "status", "startIndex", "startDate", "publisherId", "orderId", "maxResults", "linkId", "eventType", "endDate", "calculateTotals", "advertiserId"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(16 + self._additional_params.len());
        params.push("role", self._role);
        params.push("roleId", self._role_id);
        params.push("reportType", self._report_type);
        if let Some(value) = self._status.as_ref() {
            params.push("status", value);
        }
        if let Some(value) = self._start_index.as_ref() {
            params.push("startIndex", value.to_string());
        }
        if let Some(value) = self._start_date.as_ref() {
            params.push("startDate", value);
        }
        if self._publisher_id.len() > 0 {
            for f in self._publisher_id.iter() {
                params.push("publisherId", f);
            }
        }
        if self._order_id.len() > 0 {
            for f in self._order_id.iter() {
                params.push("orderId", f);
            }
        }
        if let Some(value) = self._max_results.as_ref() {
            params.push("maxResults", value.to_string());
        }
        if self._link_id.len() > 0 {
            for f in self._link_id.iter() {
                params.push("linkId", f);
            }
        }
        if let Some(value) = self._event_type.as_ref() {
            params.push("eventType", value);
        }
        if let Some(value) = self._end_date.as_ref() {
            params.push("endDate", value);
        }
        if let Some(value) = self._calculate_totals.as_ref() {
            params.push("calculateTotals", value.to_string());
        }
        if self._advertiser_id.len() > 0 {
            for f in self._advertiser_id.iter() {
                params.push("advertiserId", f);
            }
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "{role}/{roleId}/report/{reportType}";
        
        match dlg.api_key() {
            Some(value) => params.push("key", value),
            None => {
                dlg.finished(false);
                return Err(client::Error::MissingAPIKey)
            }
        }

        for &(find_this, param_name) in [("{role}", "role"), ("{roleId}", "roleId"), ("{reportType}", "reportType")].iter() {
            url = params.uri_replacement(url, param_name, find_this, false);
        }
        {
            let to_remove = ["reportType", "roleId", "role"];
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


    /// The role of the requester. Valid values: 'advertisers' or 'publishers'.
    ///
    /// Sets the *role* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._role = new_value.to_string();
        self
    }
    /// The ID of the requesting advertiser or publisher.
    ///
    /// Sets the *role id* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn role_id(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._role_id = new_value.to_string();
        self
    }
    /// The type of report being requested. Valid values: 'order_delta'. Required.
    ///
    /// Sets the *report type* path property to the given value.
    ///
    /// Even though the property as already been set when instantiating this call,
    /// we provide this method for API completeness.
    pub fn report_type(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._report_type = new_value.to_string();
        self
    }
    /// Filters out all events that do not have the given status. Valid values: 'active', 'canceled', or 'invalid'. Optional.
    ///
    /// Sets the *status* query property to the given value.
    pub fn status(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._status = Some(new_value.to_string());
        self
    }
    /// Offset on which to return results when paging. Optional.
    ///
    /// Sets the *start index* query property to the given value.
    pub fn start_index(mut self, new_value: u32) -> ReportGetCall<'a, S> {
        self._start_index = Some(new_value);
        self
    }
    /// The start date (inclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day before endDate, if that is given, or yesterday. Optional.
    ///
    /// Sets the *start date* query property to the given value.
    pub fn start_date(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._start_date = Some(new_value.to_string());
        self
    }
    /// The IDs of the publishers to look up, if applicable.
    ///
    /// Append the given value to the *publisher id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_publisher_id(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._publisher_id.push(new_value.to_string());
        self
    }
    /// Filters to capture one of the given order IDs. Optional.
    ///
    /// Append the given value to the *order id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_order_id(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._order_id.push(new_value.to_string());
        self
    }
    /// Max number of items to return in this page. Optional. Defaults to return all results.
    ///
    /// Sets the *max results* query property to the given value.
    pub fn max_results(mut self, new_value: u32) -> ReportGetCall<'a, S> {
        self._max_results = Some(new_value);
        self
    }
    /// Filters to capture one of given link IDs. Optional.
    ///
    /// Append the given value to the *link id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_link_id(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._link_id.push(new_value.to_string());
        self
    }
    /// Filters out all events that are not of the given type. Valid values: 'action', 'transaction', or 'charge'. Optional.
    ///
    /// Sets the *event type* query property to the given value.
    pub fn event_type(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._event_type = Some(new_value.to_string());
        self
    }
    /// The end date (exclusive), in RFC 3339 format, for the report data to be returned. Defaults to one day after startDate, if that is given, or today. Optional.
    ///
    /// Sets the *end date* query property to the given value.
    pub fn end_date(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._end_date = Some(new_value.to_string());
        self
    }
    /// Whether or not to calculate totals rows. Optional.
    ///
    /// Sets the *calculate totals* query property to the given value.
    pub fn calculate_totals(mut self, new_value: bool) -> ReportGetCall<'a, S> {
        self._calculate_totals = Some(new_value);
        self
    }
    /// The IDs of the advertisers to look up, if applicable.
    ///
    /// Append the given value to the *advertiser id* query property.
    /// Each appended value will retain its original ordering and be '/'-separated in the URL's parameters.
    pub fn add_advertiser_id(mut self, new_value: &str) -> ReportGetCall<'a, S> {
        self._advertiser_id.push(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> ReportGetCall<'a, S> {
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
    /// * *alt* (query-string) - Data format for the response.
    /// * *fields* (query-string) - Selector specifying which fields to include in a partial response.
    /// * *key* (query-string) - API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    /// * *oauth_token* (query-string) - OAuth 2.0 token for the current user.
    /// * *prettyPrint* (query-boolean) - Returns response with indentations and line breaks.
    /// * *quotaUser* (query-string) - Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters. Overrides userIp if both are provided.
    /// * *userIp* (query-string) - IP address of the site where the request originates. Use this if you want to enforce per-user limits.
    pub fn param<T>(mut self, name: T, value: T) -> ReportGetCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


