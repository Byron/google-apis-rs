use super::*;
/// A builder providing access to all free methods, which are not associated with a particular resource.
/// It is not used directly, but through the [`CloudSecurityToken`] hub.
///
/// # Example
///
/// Instantiate a resource builder
///
/// ```test_harness,no_run
/// extern crate hyper;
/// extern crate hyper_rustls;
/// extern crate google_sts1 as sts1;
/// 
/// # async fn dox() {
/// use std::default::Default;
/// use sts1::{CloudSecurityToken, oauth2, hyper, hyper_rustls, chrono, FieldMask};
/// 
/// let secret: oauth2::ApplicationSecret = Default::default();
/// let auth = oauth2::InstalledFlowAuthenticator::builder(
///         secret,
///         oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     ).build().await.unwrap();
/// let mut hub = CloudSecurityToken::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_or_http().enable_http1().build()), auth);
/// // Usually you wouldn't bind this to a variable, but keep calling *CallBuilders*
/// // like `introspect(...)`, `oauthtoken(...)` and `token(...)`
/// // to build up your call.
/// let rb = hub.methods();
/// # }
/// ```
pub struct MethodMethods<'a, S>
    where S: 'a {

   pub(super) hub: &'a CloudSecurityToken<S>,
}

impl<'a, S> client::MethodsBuilder for MethodMethods<'a, S> {}

impl<'a, S> MethodMethods<'a, S> {
    
    /// Create a builder to help you perform the following task:
    ///
    /// Gets information about a Google OAuth 2.0 access token issued by the Google Cloud [Security Token Service API](https://cloud.google.com/iam/docs/reference/sts/rest).
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn introspect(&self, request: GoogleIdentityStsV1IntrospectTokenRequest) -> MethodIntrospectCall<'a, S> {
        MethodIntrospectCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exchanges a credential that represents the resource owner's authorization for a Google-generated [OAuth 2.0 access token] (https://www.rfc-editor.org/rfc/rfc6749#section-5) or [refreshes an accesstoken] (https://www.rfc-editor.org/rfc/rfc6749#section-6) following [the OAuth 2.0 authorization framework] (https://tools.ietf.org/html/rfc8693) The credential can be one of the following: - An authorization code issued by the workforce identity federation authorization endpoint - A [refresh token](https://www.rfc-editor.org/rfc/rfc6749#section-10.4) issued by this endpoint This endpoint is only meant to be called by the Google Cloud CLI. Also note that this API only accepts the authorization code issued for workforce pools.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn oauthtoken(&self, request: GoogleIdentityStsV1ExchangeOauthTokenRequest) -> MethodOauthtokenCall<'a, S> {
        MethodOauthtokenCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
    
    /// Create a builder to help you perform the following task:
    ///
    /// Exchanges a credential for a Google OAuth 2.0 access token. The token asserts an external identity within an identity pool, or it applies a Credential Access Boundary to a Google access token. Note that workforce pools do not support Credential Access Boundaries. When you call this method, do not send the `Authorization` HTTP header in the request. This method does not require the `Authorization` header, and using the header can cause the request to fail.
    /// 
    /// # Arguments
    ///
    /// * `request` - No description provided.
    pub fn token(&self, request: GoogleIdentityStsV1ExchangeTokenRequest) -> MethodTokenCall<'a, S> {
        MethodTokenCall {
            hub: self.hub,
            _request: request,
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}



