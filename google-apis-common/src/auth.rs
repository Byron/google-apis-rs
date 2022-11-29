//! Authentication for Google API endpoints
//!
//! Allows users to provide custom authentication implementations to suite their needs.
//!
//! By default, [`GetToken`] is implemented for:
//! - [`Authenticator`] : An authenticator which supports a variety of authentication methods
//! - [`String`] : Plain oauth2 token in String format
//! - [`NoToken`] : No token, used for APIs which do not require a token
//!
//! # Usage
//! [`GetToken`] instances are designed to be used with the Hub constructor provided by the
//! generated APIs.
//!
//! If you intend to use the API libraries on client devices,
//! [`Authenticator`] supports a variety of client-side authentication methods,
//! and should be used to provide authentication.
//!
//! If you intend to use the API libraries server-side, with server-side client authentication,
//! use the [`oauth2`] crate and convert the resulting [`AccessToken`] to [`String`].
//!
//! If you intend to use APIs which do not require authentication, use [`NoToken`].
//!
//! If you have custom authentication requirements, you can implement [`GetToken`] manually.
//!
//! # Example
//! ```rust
//! use core::future::Future;
//! use core::pin::Pin;
//!
//! use google_apis_common::{GetToken, oauth2};
//!
//! use http::Uri;
//! use hyper::client::connect::Connection;
//! use tokio::io::{AsyncRead, AsyncWrite};
//! use tower_service::Service;
//! use oauth2::authenticator::Authenticator;
//!
//! #[derive(Clone)]
//! struct AuthenticatorWithRetry<S> {
//!     auth: Authenticator<S>,
//!     retries: usize,
//! }
//!
//! impl<S> GetToken for AuthenticatorWithRetry<S>
//! where
//!     S: Service<Uri> + Clone + Send + Sync + 'static,
//!     S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
//!     S::Future: Send + Unpin + 'static,
//!     S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
//! {
//!     fn get_token<'a>(
//!         &'a self,
//!         scopes: &'a [&str],
//!     ) -> Pin<Box<dyn Future<Output = Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
//!         Box::pin(async move {
//!             let mut auth_token = Ok(None);
//!             for _ in 0..=self.retries {
//!                 match self.auth.token(scopes).await {
//!                     Ok(token) => {
//!                         auth_token = Ok(token.token().map(|t| t.to_owned()));
//!                         break;
//!                     },
//!                     Err(e) => auth_token = Err(e.into()),
//!                 }
//!             }
//!             auth_token
//!         })
//!     }
//! }
//! ```
//! [`oauth2`]: https://docs.rs/oauth2/latest/oauth2/
//! [`AccessToken`]: https://docs.rs/oauth2/latest/oauth2/struct.AccessToken.html
//! [`Authenticator`]: yup_oauth2::authenticator::Authenticator
use std::future::Future;
use std::pin::Pin;

type GetTokenOutput<'a> = Pin<
    Box<
        dyn Future<Output = Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>>
            + Send
            + 'a,
    >,
>;

pub trait GetToken: GetTokenClone + Send + Sync {
    /// Called whenever an API call requires authentication via an oauth2 token.
    /// Returns `Ok(None)` if a token is not necessary - otherwise, returns an error
    /// indicating the reason why a token could not be produced.
    fn get_token<'a>(&'a self, _scopes: &'a [&str]) -> GetTokenOutput<'a>;
}

pub trait GetTokenClone {
    fn clone_box(&self) -> Box<dyn GetToken>;
}

impl<T> GetTokenClone for T
where
    T: 'static + GetToken + Clone,
{
    fn clone_box(&self) -> Box<dyn GetToken> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn GetToken> {
    fn clone(&self) -> Box<dyn GetToken> {
        self.clone_box()
    }
}

impl GetToken for String {
    fn get_token<'a>(&'a self, _scopes: &'a [&str]) -> GetTokenOutput<'a> {
        Box::pin(async move { Ok(Some(self.clone())) })
    }
}

/// In the event that the API endpoint does not require an oauth2 token, `NoToken` should be provided to the hub to avoid specifying an
/// authenticator.
#[derive(Default, Clone)]
pub struct NoToken;

impl GetToken for NoToken {
    fn get_token<'a>(&'a self, _scopes: &'a [&str]) -> GetTokenOutput<'a> {
        Box::pin(async move { Ok(None) })
    }
}

#[cfg(feature = "yup-oauth2")]
mod yup_oauth2_impl {
    use super::{GetToken, GetTokenOutput};

    use http::Uri;
    use hyper::client::connect::Connection;
    use tokio::io::{AsyncRead, AsyncWrite};
    use tower_service::Service;
    use yup_oauth2::authenticator::Authenticator;

    impl<S> GetToken for Authenticator<S>
    where
        S: Service<Uri> + Clone + Send + Sync + 'static,
        S::Response: Connection + AsyncRead + AsyncWrite + Send + Unpin + 'static,
        S::Future: Send + Unpin + 'static,
        S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        fn get_token<'a>(&'a self, scopes: &'a [&str]) -> GetTokenOutput<'a> {
            Box::pin(async move {
                self.token(scopes)
                    .await
                    .map(|t| t.token().map(|t| t.to_owned()))
                    .map_err(|e| e.into())
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dyn_get_token_is_send() {
        fn with_send(_x: impl Send) {}

        let mut gt = String::new();
        let dgt: &mut dyn GetToken = &mut gt;
        with_send(dgt);
    }
}
