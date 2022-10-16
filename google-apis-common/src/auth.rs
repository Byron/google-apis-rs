use std::future::Future;
use std::pin::Pin;

// TODO: Simplify this to Option<String>
type TokenResult = Option<String>;

pub trait GetToken: GetTokenClone + Send + Sync {
    /// Called whenever there is the need for an oauth token after
    /// the official authenticator implementation didn't provide one, for some reason.
    /// If this method returns None as well, the underlying operation will fail
    fn get_token<'a>(
        &'a self,
        _scopes: &'a [&str],
    ) -> Pin<Box<dyn Future<Output = TokenResult> + Send + 'a>> {
        Box::pin(async move { None })
    }
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
    fn get_token<'a>(
        &'a self,
        _scopes: &'a [&str],
    ) -> Pin<Box<dyn Future<Output = TokenResult> + Send + 'a>> {
        Box::pin(async move { Some(self.clone()) })
    }
}

/// In the event that the API endpoint does not require an oauth2 token, `NoToken` should be provided to the hub to avoid specifying an
/// authenticator.
#[derive(Default, Clone)]
pub struct NoToken;

impl GetToken for NoToken {}

// TODO: Make this optional
// #[cfg(feature = "yup-oauth2")]
mod yup_oauth2_impl {
    use core::future::Future;
    use core::pin::Pin;

    use super::{GetToken, TokenResult};

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
        fn get_token<'a>(
            &'a self,
            scopes: &'a [&str],
        ) -> Pin<Box<dyn Future<Output = TokenResult> + Send + 'a>> {
            Box::pin(async move { self.token(scopes).await.ok().map(|t| t.as_str().to_owned()) })
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
