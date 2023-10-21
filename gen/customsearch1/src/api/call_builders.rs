use super::*;
/// Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns.
///
/// A builder for the *siterestrict.list* method supported by a *cse* resource.
/// It is not used directly, but through a [`CseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_customsearch1 as customsearch1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().siterestrict_list()
///              .start(23)
///              .sort("amet.")
///              .site_search_filter("ea")
///              .site_search("sadipscing")
///              .search_type("Lorem")
///              .safe("invidunt")
///              .rights("no")
///              .related_site("est")
///              .q("At")
///              .or_terms("sed")
///              .num(-98)
///              .lr("et")
///              .low_range("tempor")
///              .link_site("aliquyam")
///              .img_type("ipsum")
///              .img_size("et")
///              .img_dominant_color("sanctus")
///              .img_color_type("Lorem")
///              .hq("est")
///              .hl("sed")
///              .high_range("diam")
///              .googlehost("dolores")
///              .gl("dolores")
///              .filter("et")
///              .file_type("sed")
///              .exclude_terms("no")
///              .exact_terms("et")
///              .date_restrict("elitr")
///              .cx("sed")
///              .cr("no")
///              .c2coff("nonumy")
///              .doit().await;
/// # }
/// ```
pub struct CseSiterestrictListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a CustomSearchAPI<S>,
   pub(super) _start: Option<u32>,
   pub(super) _sort: Option<String>,
   pub(super) _site_search_filter: Option<String>,
   pub(super) _site_search: Option<String>,
   pub(super) _search_type: Option<String>,
   pub(super) _safe: Option<String>,
   pub(super) _rights: Option<String>,
   pub(super) _related_site: Option<String>,
   pub(super) _q: Option<String>,
   pub(super) _or_terms: Option<String>,
   pub(super) _num: Option<i32>,
   pub(super) _lr: Option<String>,
   pub(super) _low_range: Option<String>,
   pub(super) _link_site: Option<String>,
   pub(super) _img_type: Option<String>,
   pub(super) _img_size: Option<String>,
   pub(super) _img_dominant_color: Option<String>,
   pub(super) _img_color_type: Option<String>,
   pub(super) _hq: Option<String>,
   pub(super) _hl: Option<String>,
   pub(super) _high_range: Option<String>,
   pub(super) _googlehost: Option<String>,
   pub(super) _gl: Option<String>,
   pub(super) _filter: Option<String>,
   pub(super) _file_type: Option<String>,
   pub(super) _exclude_terms: Option<String>,
   pub(super) _exact_terms: Option<String>,
   pub(super) _date_restrict: Option<String>,
   pub(super) _cx: Option<String>,
   pub(super) _cr: Option<String>,
   pub(super) _c2coff: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CseSiterestrictListCall<'a, S> {}

