use super::*;
/// A builder providing access to all methods supported on *searchanalytic* resources.
/// It is not used directly, but through the [`SearchConsole`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_searchconsole1 as searchconsole1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use searchconsole1::{SearchConsole, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SearchConsole::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `query(...)`
/// // to build up your call.
/// let rb = hub.searchanalytics();
/// # }
/// ```
pub struct SearchanalyticMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SearchConsole<S>,
}

impl<'a, S> client::MethodsBuilder for SearchanalyticMethods<'a, S> {}

impl<'a, S> SearchanalyticMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Query your data with filters and parameters that you define. Returns zero or more rows grouped by the row keys that you define. You must define a date range of one or more days. When date is one of the group by values, any days without data are omitted from the result list. If you need to know which days have data, issue a broad date range query grouped by date for any metric, and see which day rows are returned.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    /// * `siteUrl` - The site's URL, including protocol. For example: `http://www.example.com/`.
    pub fn query(&self, request: SearchAnalyticsQueryRequest, site_url: &str) -> SearchanalyticQueryCall<'a, S> {
        SearchanalyticQueryCall {
            hub: self.hub,
            _request: request,
            _site_url: site_url.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *sitemap* resources.
/// It is not used directly, but through the [`SearchConsole`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_searchconsole1 as searchconsole1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use searchconsole1::{SearchConsole, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SearchConsole::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `delete(...)`, `get(...)`, `list(...)` and `submit(...)`
/// // to build up your call.
/// let rb = hub.sitemaps();
/// # }
/// ```
pub struct SitemapMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SearchConsole<S>,
}

impl<'a, S> client::MethodsBuilder for SitemapMethods<'a, S> {}

impl<'a, S> SitemapMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Deletes a sitemap from the Sitemaps report. Does not stop Google from crawling this sitemap or the URLs that were previously crawled in the deleted sitemap.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The site's URL, including protocol. For example: `http://www.example.com/`.
    /// * `feedpath` - The URL of the actual sitemap. For example: `http://www.example.com/sitemap.xml`.
    pub fn delete(&self, site_url: &str, feedpath: &str) -> SitemapDeleteCall<'a, S> {
        SitemapDeleteCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _feedpath: feedpath.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Retrieves information about a specific sitemap.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The site's URL, including protocol. For example: `http://www.example.com/`.
    /// * `feedpath` - The URL of the actual sitemap. For example: `http://www.example.com/sitemap.xml`.
    pub fn get(&self, site_url: &str, feedpath: &str) -> SitemapGetCall<'a, S> {
        SitemapGetCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _feedpath: feedpath.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Lists the [sitemaps-entries](https://developers.google.com/webmaster-tools/v3/sitemaps) submitted for this site, or included in the sitemap index file (if `sitemapIndex` is specified in the request).
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The site's URL, including protocol. For example: `http://www.example.com/`.
    pub fn list(&self, site_url: &str) -> SitemapListCall<'a, S> {
        SitemapListCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _sitemap_index: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Submits a sitemap for a site.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The site's URL, including protocol. For example: `http://www.example.com/`.
    /// * `feedpath` - The URL of the actual sitemap. For example: `http://www.example.com/sitemap.xml`.
    pub fn submit(&self, site_url: &str, feedpath: &str) -> SitemapSubmitCall<'a, S> {
        SitemapSubmitCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _feedpath: feedpath.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *site* resources.
/// It is not used directly, but through the [`SearchConsole`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_searchconsole1 as searchconsole1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use searchconsole1::{SearchConsole, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SearchConsole::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `add(...)`, `delete(...)`, `get(...)` and `list(...)`
/// // to build up your call.
/// let rb = hub.sites();
/// # }
/// ```
pub struct SiteMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SearchConsole<S>,
}

impl<'a, S> client::MethodsBuilder for SiteMethods<'a, S> {}

impl<'a, S> SiteMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    ///  Adds a site to the set of the user's sites in Search Console.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The URL of the site to add.
    pub fn add(&self, site_url: &str) -> SiteAddCall<'a, S> {
        SiteAddCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///  Removes a site from the set of the user's Search Console sites.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The URI of the property as defined in Search Console. **Examples:** `http://www.example.com/` or `sc-domain:example.com`.
    pub fn delete(&self, site_url: &str) -> SiteDeleteCall<'a, S> {
        SiteDeleteCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///  Retrieves information about specific site.
    /// 
    /// # Arguments
    ///
    /// * `siteUrl` - The URI of the property as defined in Search Console. **Examples:** `http://www.example.com/` or `sc-domain:example.com`.
    pub fn get(&self, site_url: &str) -> SiteGetCall<'a, S> {
        SiteGetCall {
            hub: self.hub,
            _site_url: site_url.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    ///  Lists the user's Search Console sites.
    pub fn list(&self) -> SiteListCall<'a, S> {
        SiteListCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *urlInspection* resources.
/// It is not used directly, but through the [`SearchConsole`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_searchconsole1 as searchconsole1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use searchconsole1::{SearchConsole, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SearchConsole::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `index_inspect(...)`
/// // to build up your call.
/// let rb = hub.url_inspection();
/// # }
/// ```
pub struct UrlInspectionMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SearchConsole<S>,
}

impl<'a, S> client::MethodsBuilder for UrlInspectionMethods<'a, S> {}

impl<'a, S> UrlInspectionMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Index inspection.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn index_inspect(&self, request: InspectUrlIndexRequest) -> UrlInspectionIndexInspectCall<'a, S> {
        UrlInspectionIndexInspectCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
            _scopes: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *urlTestingTool* resources.
/// It is not used directly, but through the [`SearchConsole`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_searchconsole1 as searchconsole1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use searchconsole1::{SearchConsole, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = SearchConsole::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `mobile_friendly_test_run(...)`
/// // to build up your call.
/// let rb = hub.url_testing_tools();
/// # }
/// ```
pub struct UrlTestingToolMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a SearchConsole<S>,
}

impl<'a, S> client::MethodsBuilder for UrlTestingToolMethods<'a, S> {}

impl<'a, S> UrlTestingToolMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Runs Mobile-Friendly Test for a given URL.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn mobile_friendly_test_run(&self, request: RunMobileFriendlyTestRequest) -> UrlTestingToolMobileFriendlyTestRunCall<'a, S> {
        UrlTestingToolMobileFriendlyTestRunCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



