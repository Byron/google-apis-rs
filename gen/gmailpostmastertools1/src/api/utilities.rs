use super::*;
/// Identifies the an OAuth2 authorization scope.
/// A scope is needed when requesting an
/// [authorization token](https://developers.google.com/youtube/v3/guides/authentication).
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Scope {
    /// See email traffic metrics for the domains you have registered in Gmail Postmaster Tools
    PostmasterReadonly,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match *self {
            Scope::PostmasterReadonly => "https://www.googleapis.com/auth/postmaster.readonly",
        }
    }
}

impl Default for Scope {
    fn default() -> Scope {
        Scope::PostmasterReadonly
    }
}