impl<'a, S> CseSiterestrictListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "search.cse.siterestrict.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(33 + self._additional_params.len());
        if let Some(value) = self._start.as_ref() {
            params.push("start", value.to_string());
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if let Some(value) = self._site_search_filter.as_ref() {
            params.push("siteSearchFilter", value);
        }
        if let Some(value) = self._site_search.as_ref() {
            params.push("siteSearch", value);
        }
        if let Some(value) = self._search_type.as_ref() {
            params.push("searchType", value);
        }
        if let Some(value) = self._safe.as_ref() {
            params.push("safe", value);
        }
        if let Some(value) = self._rights.as_ref() {
            params.push("rights", value);
        }
        if let Some(value) = self._related_site.as_ref() {
            params.push("relatedSite", value);
        }
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._or_terms.as_ref() {
            params.push("orTerms", value);
        }
        if let Some(value) = self._num.as_ref() {
            params.push("num", value.to_string());
        }
        if let Some(value) = self._lr.as_ref() {
            params.push("lr", value);
        }
        if let Some(value) = self._low_range.as_ref() {
            params.push("lowRange", value);
        }
        if let Some(value) = self._link_site.as_ref() {
            params.push("linkSite", value);
        }
        if let Some(value) = self._img_type.as_ref() {
            params.push("imgType", value);
        }
        if let Some(value) = self._img_size.as_ref() {
            params.push("imgSize", value);
        }
        if let Some(value) = self._img_dominant_color.as_ref() {
            params.push("imgDominantColor", value);
        }
        if let Some(value) = self._img_color_type.as_ref() {
            params.push("imgColorType", value);
        }
        if let Some(value) = self._hq.as_ref() {
            params.push("hq", value);
        }
        if let Some(value) = self._hl.as_ref() {
            params.push("hl", value);
        }
        if let Some(value) = self._high_range.as_ref() {
            params.push("highRange", value);
        }
        if let Some(value) = self._googlehost.as_ref() {
            params.push("googlehost", value);
        }
        if let Some(value) = self._gl.as_ref() {
            params.push("gl", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._file_type.as_ref() {
            params.push("fileType", value);
        }
        if let Some(value) = self._exclude_terms.as_ref() {
            params.push("excludeTerms", value);
        }
        if let Some(value) = self._exact_terms.as_ref() {
            params.push("exactTerms", value);
        }
        if let Some(value) = self._date_restrict.as_ref() {
            params.push("dateRestrict", value);
        }
        if let Some(value) = self._cx.as_ref() {
            params.push("cx", value);
        }
        if let Some(value) = self._cr.as_ref() {
            params.push("cr", value);
        }
        if let Some(value) = self._c2coff.as_ref() {
            params.push("c2coff", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "customsearch/v1/siterestrict";
        
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


    /// The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseSiterestrictListCall<'a, S> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute).
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `"e"`: exclude * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are: * `"active"`: Enables SafeSearch filtering. * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the specified URL.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return. * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseSiterestrictListCall<'a, S> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `"lang_ar"`: Arabic * `"lang_bg"`: Bulgarian * `"lang_ca"`: Catalan * `"lang_cs"`: Czech * `"lang_da"`: Danish * `"lang_de"`: German * `"lang_el"`: Greek * `"lang_en"`: English * `"lang_es"`: Spanish * `"lang_et"`: Estonian * `"lang_fi"`: Finnish * `"lang_fr"`: French * `"lang_hr"`: Croatian * `"lang_hu"`: Hungarian * `"lang_id"`: Indonesian * `"lang_is"`: Icelandic * `"lang_it"`: Italian * `"lang_iw"`: Hebrew * `"lang_ja"`: Japanese * `"lang_ko"`: Korean * `"lang_lt"`: Lithuanian * `"lang_lv"`: Latvian * `"lang_nl"`: Dutch * `"lang_no"`: Norwegian * `"lang_pl"`: Polish * `"lang_pt"`: Portuguese * `"lang_ro"`: Romanian * `"lang_ru"`: Russian * `"lang_sk"`: Slovak * `"lang_sl"`: Slovenian * `"lang_sr"`: Serbian * `"lang_sv"`: Swedish * `"lang_tr"`: Turkish * `"lang_zh-CN"`: Chinese (Simplified) * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are: * `"clipart"` * `"face"` * `"lineart"` * `"stock"` * `"photo"` * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are: * `"huge"` * `"icon"` * `"large"` * `"medium"` * `"small"` * `"xlarge"` * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are: * `"black"` * `"blue"` * `"brown"` * `"gray"` * `"green"` * `"orange"` * `"pink"` * `"purple"` * `"red"` * `"teal"` * `"white"` * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `"color"` * `"gray"` * `"mono"`: black and white * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The Programmable Search Engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseSiterestrictListCall<'a, S> {
        self._c2coff = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CseSiterestrictListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseSiterestrictListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


/// Returns metadata about the search performed, metadata about the engine used for the search, and the search results.
///
/// A builder for the *list* method supported by a *cse* resource.
/// It is not used directly, but through a [`CseMethods`] instance.
///
/// # Example
///
/// Instantiate a resource method builder
///
/// ```test_harness,no_run
/// # extern crate hyper;
/// # extern crate hyper_rustls;
/// # extern crate google_customsearch1 as customsearch1;
/// # async fn dox() {
/// # use std::default::Default;
/// # use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// # let secret: oauth2::ApplicationSecret = Default::default();
/// # let auth = oauth2::InstalledFlowAuthenticator::builder(
/// #         secret,
/// #         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     ).build().await.unwrap();
/// # let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // You can configure optional parameters by calling the respective setters at will, and
/// // execute the final call using `doit()`.
/// // Values shown here are possibly random and not representative !
/// let result = hub.cse().list()
///              .start(24)
///              .sort("sadipscing")
///              .site_search_filter("aliquyam")
///              .site_search("dolores")
///              .search_type("sadipscing")
///              .safe("erat")
///              .rights("aliquyam")
///              .related_site("amet")
///              .q("est")
///              .or_terms("et")
///              .num(-10)
///              .lr("consetetur")
///              .low_range("consetetur")
///              .link_site("Stet")
///              .img_type("est")
///              .img_size("aliquyam")
///              .img_dominant_color("elitr")
///              .img_color_type("duo")
///              .hq("diam")
///              .hl("est")
///              .high_range("sit")
///              .googlehost("sed")
///              .gl("eos")
///              .filter("Lorem")
///              .file_type("ea")
///              .exclude_terms("Stet")
///              .exact_terms("dolores")
///              .date_restrict("eos")
///              .cx("et")
///              .cr("sea")
///              .c2coff("et")
///              .doit().await;
/// # }
/// ```
pub struct CseListCall<'a, S>
    where S: 'a {

   pub(super) hub: &'a CustomSearchAPI<S>,
   pub(super) _start: Option<u32>,
   pub(super) _sort: Option<String>,
   pub(super) _site_search_filter: Option<String>,
   pub(super) _site_search: Option<String>,
   pub(super) _search_type: Option<String>,
   pub(super) _safe: Option<String>,
   pub(super) _rights: Option<String>,
   pub(super) _related_site: Option<String>,
   pub(super) _q: Option<String>,
   pub(super) _or_terms: Option<String>,
   pub(super) _num: Option<i32>,
   pub(super) _lr: Option<String>,
   pub(super) _low_range: Option<String>,
   pub(super) _link_site: Option<String>,
   pub(super) _img_type: Option<String>,
   pub(super) _img_size: Option<String>,
   pub(super) _img_dominant_color: Option<String>,
   pub(super) _img_color_type: Option<String>,
   pub(super) _hq: Option<String>,
   pub(super) _hl: Option<String>,
   pub(super) _high_range: Option<String>,
   pub(super) _googlehost: Option<String>,
   pub(super) _gl: Option<String>,
   pub(super) _filter: Option<String>,
   pub(super) _file_type: Option<String>,
   pub(super) _exclude_terms: Option<String>,
   pub(super) _exact_terms: Option<String>,
   pub(super) _date_restrict: Option<String>,
   pub(super) _cx: Option<String>,
   pub(super) _cr: Option<String>,
   pub(super) _c2coff: Option<String>,
   pub(super) _delegate: Option<&'a mut dyn client::Delegate>,
   pub(super) _additional_params: HashMap<String, String>,
}

impl<'a, S> client::CallBuilder for CseListCall<'a, S> {}

impl<'a, S> CseListCall<'a, S>
where
    S: tower_service::Service<http::Uri> + Clone + Send + Sync + 'static,
    S::Response: hyper::client::connect::Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S::Future: Send + Unpin + 'static,
    S::Error: Into<Box<dyn StdError + Send + Sync>>,
{


    /// Perform the operation you have build so far.
    pub async fn doit(mut self) -> client::Result<(hyper::Response<hyper::body::Body>, Search)> {
        use std::io::{Read, Seek};
        use hyper::header::{CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, USER_AGENT, LOCATION};
        use client::{ToParts, url::Params};
        use std::borrow::Cow;

        let mut dd = client::DefaultDelegate;
        let mut dlg: &mut dyn client::Delegate = self._delegate.unwrap_or(&mut dd);
        dlg.begin(client::MethodInfo { id: "search.cse.list",
                               http_method: hyper::Method::GET });

        for &field in ["alt", "start", "sort", "siteSearchFilter", "siteSearch", "searchType", "safe", "rights", "relatedSite", "q", "orTerms", "num", "lr", "lowRange", "linkSite", "imgType", "imgSize", "imgDominantColor", "imgColorType", "hq", "hl", "highRange", "googlehost", "gl", "filter", "fileType", "excludeTerms", "exactTerms", "dateRestrict", "cx", "cr", "c2coff"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(client::Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(33 + self._additional_params.len());
        if let Some(value) = self._start.as_ref() {
            params.push("start", value.to_string());
        }
        if let Some(value) = self._sort.as_ref() {
            params.push("sort", value);
        }
        if let Some(value) = self._site_search_filter.as_ref() {
            params.push("siteSearchFilter", value);
        }
        if let Some(value) = self._site_search.as_ref() {
            params.push("siteSearch", value);
        }
        if let Some(value) = self._search_type.as_ref() {
            params.push("searchType", value);
        }
        if let Some(value) = self._safe.as_ref() {
            params.push("safe", value);
        }
        if let Some(value) = self._rights.as_ref() {
            params.push("rights", value);
        }
        if let Some(value) = self._related_site.as_ref() {
            params.push("relatedSite", value);
        }
        if let Some(value) = self._q.as_ref() {
            params.push("q", value);
        }
        if let Some(value) = self._or_terms.as_ref() {
            params.push("orTerms", value);
        }
        if let Some(value) = self._num.as_ref() {
            params.push("num", value.to_string());
        }
        if let Some(value) = self._lr.as_ref() {
            params.push("lr", value);
        }
        if let Some(value) = self._low_range.as_ref() {
            params.push("lowRange", value);
        }
        if let Some(value) = self._link_site.as_ref() {
            params.push("linkSite", value);
        }
        if let Some(value) = self._img_type.as_ref() {
            params.push("imgType", value);
        }
        if let Some(value) = self._img_size.as_ref() {
            params.push("imgSize", value);
        }
        if let Some(value) = self._img_dominant_color.as_ref() {
            params.push("imgDominantColor", value);
        }
        if let Some(value) = self._img_color_type.as_ref() {
            params.push("imgColorType", value);
        }
        if let Some(value) = self._hq.as_ref() {
            params.push("hq", value);
        }
        if let Some(value) = self._hl.as_ref() {
            params.push("hl", value);
        }
        if let Some(value) = self._high_range.as_ref() {
            params.push("highRange", value);
        }
        if let Some(value) = self._googlehost.as_ref() {
            params.push("googlehost", value);
        }
        if let Some(value) = self._gl.as_ref() {
            params.push("gl", value);
        }
        if let Some(value) = self._filter.as_ref() {
            params.push("filter", value);
        }
        if let Some(value) = self._file_type.as_ref() {
            params.push("fileType", value);
        }
        if let Some(value) = self._exclude_terms.as_ref() {
            params.push("excludeTerms", value);
        }
        if let Some(value) = self._exact_terms.as_ref() {
            params.push("exactTerms", value);
        }
        if let Some(value) = self._date_restrict.as_ref() {
            params.push("dateRestrict", value);
        }
        if let Some(value) = self._cx.as_ref() {
            params.push("cx", value);
        }
        if let Some(value) = self._cr.as_ref() {
            params.push("cr", value);
        }
        if let Some(value) = self._c2coff.as_ref() {
            params.push("c2coff", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let mut url = self.hub._base_url.clone() + "customsearch/v1";
        
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


    /// The index of the first result to return. The default number of results per page is 10, so `&start=11` would start at the top of the second page of results. **Note**: The JSON API will never return more than 100 results, even if more than 100 documents match the query, so setting the sum of `start + num` to a number greater than 100 will produce an error. Also note that the maximum value for `num` is 10.
    ///
    /// Sets the *start* query property to the given value.
    pub fn start(mut self, new_value: u32) -> CseListCall<'a, S> {
        self._start = Some(new_value);
        self
    }
    /// The sort expression to apply to the results. The sort parameter specifies that the results be sorted according to the specified expression i.e. sort by date. [Example: sort=date](https://developers.google.com/custom-search/docs/structured_search#sort-by-attribute).
    ///
    /// Sets the *sort* query property to the given value.
    pub fn sort(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._sort = Some(new_value.to_string());
        self
    }
    /// Controls whether to include or exclude results from the site named in the `siteSearch` parameter. Acceptable values are: * `"e"`: exclude * `"i"`: include
    ///
    /// Sets the *site search filter* query property to the given value.
    pub fn site_search_filter(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._site_search_filter = Some(new_value.to_string());
        self
    }
    /// Specifies a given site which should always be included or excluded from results (see `siteSearchFilter` parameter, below).
    ///
    /// Sets the *site search* query property to the given value.
    pub fn site_search(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._site_search = Some(new_value.to_string());
        self
    }
    /// Specifies the search type: `image`. If unspecified, results are limited to webpages. Acceptable values are: * `"image"`: custom image search.
    ///
    /// Sets the *search type* query property to the given value.
    pub fn search_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._search_type = Some(new_value.to_string());
        self
    }
    /// Search safety level. Acceptable values are: * `"active"`: Enables SafeSearch filtering. * `"off"`: Disables SafeSearch filtering. (default)
    ///
    /// Sets the *safe* query property to the given value.
    pub fn safe(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._safe = Some(new_value.to_string());
        self
    }
    /// Filters based on licensing. Supported values include: `cc_publicdomain`, `cc_attribute`, `cc_sharealike`, `cc_noncommercial`, `cc_nonderived` and combinations of these. See [typical combinations](https://wiki.creativecommons.org/wiki/CC_Search_integration).
    ///
    /// Sets the *rights* query property to the given value.
    pub fn rights(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._rights = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should be pages that are related to the specified URL.
    ///
    /// Sets the *related site* query property to the given value.
    pub fn related_site(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._related_site = Some(new_value.to_string());
        self
    }
    /// Query
    ///
    /// Sets the *q* query property to the given value.
    pub fn q(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._q = Some(new_value.to_string());
        self
    }
    /// Provides additional search terms to check for in a document, where each document in the search results must contain at least one of the additional search terms.
    ///
    /// Sets the *or terms* query property to the given value.
    pub fn or_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._or_terms = Some(new_value.to_string());
        self
    }
    /// Number of search results to return. * Valid values are integers between 1 and 10, inclusive.
    ///
    /// Sets the *num* query property to the given value.
    pub fn num(mut self, new_value: i32) -> CseListCall<'a, S> {
        self._num = Some(new_value);
        self
    }
    /// Restricts the search to documents written in a particular language (e.g., `lr=lang_ja`). Acceptable values are: * `"lang_ar"`: Arabic * `"lang_bg"`: Bulgarian * `"lang_ca"`: Catalan * `"lang_cs"`: Czech * `"lang_da"`: Danish * `"lang_de"`: German * `"lang_el"`: Greek * `"lang_en"`: English * `"lang_es"`: Spanish * `"lang_et"`: Estonian * `"lang_fi"`: Finnish * `"lang_fr"`: French * `"lang_hr"`: Croatian * `"lang_hu"`: Hungarian * `"lang_id"`: Indonesian * `"lang_is"`: Icelandic * `"lang_it"`: Italian * `"lang_iw"`: Hebrew * `"lang_ja"`: Japanese * `"lang_ko"`: Korean * `"lang_lt"`: Lithuanian * `"lang_lv"`: Latvian * `"lang_nl"`: Dutch * `"lang_no"`: Norwegian * `"lang_pl"`: Polish * `"lang_pt"`: Portuguese * `"lang_ro"`: Romanian * `"lang_ru"`: Russian * `"lang_sk"`: Slovak * `"lang_sl"`: Slovenian * `"lang_sr"`: Serbian * `"lang_sv"`: Swedish * `"lang_tr"`: Turkish * `"lang_zh-CN"`: Chinese (Simplified) * `"lang_zh-TW"`: Chinese (Traditional)
    ///
    /// Sets the *lr* query property to the given value.
    pub fn lr(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._lr = Some(new_value.to_string());
        self
    }
    /// Specifies the starting value for a search range. Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *low range* query property to the given value.
    pub fn low_range(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._low_range = Some(new_value.to_string());
        self
    }
    /// Specifies that all search results should contain a link to a particular URL.
    ///
    /// Sets the *link site* query property to the given value.
    pub fn link_site(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._link_site = Some(new_value.to_string());
        self
    }
    /// Returns images of a type. Acceptable values are: * `"clipart"` * `"face"` * `"lineart"` * `"stock"` * `"photo"` * `"animated"`
    ///
    /// Sets the *img type* query property to the given value.
    pub fn img_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_type = Some(new_value.to_string());
        self
    }
    /// Returns images of a specified size. Acceptable values are: * `"huge"` * `"icon"` * `"large"` * `"medium"` * `"small"` * `"xlarge"` * `"xxlarge"`
    ///
    /// Sets the *img size* query property to the given value.
    pub fn img_size(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_size = Some(new_value.to_string());
        self
    }
    /// Returns images of a specific dominant color. Acceptable values are: * `"black"` * `"blue"` * `"brown"` * `"gray"` * `"green"` * `"orange"` * `"pink"` * `"purple"` * `"red"` * `"teal"` * `"white"` * `"yellow"`
    ///
    /// Sets the *img dominant color* query property to the given value.
    pub fn img_dominant_color(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_dominant_color = Some(new_value.to_string());
        self
    }
    /// Returns black and white, grayscale, transparent, or color images. Acceptable values are: * `"color"` * `"gray"` * `"mono"`: black and white * `"trans"`: transparent background
    ///
    /// Sets the *img color type* query property to the given value.
    pub fn img_color_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._img_color_type = Some(new_value.to_string());
        self
    }
    /// Appends the specified query terms to the query, as if they were combined with a logical AND operator.
    ///
    /// Sets the *hq* query property to the given value.
    pub fn hq(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._hq = Some(new_value.to_string());
        self
    }
    /// Sets the user interface language. * Explicitly setting this parameter improves the performance and the quality of your search results. * See the [Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#wsInterfaceLanguages) section of [Internationalizing Queries and Results Presentation](https://developers.google.com/custom-search/docs/json_api_reference#wsInternationalizing) for more information, and [Supported Interface Languages](https://developers.google.com/custom-search/docs/json_api_reference#interfaceLanguages) for a list of supported languages.
    ///
    /// Sets the *hl* query property to the given value.
    pub fn hl(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._hl = Some(new_value.to_string());
        self
    }
    /// Specifies the ending value for a search range. * Use `lowRange` and `highRange` to append an inclusive search range of `lowRange...highRange` to the query.
    ///
    /// Sets the *high range* query property to the given value.
    pub fn high_range(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._high_range = Some(new_value.to_string());
        self
    }
    /// **Deprecated**. Use the `gl` parameter for a similar effect. The local Google domain (for example, google.com, google.de, or google.fr) to use to perform the search.
    ///
    /// Sets the *googlehost* query property to the given value.
    pub fn googlehost(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._googlehost = Some(new_value.to_string());
        self
    }
    /// Geolocation of end user. * The `gl` parameter value is a two-letter country code. The `gl` parameter boosts search results whose country of origin matches the parameter value. See the [Country Codes](https://developers.google.com/custom-search/docs/json_api_reference#countryCodes) page for a list of valid values. * Specifying a `gl` parameter value should lead to more relevant results. This is particularly true for international customers and, even more specifically, for customers in English- speaking countries other than the United States.
    ///
    /// Sets the *gl* query property to the given value.
    pub fn gl(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._gl = Some(new_value.to_string());
        self
    }
    /// Controls turning on or off the duplicate content filter. * See [Automatic Filtering](https://developers.google.com/custom-search/docs/json_api_reference#automaticFiltering) for more information about Google's search results filters. Note that host crowding filtering applies only to multi-site searches. * By default, Google applies filtering to all search results to improve the quality of those results. Acceptable values are: * `0`: Turns off duplicate content filter. * `1`: Turns on duplicate content filter.
    ///
    /// Sets the *filter* query property to the given value.
    pub fn filter(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._filter = Some(new_value.to_string());
        self
    }
    /// Restricts results to files of a specified extension. A list of file types indexable by Google can be found in Search Console [Help Center](https://support.google.com/webmasters/answer/35287).
    ///
    /// Sets the *file type* query property to the given value.
    pub fn file_type(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._file_type = Some(new_value.to_string());
        self
    }
    /// Identifies a word or phrase that should not appear in any documents in the search results.
    ///
    /// Sets the *exclude terms* query property to the given value.
    pub fn exclude_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._exclude_terms = Some(new_value.to_string());
        self
    }
    /// Identifies a phrase that all documents in the search results must contain.
    ///
    /// Sets the *exact terms* query property to the given value.
    pub fn exact_terms(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._exact_terms = Some(new_value.to_string());
        self
    }
    /// Restricts results to URLs based on date. Supported values include: * `d[number]`: requests results from the specified number of past days. * `w[number]`: requests results from the specified number of past weeks. * `m[number]`: requests results from the specified number of past months. * `y[number]`: requests results from the specified number of past years.
    ///
    /// Sets the *date restrict* query property to the given value.
    pub fn date_restrict(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._date_restrict = Some(new_value.to_string());
        self
    }
    /// The Programmable Search Engine ID to use for this request.
    ///
    /// Sets the *cx* query property to the given value.
    pub fn cx(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._cx = Some(new_value.to_string());
        self
    }
    /// Restricts search results to documents originating in a particular country. You may use [Boolean operators](https://developers.google.com/custom-search/docs/json_api_reference#booleanOperators) in the cr parameter's value. Google Search determines the country of a document by analyzing: * the top-level domain (TLD) of the document's URL * the geographic location of the Web server's IP address See the [Country Parameter Values](https://developers.google.com/custom-search/docs/json_api_reference#countryCollections) page for a list of valid values for this parameter.
    ///
    /// Sets the *cr* query property to the given value.
    pub fn cr(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._cr = Some(new_value.to_string());
        self
    }
    /// Enables or disables [Simplified and Traditional Chinese Search](https://developers.google.com/custom-search/docs/json_api_reference#chineseSearch). The default value for this parameter is 0 (zero), meaning that the feature is enabled. Supported values are: * `1`: Disabled * `0`: Enabled (default)
    ///
    /// Sets the *c2coff* query property to the given value.
    pub fn c2coff(mut self, new_value: &str) -> CseListCall<'a, S> {
        self._c2coff = Some(new_value.to_string());
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
    pub fn delegate(mut self, new_value: &'a mut dyn client::Delegate) -> CseListCall<'a, S> {
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
    pub fn param<T>(mut self, name: T, value: T) -> CseListCall<'a, S>
                                                        where T: AsRef<str> {
        self._additional_params.insert(name.as_ref().to_string(), value.as_ref().to_string());
        self
    }

}


