use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// View and manage Search Console data for your verified sites
    Webmaster,

    /// View Search Console data for your verified sites
    WebmasterReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::Webmaster => "https://www.googleapis.com/auth/webmasters",
            Scope::WebmasterReadonly => "https://www.googleapis.com/auth/webmasters.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::WebmasterReadonly
    }
}

