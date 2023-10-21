use super::*;
/// A builder providing access to all methods supported on *autnum* resources.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.autnum();
/// # }
/// ```
pub struct AutnumMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for AutnumMethods<'a, S> {}

impl<'a, S> AutnumMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    /// 
    /// # Arguments
    ///
    /// * `autnumId` - No description provided.
    pub fn get(&self, autnum_id: &str) -> AutnumGetCall<'a, S> {
        AutnumGetCall {
            hub: self.hub,
            _autnum_id: autnum_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *domain* resources.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.domain();
/// # }
/// ```
pub struct DomainMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for DomainMethods<'a, S> {}

impl<'a, S> DomainMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Look up RDAP information for a domain by name.
    /// 
    /// # Arguments
    ///
    /// * `domainName` - Full domain name to look up. Example: "example.com"
    pub fn get(&self, domain_name: &str) -> DomainGetCall<'a, S> {
        DomainGetCall {
            hub: self.hub,
            _domain_name: domain_name.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *entity* resources.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.entity();
/// # }
/// ```
pub struct EntityMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for EntityMethods<'a, S> {}

impl<'a, S> EntityMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    /// 
    /// # Arguments
    ///
    /// * `entityId` - No description provided.
    pub fn get(&self, entity_id: &str) -> EntityGetCall<'a, S> {
        EntityGetCall {
            hub: self.hub,
            _entity_id: entity_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *ip* resources.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.ip();
/// # }
/// ```
pub struct IpMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for IpMethods<'a, S> {}

impl<'a, S> IpMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    /// 
    /// # Arguments
    ///
    /// * `ipId` - No description provided.
    /// * `ipId1` - No description provided.
    pub fn get(&self, ip_id: &str, ip_id1: &str) -> IpGetCall<'a, S> {
        IpGetCall {
            hub: self.hub,
            _ip_id: ip_id.to_string(),
            _ip_id1: ip_id1.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all methods supported on *nameserver* resources.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get(...)`
/// // to build up your call.
/// let rb = hub.nameserver();
/// # }
/// ```
pub struct NameserverMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for NameserverMethods<'a, S> {}

impl<'a, S> NameserverMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    /// 
    /// # Arguments
    ///
    /// * `nameserverId` - No description provided.
    pub fn get(&self, nameserver_id: &str) -> NameserverGetCall<'a, S> {
        NameserverGetCall {
            hub: self.hub,
            _nameserver_id: nameserver_id.to_string(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`DomainsRDAP`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_domainsrdap1 as domainsrdap1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use domainsrdap1::{DomainsRDAP, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = DomainsRDAP::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `get_domains(...)`, `get_entities(...)`, `get_help(...)`, `get_ip(...)` and `get_nameservers(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a DomainsRDAP<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    pub fn get_domains(&self) -> MethodGetDomainCall<'a, S> {
        MethodGetDomainCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    pub fn get_entities(&self) -> MethodGetEntityCall<'a, S> {
        MethodGetEntityCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Get help information for the RDAP API, including links to documentation.
    pub fn get_help(&self) -> MethodGetHelpCall<'a, S> {
        MethodGetHelpCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    pub fn get_ip(&self) -> MethodGetIpCall<'a, S> {
        MethodGetIpCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// The RDAP API recognizes this command from the RDAP specification but does not support it. The response is a formatted 501 error.
    pub fn get_nameservers(&self) -> MethodGetNameserverCall<'a, S> {
        MethodGetNameserverCall {
            hub: self.hub,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



