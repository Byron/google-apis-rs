use super::*;
/// A builder providing access to all methods supported on *cse* resources.
/// It is not used directly, but through the [`CustomSearchAPI`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_customsearch1 as customsearch1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use customsearch1::{CustomSearchAPI, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CustomSearchAPI::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `list(...)` and `siterestrict_list(...)`
/// // to build up your call.
/// let rb = hub.cse();
/// # }
/// ```
pub struct CseMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CustomSearchAPI<S>,
}

impl<'a, S> client::MethodsBuilder for CseMethods<'a, S> {}

impl<'a, S> CseMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the engine used for the search, and the search results. Uses a small set of url patterns.
    pub fn siterestrict_list(&self) -> CseSiterestrictListCall<'a, S> {
        CseSiterestrictListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
            _or_terms: Default::default(),
            _num: Default::default(),
            _lr: Default::default(),
            _low_range: Default::default(),
            _link_site: Default::default(),
            _img_type: Default::default(),
            _img_size: Default::default(),
            _img_dominant_color: Default::default(),
            _img_color_type: Default::default(),
            _hq: Default::default(),
            _hl: Default::default(),
            _high_range: Default::default(),
            _googlehost: Default::default(),
            _gl: Default::default(),
            _filter: Default::default(),
            _file_type: Default::default(),
            _exclude_terms: Default::default(),
            _exact_terms: Default::default(),
            _date_restrict: Default::default(),
            _cx: Default::default(),
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Returns metadata about the search performed, metadata about the engine used for the search, and the search results.
    pub fn list(&self) -> CseListCall<'a, S> {
        CseListCall {
            hub: self.hub,
            _start: Default::default(),
            _sort: Default::default(),
            _site_search_filter: Default::default(),
            _site_search: Default::default(),
            _search_type: Default::default(),
            _safe: Default::default(),
            _rights: Default::default(),
            _related_site: Default::default(),
            _q: Default::default(),
            _or_terms: Default::default(),
            _num: Default::default(),
            _lr: Default::default(),
            _low_range: Default::default(),
            _link_site: Default::default(),
            _img_type: Default::default(),
            _img_size: Default::default(),
            _img_dominant_color: Default::default(),
            _img_color_type: Default::default(),
            _hq: Default::default(),
            _hl: Default::default(),
            _high_range: Default::default(),
            _googlehost: Default::default(),
            _gl: Default::default(),
            _filter: Default::default(),
            _file_type: Default::default(),
            _exclude_terms: Default::default(),
            _exact_terms: Default::default(),
            _date_restrict: Default::default(),
            _cx: Default::default(),
            _cr: Default::default(),
            _c2coff: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



